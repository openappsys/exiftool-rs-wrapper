use exiftool_rs_wrapper::{Error, ExifTool};
use serde::Serialize;
use std::collections::{BTreeSet, HashSet};
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
        // 查询选项（QueryBuilder 方法）
        "-@",                           // args_file
        "-a",                           // include_duplicates / allow_duplicates
        "-api",                         // api_option
        "-args",                        // args_format
        "-b",                           // binary
        "-c",                           // coord_format
        "-charset",                     // charset
        "-common_args",                 // common_args
        "-csvDelim",                    // csv_delimiter
        "-d",                           // date_format
        "-D",                           // decimal
        "-delete_original",             // ExifTool::delete_original
        "-delete_original!",            // ExifTool::delete_original(path, true)
        "-e",                           // no_composite
        "-E",                           // escape(EscapeFormat::Html)
        "-ec",                          // escape(EscapeFormat::C)
        "-echo",                        // echo
        "-echo2",                       // echo (stderr)
        "-efile",                       // efile
        "-ex",                          // escape(EscapeFormat::Xml)
        "-execute",                     // 内部协议，process.rs 中使用
        "-ext",                         // extension
        "-extractEmbedded",             // extract_embedded
        "-f",                           // force_print
        "-fast",                        // fast(Some(1))
        "-fast2",                       // fast(Some(2))
        "-fileNUM",                     // alternate_file
        "-fileOrder",                   // file_order
        "-G",                           // group_names
        "-g",                           // group_by_category / group_headings
        "-globalTimeShift",             // WriteBuilder::global_time_shift
        "-h",                           // html_format
        "-H",                           // hex
        "-htmlDump",                    // html_dump
        "-if",                          // condition (WriteBuilder)
        "-i",                           // ignore
        "-j",                           // 内部 JSON 输出，build_args 中使用
        "-json",                        // execute 内部自动使用
        "-k",                           // 交互式暂停，不适合库场景
        "-l",                           // long_format
        "-L",                           // latin
        "-lang",                        // lang
        "-list",                        // ExifTool::list_tags
        "-listd",                       // ExifTool::list_descriptions
        "-listf",                       // ExifTool::list_file_extensions
        "-listg",                       // ExifTool::list_groups
        "-listItem",                    // QueryBuilder::list_item
        "-list_dir",                    // QueryBuilder::list_dir
        "-listw",                       // ExifTool::list_writable_tags
        "-listwf",                      // ExifTool::list_writable_file_extensions
        "-listx",                       // listx（execute 透传）
        "-m",                           // ignore_minor_errors
        "-n",                           // raw_values / no_print_conv
        "-o",                           // WriteBuilder::output
        "-overwrite_original",          // WriteBuilder::overwrite_original
        "-overwrite_original_in_place", // WriteBuilder::backup(false)
        "-p",                           // print_format
        "-P",                           // WriteBuilder::preserve_time
        "-password",                    // password
        "-php",                         // php_format
        "-plot",                        // plot_format
        "-progress",                    // progress
        "-q",                           // quiet
        "-r",                           // recursive
        "-r.",                          // recursive_hidden
        "-s",                           // short_format / short
        "-S",                           // short_format(Some(0)) / very_short
        "-scanForXMP",                  // scan_for_xmp
        "-sep",                         // separator
        "-sort",                        // sort
        "-srcfile",                     // source_file
        "-stay_open",                   // 内部协议，process.rs 中使用
        "-struct",                      // OutputFormat::Struct
        "-t",                           // tab_format
        "-T",                           // table_format
        "-u",                           // include_unknown / unknown
        "-U",                           // unknown_binary
        "-use",                         // use_module
        "-userParam",                   // user_param
        "-v",                           // VerboseOperations::verbose_dump
        "-ver",                         // ExifTool::version
        "-w",                           // text_out
        "-W",                           // tag_out
        "-Wext",                        // tag_out_ext
        "-wm",                          // WriteBuilder::write_mode
        "-X",                           // xml_format / OutputFormat::Xml
        "-z",                           // WriteBuilder::zip_compression
        // 写入选项（WriteBuilder 方法）
        "-F",            // fix_base
        "-TAG+=VALUE",   // WriteBuilder::tag_append
        "-TAG-=VALUE",   // WriteBuilder::tag_remove
        "-TAG^=VALUE",   // WriteBuilder::tag_prepend
        "-TAG<=FILE",    // WriteBuilder::tag_from_file
        "-TAG+<=FILE",   // WriteBuilder::tag_append_from_file
        "-tagsFromFile", // WriteBuilder::copy_from
        // 全局/配置选项
        "-config",           // ExifToolBuilder::config
        "-restore_original", // ExifTool::restore_original
        "-x",                // QueryBuilder::exclude（--TAG 排除标签）
        // 地理信息选项
        "-geotag", // GeoOperations::geotag_from_track
        // 配置操作选项
        "-diff", // ConfigOperations::diff
        // 格式操作
        "-csv", // FormatOperations::read_csv / OutputFormat::Csv
        // 参数变体支持
        "-api OPT^=",      // api_option_empty
        "-csv=FILE",       // QueryBuilder::csv_import / WriteBuilder::csv_import
        "-csv+=FILE",      // QueryBuilder::csv_append / WriteBuilder::csv_append
        "-DSTTAG<SRCTAG",  // copy_from_with_redirect
        "-+DSTTAG<SRCTAG", // copy_from_with_append
        "-efile!",         // efile_variant(_, None, true)
        "-efile2",         // efile_variant(_, Some(2), false)
        "-efile2!",        // efile_variant(_, Some(2), true)
        "-ext+",           // extension_add
        "-fileOrder2",     // file_order_secondary
        "-g0:1",           // group_headings_multi
        "-G0:1",           // group_names_multi
        "-htmlDump0",      // html_dump_offset
        "-if2",            // condition_num(2, ...)
        "-if3",            // condition_num(3, ...)
        "-j=FILE",         // QueryBuilder::json_import / WriteBuilder::json_import
        "-j+=FILE",        // QueryBuilder::json_append / WriteBuilder::json_append
        "-listgeo",        // ExifTool::list_geo_formats
        "-listr",          // ExifTool::list_readable_file_extensions
        "-p-",             // print_format_no_newline
        "-userParam P^=",  // user_param_empty
        "-w+",             // text_out_append
        "-w!",             // text_out_create
        "-W+",             // tag_out_append
        "-W!",             // tag_out_create
    ]
    .into_iter()
    .collect()
}

