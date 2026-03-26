//! 错误处理和重试示例
//!
//! 本示例展示如何处理各种错误情况以及实现重试机制。
//! 包括：错误分类、可恢复错误、重试策略、批量操作的部分成功处理等。
//!
//! 运行方式:
//! ```bash
//! cargo run --example error_handling
//! ```

use exiftool_rs_wrapper::{
    BatchResult, Error, ExifTool, Recoverable, RetryPolicy, with_retry_sync,
};
use std::path::PathBuf;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ExifTool 错误处理和重试示例 ===\n");

    // ============================================================
    // 1. ExifTool 未安装的错误处理
    // ============================================================
    println!("1️⃣  ExifTool 未安装的错误处理");

    let exiftool = match ExifTool::new() {
        Ok(et) => {
            println!("   ✅ ExifTool 已安装并可用\n");
            et
        }
        Err(Error::ExifToolNotFound) => {
            eprintln!("   ❌ ExifTool 未找到！");
            eprintln!("   💡 请按照以下步骤安装 ExifTool:");
            eprintln!("      • macOS: brew install exiftool");
            eprintln!("      • Ubuntu/Debian: sudo apt-get install libimage-exiftool-perl");
            eprintln!("      • Windows: 从 https://exiftool.org/ 下载安装");
            eprintln!("      • 其他系统: 访问 https://exiftool.org/install.html");
            return Err(Error::ExifToolNotFound.into());
        }
        Err(e) => {
            eprintln!("   ❌ 启动 ExifTool 失败: {}", e);
            return Err(e.into());
        }
    };

    // ============================================================
    // 2. 文件不存在的错误处理
    // ============================================================
    println!("2️⃣  文件不存在的错误处理");

    let non_existent = PathBuf::from("/path/that/does/not/exist.jpg");

    match exiftool.query(&non_existent).execute() {
        Ok(_) => {
            println!("   ⚠️  意外成功（文件居然存在）");
        }
        Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("   ✅ 正确捕获文件不存在错误");
            println!("      错误信息: {}", e);
            println!("      💡 建议: 请检查文件路径是否正确");
        }
        Err(Error::Process { message, exit_code }) => {
            println!("   ⚠️  ExifTool 进程错误");
            println!("      消息: {}", message);
            if let Some(code) = exit_code {
                println!("      退出码: {}", code);
            }
        }
        Err(e) => {
            println!("   ⚠️  其他错误: {}", e);
        }
    }
    println!();

    // ============================================================
    // 3. 标签不存在的错误处理
    // ============================================================
    println!("3️⃣  标签不存在的错误处理");

    // 查找一个存在的测试文件
    let test_image = find_test_image();
    if let Some(ref image) = test_image {
        println!("   📷 测试文件: {}", image.display());

        // 尝试读取可能不存在的标签
        match exiftool.read_tag::<String, _, _>(image, "NonExistentTag") {
            Ok(value) => {
                println!("   ⚠️  意外成功: {}", value);
            }
            Err(Error::TagNotFound(tag)) => {
                println!("   ✅ 正确捕获标签不存在错误");
                println!("      标签: {}", tag);
                println!("      💡 建议: 使用 query() 获取所有可用标签");
            }
            Err(e) => {
                println!("   ⚠️  其他错误: {}", e);
            }
        }
    }
    println!();

    // ============================================================
    // 4. 使用 Recoverable trait 检查错误是否可恢复
    // ============================================================
    println!("4️⃣  使用 Recoverable trait 分类错误");

    let test_errors = vec![
        Error::ExifToolNotFound,
        Error::Timeout,
        Error::TagNotFound("TestTag".to_string()),
        Error::InvalidArgument("bad arg".to_string()),
        Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test")),
    ];

    for error in test_errors {
        let is_recoverable = error.is_recoverable();
        let suggestion = error.recovery_suggestion();

        println!(
            "   {} {} - {}",
            if is_recoverable { "🔄" } else { "⛔" },
            error,
            if is_recoverable {
                "可恢复"
            } else {
                "不可恢复"
            }
        );

        if let Some(sugg) = suggestion {
            println!("      💡 {}", sugg);
        }
    }
    println!();

    // ============================================================
    // 5. 重试策略配置
    // ============================================================
    println!("5️⃣  重试策略配置");

    // 创建自定义重试策略
    let custom_policy = RetryPolicy::new(5) // 最多重试 5 次
        .initial_delay(Duration::from_millis(200)) // 初始延迟 200ms
        .backoff(1.5); // 退避倍数 1.5

    println!("   📋 自定义重试策略:");
    println!("      最大重试次数: {}", custom_policy.max_attempts);
    println!("      初始延迟: {:?}", custom_policy.initial_delay);
    println!("      退避倍数: {}", custom_policy.backoff_multiplier);

    // 计算各次重试的延迟
    println!("\n   ⏱️  预计重试时间线:");
    for i in 0..=custom_policy.max_attempts {
        let delay = custom_policy.delay_for_attempt(i);
        if i == 0 {
            println!("      第 1 次尝试: 立即执行");
        } else {
            println!("      第 {} 次尝试: 延迟 {:?}", i + 1, delay);
        }
    }
    println!();

    // ============================================================
    // 6. 使用重试机制（同步版本）
    // ============================================================
    println!("6️⃣  使用重试机制处理临时错误");

    use std::cell::Cell;
    let policy = RetryPolicy::new(3);
    let attempt = Cell::new(0);

    let result = with_retry_sync(&policy, || {
        let current = attempt.get() + 1;
        attempt.set(current);
        println!("   🔄 尝试 #{}", current);

        // 模拟前两次失败
        if current < 3 {
            println!("      ❌ 模拟失败（将重试）");
            Err(Error::Timeout)
        } else {
            println!("      ✅ 成功！");
            Ok::<String, Error>("操作成功".to_string())
        }
    });

    match result {
        Ok(msg) => println!("   ✅ 最终结果: {}\n", msg),
        Err(e) => println!("   ❌ 最终失败: {}\n", e),
    }

    // ============================================================
    // 7. 批量操作的部分成功处理
    // ============================================================
    println!("7️⃣  批量操作的部分成功处理");

    let test_images = find_test_images();
    if !test_images.is_empty() {
        let mut batch_result: BatchResult<String, Error> = BatchResult::new();

        println!("   📁 处理 {} 个文件", test_images.len());

        for (i, image) in test_images.iter().enumerate() {
            // 模拟某些操作失败
            if i == 1 {
                batch_result.add_failure(Error::Io(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "权限不足",
                )));
            } else {
                match exiftool.read_tag::<String, _, _>(image, "FileName") {
                    Ok(filename) => batch_result.add_success(filename),
                    Err(e) => batch_result.add_failure(e),
                }
            }
        }

        // 显示批量操作结果
        println!("\n   📊 批量操作结果统计:");
        println!("      总数: {}", batch_result.total);
        println!(
            "      成功: {} ({:.1}%)",
            batch_result.successes.len(),
            batch_result.success_rate()
        );
        println!(
            "      失败: {} ({:.1}%)",
            batch_result.failures.len(),
            batch_result.failure_rate()
        );
        println!(
            "      全部成功: {}",
            if batch_result.is_complete() {
                "是"
            } else {
                "否"
            }
        );

        if !batch_result.successes.is_empty() {
            println!("\n   ✅ 成功的文件:");
            for success in &batch_result.successes {
                println!("      • {}", success);
            }
        }

        if !batch_result.failures.is_empty() {
            println!("\n   ❌ 失败的文件:");
            for (i, failure) in batch_result.failures.iter().enumerate() {
                println!(
                    "      {}. {} - {}",
                    i + 1,
                    failure,
                    failure.recovery_suggestion().unwrap_or_default()
                );
            }
        }
    }
    println!();

    // ============================================================
    // 8. 错误恢复建议
    // ============================================================
    println!("8️⃣  常见错误及恢复建议");

    let common_errors = vec![
        (
            "IO 错误（文件不存在）",
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No such file",
            )),
        ),
        (
            "IO 错误（权限不足）",
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Permission denied",
            )),
        ),
        ("超时错误", Error::Timeout),
        ("进程错误", Error::process("ExifTool process failed")),
        ("互斥锁污染", Error::MutexPoisoned),
    ];

    for (desc, error) in common_errors {
        println!("   📌 {}", desc);
        println!("      错误: {}", error);
        println!(
            "      可恢复: {}",
            if error.is_recoverable() { "是" } else { "否" }
        );
        if let Some(sugg) = error.recovery_suggestion() {
            println!("      建议: {}", sugg);
        }
        println!();
    }

    // ============================================================
    // 9. 组合错误处理策略
    // ============================================================
    println!("9️⃣  组合错误处理策略示例");

    fn process_with_fallback(image: &PathBuf, exiftool: &ExifTool) -> Result<String, Error> {
        // 策略 1: 尝试读取 Make 标签
        match exiftool.read_tag::<String, _, _>(image, "Make") {
            Ok(make) => Ok(format!("Make: {}", make)),
            Err(Error::TagNotFound(_)) => {
                // 策略 2: 如果 Make 不存在，尝试 Model
                match exiftool.read_tag::<String, _, _>(image, "Model") {
                    Ok(model) => Ok(format!("Model: {}", model)),
                    Err(Error::TagNotFound(_)) => {
                        // 策略 3: 获取所有元数据
                        match exiftool.query(image).execute() {
                            Ok(metadata) => {
                                if metadata.is_empty() {
                                    return Err(Error::parse("文件没有元数据"));
                                }
                                // 返回第一个可用的标签
                                let first_tag = metadata.iter().next();
                                if let Some((tag, value)) = first_tag {
                                    return Ok(format!("{}: {}", tag, value));
                                }
                                Err(Error::parse("无法获取元数据"))
                            }
                            Err(e) => Err(e),
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    if let Some(ref image) = test_image {
        match process_with_fallback(image, &exiftool) {
            Ok(result) => println!("   ✅ 处理结果: {}", result),
            Err(e) => println!("   ❌ 处理失败: {}", e),
        }
    }
    println!();

    // ============================================================
    // 10. 优雅地关闭资源
    // ============================================================
    println!("🔟  优雅地关闭资源");

    // 即使之前的操作失败，也要确保资源被正确释放
    match exiftool.close() {
        Ok(_) => println!("   ✅ ExifTool 进程已优雅关闭"),
        Err(e) => {
            eprintln!("   ⚠️  关闭时出错（但不影响程序）: {}", e);
            // 这里可以继续执行，因为 Drop 实现会处理清理
        }
    }

    println!("\n✨ 错误处理示例完成！");
    println!("\n💡 最佳实践:");
    println!("   1. 始终检查 ExifToolNotFound 错误并提供安装指导");
    println!("   2. 使用 Recoverable trait 判断错误是否可以重试");
    println!("   3. 批量操作时使用 BatchResult 处理部分成功");
    println!("   4. 实现回退策略以提高鲁棒性");
    println!("   5. 使用重试机制处理临时性错误（网络、IO）");

    Ok(())
}

/// 查找测试图片
fn find_test_image() -> Option<PathBuf> {
    let paths = vec![
        PathBuf::from("examples/sample.jpg"),
        PathBuf::from("sample.jpg"),
        PathBuf::from("test.jpg"),
    ];

    paths.into_iter().find(|p| p.exists())
}

/// 查找测试图片（多个）
fn find_test_images() -> Vec<PathBuf> {
    let mut images = Vec::new();

    // 添加一个存在的文件
    if let Some(img) = find_test_image() {
        images.push(img);
    }

    // 添加一个不存在的文件用于测试
    images.push(PathBuf::from("/non/existent/file.jpg"));

    images
}
