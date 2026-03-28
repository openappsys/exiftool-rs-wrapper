//! 高级功能示例
//!
//! 本示例展示 exiftool-rs-wrapper 的高级功能，包括：
//! - 日期时间偏移
//! - 数值运算
//! - 校验和计算
//! - 文件比较
//! - 十六进制转储
//! - 详细输出
//!
//! 运行方式:
//! ```bash
//! cargo run --example advanced_features
//! ```

use exiftool_rs_wrapper::{
    AdvancedWriteOperations, ConfigOperations, DateTimeOffset, DiffResult, ExifTool,
    HexDumpOperations, HexDumpOptions, NumericOperation, TagId, VerboseOperations, VerboseOptions,
};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ExifTool 高级功能示例 ===\n");

    let exiftool = ExifTool::new()?;
    println!("✅ ExifTool 实例创建成功\n");

    // 查找测试图片
    let test_image = find_test_image();
    if test_image.is_none() {
        println!("⚠️  未找到测试图片，示例将以演示模式运行");
        println!("   请在 examples 目录下放置一张图片用于测试");
        return Ok(());
    }

    let image_path = test_image.unwrap();
    println!("📷 使用图片: {}\n", image_path.display());

    // ============================================================
    // 1. 日期时间偏移
    // ============================================================
    println!("1️⃣  日期时间偏移");
    println!("   用于纠正相机时间设置错误或调整时区\n");

    // 创建测试副本
    let test_copy = create_test_copy(&image_path)?;

    // 1.1 查看原始日期
    println!("   📅 原始日期:");
    match exiftool.read_tag::<String, _, _>(&test_copy, TagId::DATE_TIME_ORIGINAL.name()) {
        Ok(date) => println!("      DateTimeOriginal: {}", date),
        Err(_) => println!("      DateTimeOriginal: [未设置]"),
    }

    // 1.2 增加 2 小时（模拟时区调整）
    println!("\n   ⏰ 执行操作: 增加 2 小时");
    let offset = DateTimeOffset::new().hours(2);

    match exiftool.shift_datetime(&test_copy, offset) {
        Ok(_) => {
            println!("   ✅ 日期时间偏移成功");

            // 验证结果
            match exiftool.read_tag::<String, _, _>(&test_copy, TagId::DATE_TIME_ORIGINAL.name()) {
                Ok(new_date) => println!("      新 DateTimeOriginal: {}", new_date),
                Err(e) => println!("      ⚠️  读取失败: {}", e),
            }
        }
        Err(e) => eprintln!("   ❌ 偏移失败: {}", e),
    }

    // 1.3 减少 1 天（模拟日期修正）
    println!("\n   ⏰ 执行操作: 减少 1 天");
    let offset = DateTimeOffset::new().days(-1);

    match exiftool.shift_datetime(&test_copy, offset) {
        Ok(_) => {
            println!("   ✅ 日期时间偏移成功");

            if let Ok(new_date) =
                exiftool.read_tag::<String, _, _>(&test_copy, TagId::DATE_TIME_ORIGINAL.name())
            {
                println!("      最终 DateTimeOriginal: {}", new_date);
            }
        }
        Err(e) => eprintln!("   ❌ 偏移失败: {}", e),
    }
    println!();

    // ============================================================
    // 2. 特定标签的日期偏移
    // ============================================================
    println!("2️⃣  仅偏移特定日期时间标签");

    let test_copy2 = create_test_copy(&image_path)?;

    // 仅偏移 CreateDate，保持 DateTimeOriginal 不变
    match exiftool.shift_specific_datetime(
        &test_copy2,
        TagId::CREATE_DATE,
        DateTimeOffset::new().hours(3),
    ) {
        Ok(_) => println!("   ✅ CreateDate 偏移成功 (+3小时)"),
        Err(e) => eprintln!("   ❌ 偏移失败: {}", e),
    }
    println!();

    // ============================================================
    // 3. 数值运算
    // ============================================================
    println!("3️⃣  数值运算（标签值加减乘除）");
    println!("   用于批量调整曝光补偿等数值标签\n");

    // 示例：给曝光补偿增加 0.5
    // 注意：实际图片可能没有 ExposureCompensation 标签
    println!("   尝试增加曝光补偿值 (+0.5 EV)...");

    match exiftool.numeric_operation(
        &test_copy,
        TagId::EXPOSURE_COMPENSATION,
        NumericOperation::Add(5), // 假设以 0.1 为单位存储
    ) {
        Ok(_) => println!("   ✅ 数值运算成功"),
        Err(e) => println!("   ⚠️  运算结果: {} (可能标签不存在)", e),
    }
    println!();

    // ============================================================
    // 4. 校验和计算（已废弃，仅展示说明）
    // ============================================================
    println!("4️⃣  校验和计算");
    println!("   注意: ExifTool 不原生支持校验和计算，该功能已废弃");
    println!("   建议使用 Rust 标准库或 sha2/md5 crate 直接计算文件校验和");
    println!();

    // ============================================================
    // 5. 文件比较
    // ============================================================
    println!("5️⃣  比较两个文件的元数据");

    if test_copy.exists() {
        match exiftool.diff(&image_path, &test_copy) {
            Ok(diff) => {
                print_diff_result(&diff);
            }
            Err(e) => eprintln!("   ❌ 比较失败: {}", e),
        }
    }
    println!();

    // ============================================================
    // 6. 比较特定标签
    // ============================================================
    println!("6️⃣  仅比较特定标签");

    let tags_to_compare = vec![
        TagId::MAKE,
        TagId::MODEL,
        TagId::DATE_TIME_ORIGINAL,
        TagId::COPYRIGHT,
    ];

    match exiftool.diff_tags(&image_path, &test_copy, &tags_to_compare) {
        Ok(diff) => {
            println!("   📋 比较标签: Make, Model, DateTimeOriginal, Copyright");
            print_diff_result(&diff);
        }
        Err(e) => eprintln!("   ❌ 比较失败: {}", e),
    }
    println!();

    // ============================================================
    // 7. 十六进制转储
    // ============================================================
    println!("7️⃣  十六进制转储");
    println!("   显示文件头部或特定标签的二进制数据\n");

    // 7.1 文件头部转储
    let hex_opts = HexDumpOptions::new().length(64).bytes_per_line(16);

    match exiftool.hex_dump(&image_path, &hex_opts) {
        Ok(hex) => {
            println!("   📄 文件头部 64 字节:");
            for (i, line) in hex.lines().take(5).enumerate() {
                println!("      {:04x}: {}", i * 16, line);
            }
        }
        Err(e) => eprintln!("   ❌ 转储失败: {}", e),
    }

    // 7.2 特定标签的十六进制值
    println!("\n   🏷️  Make 标签的十六进制值:");
    match exiftool.hex_dump_tag(&image_path, TagId::MAKE) {
        Ok(hex) => {
            if hex.trim().is_empty() {
                println!("      [标签不存在或没有二进制数据]");
            } else {
                for line in hex.lines().take(3) {
                    println!("      {}", line);
                }
            }
        }
        Err(e) => println!("      ⚠️  {}", e),
    }
    println!();

    // ============================================================
    // 8. 详细输出
    // ============================================================
    println!("8️⃣  详细输出（Verbose Output）");
    println!("   显示 ExifTool 处理的详细信息\n");

    let verbose_opts = VerboseOptions::new(2); // 详细级别 2

    match exiftool.verbose_dump(&image_path, &verbose_opts) {
        Ok(output) => {
            let lines: Vec<&str> = output.lines().collect();
            println!("   📊 详细输出（前 10 行）:");
            for line in lines.iter().take(10) {
                println!("      {}", line);
            }
            if lines.len() > 10 {
                println!("      ... (共 {} 行)", lines.len());
            }
        }
        Err(e) => eprintln!("   ❌ 获取详细输出失败: {}", e),
    }
    println!();

    // ============================================================
    // 9. 字符串追加
    // ============================================================
    println!("9️⃣  字符串追加操作");

    let test_copy3 = create_test_copy(&image_path)?;

    // 首先设置基础值
    match exiftool
        .write(&test_copy3)
        .tag("Copyright", "© 2026")
        .overwrite_original(true)
        .execute()
    {
        Ok(_) => {
            // 追加字符串
            match exiftool.append_string(&test_copy3, TagId::COPYRIGHT, " Example Company") {
                Ok(_) => {
                    println!("   ✅ 字符串追加成功");

                    // 验证
                    if let Ok(value) =
                        exiftool.read_tag::<String, _, _>(&test_copy3, TagId::COPYRIGHT.name())
                    {
                        println!("      最终值: {}", value)
                    }
                }
                Err(e) => eprintln!("   ❌ 追加失败: {}", e),
            }
        }
        Err(e) => eprintln!("   ❌ 初始设置失败: {}", e),
    }
    println!();

    // ============================================================
    // 10. 清理
    // ============================================================
    println!("🔟  清理测试文件");

    let test_files = vec![test_copy, test_copy2, test_copy3];
    let mut cleaned = 0;

    for file in test_files {
        if file.exists() && file != image_path && std::fs::remove_file(&file).is_ok() {
            cleaned += 1;
        }
    }

    println!("   ✅ 已清理 {} 个测试文件", cleaned);

    exiftool.close()?;
    println!("\n✅ ExifTool 已关闭");
    println!("\n✨ 高级功能示例完成！");

    Ok(())
}