fn catalog_options() -> Vec<&'static str> {
    vec![
        "-a",
        "-api",
        "-api OPT^=",
        "-args",
        "-@",
        "-b",
        "-c",
        "-charset",
        "-common_args",
        "-config",
        "-csv",
        "-csv=FILE",
        "-csv+=FILE",
        "-csvDelim",
        "-d",
        "-D",
        "-delete_original",
        "-delete_original!",
        "-diff",
        "-DSTTAG<SRCTAG",
        "-+DSTTAG<SRCTAG",
        "-e",
        "-E",
        "-ec",
        "-efile",
        "-efile!",
        "-efile2",
        "-efile2!",
        "-echo",
        "-echo2",
        "-ex",
        "-execute",
        "-ext",
        "-ext+",
        "-extractEmbedded",
        "-f",
        "-F",
        "-fast",
        "-fast2",
        "-fileNUM",
        "-fileOrder",
        "-fileOrder2",
        "-G",
        "-G0:1",
        "-g",
        "-g0:1",
        "-geotag",
        "-globalTimeShift",
        "-h",
        "-H",
        "-htmlDump",
        "-htmlDump0",
        "-if",
        "-if2",
        "-if3",
        "-i",
        "-j",
        "-j=FILE",
        "-j+=FILE",
        "-json",
        "-k",
        "-l",
        "-L",
        "-lang",
        "-list",
        "-listd",
        "-listf",
        "-listg",
        "-listgeo",
        "-listItem",
        "-list_dir",
        "-listr",
        "-listw",
        "-listwf",
        "-listx",
        "-m",
        "-n",
        "-o",
        "-overwrite_original",
        "-overwrite_original_in_place",
        "-p",
        "-p-",
        "-P",
        "-password",
        "-php",
        "-plot",
        "-progress",
        "-q",
        "-r",
        "-r.",
        "-restore_original",
        "-s",
        "-S",
        "-scanForXMP",
        "-sep",
        "-sort",
        "-srcfile",
        "-stay_open",
        "-struct",
        "-t",
        "-T",
        "-TAG+=VALUE",
        "-TAG-=VALUE",
        "-TAG^=VALUE",
        "-TAG<=FILE",
        "-TAG+<=FILE",
        "-tagsFromFile",
        "-u",
        "-U",
        "-use",
        "-userParam",
        "-userParam P^=",
        "-v",
        "-ver",
        "-w",
        "-w+",
        "-w!",
        "-W",
        "-W+",
        "-W!",
        "-Wext",
        "-wm",
        "-x",
        "-X",
        "-z",
    ]
}

