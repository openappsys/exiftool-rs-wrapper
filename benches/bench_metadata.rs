//! 元数据读取性能基准测试
//!
//! 测试不同场景下的元数据读取性能

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use exiftool_rs_wrapper::{ExifTool, ExifToolPool};
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

/// 创建测试用的临时图片文件
fn create_test_file_with_size(size_kb: usize) -> PathBuf {
    // 创建一个简单的测试文件
    let mut temp_file = NamedTempFile::new().unwrap();

    // 写入基本的 JPEG 文件头（如果文件较小）
    // 写入一些基本的元数据
    let data = vec![0xFFu8; size_kb * 1024];
    temp_file.write_all(&data).unwrap();

    let path = temp_file.path().to_path_buf();
    // 保持文件不被删除
    std::mem::forget(temp_file);

    path
}

/// 创建指定数量的测试文件
fn create_test_files(count: usize) -> Vec<PathBuf> {
    let mut paths = Vec::with_capacity(count);

    for i in 0..count {
        // 文件大小逐渐增加：1KB, 10KB, 100KB
        let size_kb = match i % 3 {
            0 => 1,
            1 => 10,
            _ => 100,
        };
        paths.push(create_test_file_with_size(size_kb));
    }

    paths
}

/// 基准测试：读取单个文件元数据（不同文件大小）
fn bench_single_file_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_file_read");

    // 测试不同大小的文件
    for size_kb in [1, 10, 100, 1024].iter() {
        group.bench_with_input(
            BenchmarkId::new("file_size_kb", size_kb),
            size_kb,
            |b, &size_kb| {
                // 初始化（不包含在计时中）
                let exiftool = ExifTool::new().expect("Failed to create ExifTool");
                let temp_file = create_test_file_with_size(size_kb);

                b.iter(|| {
                    // 读取元数据
                    let _metadata = exiftool.query(&temp_file).execute();
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：批量读取不同数量的文件
fn bench_batch_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_read");

    for file_count in [1, 10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("file_count", file_count),
            file_count,
            |b, &file_count| {
                let exiftool = ExifTool::new().expect("Failed to create ExifTool");
                let paths = create_test_files(file_count);
                let path_refs: Vec<_> = paths.iter().map(|p| p.as_path()).collect();

                b.iter(|| {
                    let _results = exiftool.query_batch(&path_refs).execute();
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：使用连接池 vs 不使用连接池
fn bench_pool_vs_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("pool_vs_single");

    for file_count in [10, 100, 500].iter() {
        let paths = create_test_files(*file_count);

        // 不使用连接池（单个实例）
        group.bench_with_input(
            BenchmarkId::new("single_instance", file_count),
            file_count,
            |b, &_file_count| {
                let exiftool = ExifTool::new().expect("Failed to create ExifTool");

                b.iter(|| {
                    for path in &paths {
                        let _metadata = exiftool.query(path).execute();
                    }
                });
            },
        );

        // 使用连接池
        group.bench_with_input(
            BenchmarkId::new("connection_pool_size_4", file_count),
            file_count,
            |b, &_file_count| {
                let pool = ExifToolPool::new(4).expect("Failed to create pool");

                b.iter(|| {
                    let results: Vec<_> = paths
                        .iter()
                        .map(|path| {
                            exiftool_rs_wrapper::with_pool(&pool, |et| et.query(path).execute())
                        })
                        .collect();
                });
            },
        );

        // 使用更大的连接池
        group.bench_with_input(
            BenchmarkId::new("connection_pool_size_8", file_count),
            file_count,
            |b, &_file_count| {
                let pool = ExifToolPool::new(8).expect("Failed to create pool");

                b.iter(|| {
                    let results: Vec<_> = paths
                        .iter()
                        .map(|path| {
                            exiftool_rs_wrapper::with_pool(&pool, |et| et.query(path).execute())
                        })
                        .collect();
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：读取特定标签 vs 读取全部
fn bench_selective_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("selective_read");
    let paths = create_test_files(100);
    let exiftool = ExifTool::new().expect("Failed to create ExifTool");

    // 读取所有标签
    group.bench_function("read_all_tags", |b| {
        b.iter(|| {
            for path in &paths {
                let _metadata = exiftool.query(path).execute();
            }
        });
    });

    // 读取特定标签
    group.bench_function("read_specific_tags", |b| {
        b.iter(|| {
            for path in &paths {
                let _metadata = exiftool
                    .query(path)
                    .tag("FileName")
                    .tag("ImageSize")
                    .tag("Make")
                    .tag("Model")
                    .execute();
            }
        });
    });

    // 读取单个标签
    group.bench_function("read_single_tag", |b| {
        b.iter(|| {
            for path in &paths {
                let _metadata = exiftool.query(path).tag("FileName").execute();
            }
        });
    });

    group.finish();
}

/// 基准测试：缓存性能
fn bench_cached_vs_uncached(c: &mut Criterion) {
    let mut group = c.benchmark_group("cached_vs_uncached");
    let paths = create_test_files(50);

    group.bench_function("without_cache", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for path in &paths {
                let _metadata = exiftool.query(path).execute();
            }
        });
    });

    group.bench_function("with_cache", |b| {
        use exiftool_rs_wrapper::Cache;
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");
        let cache = Cache::new(100);

        b.iter(|| {
            for path in &paths {
                // 尝试从缓存获取
                if let Some(_cached) = cache.get(path) {
                    continue;
                }
                let metadata = exiftool.query(path).execute();
                cache.put(path.to_path_buf(), metadata.unwrap());
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_single_file_read,
    bench_batch_read,
    bench_pool_vs_single,
    bench_selective_read,
    bench_cached_vs_uncached
);
criterion_main!(benches);
