//! 异步 API 使用示例
//!
//! 本示例展示如何使用 exiftool-rs-wrapper 的异步 API。
//! 需要在 Cargo.toml 中启用 "async" 特性：
//!
//! ```toml
//! [dependencies]
//! exiftool-rs-wrapper = { version = "0.1.0", features = ["async"] }
//! tokio = { version = "1.0", features = ["full"] }
//! ```
//!
//! 运行方式:
//! ```bash
//! cargo run --example async_usage --features async
//! ```

use exiftool_rs_wrapper::{
    RetryPolicy, TagId,
    async_ext::{AsyncExifTool, process_files_parallel, read_metadata_parallel},
    with_retry,
};
use std::path::PathBuf;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ExifTool 异步 API 使用示例 ===\n");

    // 查找测试图片
    let test_images = find_test_images();
    if test_images.is_empty() {
        println!("⚠️  未找到测试图片，示例将以演示模式运行");
        println!("   请在 examples 目录下放置一些 .jpg 图片用于测试");
        return Ok(());
    }

    println!("📁 找到 {} 个测试图片\n", test_images.len());

    // ============================================================
    // 1. 创建异步 ExifTool 实例
    // ============================================================
    println!("1️⃣  创建异步 ExifTool 实例");

    let exiftool = match AsyncExifTool::new() {
        Ok(et) => {
            println!("   ✅ 异步 ExifTool 实例创建成功\n");
            et
        }
        Err(e) => {
            eprintln!("   ❌ 创建失败: {}", e);
            return Err(e.into());
        }
    };

    // 获取版本
    match exiftool.version().await {
        Ok(version) => println!("   📦 ExifTool 版本: {}\n", version),
        Err(e) => println!("   ⚠️  无法获取版本: {}\n", e),
    }

    // ============================================================
    // 2. 异步查询单个文件
    // ============================================================
    println!("2️⃣  异步查询单个文件");

    let first_image = &test_images[0];
    println!("   📷 查询: {}", first_image.display());

    let start = tokio::time::Instant::now();

    match exiftool.query(first_image).await {
        Ok(metadata) => {
            let duration = start.elapsed();
            println!("   ✅ 查询成功（耗时 {:?}）", duration);
            println!("   📊 找到 {} 个标签", metadata.len());

            if let Some(make) = metadata.get("Make") {
                println!("   📷 制造商: {}", make);
            }
            if let Some(model) = metadata.get("Model") {
                println!("   📷 型号: {}", model);
            }
        }
        Err(e) => eprintln!("   ❌ 查询失败: {}", e),
    }
    println!();

    // ============================================================
    // 3. 异步读取特定标签
    // ============================================================
    println!("3️⃣  异步读取特定标签");

    let tasks = vec![
        (
            "Make",
            exiftool.read_tag::<String, _, _>(first_image, TagId::MAKE),
        ),
        (
            "Model",
            exiftool.read_tag::<String, _, _>(first_image, TagId::MODEL),
        ),
        (
            "DateTimeOriginal",
            exiftool.read_tag::<String, _, _>(first_image, TagId::DATE_TIME_ORIGINAL),
        ),
    ];

    for (tag_name, task) in tasks {
        match task.await {
            Ok(value) => println!("   ✅ {}: {}", tag_name, value),
            Err(e) => println!("   ⚠️  {}: {}", tag_name, e),
        }
    }
    println!();

    // ============================================================
    // 4. 异步批量查询
    // ============================================================
    println!("4️⃣  异步批量查询所有文件");

    let start = tokio::time::Instant::now();

    match exiftool.query_batch(&test_images).await {
        Ok(results) => {
            let duration = start.elapsed();
            println!("   ✅ 批量查询完成（耗时 {:?}）", duration);
            println!("   📊 处理 {} 个文件\n", results.len());

            for (i, (path, metadata)) in results.iter().enumerate() {
                let filename = path.file_name().unwrap_or_default().to_string_lossy();
                println!("   {}. {}", i + 1, filename);

                if let Some(make) = metadata.get("Make") {
                    if let Some(model) = metadata.get("Model") {
                        println!("      📷 {} {}", make, model);
                    }
                }

                if let Some(size) = metadata.get("ImageSize") {
                    println!("      📐 尺寸: {}", size);
                }
            }
        }
        Err(e) => eprintln!("   ❌ 批量查询失败: {}\n", e),
    }
    println!();

    // ============================================================
    // 5. 并行处理多个文件
    // ============================================================
    println!("5️⃣  并行处理（使用 process_files_parallel）");
    println!("   使用 futures::stream 实现高并发处理\n");

    let start = tokio::time::Instant::now();

    let results = process_files_parallel(
        test_images.clone(),
        4, // 并发数
        |path| {
            let et = exiftool.clone();
            async move {
                let metadata = et.query(&path).await?;
                let filename = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                Ok::<(String, usize), exiftool_rs_wrapper::Error>((filename, metadata.len()))
            }
        },
    )
    .await;

    let duration = start.elapsed();
    println!("   ✅ 并行处理完成（并发度: 4，耗时 {:?}）", duration);

    let mut success_count = 0;
    let mut fail_count = 0;

    for result in &results {
        match result {
            Ok((filename, tag_count)) => {
                success_count += 1;
                if success_count <= 5 {
                    println!("   ✓ {} - {} 个标签", filename, tag_count);
                }
            }
            Err(_) => fail_count += 1,
        }
    }

    if success_count > 5 {
        println!("   ... 还有 {} 个文件", success_count - 5);
    }
    println!("   📊 成功: {}, 失败: {}\n", success_count, fail_count);

    // ============================================================
    // 6. 并发元数据读取
    // ============================================================
    println!("6️⃣  并发元数据读取（read_metadata_parallel）");

    let start = tokio::time::Instant::now();

    let parallel_results = read_metadata_parallel(
        exiftool.clone(),
        test_images.clone(),
        3, // 并发数
    )
    .await;

    let duration = start.elapsed();
    println!("   ✅ 并发读取完成（并发度: 3，耗时 {:?}）", duration);
    println!("   📊 读取 {} 个文件的元数据\n", parallel_results.len());

    // ============================================================
    // 7. 异步写入标签
    // ============================================================
    println!("7️⃣  异步写入标签");

    if !test_images.is_empty() {
        // 创建测试副本
        let test_copy = create_test_copy(&test_images[0])?;
        println!("   📝 写入到: {}", test_copy.display());

        match exiftool
            .write_tag(&test_copy, "Copyright", "© 2026 Async Example")
            .await
        {
            Ok(_) => {
                println!("   ✅ 异步写入成功");

                // 验证
                match exiftool
                    .read_tag::<String, _, _>(&test_copy, "Copyright")
                    .await
                {
                    Ok(value) => println!("   📋 验证: {}", value),
                    Err(e) => println!("   ⚠️  验证失败: {}", e),
                }
            }
            Err(e) => eprintln!("   ❌ 写入失败: {}", e),
        }

        // 清理
        if test_copy.exists() {
            std::fs::remove_file(&test_copy)?;
            println!("   ✅ 测试文件已清理");
        }
    }
    println!();

    // ============================================================
    // 8. 异步删除标签
    // ============================================================
    println!("8️⃣  异步删除标签");

    if test_images.len() > 1 {
        let test_copy = create_test_copy(&test_images[1])?;

        match exiftool.delete_tag(&test_copy, "Comment").await {
            Ok(_) => println!("   ✅ 异步删除成功"),
            Err(e) => println!("   ⚠️  删除结果: {} (可能标签不存在)", e),
        }

        if test_copy.exists() {
            std::fs::remove_file(&test_copy)?;
            println!("   ✅ 测试文件已清理");
        }
    }
    println!();

    // ============================================================
    // 9. 带重试的异步操作
    // ============================================================
    println!("9️⃣  带重试机制的异步操作");

    let retry_policy = RetryPolicy::new(3)
        .initial_delay(Duration::from_millis(100))
        .backoff(2.0);

    println!(
        "   🔄 重试策略: 最多 {} 次, 初始延迟 {:?}",
        retry_policy.max_attempts, retry_policy.initial_delay
    );

    let start = tokio::time::Instant::now();

    let result = with_retry(&retry_policy, || async {
        exiftool.query(first_image).await
    })
    .await;

    let duration = start.elapsed();

    match result {
        Ok(metadata) => {
            println!("   ✅ 带重试的查询成功（总耗时 {:?}）", duration);
            println!("   📊 获取 {} 个标签", metadata.len());
        }
        Err(e) => eprintln!("   ❌ 查询最终失败: {}", e),
    }
    println!();

    // ============================================================
    // 10. 异步批量写入
    // ============================================================
    println!("🔟  异步批量写入");

    let test_copies: Vec<PathBuf> = test_images
        .iter()
        .take(3)
        .map(|p| create_test_copy(p).unwrap_or_else(|_| p.clone()))
        .collect();

    if !test_copies.is_empty() {
        println!("   📝 批量添加版权信息到 {} 个文件", test_copies.len());

        let operations: Vec<_> = test_copies
            .iter()
            .map(|path| {
                let path = path.clone();
                (path, |builder: exiftool_rs_wrapper::WriteBuilder<'_>| {
                    builder
                        .tag("Copyright", "© 2026 Batch Async")
                        .overwrite_original(true)
                })
            })
            .collect();

        let results = exiftool.write_batch(operations).await;

        let success = results.iter().filter(|r| r.is_ok()).count();
        let failed = results.iter().filter(|r| r.is_err()).count();

        println!("   ✅ 成功: {}, ❌ 失败: {}", success, failed);

        // 清理
        for file in test_copies {
            if file.exists() {
                let _ = std::fs::remove_file(&file);
            }
        }
        println!("   ✅ 测试文件已清理");
    }
    println!();

    // ============================================================
    // 11. 异步关闭
    // ============================================================
    println!("1️⃣1️⃣  异步关闭 ExifTool");

    match exiftool.close().await {
        Ok(_) => println!("   ✅ 异步关闭成功"),
        Err(e) => eprintln!("   ⚠️  关闭时出错: {}", e),
    }

    println!("\n✨ 异步 API 示例完成！");

    Ok(())
}

/// 查找测试图片
fn find_test_images() -> Vec<PathBuf> {
    let mut images = Vec::new();

    if let Ok(entries) = std::fs::read_dir("examples") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "tiff" {
                    images.push(path);
                    if images.len() >= 10 {
                        break;
                    }
                }
            }
        }
    }

    images
}

/// 创建测试副本
fn create_test_copy(original: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let copy_path = original.with_file_name(format!(
        "{}_async_test.{}",
        original.file_stem().unwrap_or_default().to_string_lossy(),
        original.extension().unwrap_or_default().to_string_lossy()
    ));

    std::fs::copy(original, &copy_path)?;
    Ok(copy_path)
}
