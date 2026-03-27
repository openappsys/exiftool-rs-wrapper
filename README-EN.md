# ExifTool Rust Wrapper

[![crates.io](https://img.shields.io/crates/v/exiftool-rs-wrapper.svg)](https://crates.io/crates/exiftool-rs-wrapper)
[![docs.rs](https://docs.rs/exiftool-rs-wrapper/badge.svg)](https://docs.rs/exiftool-rs-wrapper)
[![CI](https://github.com/openappsys/exiftool-rs-wrapper/workflows/CI/badge.svg)](https://github.com/openappsys/exiftool-rs-wrapper/actions)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.94%2B-orange.svg)](https://www.rust-lang.org)

[中文](README.md) | **English**

> Aims to build an elegantly architected, high-performance, and type-safe Rust wrapper for ExifTool, featuring full async API support and striving for 100% functional coverage.

## Introduction

`exiftool-rs-wrapper` is a modern Rust library for reading, writing, and managing metadata of multimedia files including images, videos, and audio. This library wraps the powerful [ExifTool](https://exiftool.org/) command-line tool with an idiomatic Rust API.

### Core Features

- **High Compatibility Goal**: Continuously improving option and behavior compatibility, aiming for 100% ExifTool support
- **2375+ Predefined Tags**: Comprehensive tag coverage for EXIF, IPTC, XMP, GPS, and major camera makers (Canon, Nikon, Sony, Fuji, Olympus, Panasonic)
- **Modular Tag System**: Split into modules by category with feature flags for selective compilation
- **High Performance**: Uses `-stay_open` mode to keep the process running, avoiding startup overhead
- **Type Safety**: Complete tag type system with strongly-typed APIs
- **Async Support**: Tokio-based async API (uses `spawn_blocking` to avoid blocking runtime threads)
- **Connection Pool**: Built-in connection pool for high-concurrency scenarios
- **Builder Pattern**: Fluent API design with method chaining

## Feature Highlights

### Metadata Reading

- Read EXIF, IPTC, XMP, and other standard metadata
- Support for 200+ file formats (JPEG, PNG, RAW, MP4, PDF, etc.)
- Selective reading of specific tags
- Batch queries for multiple files
- Raw values and formatted values
- Grouped output by category

### Metadata Writing

- Write any tag supported by ExifTool
- Delete specific tags
- Batch write operations
- Conditional writes (only modify when conditions are met)
- DateTime offset adjustments
- Copy tags from other files
- Backup and overwrite modes supported

### Advanced Features

- **File Operations**: Rename and organize files based on metadata
- **Geolocation**: GPS coordinate read/write, reverse geocoding
- **Binary Data**: Thumbnail and preview extraction
- **Format Conversion**: Multiple output formats (JSON, XML, CSV, etc.)
- **Checksums**: Calculate file checksums (MD5, SHA256, etc.)
- **Streaming**: Large file processing with progress tracking
- **Error Recovery**: Configurable retry strategies

### Modular Tag System

- **Feature Flags**: Compile only the tags you need via Cargo features
- **Module Organization**: Tags split by category (exif, iptc, xmp, gps, canon, nikon, etc.)
- **Vendor Support**: Complete MakerNotes for Canon (528 tags), Nikon (514 tags), Sony (476 tags), FujiFilm (143 tags), Olympus (357 tags), Panasonic (159 tags)
- **String API Fallback**: Access any tag via string for tags not yet predefined
- **Tag Generation Scripts**: Automated scripts to extract tags from ExifTool (`scripts/generate_tags.sh`)

### Performance Optimizations

- Connection pooling for concurrent access
- LRU cache to reduce repeated queries
- Batch operation optimization
- Streaming for large files

### Compatibility Report (Evidence for 100% Goal)

The project includes an automated compatibility report to quantify capability coverage against ExifTool:

```bash
./scripts/generate_capability_report.sh
```

Outputs:
- `target/compatibility/capability-report.json`
- `target/compatibility/exiftool-version.txt`

## Installation

### 1. Install ExifTool

Before using this library, you need to install ExifTool on your system:

**macOS:**
```bash
brew install exiftool
```

**Ubuntu/Debian:**
```bash
sudo apt-get install libimage-exiftool-perl
```

**Windows:**
Download and install from [Windows version](https://exiftool.org/)

**Verify Installation:**
```bash
exiftool -ver
```

### 2. Add Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
exiftool-rs-wrapper = "0.1.4"

# Enable async support (optional)
exiftool-rs-wrapper = { version = "0.1.4", features = ["async"] }

# Enable Serde struct support
exiftool-rs-wrapper = { version = "0.1.4", features = ["serde-structs"] }

# Minimal build - only basic EXIF tags
exiftool-rs-wrapper = { version = "0.1.4", default-features = false, features = ["exif"] }

# Standard metadata (EXIF + IPTC + XMP + GPS)
exiftool-rs-wrapper = { version = "0.1.4", default-features = false, features = ["standard"] }

# Specific vendors only
exiftool-rs-wrapper = { version = "0.1.4", default-features = false, features = ["exif", "canon", "nikon"] }
```

Custom ExifTool executable and config file:

```rust
let exiftool = ExifTool::builder()
    .executable("/usr/local/bin/exiftool")
    .config("/path/to/.ExifTool_config")
    .build()?;
```

### Available Features

- `exif` - EXIF standard tags (ImageWidth, ExposureTime, ISO, etc.)
- `iptc` - IPTC tags (Headline, Keywords, Copyright, etc.)
- `xmp` - XMP tags (Title, Creator, Rights, etc.)
- `gps` - GPS tags (Latitude, Longitude, Altitude, etc.)
- `canon` - Canon MakerNotes (528 tags)
- `nikon` - Nikon MakerNotes (514 tags)
- `sony` - Sony MakerNotes (476 tags)
- `fuji` - FujiFilm MakerNotes (143 tags)
- `olympus` - Olympus MakerNotes (357 tags)
- `panasonic` - Panasonic MakerNotes (159 tags)
- `other` - All other tags (15000+, in progress)
- `standard` - Convenience feature: exif + iptc + xmp + gps
- `vendors` - Convenience feature: canon + nikon + sony + fuji + olympus + panasonic
- `all-tags` - All available tags (default)
- `async` - Async API support via Tokio

## Quick Start

### Basic Example

```rust
use exiftool_rs_wrapper::ExifTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create ExifTool instance (-stay_open mode)
    let exiftool = ExifTool::new()?;
    
    // Read file metadata
    let metadata = exiftool.query("photo.jpg").execute()?;
    
    // Access specific tags
    if let Some(make) = metadata.get("Make") {
        println!("Camera Make: {}", make);
    }
    
    if let Some(model) = metadata.get("Model") {
        println!("Camera Model: {}", model);
    }
    
    // Get image dimensions
    let width: i64 = exiftool.read_tag("photo.jpg", "ImageWidth")?;
    let height: i64 = exiftool.read_tag("photo.jpg", "ImageHeight")?;
    println!("Image Size: {} x {}", width, height);
    
    Ok(())
}
```

### Writing Metadata

```rust
use exiftool_rs_wrapper::ExifTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // Basic write (creates backup)
    exiftool.write("photo.jpg")
        .tag("Copyright", "© 2026 My Company")
        .tag("Artist", "John Doe")
        .execute()?;
    
    // Overwrite original file (no backup)
    exiftool.write("photo.jpg")
        .tag("Comment", "Processed with Rust")
        .overwrite_original(true)
        .execute()?;
    
    // Delete tags
    exiftool.write("photo.jpg")
        .delete("GPSPosition")
        .overwrite_original(true)
        .execute()?;
    
    Ok(())
}
```

### Batch Processing

```rust
use exiftool_rs_wrapper::ExifTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    let paths = vec!["photo1.jpg", "photo2.jpg", "photo3.jpg"];
    
    // Batch query
    let results = exiftool.query_batch(&paths)
        .tag("FileName")
        .tag("ImageSize")
        .tag("DateTimeOriginal")
        .execute()?;
    
    for (path, metadata) in results {
        println!("{}: {:?}", path.display(), metadata.get("FileName"));
    }
    
    Ok(())
}
```

## Detailed API Usage Examples

### Using Tag Constants (Type Safety)

```rust
use exiftool_rs_wrapper::{ExifTool, TagId};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // Use TagId constants instead of strings
    let make: String = exiftool.read_tag("photo.jpg", TagId::MAKE)?;
    let model: String = exiftool.read_tag("photo.jpg", TagId::MODEL)?;
    let iso: i64 = exiftool.read_tag("photo.jpg", TagId::ISO)?;
    
    println!("{} {} @ ISO {}", make, model, iso);
    
    // Write using TagId
    exiftool.write("photo.jpg")
        .tag_id(TagId::COPYRIGHT, "© 2026")
        .tag_id(TagId::ARTIST, "Photographer")
        .overwrite_original(true)
        .execute()?;
    
    Ok(())
}
```

### Advanced Query Options

```rust
use exiftool_rs_wrapper::ExifTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // Advanced query configuration
    let metadata = exiftool.query("photo.jpg")
        .include_unknown(true)          // Include unknown tags
        .include_duplicates(true)       // Include duplicate tags
        .raw_values(true)               // Return raw values
        .group_by_category(true)        // Group by category
        .tag("Make")                     // Query only specific tags
        .tag("Model")
        .tag("DateTimeOriginal")
        .exclude("MakerNotes")           // Exclude specific tags
        .execute()?;
    
    // Output as JSON
    let json = exiftool.query("photo.jpg")
        .execute_json()?;
    println!("{}", serde_json::to_string_pretty(&json)?);
    
    // Deserialize to custom type
    #[derive(serde::Deserialize)]
    struct PhotoInfo {
        #[serde(rename = "FileName")]
        file_name: String,
        #[serde(rename = "ImageWidth")]
        width: i64,
        #[serde(rename = "ImageHeight")]
        height: i64,
    }
    
    let info: PhotoInfo = exiftool.query("photo.jpg")
        .execute_as()?;
    
    Ok(())
}
```

### Async API

```rust
use exiftool_rs_wrapper::async_ext::AsyncExifTool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create async ExifTool instance
    let exiftool = AsyncExifTool::new()?;
    
    // Async query
    let metadata = exiftool.query("photo.jpg").await?;
    println!("Camera: {:?}", metadata.get("Make"));
    
    // Async batch query
    let paths = vec!["photo1.jpg", "photo2.jpg", "photo3.jpg"];
    let results = exiftool.query_batch(&paths).await?;
    
    for (path, metadata) in results {
        println!("{}: {:?}", path.display(), metadata.get("FileName"));
    }
    
    // Async write
    exiftool.write_tag("photo.jpg", "Copyright", "© 2026").await?;
    
    // Async delete
    exiftool.delete_tag("photo.jpg", "GPSPosition").await?;
    
    Ok(())
}
```

### Connection Pool (High Concurrency)

```rust
use exiftool_rs_wrapper::pool::ExifToolPool;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create connection pool with 4 connections
    let pool = ExifToolPool::new(4)?;
    let pool_clone = pool.clone();
    
    // Use pool in multiple threads
    let handles: Vec<_> = (0..8).map(|i| {
        let pool = pool_clone.clone();
        thread::spawn(move || {
            // Acquire connection from pool
            let conn = pool.acquire()?;
            let exiftool = conn.get().unwrap();
            
            let metadata = exiftool.query(format!("photo{}.jpg", i))
                .execute()?;
            
            println!("Thread {}: Processing complete", i);
            Ok::<(), exiftool_rs_wrapper::Error>(())
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap()?;
    }
    
    Ok(())
}
```

### File Organization and Renaming

```rust
use exiftool_rs_wrapper::{
    ExifTool, 
    file_ops::{FileOperations, RenamePattern}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // Rename based on DateTime
    exiftool.rename_by_pattern(
        "photo.jpg",
        RenamePattern::datetime("%Y%m%d_%H%M%S"),
    )?;
    
    // Rename based on camera model
    exiftool.rename_by_pattern(
        "photo.jpg",
        RenamePattern::tag_with_suffix(
            exiftool_rs_wrapper::TagId::MODEL,
            "_IMG"
        ),
    )?;
    
    // Organize files into directory structure
    use exiftool_rs_wrapper::file_ops::OrganizeOptions;
    
    let options = OrganizeOptions::new("/output/directory")
        .subdir(RenamePattern::datetime("%Y/%m"))  // Create subdirs by year/month
        .filename(RenamePattern::datetime("%Y%m%d_%H%M%S"))
        .extension("jpg");
    
    exiftool.organize_files(&["photo1.jpg", "photo2.jpg"], &options)?;
    
    Ok(())
}
```

### Geolocation Processing

```rust
use exiftool_rs_wrapper::{ExifTool, geo::GeoOperations};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // Read GPS coordinates
    if let Some(coord) = exiftool.get_gps_coordinates("photo.jpg")? {
        println!("Latitude: {}", coord.latitude);
        println!("Longitude: {}", coord.longitude);
        println!("Altitude: {:?}", coord.altitude);
    }
    
    // Write GPS coordinates
    use exiftool_rs_wrapper::geo::GpsCoordinate;
    
    let coord = GpsCoordinate::new(39.9042, 116.4074)
        .altitude(43.5);
    
    exiftool.set_gps_coordinates("photo.jpg", &coord)?;
    
    // Reverse geocoding (requires internet connection)
    if let Some(location) = exiftool.reverse_geocode(&coord)? {
        println!("City: {}", location.city);
        println!("Country: {}", location.country);
    }
    
    Ok(())
}
```

### Error Handling and Retries

```rust
use exiftool_rs_wrapper::{
    ExifTool, 
    retry::{RetryPolicy, with_retry_sync}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // Configure retry policy
    let policy = RetryPolicy::new()
        .max_attempts(3)
        .initial_delay(std::time::Duration::from_millis(100))
        .exponential_backoff(true);
    
    // Execute operation with retry
    let metadata = with_retry_sync(&policy, || {
        exiftool.query("photo.jpg").execute()
    })?;
    
    println!("Successfully read metadata: {:?}", metadata.get("FileName"));
    
    Ok(())
}
```

### Streaming and Progress Tracking

```rust
use exiftool_rs_wrapper::{
    ExifTool, 
    stream::{StreamingOperations, StreamOptions}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exiftool = ExifTool::new()?;
    
    // Define progress callback
    let on_progress = |processed: usize, total: usize, current: &str| {
        let percent = (processed as f64 / total as f64) * 100.0;
        println!("Progress: {:.1}% ({}/{}): {}", percent, processed, total, current);
    };
    
    let options = StreamOptions::new()
        .chunk_size(1024 * 1024)  // 1MB chunks
        .progress_callback(on_progress);
    
    // Stream process large file
    let metadata = exiftool.stream_query("large_video.mp4", &options)?;
    
    Ok(())
}
```

## Performance Benchmarks

Performance test results on typical hardware (for reference only):

| Operation | Single Thread | Pool(4) | Async |
|-----------|---------------|---------|-------|
| Read single JPEG | ~5ms | - | ~5ms |
| Batch read 100 files | 450ms | 120ms | 110ms |
| Write single tag | ~15ms | - | ~15ms |

## Tag System Architecture

### Current Status

- **2375 Predefined Tags**: Complete coverage of standard EXIF, IPTC, XMP, GPS, and major camera makers
- **Modular Design**: Tags organized by category in `src/tags/` directory
- **Feature Flags**: Selective compilation via Cargo features

### Tag Generation

The library includes scripts to extract tags directly from ExifTool:

```bash
# Generate all vendor tags
./scripts/generate_tags.sh

# Generate specific vendor
./scripts/generate_tags.sh Canon
./scripts/generate_tags.sh Nikon
./scripts/generate_tags.sh Sony
```

This ensures 100% compatibility with your installed ExifTool version.

### Working Towards 18000+ Tags

The library is actively being expanded to support all ~18000 ExifTool tags:

- ✅ **Phase 1**: Standard metadata (920 tags) - COMPLETE
- ✅ **Phase 2**: Major camera makers (2375 tags total) - COMPLETE
- 🔄 **Phase 3**: All remaining tags (~15600 tags) - IN PROGRESS

You can access any tag (even undefined ones) via the string API:

```rust
// Access any ExifTool tag
let rare_tag: String = exiftool.read_tag("photo.jpg", "SomeRareTag")?;
```
| Batch write 100 files | 1.5s | 450ms | 420ms |

### Optimization Tips

1. **Use Connection Pool**: Connection pooling significantly improves performance in high-concurrency scenarios
2. **Batch Operations**: Use batch APIs instead of looping single files
3. **Selective Queries**: Only query needed tags, avoid reading full metadata
4. **Enable Caching**: Use built-in LRU cache for repeated queries

## Command Line Tool

This project also provides a command-line tool:

```bash
# Install CLI tool
cargo install exiftool-rs-wrapper

# Read file metadata
exiftool-rs-wrapper read photo.jpg

# Write tags
exiftool-rs-wrapper write photo.jpg Copyright "© 2026"

# Explicitly overwrite original file (default keeps backup)
exiftool-rs-wrapper write photo.jpg Copyright "© 2026" --overwrite

# Delete tags
exiftool-rs-wrapper delete photo.jpg GPSPosition

# Batch processing
exiftool-rs-wrapper batch --input-dir ./photos --output-dir ./organized

# View version
exiftool-rs-wrapper version

# List supported tags
exiftool-rs-wrapper list-tags
```

## Contributing

We welcome all forms of contributions! Please follow these steps:

### Submitting Issues

- When reporting bugs, please provide detailed reproduction steps and environment information
- When requesting features, describe the use case and expected behavior
- Search for existing issues before submitting

### Submitting Pull Requests

1. Fork this repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push branch: `git push origin feature/amazing-feature`
5. Submit Pull Request

### Development Environment

```bash
# Clone repository
git clone https://github.com/openappsys/exiftool-rs-wrapper.git
cd exiftool-rs-wrapper

# Build project
cargo build --release

# Run tests
cargo test
cargo test --lib

# Code check
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

### Code Standards

- Follow Rust API Guidelines
- Ensure passing `cargo clippy` and `cargo fmt` checks
- Add tests for new features
- Update relevant documentation

## License

This project is licensed under the Apache-2.0 License.

- Apache-2.0 License: See [LICENSE](LICENSE) file

## Acknowledgments

- [ExifTool](https://exiftool.org/) by Phil Harvey - Powerful metadata processing tool
- Rust Community - Excellent language and ecosystem

## Related Links

- [Documentation](https://docs.rs/exiftool-rs-wrapper)
- [Crates.io](https://crates.io/crates/exiftool-rs-wrapper)
- [GitHub Repository](https://github.com/openappsys/exiftool-rs-wrapper)
- [Issue Tracker](https://github.com/openappsys/exiftool-rs-wrapper/issues)

---

**Note**: This library requires ExifTool to be installed on the system. ExifTool is independent software developed by Phil Harvey with its own license.
