# 开发工具脚本

本目录包含用于开发和维护 exiftool-rs-wrapper 的工具脚本。

## 脚本说明

### generate_tags.sh
**用途**: 从 ExifTool 提取厂商标签并生成 Rust 代码

**用法**:
```bash
# 生成所有厂商的标签
./scripts/generate_tags.sh

# 或指定单个厂商
./scripts/generate_tags.sh Canon
./scripts/generate_tags.sh Nikon
./scripts/generate_tags.sh Sony
```

**输出**: 生成的代码保存在 `/tmp/` 目录下，文件名格式为 `{Vendor}_new.rs`

**使用场景**:
- ExifTool 版本升级后，需要同步新标签
- 添加新的相机厂商支持
- 批量更新标签定义

## 注意事项

- 这些脚本仅在开发时使用，不会被打包到发布版本中
- 运行脚本需要本地安装 ExifTool
- 生成的代码需要手动审查后插入到 `src/types.rs`

### generate_capability_report.sh
**用途**: 生成 ExifTool 兼容性能力报告（100% 目标证据）

**用法**:
```bash
./scripts/generate_capability_report.sh
```

**输出**:
- `target/compatibility/capability-report.json`
- `target/compatibility/exiftool-version.txt`

**使用场景**:
- 评估当前版本与 ExifTool 能力差距
- CI 产出兼容性报告工件
- 发布前验证兼容性基线