/// 打印比较结果
fn print_diff_result(diff: &DiffResult) {
    if diff.is_identical {
        println!("   ✅ 两个文件元数据完全相同");
        return;
    }

    println!("   📊 差异分析:");

    if !diff.source_only.is_empty() {
        println!("      📄 仅在源文件存在的标签:");
        for tag in &diff.source_only {
            println!("         • {}", tag);
        }
    }

    if !diff.target_only.is_empty() {
        println!("      📄 仅在目标文件存在的标签:");
        for tag in &diff.target_only {
            println!("         • {}", tag);
        }
    }

    if !diff.different.is_empty() {
        println!("      📄 值不同的标签:");
        for (tag, source_val, target_val) in &diff.different {
            println!("         • {}:", tag);
            println!("           源: {}", source_val);
            println!("           目标: {}", target_val);
        }
    }
}

/// 查找测试图片
fn find_test_image() -> Option<PathBuf> {
    let paths = vec![
        PathBuf::from("examples/sample.jpg"),
        PathBuf::from("examples/test.jpg"),
        PathBuf::from("sample.jpg"),
        PathBuf::from("test.jpg"),
    ];

    paths.into_iter().find(|path| path.exists())
}

/// 创建测试副本
fn create_test_copy(original: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let copy_path = original.with_file_name(format!(
        "{}_advanced_test.{}",
        original.file_stem().unwrap_or_default().to_string_lossy(),
        original.extension().unwrap_or_default().to_string_lossy()
    ));

    std::fs::copy(original, &copy_path)?;
    Ok(copy_path)
}
