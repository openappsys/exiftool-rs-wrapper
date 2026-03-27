#!/bin/bash
# 标签生成工具 - 从 ExifTool 提取标签并生成 Rust 代码
# 用法: ./scripts/generate_tags.sh [vendor]
# 示例: ./scripts/generate_tags.sh Canon
#        ./scripts/generate_tags.sh all

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

generate_vendor_tags() {
    local vendor=$1
    local prefix=$(echo "$vendor" | tr '[:lower:]' '[:upper:]' | sed 's/FILM//')
    
    echo "=== 生成 $vendor 标签 ==="
    
    # 从 ExifTool 提取标签
    local tmp_tags="/tmp/${vendor}_tags.txt"
    exiftool -list -${vendor}:all 2>/dev/null | grep -oE '[A-Z][a-zA-Z0-9]+' | sort -u > "$tmp_tags"
    
    local total_tags=$(wc -l < "$tmp_tags")
    echo "  从 ExifTool 提取: $total_tags 个标签"
    
    # 生成 Rust 代码
    local tmp_rs="/tmp/${vendor}_generated.rs"
    > "$tmp_rs"
    
    while read -r tag; do
        [ -z "$tag" ] && continue
        local const_name=$(echo "$tag" | sed 's/\([A-Z]\)/_\1/g' | sed 's/^_//' | tr '[:lower:]' '[:upper:]')
        echo "    pub const ${prefix}_${const_name}: Self = Self(\"${tag}\");" >> "$tmp_rs"
    done < "$tmp_tags"
    
    # 过滤已存在的标签
    local new_tags=0
    while read -r line; do
        local tag=$(echo "$line" | sed "s/.*${prefix}_\([A-Z_]*\):.*/\1/")
        if ! grep -q "${prefix}_${tag}:" "$PROJECT_ROOT/src/types.rs" 2>/dev/null; then
            echo "$line"
            ((new_tags++))
        fi
    done < "$tmp_rs" > "/tmp/${vendor}_new.rs"
    
    echo "  新增标签: $(wc -l < "/tmp/${vendor}_new.rs") 个"
}

# 主逻辑
if [ $# -eq 0 ] || [ "$1" == "all" ]; then
    VENDORS=("Canon" "Nikon" "Sony" "FujiFilm" "Olympus" "Panasonic")
else
    VENDORS=("$1")
fi

for vendor in "${VENDORS[@]}"; do
    generate_vendor_tags "$vendor"
done

echo ""
echo "=== 使用说明 ==="
echo "生成的标签代码在 /tmp/*_new.rs"
echo "可以手动插入到 src/types.rs 中"
