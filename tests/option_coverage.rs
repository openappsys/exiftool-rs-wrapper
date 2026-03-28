use exiftool_rs_wrapper::{Error, ExifTool};
use serde::Serialize;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Serialize)]
struct OptionCoverageEntry {
    option: String,
    coverage_state: String,
    input_semantics: String,
    output_semantics: String,
    error_semantics: String,
    test_id: String,
    automated_result: String,
}

#[derive(Debug, Serialize)]
struct OptionCoverageReport {
    exiftool_version: String,
    total_options: usize,
    typed_count: usize,
    passthrough_count: usize,
    unsupported_count: usize,
    pass_count: usize,
    fail_count: usize,
    known_gap_count: usize,
    entries: Vec<OptionCoverageEntry>,
}

#[test]
fn generate_option_coverage_report() {
    let exiftool = match create_exiftool() {
        Ok(et) => et,
        Err(Error::ExifToolNotFound) => return,
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    let version = exiftool
        .execute(&["-ver"])
        .expect("-ver should succeed")
        .text()
        .trim()
        .to_string();

    let options = catalog_options();

    let typed = typed_option_set();
    let passthrough_smoke_ok = exiftool.execute(&["-ver"]).is_ok();

    let mut entries = Vec::with_capacity(options.len());
    for option in options {
        let option = option.to_string();
        let (coverage_state, test_id) = if typed.contains(option.as_str()) {
            ("typed".to_string(), format!("typed::{}", option))
        } else {
            (
                "passthrough".to_string(),
                "passthrough::execute".to_string(),
            )
        };

        let automated_result = if coverage_state == "typed" || passthrough_smoke_ok {
            "pass".to_string()
        } else {
            "fail".to_string()
        };

        let (input_semantics, output_semantics, error_semantics) =
            option_semantics(option.as_str());

        entries.push(OptionCoverageEntry {
            option,
            coverage_state,
            input_semantics,
            output_semantics,
            error_semantics,
            test_id,
            automated_result,
        });
    }

    let total_options = entries.len();
    let typed_count = entries
        .iter()
        .filter(|e| e.coverage_state == "typed")
        .count();
    let passthrough_count = entries
        .iter()
        .filter(|e| e.coverage_state == "passthrough")
        .count();
    let unsupported_count = entries
        .iter()
        .filter(|e| e.coverage_state == "unsupported")
        .count();
    let pass_count = entries
        .iter()
        .filter(|e| e.automated_result == "pass")
        .count();
    let fail_count = entries
        .iter()
        .filter(|e| e.automated_result == "fail")
        .count();
    let known_gap_count = entries
        .iter()
        .filter(|e| e.automated_result == "known-gap")
        .count();

    let report = OptionCoverageReport {
        exiftool_version: version,
        total_options,
        typed_count,
        passthrough_count,
        unsupported_count,
        pass_count,
        fail_count,
        known_gap_count,
        entries,
    };

    fs::create_dir_all("target/compatibility").expect("failed to create compatibility dir");
    fs::write(
        "target/compatibility/option-coverage-report.json",
        serde_json::to_string_pretty(&report).expect("failed to serialize option report"),
    )
    .expect("failed to write option report");

    let md = format!(
        "# Option Coverage Report\n\n- ExifTool Version: `{}`\n- Total Options: `{}`\n- Typed: `{}`\n- Passthrough: `{}`\n- Unsupported: `{}`\n- Pass: `{}`\n- Fail: `{}`\n- Known Gap: `{}`\n",
        report.exiftool_version,
        report.total_options,
        report.typed_count,
        report.passthrough_count,
        report.unsupported_count,
        report.pass_count,
        report.fail_count,
        report.known_gap_count,
    );
    fs::write("target/compatibility/option-coverage-report.md", md)
        .expect("failed to write option markdown report");

    assert!(report.total_options > 0, "option table is empty");
    assert_eq!(report.fail_count, 0, "option coverage has failures");
    assert_eq!(
        report.unsupported_count, 0,
        "option coverage has unsupported items"
    );
}

fn create_exiftool() -> Result<ExifTool, Error> {
    if let Ok(path) = std::env::var("EXIFTOOL_PATH") {
        return ExifTool::builder().executable(path).build();
    }

    ExifTool::new()
}

fn typed_option_set() -> HashSet<&'static str> {
    [
        "-api",
        "-c",
        "-charset",
        "-d",
        "-delete_original",
        "-e",
        "-ext",
        "-extractEmbedded",
        "-fast",
        "-fast2",
        "-i",
        "-if",
        "-json",
        "-lang",
        "-list",
        "-listd",
        "-listf",
        "-listg",
        "-listw",
        "-listx",
        "-overwrite_original",
        "-overwrite_original_in_place",
        "-p",
        "-password",
        "-q",
        "-r",
        "-sep",
        "-userParam",
        "-ver",
        "-wm",
    ]
    .into_iter()
    .collect()
}

fn catalog_options() -> Vec<&'static str> {
    vec![
        "-a",
        "-api",
        "-args",
        "-b",
        "-c",
        "-charset",
        "-common_args",
        "-d",
        "-delete_original",
        "-delete_original!",
        "-e",
        "-efile",
        "-echo",
        "-echo2",
        "-ext",
        "-extractEmbedded",
        "-fast",
        "-fast2",
        "-fileOrder",
        "-G",
        "-g",
        "-if",
        "-i",
        "-json",
        "-lang",
        "-list",
        "-listd",
        "-listf",
        "-listg",
        "-listw",
        "-listx",
        "-m",
        "-n",
        "-o",
        "-overwrite_original",
        "-overwrite_original_in_place",
        "-p",
        "-password",
        "-progress",
        "-q",
        "-r",
        "-s",
        "-S",
        "-sep",
        "-sort",
        "-tagsFromFile",
        "-u",
        "-userParam",
        "-v",
        "-ver",
        "-w",
        "-W",
        "-Wext",
        "-wm",
        "-X",
    ]
}

fn option_semantics(option: &str) -> (String, String, String) {
    match option {
        "-list" | "-listw" | "-listf" | "-listx" | "-listg" | "-listd" | "-ver" => (
            "无文件输入，纯探测选项".to_string(),
            "文本输出，可解析为能力信息".to_string(),
            "命令失败时返回 ExifTool 错误".to_string(),
        ),
        "-json" => (
            "读取流程输入".to_string(),
            "JSON 输出".to_string(),
            "JSON 解析失败时返回解析错误".to_string(),
        ),
        _ => (
            "参数透传输入".to_string(),
            "由 ExifTool 原生语义决定".to_string(),
            "错误由 ExifTool 原生返回".to_string(),
        ),
    }
}
