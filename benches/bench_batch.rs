//! 批量处理性能基准测试
//!
//! 测试批量处理与单次处理的性能对比

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use exiftool_rs_wrapper::{ExifTool, ExifToolPool, batch_with_pool, with_pool};
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use tempfile::NamedTempFile;

/// 创建测试用的临时图片文件
fn create_test_file_with_size(size_kb: usize) -> PathBuf {
    let mut temp_file = NamedTempFile::new().unwrap();
    let data = vec![0xFFu8; size_kb * 1024];
    temp_file.write_all(&data).unwrap();

    let path = temp_file.path().to_path_buf();
    std::mem::forget(temp_file);

    path
}

/// 创建指定数量的测试文件
fn create_test_files(count: usize) -> Vec<PathBuf> {
    let mut paths = Vec::with_capacity(count);

    for i in 0..count {
        let size_kb = match i % 3 {
            0 => 10,
            1 => 100,
            _ => 1000,
        };
        paths.push(create_test_file_with_size(size_kb));
    }

    paths
}

/// 基准测试：批量读取 vs 单次读取
fn bench_batch_vs_single_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_vs_single_read");

    for file_count in [1, 10, 50, 100, 500].iter() {
        let paths = create_test_files(*file_count);

        // 批量读取
        group.bench_with_input(
            BenchmarkId::new("batch_read", file_count),
            file_count,
            |b, &_file_count| {
                let exiftool = ExifTool::new().expect("Failed to create ExifTool");
                let path_refs: Vec<_> = paths.iter().map(|p| p.as_path()).collect();

                b.iter(|| {
                    let _results = exiftool.query_batch(&path_refs).execute();
                });
            },
        );

        // 单次读取（单个实例）
        group.bench_with_input(
            BenchmarkId::new("single_read", file_count),
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
    }

    group.finish();
}

/// 基准测试：并行批量处理
fn bench_parallel_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_batch");

    for file_count in [10, 50, 100, 200].iter() {
        let paths = create_test_files(*file_count);

        // 串行处理
        group.bench_with_input(
            BenchmarkId::new("sequential", file_count),
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

        // 并行处理（使用连接池）
        let paths_for_parallel = paths.clone();
        group.bench_with_input(
            BenchmarkId::new("parallel_pool_4", file_count),
            file_count,
            |b, &_file_count| {
                let pool = ExifToolPool::new(4).expect("Failed to create pool");

                b.iter(|| {
                    let results: Vec<_> = paths_for_parallel
                        .clone()
                        .into_iter()
                        .map(|path| {
                            let pool = pool.clone();
                            thread::spawn(move || with_pool(&pool, |et| et.query(&path).execute()))
                        })
                        .collect();

                    // 等待所有线程完成
                    for handle in results {
                        let _ = handle.join();
                    }
                });
            },
        );

        // 使用 batch_with_pool
        group.bench_with_input(
            BenchmarkId::new("batch_with_pool", file_count),
            file_count,
            |b, &_file_count| {
                let pool = ExifToolPool::new(4).expect("Failed to create pool");
                let path_clones: Vec<_> = paths.clone();

                b.iter(|| {
                    let _results = batch_with_pool(&pool, path_clones.clone(), |et, path| {
                        et.query(&path).execute()
                    });
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：批量写入性能
fn bench_batch_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_write");

    for file_count in [10, 50, 100].iter() {
        let paths = create_test_files(*file_count);

        // 串行写入
        group.bench_with_input(
            BenchmarkId::new("sequential_write", file_count),
            file_count,
            |b, &_file_count| {
                let exiftool = ExifTool::new().expect("Failed to create ExifTool");

                b.iter(|| {
                    for path in &paths {
                        let _ = exiftool
                            .write(path)
                            .tag("Copyright", "Test")
                            .overwrite_original(true)
                            .execute();
                    }
                });
            },
        );

        // 并行写入（使用连接池）
        group.bench_with_input(
            BenchmarkId::new("parallel_write_pool_4", file_count),
            file_count,
            |b, &_file_count| {
                let pool = ExifToolPool::new(4).expect("Failed to create pool");

                b.iter(|| {
                    let _results = batch_with_pool(&pool, paths.clone(), |et, path| {
                        et.write(&path)
                            .tag("Copyright", "Test")
                            .overwrite_original(true)
                            .execute()
                    });
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：批量处理不同工作负载大小
fn bench_batch_size_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_size_comparison");
    let paths = create_test_files(1000);

    // 一次性处理所有文件
    group.bench_function("batch_size_1000", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");
        let path_refs: Vec<_> = paths.iter().map(|p| p.as_path()).collect();

        b.iter(|| {
            let _results = exiftool.query_batch(&path_refs).execute();
        });
    });

    // 分批次处理（每批100个）
    group.bench_function("batch_size_100", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for chunk in paths.chunks(100) {
                let path_refs: Vec<_> = chunk.iter().map(|p| p.as_path()).collect();
                let _results = exiftool.query_batch(&path_refs).execute();
            }
        });
    });

    // 分批次处理（每批50个）
    group.bench_function("batch_size_50", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for chunk in paths.chunks(50) {
                let path_refs: Vec<_> = chunk.iter().map(|p| p.as_path()).collect();
                let _results = exiftool.query_batch(&path_refs).execute();
            }
        });
    });

    // 分批次处理（每批10个）
    group.bench_function("batch_size_10", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for chunk in paths.chunks(10) {
                let path_refs: Vec<_> = chunk.iter().map(|p| p.as_path()).collect();
                let _results = exiftool.query_batch(&path_refs).execute();
            }
        });
    });

    group.finish();
}

/// 基准测试：线程数对性能的影响
fn bench_thread_count_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("thread_count_scaling");
    let paths = create_test_files(200);

    for thread_count in [1, 2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::new("threads", thread_count),
            thread_count,
            |b, &thread_count| {
                let pool = ExifToolPool::new(thread_count).expect("Failed to create pool");

                b.iter(|| {
                    let _results =
                        batch_with_pool(&pool, paths.clone(), |et, path| et.query(&path).execute());
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_batch_vs_single_read,
    bench_parallel_batch,
    bench_batch_write,
    bench_batch_size_comparison,
    bench_thread_count_scaling
);
criterion_main!(benches);
