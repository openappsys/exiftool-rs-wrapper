//! 性能优化示例
//!
//! 本示例展示如何使用各种性能优化技术：
//! - 连接池（Connection Pool）
//! - 缓存（Caching）
//! - 进度追踪
//! - 性能统计
//! - 批量处理优化
//!
//! 运行方式:
//! ```bash
//! cargo run --example performance_optimization --release
//! ```

use exiftool_rs_wrapper::{
    Cache, ExifTool, ExifToolPool, PerformanceStats, ProgressCallback, ProgressTracker,
    StreamOptions, TagId, batch_with_pool, with_pool,
};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ExifTool 性能优化示例 ===\n");

    // 查找测试图片
    let test_images = find_test_images();
    if test_images.is_empty() {
        println!("⚠️  未找到测试图片，示例将以演示模式运行");
        println!("   请在 examples 目录下放置一些图片用于测试");
        return Ok(());
    }

    println!("📁 找到 {} 个测试图片\n", test_images.len());

    // ============================================================
    // 1. 基准测试：单实例顺序处理
    // ============================================================
    println!("1️⃣  基准测试：单实例顺序处理");

    let exiftool = ExifTool::new()?;
    let start = Instant::now();

    let mut single_results = 0;
    for image in &test_images {
        if exiftool.query(image).tag("FileName").execute().is_ok() {
            single_results += 1
        }
    }

    let single_duration = start.elapsed();
    println!("   ✅ 处理 {} 个文件", single_results);
    println!("   ⏱️  耗时: {:?}", single_duration);
    println!(
        "   📊 平均: {:?} 每文件\n",
        single_duration / single_results.max(1) as u32
    );

    // ============================================================
    // 2. 连接池：多线程并行处理
    // ============================================================
    println!("2️⃣  使用连接池进行多线程并行处理");

    // 创建包含 4 个连接的池
    let pool_size = 4.min(test_images.len()).max(1);
    println!("   🔧 创建连接池（大小: {}）", pool_size);

    let pool = ExifToolPool::new(pool_size)?;
    println!("   ✅ 连接池创建成功");
    println!("   📊 可用连接: {}\n", pool.available()?);

    let start = Instant::now();

    // 使用连接池并行处理
    let results: Vec<_> = test_images
        .iter()
        .map(|image| {
            let path = image.clone();
            with_pool(&pool, |exiftool| {
                exiftool.query(&path).tag("FileName").execute()
            })
        })
        .collect();

    let pool_duration = start.elapsed();
    let pool_success = results.iter().filter(|r| r.is_ok()).count();

    println!("   ✅ 成功: {} 个文件", pool_success);
    println!("   ⏱️  耗时: {:?}", pool_duration);
    println!(
        "   📊 平均: {:?} 每文件",
        pool_duration / pool_success.max(1) as u32
    );

    // 性能对比
    if single_duration > std::time::Duration::from_secs(0) {
        let speedup = single_duration.as_secs_f64() / pool_duration.as_secs_f64();
        println!("   🚀 性能提升: {:.2}x\n", speedup);
    }

    // ============================================================
    // 3. 使用 batch_with_pool 进行批量处理
    // ============================================================
    println!("3️⃣  使用 batch_with_pool 批量处理");

    let items: Vec<(PathBuf, String)> = test_images
        .iter()
        .map(|img| {
            let copy_path = create_test_copy(img).unwrap_or_else(|_| img.clone());
            (copy_path, "© 2026 Performance Test".to_string())
        })
        .collect();

    let start = Instant::now();

    let batch_results = batch_with_pool(&pool, items.clone(), |exiftool, (path, copyright)| {
        exiftool
            .write(&path)
            .tag("Copyright", &copyright)
            .overwrite_original(true)
            .execute()
    });

    let batch_duration = start.elapsed();
    let batch_success = batch_results.iter().filter(|r| r.is_ok()).count();

    println!("   ✅ 成功写入: {} 个文件", batch_success);
    println!("   ⏱️  耗时: {:?}", batch_duration);
    println!(
        "   📊 平均: {:?} 每文件\n",
        batch_duration / batch_success.max(1) as u32
    );

    // 清理测试文件
    for (path, _) in items {
        if path.exists() {
            let _ = std::fs::remove_file(&path);
        }
    }

    // ============================================================
    // 4. 缓存机制
    // ============================================================
    println!("4️⃣  使用缓存优化重复查询");

    // 创建 LRU 缓存，容量为 10
    let cache: Cache<String, String> = Cache::new(10);

    // 测试缓存命中率
    let test_image = &test_images[0];
    let cache_key = format!("make:{}", test_image.display());

    // 第一次查询（缓存未命中）
    let start = Instant::now();
    let _make_value = if let Some(cached) = cache.get(&cache_key) {
        println!("   💾 缓存命中: {}", cached);
        cached
    } else {
        let value = exiftool
            .read_tag::<String, _, _>(test_image, TagId::MAKE.name())
            .unwrap_or_default();
        cache.put(cache_key.clone(), value.clone());
        println!("   📥 缓存未命中，已缓存: {}", value);
        value
    };
    let first_query_time = start.elapsed();

    // 第二次查询（应该命中缓存）
    let start = Instant::now();
    let cached_value = cache.get(&cache_key);
    let second_query_time = start.elapsed();

    println!("   ⏱️  首次查询: {:?}", first_query_time);
    println!("   ⏱️  缓存查询: {:?}", second_query_time);
    println!("   💾 缓存命中: {}", cached_value.is_some());
    println!("   📈 缓存命中率: {:.1}%\n", cache.hit_rate());

    // ============================================================
    // 5. 进度追踪
    // ============================================================
    println!("5️⃣  使用进度追踪器");

    let total_files = test_images.len();
    let progress_counter = Arc::new(AtomicU64::new(0));
    let progress_counter_clone = Arc::clone(&progress_counter);

    // 创建带回调的进度追踪器
    let progress: ProgressCallback = Arc::new(move |processed, total| {
        let current = progress_counter_clone.fetch_add(1, Ordering::SeqCst);
        let percentage = (processed as f64 / total as f64) * 100.0;

        if current.is_multiple_of(5) || processed == total {
            println!("   📊 进度: {}/{} ({:.1}%)", processed, total, percentage);
        }

        // 返回 true 表示继续，false 表示取消
        true
    });

    let tracker = ProgressTracker::new(total_files, Some(progress));

    println!("   🔄 开始处理 {} 个文件...", total_files);

    for (i, image) in test_images.iter().enumerate() {
        // 模拟处理
        let _ = exiftool.query(image).tag("FileName").execute();
        tracker.update(1);

        // 模拟一些处理时间
        if i < 3 {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    println!("   ✅ 处理完成");
    println!("   📊 最终进度: {:.1}%\n", tracker.percentage());

    // ============================================================
    // 6. 性能统计
    // ============================================================
    println!("6️⃣  使用性能统计");

    let stats = PerformanceStats::default();

    // 模拟一些操作并记录统计
    for image in &test_images {
        let start = Instant::now();

        let result = exiftool.query(image).tag("Make").execute();
        let success = result.is_ok();

        let elapsed = start.elapsed();
        stats.record(success, elapsed.as_micros() as u64);
    }

    println!("   📊 性能统计:");
    println!(
        "      总操作数: {}",
        stats.total_operations.load(Ordering::SeqCst)
    );
    println!(
        "      成功: {}",
        stats.successful_operations.load(Ordering::SeqCst)
    );
    println!(
        "      失败: {}",
        stats.failed_operations.load(Ordering::SeqCst)
    );
    println!("      成功率: {:.1}%", stats.success_rate());
    println!("      平均耗时: {} μs\n", stats.avg_time_us());

    // ============================================================
    // 7. 流式处理选项
    // ============================================================
    println!("7️⃣  流式处理选项");

    let stream_opts = StreamOptions::new()
        .buffer_size(128 * 1024) // 128KB 缓冲区
        .on_progress(|processed, total| {
            println!("   📊 流式进度: {}/{} bytes", processed, total);
            true // 继续处理
        });

    println!("   📋 流式处理配置:");
    println!("      缓冲区大小: {} bytes", stream_opts.buffer_size);
    println!(
        "      有进度回调: {}\n",
        stream_opts.progress_callback.is_some()
    );

    // ============================================================
    // 8. 连接池性能测试对比
    // ============================================================
    println!("8️⃣  连接池大小性能对比");

    let pool_sizes = vec![1, 2, 4];
    let mut results = Vec::new();

    for size in pool_sizes {
        if size > test_images.len() {
            continue;
        }

        println!("   🔧 测试连接池大小: {}", size);

        let test_pool = ExifToolPool::new(size)?;
        let start = Instant::now();

        let _: Vec<_> = test_images
            .iter()
            .map(|image| {
                with_pool(&test_pool, |exiftool| {
                    exiftool.query(image).tag("FileName").execute()
                })
            })
            .collect();

        let duration = start.elapsed();
        results.push((size, duration));

        println!("      ⏱️  耗时: {:?}", duration);

        test_pool.close()?;
    }

    println!("\n   📊 性能对比总结:");
    for (size, duration) in &results {
        println!("      池大小 {}: {:?}", size, duration);
    }
    println!();

    // ============================================================
    // 9. 最佳实践总结
    // ============================================================
    println!("9️⃣  性能优化最佳实践");

    println!("\n   🎯 连接池:");
    println!("      • 对于多线程应用，使用 ExifToolPool 而非单个实例");
    println!("      • 池大小建议设置为 CPU 核心数的 1-2 倍");
    println!("      • 使用 with_pool() 自动管理连接生命周期");

    println!("\n   💾 缓存:");
    println!("      • 对于重复查询的场景，使用 Cache 缓存结果");
    println!("      • 选择合适的缓存容量（LRU 策略）");
    println!("      • 监控缓存命中率以优化容量");

    println!("\n   📊 进度追踪:");
    println!("      • 为长时间运行的操作添加进度回调");
    println!("      • 允许用户取消操作");
    println!("      • 提供有意义的进度信息");

    println!("\n   📈 性能统计:");
    println!("      • 使用 PerformanceStats 监控操作性能");
    println!("      • 识别瓶颈和低效操作");
    println!("      • 设置性能基准和警报阈值");

    // ============================================================
    // 10. 清理资源
    // ============================================================
    println!("\n🔟  清理资源");

    pool.close()?;
    println!("   ✅ 连接池已关闭");

    exiftool.close()?;
    println!("   ✅ ExifTool 实例已关闭");

    println!("\n✨ 性能优化示例完成！");
    println!("\n💡 提示: 使用 --release 模式编译可获得最佳性能");
    println!("   cargo run --example performance_optimization --release");

    Ok(())
}

/// 查找测试图片
fn find_test_images() -> Vec<PathBuf> {
    let mut images = Vec::new();

    // 检查 examples 目录
    if let Ok(entries) = std::fs::read_dir("examples") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "tiff" || ext == "raw" {
                    images.push(path);
                }
            }
        }
    }

    // 如果没有足够的图片，复制已有的
    if images.len() < 4 && !images.is_empty() {
        let first = images[0].clone();
        for i in 1..4 {
            let copy = first.with_file_name(format!("sample_{}.jpg", i));
            if !copy.exists() {
                let _ = std::fs::copy(&first, &copy);
            }
            images.push(copy);
        }
    }

    // 限制数量
    images.truncate(8);

    images
}

/// 创建测试副本
fn create_test_copy(original: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let copy_path = original.with_file_name(format!(
        "{}_perf_test.{}",
        original.file_stem().unwrap_or_default().to_string_lossy(),
        original.extension().unwrap_or_default().to_string_lossy()
    ));

    std::fs::copy(original, &copy_path)?;
    Ok(copy_path)
}
