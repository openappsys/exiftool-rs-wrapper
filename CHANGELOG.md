# 更新日志

所有该项目的显著变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且该项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [0.1.5] - 2026-03-29

### 变更 (Breaking Changes)

- **标签命名规范重构**: 所有标签常量名改为与 ExifTool 原样一致（CamelCase）
  - 常量名 = ExifTool 标签名（如 `DateTimeOriginal`、`Make`、`Model`）
  - 横杠(-)替换为下划线(_)（如 `Wi-FiPassword` → `Wi_FiPassword`）
  - 使用 `#![allow(non_upper_case_globals)]` 抑制命名规范警告
  - **破坏性变更**: 所有使用旧常量名（如 `DATE_TIME_ORIGINAL`）的代码需要更新
  - 共 18,058 个标签，与 ExifTool 完全一致

- **标签模块组织优化**:
  - 按功能分组：standard（exif/iptc/xmp/gps）、vendors（canon/nikon/sony 等）、formats（jpeg/png/pdf 等）、video、audio
  - 新增 `unified` 模块统一导出常用标签
  - 移除丑陋的自动转换命名（如 `A_C_D_SEE_REGION` → `ACDSeeRegion`）

### 新增

- 新增标签生成脚本 `scripts/generate_tags_camelcase.py`
- 新增性能最佳实践文档和示例代码

## [0.1.4] - 2026-03-28

### 新增

- **多命令执行支持**: 实现 `-execute[NUM]` 多命令协议
  - 新增 `CommandId` 和 `CommandRequest` 类型
  - 新增 `execute_multiple()` 方法支持原子批量执行
  - 支持通过编号区分多条命令的响应
- **异步流处理支持** (需要 `async` feature):
  - 新增 `stream::async_stream` 模块
  - 新增 `StreamEvent` 枚举支持进度和元数据事件
  - 新增 `AsyncStreamHandle` 用于控制流处理
  - 为 `AsyncExifTool` 添加 `stream_query()`, `stream_batch()`, `stream_large_file()` 方法
  - 支持实时进度跟踪和取消操作
- 支持通过 Builder 模式自定义 ExifTool 可执行文件路径
- 新增 `serde-structs` feature，支持通过 serde 反序列化元数据到结构体
- 添加常用元数据结构体（Metadata、FileInfo、ExifInfo、GpsInfo 等）
- 新增公共透传执行 API：`ExifTool::execute(...)`
- 新增 `-config` 配置支持：`ExifTool::builder().config(...)` 与实例级 `with_config(...)`
- 新增兼容性报告体系

### 变更

- **核心改进**: 选项类型化覆盖率达到 135/135 (100%)
- 改进进程响应处理，支持 `{ready...}` 结束标记识别
- 异步接口改为 `spawn_blocking` 执行，避免阻塞 Tokio runtime
- 连接池新增 `acquire_timeout`
- CLI 升级为 `clap` 参数解析
- 灵活的标签模块系统，支持通过 Cargo features 按需编译标签组
- 标签目录重构，按功能分组（standard/vendors/formats/video/audio）

### 修复

- 修复参数构造问题，`query`/`write` 改为逐 token 传输
- 修复标签数量不匹配问题，严格匹配 ExifTool 的 18,046+ 个标签
- 修复 `QueryBuilder::execute_text` 与 `-json` 冲突问题
- 修复 `exclude` 语义为 `--TAG`

## [0.1.3] - 2026-03-27

### 新增

- 灵活的标签模块系统，支持通过 Cargo features 按需编译标签组
- 新增 `standard`、`vendors-common`、`balanced`、`full` 等预设 feature 组合
- 标签目录重构，按功能分组（standard/vendors/formats/video/audio）

### 变更

- `serde` 和 `serde_json` 改为非可选依赖（始终启用）

### 修复

- 修复标签数量不匹配问题
- 删除多余的标签组快捷方式
- 清理无效的 XMP 命名空间标签

## [0.1.0] - 2026-03-26

### 新增

- 初始版本发布
- 支持 `-stay_open` 模式，保持进程运行以获得最佳性能
- 完整的标签类型系统，提供类型安全的 API
- Builder 模式 API，符合 Rust 习惯
- 线程安全设计，支持多线程并发访问
- 异步 API 支持（通过 `async` feature）
- 连接池支持，用于高并发场景
- 批量查询和处理功能
- 写入元数据支持
- 删除标签功能
- 二进制数据处理
- 地理信息操作（GPS 坐标、地理编码）
- 文件组织和重命名功能
- 格式化输出支持
- 流式处理和进度追踪
- 重试机制和错误恢复
- 校验和计算（MD5、SHA256 等）
- 详细的文档和示例

### 特性

- `default`: 同步 API
- `async`: 异步 API 支持（依赖 tokio 和 futures）

[0.1.5]: https://github.com/openappsys/exiftool-rs-wrapper/releases/tag/v0.1.5
[0.1.4]: https://github.com/openappsys/exiftool-rs-wrapper/releases/tag/v0.1.4
[0.1.3]: https://github.com/openappsys/exiftool-rs-wrapper/releases/tag/v0.1.3
[0.1.0]: https://github.com/openappsys/exiftool-rs-wrapper/releases/tag/v0.1.0
