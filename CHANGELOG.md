# 更新日志

所有该项目的显著变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且该项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [未发布]

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

[未发布]: https://github.com/openappsys/exiftool-rs-wrapper/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/openappsys/exiftool-rs-wrapper/releases/tag/v0.1.0