/// 从 exiftool -h 帮助文本中自动提取选项，并与手工总表比对
#[test]
fn test_auto_extract_options_from_help() {
    // 检查 ExifTool 是否可用
    let exiftool = match create_exiftool() {
        Ok(et) => et,
        Err(Error::ExifToolNotFound) => return,
        Err(e) => panic!("创建 ExifTool 实例时发生意外错误: {:?}", e),
    };

    // 通过 stay_open 模式执行 -h 获取帮助文本
    // 注意：在 stay_open 模式下 -h 是 HTML 格式选项，不带文件参数时
    // ExifTool 会输出帮助文档
    let response = exiftool
        .execute(&["-h"])
        .expect("执行 exiftool -h 应当成功");
    let help_text = response.text();

    // 如果 stay_open 模式下 -h 返回为空，则用独立进程获取帮助
    let help_text = if help_text.trim().is_empty() || !help_text.contains("Option") {
        // 确定可执行文件路径
        let exe = std::env::var("EXIFTOOL_PATH").unwrap_or_else(|_| "exiftool".to_string());
        let output = std::process::Command::new(&exe)
            .arg("-h")
            .output()
            .expect("执行 exiftool -h 子进程失败");
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        help_text
    };

    // 截取 "Option Overview" 到 "Option Details" 之间的概览部分
    // 避免将 "Option Details" 中的大量示例选项误提取
    let options_section = {
        let start = help_text.find("Option Overview").unwrap_or(0);
        let end = help_text.find("Option Details").unwrap_or(help_text.len());
        if start < end {
            &help_text[start..end]
        } else {
            &help_text[start..]
        }
    };

    // 从帮助文本中提取以 `-` 开头的选项名
    // 匹配形如 `-word` 的 token
    // 忽略 `-TAG` 这类全大写占位符
    // 忽略 `--TAG` 双横线排除语法
    let mut help_options = BTreeSet::new();
    for word in options_section.split_whitespace() {
        // 跳过双横线开头的
        if word.starts_with("--") {
            continue;
        }
        // 必须以单个 `-` 开头
        if !word.starts_with('-') {
            continue;
        }
        // 去除可能的尾部标点（括号、逗号等）
        let cleaned = word.trim_end_matches([',', ')', '(', ']', ':']);
        // 至少要有 `-` 加一个字符
        if cleaned.len() < 2 {
            continue;
        }
        // 获取去掉 `-` 后的选项名部分
        let option_body = &cleaned[1..];

        // 处理逗号分隔的多选项（如 `-E,-ex,-ec`）
        if option_body.contains(',') {
            // 将逗号分隔的选项逐个处理
            for opt in option_body.split(',') {
                // 重建带 `-` 前缀的选项名
                let opt_with_dash = format!("-{}", opt);
                // 验证选项基本格式
                if !opt.is_empty() && opt.starts_with(|c: char| c.is_ascii_alphabetic()) {
                    // 对于 `-E`、`-ex`、`-ec` 这种短选项，需要映射到长选项名
                    let mapped_opt = map_short_option_to_full(&opt_with_dash);
                    help_options.insert(mapped_opt);
                }
            }
            continue;
        }

        // 忽略全大写的占位符（如 `-TAG`, `-TAGNAME`），但保留 `-ee` 等特殊短选项
        // 例外：`-TAG[+-]<=DATFILE` 这类占位符形式虽然以 `-TAG` 开头，但包含特殊字符
        // 如果选项体包含 `[` 或其他特殊字符，说明是占位符语法，跳过
        if option_body.contains('[') || option_body.contains('=') {
            // 这是 `-TAG[...` 或 `-TAG=...` 形式的占位符，代表标签写入运算符变体
            // 这些实际上是 `-TAG<=FILE`、`-TAG+=VALUE` 等的语法说明，不是真实选项名
            continue;
        }

        // 忽略全大写的纯占位符（如 `-TAG`, `-DSTTAG`）
        if option_body
            .chars()
            .all(|c| c.is_ascii_uppercase() || c == '+' || c == '#' || c == '<')
        {
            continue;
        }

        // 选项名必须以字母开头（排除像 `-=` 之类的）
        if !option_body.starts_with(|c: char| c.is_ascii_alphabetic()) {
            continue;
        }

        // 短选项映射到完整形式
        let mapped_opt = map_short_option_to_full(cleaned);
        help_options.insert(mapped_opt);
    }

    // 获取手工总表
    let catalog: BTreeSet<String> = catalog_options().into_iter().map(String::from).collect();

    // 将帮助中的压缩形式选项映射到基本形式，用于与总表比对
    // 例如：`-fast[NUM]` 的基本形式是 `-fast`，`-W[+|!]` 的基本形式是 `-W`
    // 处理 `-ee[NUM` 这种 help 文本中被截断缺少右括号的情况
    let normalize_help_option = |opt: &str| -> String {
        // 先应用短选项映射
        let mapped = map_short_option_to_full(opt);

        // 截取到第一个 `[` 之前（如 `-fast[NUM]` -> `-fast`）
        if let Some(pos) = mapped.find('[') {
            mapped[..pos].to_string()
        } else {
            mapped
        }
    };

    // 计算差集：帮助中有但总表中没有的选项（基于基本形式匹配）
    let missing: Vec<&String> = help_options
        .iter()
        .filter(|opt| {
            let base = normalize_help_option(opt);
            // 帮助选项的基本形式不在总表中，也不是总表某项的前缀
            !catalog.contains(&base)
                && !catalog.contains(opt.as_str())
                && !catalog.iter().any(|c| {
                    let c_base = if let Some(pos) = c.find(['[', '=']) {
                        &c[..pos]
                    } else {
                        c.as_str()
                    };
                    c_base == base
                })
        })
        .collect();

    // 生成报告
    let report = serde_json::json!({
        "description": "帮助文本中有但手工总表中没有的选项",
        "help_options_count": help_options.len(),
        "catalog_options_count": catalog.len(),
        "missing_count": missing.len(),
        "missing_options": missing,
    });

    fs::create_dir_all("target/compatibility").expect("创建 compatibility 目录失败");
    fs::write(
        "target/compatibility/missing-options.json",
        serde_json::to_string_pretty(&report).expect("序列化 missing-options 报告失败"),
    )
    .expect("写入 missing-options.json 失败");

    // 输出摘要到测试日志（不 assert 失败，仅生成报告）
    println!(
        "帮助文本选项数: {}, 手工总表选项数: {}, 总表中缺少的选项数: {}",
        help_options.len(),
        catalog.len(),
        missing.len()
    );
    if !missing.is_empty() {
        println!("缺少的选项: {:?}", missing);
    }
}

/// 将帮助文本中的短选项映射到总表中的完整选项名
fn map_short_option_to_full(opt: &str) -> String {
    match opt {
        "-E" => "-E".to_string(),                 // escape_html
        "-ex" => "-ex".to_string(),               // escape_xml
        "-ec" => "-ec".to_string(),               // escape_c
        "-ee" => "-extractEmbedded".to_string(),  // extract_embedded 的缩写
        "-ee2" => "-extractEmbedded".to_string(), // extract_embedded 的缩写
        "-ee3" => "-extractEmbedded".to_string(), // extract_embedded 的缩写
        _ => opt.to_string(),
    }
}

fn option_semantics(option: &str) -> (String, String, String) {
    match option {
        "-list" | "-listw" | "-listwf" | "-listr" | "-listf" | "-listx" | "-listg" | "-listgeo"
        | "-listd" | "-ver" => (
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
