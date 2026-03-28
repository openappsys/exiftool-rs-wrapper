#!/usr/bin/env python3
"""
生成标签定义 - 使用 ExifTool 原样命名（CamelCase）

规则：
1. 常量名 = ExifTool 标签名（CamelCase）
2. 横杠(-)替换为下划线(_)
"""

import subprocess
import re
from pathlib import Path
from collections import defaultdict


def get_exiftool_tags():
    """从 ExifTool 获取所有标签"""
    try:
        result = subprocess.run(["exiftool", "-list"], capture_output=True, text=True)

        # 解析输出
        lines = result.stdout.split("\n")
        tags = set()

        for line in lines:
            # 跳过标题行
            if "Available tags:" in line or not line.strip():
                continue

            # 分割标签（空格分隔）
            for tag in line.split():
                tag = tag.strip()
                if tag and not tag.startswith("-"):
                    tags.add(tag)

        return sorted(tags)
    except Exception as e:
        print(f"Error getting tags from exiftool: {e}")
        # 如果 exiftool 不可用，返回空列表
        return []


def to_rust_identifier(tag):
    """转换为合法的 Rust 标识符"""
    # 规则1：横杠转下划线
    identifier = tag.replace("-", "_")

    # 规则2：移除冒号（如 shortcuts:）
    identifier = identifier.replace(":", "")

    # 规则3：确保是合法的 Rust 标识符（以字母或下划线开头）
    if identifier and identifier[0].isdigit():
        identifier = "_" + identifier

    # 规则4：不能是 Rust 关键字
    rust_keywords = {
        "as",
        "break",
        "const",
        "continue",
        "crate",
        "else",
        "enum",
        "extern",
        "false",
        "fn",
        "for",
        "if",
        "impl",
        "in",
        "let",
        "loop",
        "match",
        "mod",
        "move",
        "mut",
        "pub",
        "ref",
        "return",
        "self",
        "Self",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "type",
        "unsafe",
        "use",
        "where",
        "while",
        "async",
        "await",
        "dyn",
        "abstract",
        "become",
        "box",
        "do",
        "final",
        "macro",
        "override",
        "priv",
        "typeof",
        "unsized",
        "virtual",
        "yield",
        "try",
    }
    if identifier in rust_keywords:
        identifier = f"{identifier}_tag"

    return identifier


def categorize_tag(tag):
    """根据标签名粗略分类"""
    tag_lower = tag.lower()

    # 厂商标签
    if tag.startswith("Canon") or tag.startswith("Canon"):
        return "vendors/canon"
    elif tag.startswith("Nikon") or tag.startswith("Nikon"):
        return "vendors/nikon"
    elif tag.startswith("Sony") or tag.startswith("Sony"):
        return "vendors/sony"
    elif tag.startswith("Fuji") or tag.startswith("Fuji") or tag.startswith("Fujifilm"):
        return "vendors/fuji"
    elif tag.startswith("Olympus") or tag.startswith("Olympus"):
        return "vendors/olympus"
    elif tag.startswith("Panasonic") or tag.startswith("Panasonic"):
        return "vendors/panasonic"

    # 标准标签
    elif any(x in tag for x in ["EXIF", "Exif", "exif"]):
        return "standard/exif"
    elif any(x in tag for x in ["IPTC", "Iptc", "iptc"]):
        return "standard/iptc"
    elif any(x in tag for x in ["XMP", "Xmp", "xmp"]):
        return "standard/xmp"
    elif any(x in tag for x in ["GPS", "Gps", "gps"]):
        return "standard/gps"

    # 文件格式
    elif any(x in tag for x in ["PDF", "Pdf", "pdf"]):
        return "formats/pdf"
    elif any(x in tag for x in ["JPEG", "Jpeg", "jpeg", "JPG", "Jpg", "jpg"]):
        return "formats/jpeg"
    elif any(x in tag for x in ["PNG", "Png", "png"]):
        return "formats/png"
    elif any(x in tag for x in ["GIF", "Gif", "gif"]):
        return "formats/gif"
    elif any(x in tag for x in ["TIFF", "Tiff", "tiff"]):
        return "formats/tiff"
    elif any(x in tag for x in ["DNG", "Dng", "dng"]):
        return "formats/dng"
    elif any(x in tag for x in ["RAW", "Raw", "raw"]):
        return "formats/raw"

    # 视频
    elif any(
        x in tag
        for x in [
            "MOV",
            "Mov",
            "mov",
            "MP4",
            "Mp4",
            "mp4",
            "AVI",
            "Avi",
            "avi",
            "MKV",
            "Mkv",
            "mkv",
        ]
    ):
        return "video/quicktime"

    # 音频
    elif any(
        x in tag
        for x in [
            "MP3",
            "Mp3",
            "mp3",
            "WAV",
            "Wav",
            "wav",
            "AAC",
            "Aac",
            "aac",
            "FLAC",
            "Flac",
            "flac",
        ]
    ):
        return "audio/id3"

    # 其他分类到 other
    else:
        return "other"


def generate_tag_files():
    """生成标签定义文件"""
    tags = get_exiftool_tags()

    if not tags:
        print("No tags found from exiftool")
        return

    print(f"Found {len(tags)} tags from exiftool")

    # 按类别分组
    categorized = defaultdict(list)
    for tag in tags:
        category = categorize_tag(tag)
        categorized[category].append(tag)

    # 输出目录
    tags_dir = Path("src/tags")

    # 为每个类别生成文件
    for category, tag_list in categorized.items():
        file_path = tags_dir / f"{category}.rs"
        file_path.parent.mkdir(parents=True, exist_ok=True)

        content = "#![allow(non_upper_case_globals)]\n\n"
        content += f"//! 标签定义 - {category}\n\n"
        content += "use crate::TagId;\n\n"

        for tag in sorted(tag_list):
            const_name = to_rust_identifier(tag)
            content += f'pub const {const_name}: TagId = TagId("{tag}");\n'

        file_path.write_text(content, encoding="utf-8")
        print(f"✓ Generated {file_path} ({len(tag_list)} tags)")

    # 更新 mod.rs
    generate_mod_rs(categorized.keys())


def generate_mod_rs(categories):
    """生成 mod.rs"""
    content = "//! 标签模块\n\n"

    # 按目录组织模块
    modules = {}
    for category in categories:
        if "/" in category:
            parent, child = category.split("/")
            if parent not in modules:
                modules[parent] = []
            modules[parent].append(child)
        else:
            modules[category] = None  # 无子模块

    # 生成模块定义
    for name in sorted(modules.keys()):
        children = modules[name]
        if children is None:
            # 简单模块
            content += f"pub mod {name};\n"
        else:
            # 包含子模块的目录
            content += f"pub mod {name} {{\n"
            for child in sorted(children):
                content += f"    pub mod {child};\n"
            content += f"}}\n"

    # 添加统一导出
    content += "\n// 统一导出常用标签\n"
    content += "pub mod unified;\n"

    mod_path = Path("src/tags/mod.rs")
    mod_path.write_text(content, encoding="utf-8")
    print(f"✓ Generated {mod_path}")


if __name__ == "__main__":
    generate_tag_files()
    print("\nDone! Run `cargo check` to verify.")
