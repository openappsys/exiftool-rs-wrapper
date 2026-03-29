use exiftool_rs_wrapper::{Error, ExifTool};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Baseline {
    minimums: BaselineMinimums,
    required_probes: Vec<ProbeSpec>,
    known_gaps: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct BaselineMinimums {
    writable_tags: usize,
    file_extensions: usize,
    probe_coverage_percent: f64,
}

#[derive(Debug, Deserialize)]
struct ProbeSpec {
    id: String,
    args: Vec<String>,
    min_words: usize,
}

#[derive(Debug, Serialize)]
struct CapabilityReport {
    exiftool_version: String,
    writable_tags: usize,
    file_extensions: usize,
    passed_probes: Vec<String>,
    failed_probes: Vec<String>,
    probe_coverage_percent: f64,
    known_gaps: Vec<String>,
    unresolved_gaps: Vec<String>,
}

#[test]
fn generate_compatibility_report() {
    let baseline_text = fs::read_to_string("tests/data/compatibility/capability_baseline.json")
        .expect("failed to read capability baseline");
    let baseline: Baseline =
        serde_json::from_str(&baseline_text).expect("failed to parse capability baseline");

    let exiftool = match create_exiftool() {
        Ok(et) => et,
        Err(Error::ExifToolNotFound) => return,
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    let mut passed_probes = Vec::new();
    let mut failed_probes = Vec::new();

    for probe in &baseline.required_probes {
        let args: Vec<&str> = probe.args.iter().map(|s| s.as_str()).collect();

        match exiftool.execute(&args) {
            Ok(response) => {
                let words = count_words(&response.text());
                if words >= probe.min_words {
                    passed_probes.push(probe.id.clone());
                } else {
                    failed_probes.push(format!(
                        "{} (output words {} < {})",
                        probe.id, words, probe.min_words
                    ));
                }
            }
            Err(e) => failed_probes.push(format!("{} ({})", probe.id, e)),
        }
    }

    let unresolved_gaps: Vec<String> = failed_probes
        .iter()
        .filter_map(|failed| {
            let id = failed.split_whitespace().next()?.to_string();
            if baseline.known_gaps.iter().any(|k| k == &id) {
                None
            } else {
                Some(failed.clone())
            }
        })
        .collect();

    let version = exiftool
        .execute(&["-ver"])
        .expect("-ver should work")
        .text()
        .trim()
        .to_string();

    let writable_tags = exiftool
        .list_writable_tags()
        .expect("-listw should work")
        .len();
    let file_extensions = exiftool
        .list_file_extensions()
        .expect("-listf should work")
        .len();

    let probe_coverage_percent = if baseline.required_probes.is_empty() {
        100.0
    } else {
        (passed_probes.len() as f64 / baseline.required_probes.len() as f64) * 100.0
    };

    let report = CapabilityReport {
        exiftool_version: version,
        writable_tags,
        file_extensions,
        probe_coverage_percent,
        passed_probes: passed_probes.clone(),
        failed_probes: failed_probes.clone(),
        known_gaps: baseline.known_gaps.clone(),
        unresolved_gaps: unresolved_gaps.clone(),
    };

    let output_dir = Path::new("target/compatibility");
    fs::create_dir_all(output_dir).expect("failed to create output directory");

    fs::write(
        output_dir.join("capability-report.json"),
        serde_json::to_string_pretty(&report).expect("failed to serialize report"),
    )
    .expect("failed to write report");

    let markdown_report = format!(
        "# Capability Report\n\n- ExifTool Version: `{}`\n- Writable Tags: `{}`\n- File Extensions: `{}`\n- Probe Coverage: `{:.2}%`\n- Passed Probes: `{}`\n- Failed Probes: `{}`\n- Known Gaps: `{}`\n- Unresolved Gaps: `{}`\n",
        report.exiftool_version,
        report.writable_tags,
        report.file_extensions,
        report.probe_coverage_percent,
        report.passed_probes.join(", "),
        report.failed_probes.join(", "),
        report.known_gaps.join(", "),
        report.unresolved_gaps.join(", ")
    );
    fs::write(output_dir.join("capability-report.md"), markdown_report)
        .expect("failed to write markdown report");

    let gap_markdown = format!(
        "# Compatibility Gap List\n\n## Known Gaps\n{}\n\n## Unresolved Gaps\n{}\n",
        to_bullet_list(&report.known_gaps),
        to_bullet_list(&report.unresolved_gaps)
    );
    fs::write(output_dir.join("compatibility-gap-list.md"), gap_markdown)
        .expect("failed to write compatibility gap list");

    assert!(
        report.writable_tags >= baseline.minimums.writable_tags,
        "writable tags below baseline: {} < {}",
        report.writable_tags,
        baseline.minimums.writable_tags
    );
    assert!(
        report.file_extensions >= baseline.minimums.file_extensions,
        "file extensions below baseline: {} < {}",
        report.file_extensions,
        baseline.minimums.file_extensions
    );
    assert!(
        report.probe_coverage_percent >= baseline.minimums.probe_coverage_percent,
        "probe coverage below target: {:.2}% < {:.2}%",
        report.probe_coverage_percent,
        baseline.minimums.probe_coverage_percent
    );
    assert!(
        report.unresolved_gaps.is_empty(),
        "unresolved gaps found: {:?}",
        report.unresolved_gaps
    );
}

fn create_exiftool() -> Result<ExifTool, Error> {
    if let Ok(path) = std::env::var("EXIFTOOL_PATH") {
        return ExifTool::builder().executable(path).build();
    }

    ExifTool::new()
}

fn to_bullet_list(items: &[String]) -> String {
    if items.is_empty() {
        return "- (none)".to_string();
    }

    items
        .iter()
        .map(|item| format!("- {}", item))
        .collect::<Vec<_>>()
        .join("\n")
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().filter(|s| !s.is_empty()).count()
}
