//! 异步 API 性能最佳实践示例
//!
//! 展示何时使用 Builder 模式，何时使用流式 API
//!
//! 运行方式:
//! cargo run --example async_performance --features async

use exiftool_rs_wrapper::AsyncExifTool;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 异步 API 性能最佳实践示例 ===\n");

    let exiftool = AsyncExifTool::new()?;

    // ===== 场景 1: 单文件查询（Builder 模式） =====
    println!("【场景 1】单文件查询 - 使用 Builder 模式");
    println!("说明: spawn_blocking 开销可忽略，代码简洁\n");

    let start = Instant::now();
    let metadata = exiftool.query("photo.jpg").await?;
    let duration = start.elapsed();

    println!("✓ 查询耗时: {:?}", duration);
    println!("  相机制造商: {:?}\n", metadata.get("Make"));

    // ===== 场景 2: 批量处理（流式 API） =====
    println!("【场景 2】批量处理 - 使用流式 API");
    println!("说明: 避免频繁 spawn_blocking 调度，自动批处理\n");

    // 模拟文件列表
    let files = vec!["1.jpg", "2.jpg", "3.jpg"];

    // ❌ 不推荐：循环中使用 Builder（会导致多次调度）
    println!("❌ 不推荐方式（循环中使用 Builder）:");
    let start = Instant::now();
    for file in &files {
        let _ = exiftool.query(file).await?; // 每次都要 spawn_blocking
    }
    println!("   耗时: {:?} (调度开销较大)\n", start.elapsed());

    // ✅ 推荐：使用 stream_batch
    println!("✅ 推荐方式（使用 stream_batch）:");
    let start = Instant::now();
    let (mut rx, _handle) = exiftool.stream_batch(&files).await?;

    let mut success = 0;
    let mut failed = 0;

    while let Some((path, result)) = rx.recv().await {
        match result {
            Ok(meta) => {
                println!("   ✓ {:?}: {} tags", path, meta.len());
                success += 1;
            }
            Err(e) => {
                println!("   ✗ {:?}: {}", path, e);
                failed += 1;
            }
        }
    }
    println!(
        "   总耗时: {:?} (成功: {}, 失败: {})\n",
        start.elapsed(),
        success,
        failed
    );

    // ===== 场景 3: 大文件带进度（流式 API） =====
    println!("【场景 3】大文件处理 - 带进度跟踪");
    println!("说明: 使用 stream_large_file 支持进度反馈\n");

    let (mut rx, _handle) = exiftool.stream_large_file("video.mp4").await?;

    println!("开始处理大文件...");
    let start = Instant::now();

    while let Some(event) = rx.recv().await {
        use exiftool_rs_wrapper::async_stream::StreamEvent;

        match event {
            StreamEvent::Progress(current, total) => {
                let pct = if total > 0 { current * 100 / total } else { 0 };
                print!("\r   进度: {}%", pct);
            }
            StreamEvent::MetadataChunk(_) => {
                println!("\n   ✓ 收到元数据");
            }
            StreamEvent::Complete => {
                println!("   ✓ 处理完成！");
                break;
            }
            StreamEvent::Cancelled => {
                println!("\n   ✗ 已取消");
                break;
            }
        }
    }
    println!("   总耗时: {:?}\n", start.elapsed());

    // ===== 场景 4: 带取消操作的流式查询 =====
    println!("【场景 4】可取消的流式查询");
    println!("说明: 使用 AsyncStreamHandle 取消长时间运行的操作\n");

    let (mut rx, handle) = exiftool.stream_query("large_raw.cr2").await?;

    // 模拟用户 100ms 后取消
    let cancel_handle = handle.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        println!("   → 模拟用户取消操作");
        let _ = cancel_handle.cancel();
    });

    while let Some(event) = rx.recv().await {
        use exiftool_rs_wrapper::async_stream::StreamEvent;

        match event {
            StreamEvent::Progress(current, total) => {
                print!("\r   进度: {}/{}", current, total);
            }
            StreamEvent::Cancelled => {
                println!("\n   ✓ 操作已取消");
                break;
            }
            _ => {}
        }
    }

    println!("\n=== 总结 ===");
    println!("✓ 单文件查询: 使用 Builder 模式 (query())");
    println!("✓ 批量处理: 使用流式 API (stream_batch())");
    println!("✓ 大文件: 使用流式 API (stream_large_file())");
    println!("✓ 需要进度/取消: 使用流式 API (stream_query())");

    Ok(())
}
