# 更新日志

所有该项目的显著变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且该项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

### 变更

- 修复 `-stay_open` 参数构造：`query`/`write` 的多项参数改为逐 token 传输，避免参数被错误解析
- 修复 `QueryBuilder::execute_text` 与 `-json` 冲突，文本输出不再强制 JSON
- 修复 `exclude` 语义为 `--TAG`
- 改进进程响应处理：支持超时控制、识别 `{ready...}` 结束标记、Warning 不再视为 Error
- 异步接口改为 `spawn_blocking` 执行同步调用，避免阻塞 Tokio runtime
- 连接池新增 `acquire_timeout`，批处理改为受控并发 worker 模式
- CLI 升级为 `clap` 参数解析，写/删/拷默认保留备份，`--overwrite` 显式覆盖
- 移除可能 panic 的 `Default` 实现（`ExifTool` 与 `AsyncExifTool`）

### 修复

- 修复 `config::hex_dump` 使用无效参数的问题
- 移除无效配置入口（`with_config` 空实现）以避免误导 API 语义

### 测试

- 增加 `query`、`write` 参数构建回归测试
- 增加 `process` Warning 级别行为测试
- 增加 `async` 并行处理辅助函数测试

## [0.1.4] - 2026-03-27

### 新增

- 支持通过 Builder 模式自定义 ExifTool 可执行文件路径
- 新增 `serde-structs` feature，支持通过 serde 反序列化元数据到结构体
- 添加常用元数据结构体（Metadata、FileInfo、ExifInfo、GpsInfo 等）

### 变更

- `serde` 和 `serde_json` 改为非可选依赖（始终启用）

## [0.1.3] - 2026-03-27

### 新增

- 灵活的标签模块系统，支持通过 Cargo features 按需编译标签组
- 新增 `standard`、`vendors-common`、`balanced`、`full` 等预设 feature 组合
- 标签目录重构，按功能分组（standard/vendors/formats/video/audio）

### 修复

- 修复标签数量不匹配问题，现在严格匹配 ExifTool 的 18,046 个标签
- 删除多余的标签组快捷方式（如 AllDates、Canon、Nikon 等）
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

[未发布]: https://github.com/openappsys/exiftool-rs-wrapper/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/openappsys/exiftool-rs-wrapper/releases/tag/v0.1.4
[0.1.3]: https://github.com/openappsys/exiftool-rs-wrapper/releases/tag/v0.1.3
[0.1.0]: https://github.com/openappsys/exiftool-rs-wrapper/releases/tag/v0.1.0
