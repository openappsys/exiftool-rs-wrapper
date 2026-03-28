#!/usr/bin/env python3
"""
批量修复 types.rs 中的常量命名
将 SCREAMING_SNAKE_CASE 转换为 CamelCase（保持首字母大写）
"""

import re
from pathlib import Path


def to_camel_case(name):
    """将 SCREAMING_SNAKE_CASE 转换为 CamelCase"""
    # 特殊处理：已经是 CamelCase 的（如 GPS、EXIF）保持不变
    if "_" not in name:
        return name

    # 拆分并转换
    parts = name.split("_")
    return "".join(part.capitalize() for part in parts)


def fix_types_rs():
    """修复 types.rs 文件"""
    filepath = Path("src/types.rs")
    content = filepath.read_text(encoding="utf-8")
    original = content

    # 匹配 pub const XXX: Self = Self("CamelCase")
    pattern = r'pub const ([A-Z][A-Z_0-9]*): Self = Self\("([^"]+)"\)'

    def replace_const(match):
        const_name = match.group(1)
        tag_name = match.group(2)

        # 转换常量名
        new_name = to_camel_case(const_name)

        return f'pub const {new_name}: Self = Self("{tag_name}")'

    content = re.sub(pattern, replace_const, content)

    if content != original:
        filepath.write_text(content, encoding="utf-8")
        print(f"✓ 修复: {filepath}")

        # 统计修复数量
        count = len(re.findall(pattern, original))
        print(f"  共修复 {count} 个常量")
    else:
        print(f"✗ 无需修复: {filepath}")


if __name__ == "__main__":
    fix_types_rs()
