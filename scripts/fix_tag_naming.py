#!/usr/bin/env python3
"""
修复标签命名质量问题

问题1: 双下划线（如 A_F__R_O_I）
问题2: 过度拆分（如 A_C_D_SEE）
"""

import re
from pathlib import Path


def fix_double_underscores(content):
    """修复双下划线问题"""
    # 匹配模式: pub const XXX__YYY
    pattern = r"(pub const [A-Z_]+)__([A-Z_]+: TagId)"

    def replace(match):
        prefix = match.group(1)
        suffix = match.group(2)
        # 移除多余的下划线
        return f"{prefix}_{suffix}"

    return re.sub(pattern, replace, content)


def fix_over_split(content):
    """修复过度拆分问题

    将连续的 A_B_C 模式转换为 ABC（常见缩写）
    但需要保留有意义的拆分
    """
    # 先定义已知的缩写列表
    common_acronyms = {
        "A_C_D": "ACD",
        "A_C_D_SEE": "ACDSEE",
        "A_F": "AF",
        "A_E": "AE",
        "A_E_B": "AEB",
        "A_E_C": "AEC",
        "A_F_C": "AFC",
        "A_F_D": "AFD",
        "A_F_E": "AFE",
        "A_F_F": "AFF",
        "A_F_INFO": "AFINFO",
        "A_F_MICRO": "AFMICRO",
        "A_F_POINT": "AFPOINT",
        "A_F_POINTS": "AFPOINTS",
        "A_F_STATUS": "AFSTATUS",
        "A_F_A": "AFA",
        "A_F_C": "AFC",
        "A_F_D": "AFD",
        "A_F_E": "AFE",
        "A_F_F": "AFF",
        "A_F_I": "AFI",
        "A_F_P": "AFP",
        "A_F_S": "AFS",
        "A_F_MODE": "AFMODE",
        "A_F_AREAS": "AFAREAS",
        "A_F_IMAGE": "AFIMAGE",
        "A_F_STATUS": "AFSTATUS",
        "A_F_MICRO_ADJ": "AFMICROADJ",
        "A_F_MICRO_ADJ_MODE": "AFMICROADJMODE",
        "A_F_MICRO_ADJ_VALUE": "AFMICROADJVALUE",
        "A_F_FINE_TUNE": "AFFINETUNE",
        "A_F_FINE_TUNE_ADJ": "AFFINETUNEADJ",
        "A_F_FINE_TUNE_ADK": "AFFINETUNEADK",
        "A_F_ON": "AFON",
        "A_F_ON_FOR": "AFONFOR",
        "A_F_ON_BUTTON": "AFONBUTTON",
        "A_F_ON_BUTTON_PLUS": "AFONBUTTONPLUS",
        "A_F_ON_OUT_OF": "AFONOUTOF",
        "A_F_ON_OUT_OF_FOCUS": "AFONOUTOFFOCUS",
        "A_F_MODE_RESTRICTIONS": "AFMODERESTRICTIONS",
        "A_F_MODE_RESTRICTIONS2": "AFMODERESTRICTIONS2",
        "A_F_MODE_RESTRICTIONS3": "AFMODERESTRICTIONS3",
        "R_O_I": "ROI",
        "U_I_D": "UID",
        "I_D": "ID",
        "I_P_T_C": "IPTC",
        "X_M_P": "XMP",
        "G_P_S": "GPS",
        "E_X_I_F": "EXIF",
        "T_I_F_F": "TIFF",
        "J_P_E_G": "JPEG",
        "P_N_G": "PNG",
        "R_A_W": "RAW",
        "H_D_R": "HDR",
        "D_N_G": "DNG",
        "C_R2": "CR2",
        "C_R3": "CR3",
        "N_E_F": "NEF",
        "A_R_W": "ARW",
        "R_W2": "RW2",
        "O_R_F": "ORF",
        "P_E_F": "PEF",
        "R_A_F": "RAF",
        "X3_F": "X3F",
        "M_O_V": "MOV",
        "M_P4": "MP4",
        "A_V_I": "AVI",
        "M_K_V": "MKV",
        "W_M_V": "WMV",
        "F_L_V": "FLV",
        "W_A_V": "WAV",
        "M_P3": "MP3",
        "A_A_C": "AAC",
        "F_L_A_C": "FLAC",
        "O_G_G": "OGG",
        "W_M_A": "WMA",
        "A_I_F_F": "AIFF",
        "C_A_F": "CAF",
    }

    # 修复已知的缩写
    for old, new in common_acronyms.items():
        content = content.replace(f"_{old}_", f"_{new}_")
        content = content.replace(f"_{old}:", f"_{new}:")
        # 开头的情况
        if content.startswith(f"pub const {old}_"):
            content = content.replace(f"pub const {old}_", f"pub const {new}_", 1)
        if content.startswith(f"pub const {old}:"):
            content = content.replace(f"pub const {old}:", f"pub const {new}:", 1)

    return content


def process_file(filepath):
    """处理单个文件"""
    content = filepath.read_text(encoding="utf-8")
    original = content

    # 应用修复
    content = fix_double_underscores(content)
    content = fix_over_split(content)

    if content != original:
        filepath.write_text(content, encoding="utf-8")
        print(f"✓ 修复: {filepath}")
        return True
    return False


def main():
    tags_dir = Path("src/tags")
    fixed_count = 0

    for rs_file in tags_dir.rglob("*.rs"):
        if process_file(rs_file):
            fixed_count += 1

    print(f"\n共修复 {fixed_count} 个文件")


if __name__ == "__main__":
    main()
