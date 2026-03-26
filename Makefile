# ExifTool Rust Wrapper 性能基准测试 Makefile

.PHONY: bench bench-metadata bench-write bench-batch bench-pool bench-comparison \
        bench-all bench-clean bench-report bench-quick bench-ci help

# 默认基准测试目录
BENCH_DIR := benches
REPORT_DIR := target/criterion

# 帮助信息
help:
	@echo "ExifTool Rust Wrapper 性能基准测试"
	@echo ""
	@echo "可用命令："
	@echo "  make bench              - 运行所有基准测试"
	@echo "  make bench-metadata     - 运行元数据读取基准测试"
	@echo "  make bench-write        - 运行写入操作基准测试"
	@echo "  make bench-batch        - 运行批量处理基准测试"
	@echo "  make bench-pool         - 运行连接池基准测试"
	@echo "  make bench-comparison   - 运行综合对比基准测试"
	@echo "  make bench-quick        - 快速运行基准测试（减少迭代次数）"
	@echo "  make bench-ci           - CI 环境下的基准测试"
	@echo "  make bench-clean        - 清理基准测试结果"
	@echo "  make bench-report       - 查看基准测试报告"

# 运行所有基准测试
bench:
	cargo bench

# 单独运行各个基准测试
bench-metadata:
	cargo bench --bench bench_metadata

bench-write:
	cargo bench --bench bench_write

bench-batch:
	cargo bench --bench bench_batch

bench-pool:
	cargo bench --bench bench_pool

bench-comparison:
	cargo bench --bench bench_comparison

# 运行特定基准测试组
bench-single-file:
	cargo bench --bench bench_metadata single_file_read

bench-batch-vs-single:
	cargo bench --bench bench_comparison batch_vs_single_read

bench-pool-vs-single:
	cargo bench --bench bench_metadata pool_vs_single

bench-scaling:
	cargo bench --bench bench_comparison scaling_by_file

bench-read-write:
	cargo bench --bench bench_comparison read_vs_write

# 快速基准测试（用于开发和调试）
bench-quick:
	cargo bench -- --quick

# CI 环境下的基准测试（快速，减少资源占用）
bench-ci:
	cargo bench -- --quiet --noplot

# 清理基准测试结果
bench-clean:
	rm -rf $(REPORT_DIR)
	@echo "基准测试结果已清理"

# 查看基准测试报告
bench-report:
	@echo "基准测试报告位置：$(REPORT_DIR)"
	@ls -la $(REPORT_DIR) 2>/dev/null || echo "暂无基准测试结果"

# 运行特定名称的基准测试
# 用法：make bench-name BENCH=single_file_read
bench-name:
	cargo bench $(BENCH)

# 基准测试并生成详细报告
bench-verbose:
	cargo bench -- --verbose

# 仅运行基准测试（不保存结果）
bench-no-save:
	cargo bench -- --no-run

# 保存基准测试结果为 JSON
bench-save:
	cargo bench -- --save-baseline current

# 与之前的基准测试对比
bench-compare:
	cargo bench -- --baseline current

# 列出所有可用的基准测试
bench-list:
	cargo bench -- --list

# 所有基准测试（全部运行）
bench-all: bench
