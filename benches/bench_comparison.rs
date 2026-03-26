//! 综合对比性能基准测试
//!
//! 对比不同使用模式下的性能差异

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use exiftool_rs_wrapper::{ExifTool, ExifToolPool, batch_with_pool, with_pool};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
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

/// 基准测试：单次执行 vs -stay_open 模式（对比不使用 -stay_open 的开销）
fn bench_stay_open_vs_single_exec(c: &mut Criterion) {
    let mut group = c.benchmark_group("stay_open_vs_single_exec");
    let paths = create_test_files(10);

    // 使用 -stay_open 模式（我们的库）
    group.bench_function("stay_open_mode", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for path in &paths {
                let _metadata = exiftool.query(path).execute();
            }
        });
    });

    // 模拟单次执行模式（每次启动新进程）
    group.bench_function("single_exec_mode", |b| {
        b.iter(|| {
            for path in &paths {
                let _output = Command::new("exiftool")
                    .arg("-json")
                    .arg("-charset")
                    .arg("UTF8")
                    .arg(path)
                    .output();
            }
        });
    });

    group.finish();
}

/// 基准测试：不同文件数量的性能对比
fn bench_scaling_by_file_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling_by_file_count");

    for file_count in [1, 10, 50, 100, 500, 1000].iter() {
        let paths = create_test_files(*file_count);
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        group.bench_with_input(
            BenchmarkId::new("single_instance", file_count),
            file_count,
            |b, &_file_count| {
                b.iter(|| {
                    for path in &paths {
                        let _metadata = exiftool.query(path).execute();
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("batch_query", file_count),
            file_count,
            |b, &_file_count| {
                let path_refs: Vec<_> = paths.iter().map(|p| p.as_path()).collect();

                b.iter(|| {
                    let _results = exiftool.query_batch(&path_refs).execute();
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("pool_4_connections", file_count),
            file_count,
            |b, &_file_count| {
                let pool = ExifToolPool::new(4).expect("Failed to create pool");
                let concurrency = 4usize.min(*file_count);
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
            },
        );
    }

    group.finish();
}

/// 基准测试：不同文件大小的性能对比
fn bench_scaling_by_file_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling_by_file_size");

    for size_kb in [1, 10, 100, 1024, 5120, 10240].iter() {
        let paths: Vec<_> = (0..50)
            .map(|_| create_test_file_with_size(*size_kb))
            .collect();
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");
        let pool = ExifToolPool::new(4).expect("Failed to create pool");

        group.bench_with_input(
            BenchmarkId::new("single_instance", size_kb),
            size_kb,
            |b, &_size_kb| {
                b.iter(|| {
                    for path in &paths {
                        let _metadata = exiftool.query(path).execute();
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("connection_pool", size_kb),
            size_kb,
            |b, &_size_kb| {
                b.iter(|| {
                    let _results =
                        batch_with_pool(&pool, paths.clone(), |et, path| et.query(&path).execute());
                });
            },
        );
    }

    group.finish();
}

/// 基准测试：读取与写入的性能对比
fn bench_read_vs_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_vs_write");
    let paths = create_test_files(100);

    group.bench_function("read_all_metadata", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for path in &paths {
                let _metadata = exiftool.query(path).execute();
            }
        });
    });

    group.bench_function("read_specific_tags", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for path in &paths {
                let _metadata = exiftool
                    .query(path)
                    .tag("FileName")
                    .tag("ImageSize")
                    .execute();
            }
        });
    });

    group.bench_function("write_single_tag", |b| {
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

    group.bench_function("write_multiple_tags", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for path in &paths {
                let _ = exiftool
                    .write(path)
                    .tag("Copyright", "Test")
                    .tag("Artist", "Artist")
                    .tag("Comment", "Comment")
                    .overwrite_original(true)
                    .execute();
            }
        });
    });

    group.finish();
}

/// 基准测试：缓存的有效性
fn bench_cache_effectiveness(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_effectiveness");
    let paths = create_test_files(100);
    let exiftool = ExifTool::new().expect("Failed to create ExifTool");

    // 不使用缓存
    group.bench_function("no_cache", |b| {
        b.iter(|| {
            for path in &paths {
                let _metadata = exiftool.query(path).execute();
            }
        });
    });

    // 使用简单的内存缓存（模拟）
    group.bench_function("with_simple_cache", |b| {
        use std::collections::HashMap;
        use std::sync::Mutex;

        let cache = Mutex::new(HashMap::new());

        b.iter(|| {
            for path in &paths {
                let mut cache = cache.lock().unwrap();
                let key = path.to_string_lossy().to_string();

                if cache.get(&key).is_none() {
                    let metadata = exiftool.query(path).execute();
                    if let Ok(m) = metadata {
                        cache.insert(key, m);
                    }
                }
            }
        });
    });

    group.finish();
}

/// 基准测试：综合场景 - 混合读写操作
fn bench_mixed_read_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_read_write");
    let paths = create_test_files(100);

    group.bench_function("sequential_mixed", |b| {
        let exiftool = ExifTool::new().expect("Failed to create ExifTool");

        b.iter(|| {
            for (i, path) in paths.iter().enumerate() {
                if i % 4 == 0 {
                    // 25% 写入操作
                    let _ = exiftool
                        .write(path)
                        .tag("Copyright", "Test")
                        .overwrite_original(true)
                        .execute();
                } else {
                    // 75% 读取操作
                    let _metadata = exiftool.query(path).execute();
                }
            }
        });
    });

    group.bench_function("parallel_mixed_with_pool", |b| {
        let pool = ExifToolPool::new(4).expect("Failed to create pool");
        let concurrency = 4;
        let chunk_size = paths.len() / concurrency;

        b.iter(|| {
            let mut handles = vec![];

            for i in 0..concurrency {
                let pool = pool.clone();
                let chunk: Vec<_> = paths[i * chunk_size..(i + 1) * chunk_size]
                    .iter()
                    .cloned()
                    .enumerate()
                    .collect();

                let handle = thread::spawn(move || {
                    for (idx, path) in chunk {
                        if idx % 4 == 0 {
                            let _ = with_pool(&pool, |et| {
                                et.write(&path)
                                    .tag("Copyright", "Test")
                                    .overwrite_original(true)
                                    .execute()
                            });
                        } else {
                            let _ = with_pool(&pool, |et| et.query(&path).execute());
                        }
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

/// 基准测试：完整的端到端工作流
fn bench_end_to_end_workflow(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end_workflow");

    for file_count in [10, 100, 500].iter() {
        let paths = create_test_files(*file_count);

        group.bench_with_input(
            BenchmarkId::new("single_threaded", file_count),
            file_count,
            |b, &_file_count| {
                b.iter(|| {
                    let exiftool = ExifTool::new().expect("Failed to create ExifTool");

                    // 1. 读取所有文件的元数据
                    for path in &paths {
                        let _metadata = exiftool.query(path).execute();
                    }

                    // 2. 写入版权信息
                    for path in &paths {
                        let _ = exiftool
                            .write(path)
                            .tag("Copyright", "Test Company")
                            .overwrite_original(true)
                            .execute();
                    }

                    // 3. 再次读取验证
                    for path in &paths {
                        let _metadata = exiftool.query(path).execute();
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("multi_threaded_with_pool", file_count),
            file_count,
            |b, &_file_count| {
                b.iter(|| {
                    let pool = ExifToolPool::new(4).expect("Failed to create pool");
                    let concurrency = 4usize.min(*file_count);
                    let chunk_size = paths.len() / concurrency;

                    // 并行读取
                    let mut read_handles = vec![];
                    for i in 0..concurrency {
                        let pool = pool.clone();
                        let chunk: Vec<_> = paths[i * chunk_size..(i + 1) * chunk_size].to_vec();

                        let handle = thread::spawn(move || {
                            for path in chunk {
                                let _ = with_pool(&pool, |et| et.query(&path).execute());
                            }
                        });
                        read_handles.push(handle);
                    }
                    for handle in read_handles {
                        handle.join().unwrap();
                    }

                    // 并行写入
                    let _results = batch_with_pool(&pool, paths.clone(), |et, path| {
                        et.write(&path)
                            .tag("Copyright", "Test Company")
                            .overwrite_original(true)
                            .execute()
                    });
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_stay_open_vs_single_exec,
    bench_scaling_by_file_count,
    bench_scaling_by_file_size,
    bench_read_vs_write,
    bench_cache_effectiveness,
    bench_mixed_read_write,
    bench_end_to_end_workflow
);
criterion_main!(benches);
