//! 写入操作性能基准测试
//!
//! 测试元数据写入操作的性能

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use exiftool_rs_wrapper::{ExifTool, ExifToolPool};
use std::io::Write;
use std::path::PathBuf;
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

/// 基准测试：写入单个标签
fn bench_write_single_tag(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_single_tag");
    let paths = create_test_files(50);
    let exiftool = ExifTool::new().expect("Failed to create ExifTool");

    // 覆盖原始文件（不创建备份，提高性能）
    group.bench_function("overwrite_original", |b| {
        let mut counter = 0;
        b.iter(|| {
            for path in &paths {
                counter += 1;
                let _ = exiftool
                    .write(path)
                    .tag("Copyright", format!("Test {}", counter))
                    .overwrite_original(true)
                    .execute();
            }
        });
    });

    // 创建备份
    group.bench_function("with_backup", |b| {
        let mut counter = 0;
        b.iter(|| {
            for path in &paths {
                counter += 1;
                let _ = exiftool
                    .write(path)
                    .tag("Copyright", format!("Test {}", counter))
                    .backup(true)
                    .execute();
            }
        });
    });

    group.finish();
}

/// 基准测试：写入多个标签
fn bench_write_multiple_tags(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_multiple_tags");
    let paths = create_test_files(50);
    let exiftool = ExifTool::new().expect("Failed to create ExifTool");

    for tag_count in [1, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("tag_count", tag_count),
            tag_count,
            |b, &tag_count| {
                b.iter(|| {
                    for path in &paths {
                        let mut builder = exiftool.write(path).overwrite_original(true);

                        // 添加指定数量的标签
                        for i in 0..tag_count {
                            builder = builder.tag(format!("Tag{}", i), format!("Value{}", i));
                        }

                        let _ = builder.execute();
                    }
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：删除标签
fn bench_delete_tags(c: &mut Criterion) {
    let mut group = c.benchmark_group("delete_tags");
    let paths = create_test_files(50);
    let exiftool = ExifTool::new().expect("Failed to create ExifTool");

    // 先写入一些标签以便删除
    for path in &paths {
        let _ = exiftool
            .write(path)
            .tag("Copyright", "Test")
            .tag("Artist", "Test Artist")
            .tag("Comment", "Test Comment")
            .overwrite_original(true)
            .execute();
    }

    group.bench_function("delete_single_tag", |b| {
        b.iter(|| {
            for path in &paths {
                let _ = exiftool
                    .write(path)
                    .delete("Copyright")
                    .overwrite_original(true)
                    .execute();
            }
        });
    });

    group.bench_function("delete_multiple_tags", |b| {
        b.iter(|| {
            for path in &paths {
                let _ = exiftool
                    .write(path)
                    .delete("Copyright")
                    .delete("Artist")
                    .delete("Comment")
                    .overwrite_original(true)
                    .execute();
            }
        });
    });

    group.finish();
}

/// 基准测试：批量写入
fn bench_batch_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_write");

    for file_count in [1, 10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("file_count", file_count),
            file_count,
            |b, &file_count| {
                let paths = create_test_files(file_count);
                let exiftool = ExifTool::new().expect("Failed to create ExifTool");

                b.iter(|| {
                    for path in &paths {
                        let _ = exiftool
                            .write(path)
                            .tag("Copyright", "Batch Test")
                            .tag("Artist", "Batch Artist")
                            .overwrite_original(true)
                            .execute();
                    }
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：连接池写入
fn bench_pool_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("pool_write");
    let paths = create_test_files(100);

    // 不使用连接池
    group.bench_function("single_instance", |b| {
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
    });

    // 使用连接池
    group.bench_function("pool_size_4", |b| {
        let pool = ExifToolPool::new(4).expect("Failed to create pool");

        b.iter(|| {
            for path in &paths {
                let _ = exiftool_rs_wrapper::with_pool(&pool, |et| {
                    et.write(path)
                        .tag("Copyright", "Test")
                        .overwrite_original(true)
                        .execute()
                });
            }
        });
    });

    group.bench_function("pool_size_8", |b| {
        let pool = ExifToolPool::new(8).expect("Failed to create pool");

        b.iter(|| {
            for path in &paths {
                let _ = exiftool_rs_wrapper::with_pool(&pool, |et| {
                    et.write(path)
                        .tag("Copyright", "Test")
                        .overwrite_original(true)
                        .execute()
                });
            }
        });
    });

    group.finish();
}

/// 基准测试：写入不同大小的文件
fn bench_write_by_file_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_by_file_size");
    let exiftool = ExifTool::new().expect("Failed to create ExifTool");

    for size_kb in [10, 100, 1024, 5120].iter() {
        group.bench_with_input(
            BenchmarkId::new("size_kb", size_kb),
            size_kb,
            |b, &size_kb| {
                let paths: Vec<_> = (0..20)
                    .map(|_| create_test_file_with_size(size_kb))
                    .collect();

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
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_write_single_tag,
    bench_write_multiple_tags,
    bench_delete_tags,
    bench_batch_write,
    bench_pool_write,
    bench_write_by_file_size
);
criterion_main!(benches);
