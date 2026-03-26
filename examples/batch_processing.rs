//! 批量处理示例
//!
//! 本示例展示如何高效地批量处理多个文件的元数据。
//! 适用于处理大量照片的场景，如批量添加水印、批量修改日期等。
//!
//! 运行方式:
//! ```bash
//! cargo run --example batch_processing
//! ```

use exiftool_rs_wrapper::ExifTool;
use std::path::{Path, PathBuf};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ExifTool 批量处理示例 ===\n");

    // 初始化 ExifTool
    let exiftool = ExifTool::new()?;
    println!("✅ ExifTool 实例创建成功\n");

    // 查找示例图片
    let test_images = find_test_images();
    if test_images.is_empty() {
        println!("⚠️  未找到测试图片，示例将以演示模式运行");
        println!("   请在 examples 目录下放置一些 .jpg 图片用于测试");
        return Ok(());
    }

    println!("📁 找到 {} 个测试图片\n", test_images.len());

    // ============================================================
    // 1. 批量查询元数据
    // ============================================================
    println!("1️⃣  批量查询元数据（单次 ExifTool 调用）");
    println!("   这比逐个文件查询更高效\n");

    let start = Instant::now();
    let results = exiftool
        .query_batch(&test_images)
        .tag("FileName")
        .tag("ImageSize")
        .tag("Make")
        .tag("Model")
        .tag("DateTimeOriginal")
        .execute();

    match results {
        Ok(metadata_list) => {
            let duration = start.elapsed();
            println!("   ✅ 批量查询完成");
            println!("   ⏱️  耗时: {:?}", duration);
            println!("   📊 处理 {} 个文件\n", metadata_list.len());

            // 显示结果
            for (i, (path, metadata)) in metadata_list.iter().enumerate() {
                let filename = metadata
                    .get("FileName")
                    .map(|v| v.to_string_lossy())
                    .unwrap_or_else(|| {
                        path.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string()
                    });

                let make = metadata
                    .get("Make")
                    .map(|v| v.to_string_lossy())
                    .unwrap_or_default();
                let model = metadata
                    .get("Model")
                    .map(|v| v.to_string_lossy())
                    .unwrap_or_default();
                let date = metadata
                    .get("DateTimeOriginal")
                    .map(|v| v.to_string_lossy())
                    .unwrap_or_default();

                println!("   {}. {}", i + 1, filename);
                if !make.is_empty() {
                    println!("      📷 {} {}", make, model);
                }
                if !date.is_empty() {
                    println!("      📅 {}", date);
                }
            }
        }
        Err(e) => eprintln!("   ❌ 批量查询失败: {}\n", e),
    }
    println!();

    // ============================================================
    // 2. 批量写入元数据（批量处理）
    // ============================================================
    println!("2️⃣  批量添加版权信息");
    println!("   为所有图片添加相同的版权信息\n");

    // 创建测试副本
    let test_copies: Vec<PathBuf> = test_images
        .iter()
        .map(|p| create_test_copy(p).unwrap_or_else(|_| p.clone()))
        .collect();

    let copyright = "© 2026 Batch Processing Example";
    let artist = "Example Photographer";

    let mut success_count = 0;
    let mut fail_count = 0;

    let start = Instant::now();

    for path in &test_copies {
        match exiftool
            .write(path)
            .tag("Copyright", copyright)
            .tag("Artist", artist)
            .overwrite_original(true)
            .execute()
        {
            Ok(_) => success_count += 1,
            Err(e) => {
                eprintln!("   ❌ 写入失败 {}: {}", path.display(), e);
                fail_count += 1;
            }
        }
    }

    let duration = start.elapsed();
    println!("   ✅ 成功: {} 个文件", success_count);
    if fail_count > 0 {
        println!("   ❌ 失败: {} 个文件", fail_count);
    }
    println!("   ⏱️  耗时: {:?}\n", duration);

    // ============================================================
    // 3. 批量验证写入结果
    // ============================================================
    println!("3️⃣  批量验证写入结果");

    match exiftool
        .query_batch(&test_copies)
        .tag("FileName")
        .tag("Copyright")
        .tag("Artist")
        .execute()
    {
        Ok(results) => {
            println!("   ✅ 验证结果:");
            for (path, metadata) in results {
                let filename = path.file_name().unwrap_or_default().to_string_lossy();
                let file_copyright = metadata
                    .get("Copyright")
                    .map(|v| v.to_string_lossy())
                    .unwrap_or_default();

                if file_copyright == copyright {
                    println!("      ✓ {} - 版权信息正确", filename);
                } else {
                    println!(
                        "      ✗ {} - 版权信息不匹配: '{}'",
                        filename, file_copyright
                    );
                }
            }
        }
        Err(e) => eprintln!("   ❌ 验证失败: {}", e),
    }
    println!();

    // ============================================================
    // 4. 批量删除元数据
    // ============================================================
    println!("4️⃣  批量删除特定标签");
    println!("   从所有文件中删除 Comment 和 UserComment 标签\n");

    let mut delete_success = 0;
    for path in &test_copies {
        match exiftool
            .write(path)
            .delete("Comment")
            .delete("UserComment")
            .overwrite_original(true)
            .execute()
        {
            Ok(_) => delete_success += 1,
            Err(e) => eprintln!("   ⚠️  {}: {}", path.display(), e),
        }
    }
    println!("   ✅ 成功处理 {} 个文件\n", delete_success);

    // ============================================================
    // 5. 批量复制元数据
    // ============================================================
    println!("5️⃣  批量复制元数据");
    println!("   将第一个文件的元数据复制到其他文件\n");

    if test_copies.len() >= 2 {
        let source = &test_copies[0];
        let targets = &test_copies[1..];

        println!(
            "   📤 源文件: {}",
            source.file_name().unwrap_or_default().to_string_lossy()
        );
        println!("   📥 目标文件数: {}\n", targets.len());

        for (i, target) in targets.iter().enumerate() {
            match exiftool
                .write(target)
                .copy_from(source)
                .overwrite_original(true)
                .execute()
            {
                Ok(_) => println!(
                    "   ✅ {} 复制完成",
                    target.file_name().unwrap_or_default().to_string_lossy()
                ),
                Err(e) => eprintln!(
                    "   ❌ {}: {}",
                    target.file_name().unwrap_or_default().to_string_lossy(),
                    e
                ),
            }

            // 每处理 5 个文件显示一次进度
            if (i + 1) % 5 == 0 {
                println!("   📊 进度: {}/{}\n", i + 1, targets.len());
            }
        }
    }
    println!();

    // ============================================================
    // 6. 处理结果统计
    // ============================================================
    println!("6️⃣  批量处理结果统计");

    match exiftool.query_batch(&test_copies).tag("FileName").execute() {
        Ok(results) => {
            println!("   📊 总文件数: {}", results.len());

            // 统计相机制造商
            let mut make_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            for (_, metadata) in &results {
                if let Some(make) = metadata.get("Make") {
                    let make_str = make.to_string_lossy();
                    *make_counts.entry(make_str).or_insert(0) += 1;
                }
            }

            if !make_counts.is_empty() {
                println!("   📷 相机制造商统计:");
                for (make, count) in make_counts {
                    println!("      • {}: {} 张", make, count);
                }
            }
        }
        Err(e) => eprintln!("   ❌ 统计失败: {}", e),
    }
    println!();

    // ============================================================
    // 7. 清理
    // ============================================================
    println!("7️⃣  清理测试文件");

    let mut cleaned = 0;
    for path in &test_copies {
        if path.exists() && path != &test_images[0] {
            match std::fs::remove_file(path) {
                Ok(_) => cleaned += 1,
                Err(e) => eprintln!("   ⚠️  删除 {} 失败: {}", path.display(), e),
            }
        }
    }
    println!("   ✅ 已清理 {} 个测试文件\n", cleaned);

    // 关闭 ExifTool
    exiftool.close()?;
    println!("✅ ExifTool 已关闭");
    println!("\n✨ 批量处理示例完成！");

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

    // 如果没有找到，尝试当前目录
    if images.is_empty()
        && let Ok(entries) = std::fs::read_dir(".")
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext == "jpg" || ext == "jpeg" {
                    images.push(path);
                    if images.len() >= 5 {
                        break;
                    }
                }
            }
        }
    }

    images
}

/// 创建测试副本
fn create_test_copy(original: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let stem = original.file_stem().unwrap_or_default().to_string_lossy();
    let ext = original
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();

    let copy_path = original.with_file_name(format!("{}_batch_test{}", stem, ext));
    std::fs::copy(original, &copy_path)?;

    Ok(copy_path)
}
