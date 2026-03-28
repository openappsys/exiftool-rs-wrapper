//! 多命令执行集成测试
//!
//! 测试 `-execute[NUM]` 多命令功能的正确性

use exiftool_rs_wrapper::ExifTool;

/// 测试基本的多命令执行
#[test]
fn test_execute_multiple_basic() {
    let exiftool = match ExifTool::new() {
        Ok(et) => et,
        Err(exiftool_rs_wrapper::Error::ExifToolNotFound) => {
            eprintln!("ExifTool not found, skipping test");
            return;
        }
        Err(e) => panic!("Failed to create ExifTool: {:?}", e),
    };

    // 执行多个命令
    let commands = vec![vec!["-ver".to_string()], vec!["-listw".to_string()]];

    let responses = exiftool
        .execute_multiple(&commands)
        .expect("Should execute multiple commands");

    assert_eq!(responses.len(), 2);

    // 验证第一个响应（版本号）
    let version_text = responses[0].text();
    let version_trimmed = version_text.trim();
    assert!(!version_trimmed.is_empty(), "Version should not be empty");
    // 版本号应该是数字格式，如 "13.50"
    assert!(
        version_trimmed.chars().next().unwrap().is_ascii_digit(),
        "Version should start with digit"
    );

    // 验证第二个响应（可写标签列表）
    let list_text = responses[1].text();
    let list_trimmed = list_text.trim();
    assert!(!list_trimmed.is_empty(), "List should not be empty");
}

/// 测试多命令执行可以处理多个连续命令
#[test]
fn test_execute_multiple_sequential() {
    let exiftool = match ExifTool::new() {
        Ok(et) => et,
        Err(exiftool_rs_wrapper::Error::ExifToolNotFound) => {
            eprintln!("ExifTool not found, skipping test");
            return;
        }
        Err(e) => panic!("Failed to create ExifTool: {:?}", e),
    };

    // 执行多个 -ver 命令来测试顺序执行
    let commands = vec![
        vec!["-ver".to_string()],
        vec!["-ver".to_string()],
        vec!["-ver".to_string()],
    ];

    let result = exiftool.execute_multiple(&commands);
    assert!(result.is_ok(), "Multi-command should complete");

    let responses = result.unwrap();
    assert_eq!(responses.len(), 3, "Should have 3 responses");

    // 所有响应都应该包含相同的版本号
    let version1_text = responses[0].text();
    let version2_text = responses[1].text();
    let version3_text = responses[2].text();

    let version1 = version1_text.trim();
    let version2 = version2_text.trim();
    let version3 = version3_text.trim();

    assert!(!version1.is_empty(), "Version 1 should not be empty");
    assert!(!version2.is_empty(), "Version 2 should not be empty");
    assert!(!version3.is_empty(), "Version 3 should not be empty");

    // 所有版本号应该相同
    assert_eq!(version1, version2, "Versions should match");
    assert_eq!(version2, version3, "Versions should match");
}

/// 测试空命令列表
#[test]
fn test_execute_multiple_empty() {
    let exiftool = match ExifTool::new() {
        Ok(et) => et,
        Err(exiftool_rs_wrapper::Error::ExifToolNotFound) => {
            eprintln!("ExifTool not found, skipping test");
            return;
        }
        Err(e) => panic!("Failed to create ExifTool: {:?}", e),
    };

    let commands: Vec<Vec<String>> = vec![];
    let responses = exiftool
        .execute_multiple(&commands)
        .expect("Should handle empty commands");

    assert!(
        responses.is_empty(),
        "Empty commands should return empty responses"
    );
}

/// 测试单命令多命令执行（与 execute 一致性）
#[test]
fn test_execute_multiple_single_command() {
    let exiftool = match ExifTool::new() {
        Ok(et) => et,
        Err(exiftool_rs_wrapper::Error::ExifToolNotFound) => {
            eprintln!("ExifTool not found, skipping test");
            return;
        }
        Err(e) => panic!("Failed to create ExifTool: {:?}", e),
    };

    // 使用 execute_multiple 执行单个命令
    let commands = vec![vec!["-ver".to_string()]];
    let multi_responses = exiftool
        .execute_multiple(&commands)
        .expect("Should execute");

    // 使用 execute 执行相同命令
    let single_response = exiftool.execute(&["-ver"]).expect("Should execute");

    // 结果应该相同
    assert_eq!(multi_responses.len(), 1);
    assert_eq!(
        multi_responses[0].text().trim(),
        single_response.text().trim(),
        "Single command results should match"
    );
}

/// 测试多命令混合不同类型
#[test]
fn test_execute_multiple_mixed() {
    let exiftool = match ExifTool::new() {
        Ok(et) => et,
        Err(exiftool_rs_wrapper::Error::ExifToolNotFound) => {
            eprintln!("ExifTool not found, skipping test");
            return;
        }
        Err(e) => panic!("Failed to create ExifTool: {:?}", e),
    };

    // 混合不同类型的命令
    let commands = vec![vec!["-ver".to_string()], vec!["-listf".to_string()]];

    let responses = exiftool
        .execute_multiple(&commands)
        .expect("Should execute");
    assert_eq!(responses.len(), 2, "Should have 2 responses");

    // 第一个响应应该是版本号
    let version_text_full = responses[0].text();
    let version_text = version_text_full.trim();
    assert!(
        version_text.chars().next().unwrap().is_ascii_digit(),
        "First response should be version number"
    );

    // 第二个响应应该是文件扩展名列表
    let list_text = responses[1].text();
    assert!(
        !list_text.trim().is_empty(),
        "Second response should be file list"
    );
}

/// 测试 CommandId 导出
#[test]
fn test_command_id_export() {
    use exiftool_rs_wrapper::CommandId;

    let id = CommandId::new(42);
    assert_eq!(id.value(), 42);
}
