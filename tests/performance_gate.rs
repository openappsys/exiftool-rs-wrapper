use exiftool_rs_wrapper::{Error, ExifTool};
use serde::Serialize;
use std::fs;
use std::time::Instant;

const TINY_JPEG: &[u8] = &[
    0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00, 0x01,
    0x00, 0x01, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08,
    0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12,
    0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20,
    0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C, 0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27,
    0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01,
    0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0xFF, 0xC4, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0xFF, 0xC4, 0x00, 0x14, 0x10,
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, 0xD2, 0xCF, 0x20, 0xFF, 0xD9,
];

const MAX_READ_MS: u128 = 200;
const MAX_WRITE_MS: u128 = 350;

#[derive(Serialize)]
struct PerfGateReport {
    exiftool_version: String,
    read_ms: u128,
    write_ms: u128,
    max_read_ms: u128,
    max_write_ms: u128,
}

fn create_exiftool() -> Result<ExifTool, Error> {
    if let Ok(path) = std::env::var("EXIFTOOL_PATH") {
        return ExifTool::builder().executable(path).build();
    }

    ExifTool::new()
}

#[test]
fn performance_gate_should_meet_thresholds() {
    let exiftool = match create_exiftool() {
        Ok(et) => et,
        Err(Error::ExifToolNotFound) => return,
        Err(e) => panic!("Unexpected error: {:?}", e),
    };

    let temp = tempfile::tempdir().expect("failed to create temp dir");
    let file = temp.path().join("perf.jpg");
    fs::write(&file, TINY_JPEG).expect("failed to write image");

    let start_read = Instant::now();
    let _ = exiftool
        .query(&file)
        .tag("FileName")
        .execute()
        .expect("read should succeed");
    let read_ms = start_read.elapsed().as_millis();

    let start_write = Instant::now();
    exiftool
        .write(&file)
        .tag("Artist", "PerfGate")
        .overwrite_original(true)
        .execute()
        .expect("write should succeed");
    let write_ms = start_write.elapsed().as_millis();

    let version = exiftool
        .execute(&["-ver"])
        .expect("version should succeed")
        .text()
        .trim()
        .to_string();

    let report = PerfGateReport {
        exiftool_version: version,
        read_ms,
        write_ms,
        max_read_ms: MAX_READ_MS,
        max_write_ms: MAX_WRITE_MS,
    };

    fs::create_dir_all("target/performance").expect("failed to create perf directory");
    fs::write(
        "target/performance/perf-gate.json",
        serde_json::to_string_pretty(&report).expect("failed to serialize perf report"),
    )
    .expect("failed to write perf report");

    assert!(
        read_ms <= MAX_READ_MS,
        "read performance regression: {}ms > {}ms",
        read_ms,
        MAX_READ_MS
    );
    assert!(
        write_ms <= MAX_WRITE_MS,
        "write performance regression: {}ms > {}ms",
        write_ms,
        MAX_WRITE_MS
    );
}
