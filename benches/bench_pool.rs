//! 连接池性能基准测试
//!
//! 测试连接池的创建、获取、归还等操作的性能

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use exiftool_rs_wrapper::{ExifTool, ExifToolPool, with_pool};
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

/// 基准测试：连接池创建时间
fn bench_pool_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("pool_creation");

    for pool_size in [1, 2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::new("size", pool_size),
            pool_size,
            |b, &pool_size| {
                b.iter(|| {
                    let _pool = ExifToolPool::new(pool_size).expect("Failed to create pool");
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：单个实例 vs 连接池
fn bench_single_vs_pool_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_vs_pool_overhead");
    let paths = create_test_files(100);

    // 创建单个实例一次，重复使用
    group.bench_function("single_instance_reused", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for path in &paths {
                let _metadata = exiftool.query(path).execute();
            }
        });
    });

    // 每次查询都创建新实例（测试进程启动开销）
    group.bench_function("create_instance_per_query", |b| {
        b.iter(|| {
            for path in &paths {
                let exiftool = ExifTool::new().expect("Failed to create ExifTool");
                let _metadata = exiftool.query(path).execute();
                // 实例会在作用域结束时自动关闭
            }
        });
    });

    // 连接池（单连接）
    group.bench_function("pool_size_1", |b| {
        let pool = ExifToolPool::new(1).expect("Failed to create pool");

        b.iter(|| {
            for path in &paths {
                let _ = with_pool(&pool, |et| et.query(path).execute());
            }
        });
    });

    // 连接池（4连接）
    group.bench_function("pool_size_4", |b| {
        let pool = ExifToolPool::new(4).expect("Failed to create pool");

        b.iter(|| {
            for path in &paths {
                let _ = with_pool(&pool, |et| et.query(path).execute());
            }
        });
    });

    group.finish();
}

/// 基准测试：连接获取/归还的开销
fn bench_connection_acquire_release(c: &mut Criterion) {
    let mut group = c.benchmark_group("connection_acquire_release");

    for pool_size in [1, 4, 8].iter() {
        let pool = ExifToolPool::new(*pool_size).expect("Failed to create pool");

        group.bench_with_input(
            BenchmarkId::new("pool_size", pool_size),
            pool_size,
            |b, &_pool_size| {
                b.iter(|| {
                    // 获取连接
                    let conn = pool.acquire().expect("Failed to acquire connection");
                    // 立即归还
                    drop(conn);
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：并发访问下的连接池性能
fn bench_concurrent_pool_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_pool_access");
    let paths = create_test_files(100);

    for concurrency_level in [1, 2, 4, 8, 16].iter() {
        let pool = ExifToolPool::new(4).expect("Failed to create pool");
        let chunk_size = paths.len() / concurrency_level;

        group.bench_with_input(
            BenchmarkId::new("concurrency", concurrency_level),
            concurrency_level,
            |b, &concurrency_level| {
                b.iter(|| {
                    let mut handles = vec![];

                    for i in 0..concurrency_level {
                        let pool = pool.clone();
                        let chunk: Vec<_> = paths[i * chunk_size..(i + 1) * chunk_size].to_vec();

                        let handle = thread::spawn(move || {
                            for path in chunk {
                                let _ = with_pool(&pool, |et| et.query(&path).execute());
                            }
                        });

                        handles.push(handle);
                    }

                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：连接池大小对性能的影响
fn bench_pool_size_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("pool_size_optimization");
    let paths = create_test_files(200);
    let concurrency = 8;
    let chunk_size = paths.len() / concurrency;

    for pool_size in [1, 2, 4, 8, 16, 32].iter() {
        let pool = ExifToolPool::new(*pool_size).expect("Failed to create pool");

        group.bench_with_input(
            BenchmarkId::new("pool_size", pool_size),
            pool_size,
            |b, &_pool_size| {
                b.iter(|| {
                    let mut handles = vec![];

                    for i in 0..concurrency {
                        let pool = pool.clone();
                        let chunk: Vec<_> = paths[i * chunk_size..(i + 1) * chunk_size].to_vec();

                        let handle = thread::spawn(move || {
                            for path in chunk {
                                let _ = with_pool(&pool, |et| et.query(&path).execute());
                            }
                        });

                        handles.push(handle);
                    }

                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：连接池在高负载下的性能
fn bench_pool_under_high_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("pool_under_high_load");
    let paths = create_test_files(500);

    // 单实例
    group.bench_function("single_instance_500_files", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for path in &paths {
                let _metadata = exiftool.query(path).execute();
            }
        });
    });

    // 连接池（4连接）处理500个文件
    group.bench_function("pool_4_connections_500_files", |b| {
        let pool = ExifToolPool::new(4).expect("Failed to create pool");
        let concurrency = 4;
        let chunk_size = paths.len() / concurrency;

        b.iter(|| {
            let mut handles = vec![];

            for i in 0..concurrency {
                let pool = pool.clone();
                let chunk: Vec<_> = paths[i * chunk_size..(i + 1) * chunk_size].to_vec();

                let handle = thread::spawn(move || {
                    for path in chunk {
                        let _ = with_pool(&pool, |et| et.query(&path).execute());
                    }
                });

                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });

    // 连接池（8连接）处理500个文件
    group.bench_function("pool_8_connections_500_files", |b| {
        let pool = ExifToolPool::new(8).expect("Failed to create pool");
        let concurrency = 8;
        let chunk_size = paths.len() / concurrency;

        b.iter(|| {
            let mut handles = vec![];

            for i in 0..concurrency {
                let pool = pool.clone();
                let chunk: Vec<_> = paths[i * chunk_size..(i + 1) * chunk_size].to_vec();

                let handle = thread::spawn(move || {
                    for path in chunk {
                        let _ = with_pool(&pool, |et| et.query(&path).execute());
                    }
                });

                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_pool_creation,
    bench_single_vs_pool_overhead,
    bench_connection_acquire_release,
    bench_concurrent_pool_access,
    bench_pool_size_optimization,
    bench_pool_under_high_load
);
criterion_main!(benches);
