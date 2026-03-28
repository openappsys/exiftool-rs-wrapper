//! 基本用法示例
//!
//! 本示例展示如何使用 exiftool-rs-wrapper 进行基本的元数据读取和写入操作。
//!
//! 运行方式:
//! ```bash
//! cargo run --example basic_usage
//! ```

use exiftool_rs_wrapper::{ExifTool, TagId};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ExifTool 基本用法示例 ===\n");

    // 检查是否有示例图片
    let test_image = find_test_image();
    if test_image.is_none() {
        println!("⚠️  未找到测试图片，示例将以演示模式运行");
        println!("   请在 examples 目录下放置一张名为 'sample.jpg' 的图片");
        return Ok(());
    }

    let image_path = test_image.unwrap();
    println!("📷 使用图片: {}\n", image_path.display());

    // ============================================================
    // 1. 创建 ExifTool 实例
    // ============================================================
    println!("1️⃣  创建 ExifTool 实例");
    println!("   使用 -stay_open 模式保持进程运行以获得最佳性能");

    let exiftool = match ExifTool::new() {
        Ok(et) => {
            println!("   ✅ ExifTool 实例创建成功");
            et
        }
        Err(e) => {
            eprintln!("   ❌ 创建失败: {}", e);
            eprintln!("   请确保已安装 ExifTool: https://exiftool.org/");
            return Err(e.into());
        }
    };

    // 获取 ExifTool 版本
    match exiftool.version() {
        Ok(version) => println!("   📦 ExifTool 版本: {}\n", version),
        Err(e) => println!("   ⚠️  无法获取版本: {}\n", e),
    }

    // ============================================================
    // 2. 读取元数据 - 基本查询
    // ============================================================
    println!("2️⃣  读取文件元数据（完整）");

    match exiftool.query(&image_path).execute() {
        Ok(metadata) => {
            println!("   ✅ 成功读取 {} 个标签\n", metadata.len());

            // 显示一些基本信息
            if let Some(make) = metadata.get("Make") {
                println!("   📷 相机制造商: {}", make);
            }
            if let Some(model) = metadata.get("Model") {
                println!("   📷 相机型号: {}", model);
            }
            if let Some(date) = metadata.get("DateTimeOriginal") {
                println!("   📅 拍摄时间: {}", date);
            }
        }
        Err(e) => {
            eprintln!("   ❌ 读取失败: {}", e);
        }
    }
    println!();

    // ============================================================
    // 3. 读取特定标签
    // ============================================================
    println!("3️⃣  读取特定标签");

    // 使用字符串标签名
    match exiftool.read_tag::<String, _, _>(&image_path, "Make") {
        Ok(make) => println!("   📷 相机制造商: {}", make),
        Err(e) => println!("   ⚠️  Make 标签: {}", e),
    }

    // 使用 TagId 常量（类型安全）- 通过 name() 方法获取字符串
    match exiftool.read_tag::<String, _, _>(&image_path, TagId::Model.name()) {
        Ok(model) => println!("   📷 相机型号: {}", model),
        Err(e) => println!("   ⚠️  Model 标签: {}", e),
    }

    // 读取图像尺寸
    if let Ok(width) = exiftool.read_tag::<i64, _, _>(&image_path, TagId::ImageWidth.name())
        && let Ok(height) = exiftool.read_tag::<i64, _, _>(&image_path, TagId::ImageHeight.name())
    {
        println!("   📐 图像尺寸: {} x {} 像素", width, height);
    }
    println!();

    // ============================================================
    // 4. 高级查询选项
    // ============================================================
    println!("4️⃣  使用高级查询选项");

    let metadata = exiftool
        .query(&image_path)
        .tag("Make") // 仅查询 Make 标签
        .tag("Model") // 仅查询 Model 标签
        .tag("DateTimeOriginal") // 仅查询拍摄时间
        .raw_values(true) // 返回原始数值
        .include_unknown(true) // 包含未知标签
        .execute();

    match metadata {
        Ok(meta) => {
            println!("   ✅ 过滤后的元数据（{} 个标签）:", meta.len());
            for (tag, value) in meta.iter() {
                println!("      • {} = {}", tag, value);
            }
        }
        Err(e) => eprintln!("   ❌ 查询失败: {}", e),
    }
    println!();

    // ============================================================
    // 5. 写入元数据
    // ============================================================
    println!("5️⃣  写入元数据");
    println!("   ⚠️  注意: 这将修改图片文件");
    println!("   建议在测试前备份图片\n");

    // 创建示例副本用于演示写入
    let test_copy = create_test_copy(&image_path)?;

    // 写入版权信息
    match exiftool
        .write(&test_copy)
        .tag("Copyright", "© 2026 Example Company")
        .tag("Artist", "Example Photographer")
        .overwrite_original(true) // 覆盖原文件（不创建备份）
        .execute()
    {
        Ok(result) => {
            println!("   ✅ 写入成功");
            if let Some(count) = result.updated_count() {
                println!("   📊 更新了 {} 个文件", count);
            }

            // 验证写入
            match exiftool.read_tag::<String, _, _>(&test_copy, "Copyright") {
                Ok(copyright) => println!("   📋 验证 - 版权: {}", copyright),
                Err(e) => println!("   ⚠️  验证失败: {}", e),
            }
        }
        Err(e) => eprintln!("   ❌ 写入失败: {}", e),
    }
    println!();

    // ============================================================
    // 6. 删除元数据标签
    // ============================================================
    println!("6️⃣  删除元数据标签");

    match exiftool
        .write(&test_copy)
        .delete("Comment") // 删除 Comment 标签
        .delete("UserComment") // 删除 UserComment 标签
        .overwrite_original(true)
        .execute()
    {
        Ok(_) => println!("   ✅ 成功删除指定标签"),
        Err(e) => eprintln!("   ❌ 删除失败: {}", e),
    }
    println!();

    // ============================================================
    // 7. 列出支持的标签
    // ============================================================
    println!("7️⃣  列出支持的标签（前 10 个）");

    match exiftool.list_tags() {
        Ok(tags) => {
            println!("   📚 共支持 {} 个标签，显示前 10 个:", tags.len());
            for (i, tag) in tags.iter().take(10).enumerate() {
                println!("      {}. {}", i + 1, tag);
            }
        }
        Err(e) => eprintln!("   ❌ 获取标签列表失败: {}", e),
    }
    println!();

    // ============================================================
    // 8. 清理
    // ============================================================
    println!("8️⃣  清理资源");

    // 关闭 ExifTool 进程（Drop 会自动处理，但也可以手动关闭）
    match exiftool.close() {
        Ok(_) => println!("   ✅ ExifTool 进程已关闭"),
        Err(e) => eprintln!("   ⚠️  关闭时出错: {}", e),
    }

    // 删除测试副本
    if test_copy.exists() {
        std::fs::remove_file(&test_copy)?;
        println!("   ✅ 测试文件已清理");
    }

    println!("\n✨ 示例运行完成！");

    Ok(())
}

/// 查找测试图片
fn find_test_image() -> Option<PathBuf> {
    // 检查常见的测试图片位置
    let possible_paths = vec![
        PathBuf::from("examples/sample.jpg"),
        PathBuf::from("examples/test.jpg"),
        PathBuf::from("sample.jpg"),
        PathBuf::from("test.jpg"),
    ];

    possible_paths.into_iter().find(|path| path.exists())
}

/// 创建测试副本
fn create_test_copy(original: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let copy_path = original.with_file_name(format!(
        "{}_test_copy{}",
        original.file_stem().unwrap_or_default().to_string_lossy(),
        original
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default()
    ));

    std::fs::copy(original, &copy_path)?;

    Ok(copy_path)
}
