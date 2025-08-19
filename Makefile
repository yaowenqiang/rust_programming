# Makefile for Rust Programming Examples

# 默认目标
.PHONY: help build clean run-all fmt

# Cargo 项目路径
CARGO_PROJECT := rust_cargo_project

# 所有示例程序
EXAMPLES := hello_world variables_and_types flow_control enum_demo borrow_check arrays_demo string_demo demo_static_global

help:
	@echo "Rust Programming Examples - Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make help             - 显示此帮助信息"
	@echo "  make build            - 构建所有示例"
	@echo "  make clean            - 清理构建产物"
	@echo "  make run-all          - 运行所有示例"
	@echo "  make fmt              - 格式化所有源代码"
	@echo ""
	@echo "运行单个示例:"
	@for example in $(EXAMPLES); do \
		echo "  make run-$$example"; \
	done
	@echo ""
	@echo "或者直接使用 Cargo:"
	@echo "  cd $(CARGO_PROJECT) && cargo run --bin <example_name>"

build:
	cd $(CARGO_PROJECT) && cargo build

clean:
	cd $(CARGO_PROJECT) && cargo clean

run-all: build
	@for example in $(EXAMPLES); do \
		echo "Running $$example..."; \
		cd $(CARGO_PROJECT) && cargo run --bin $$example; \
		echo ""; \
	done

fmt:
	find . -name "*.rs" -not -path "./target/*" -exec rustfmt {} \;
	cd $(CARGO_PROJECT) && find src -name "*.rs" -exec rustfmt {} \;

# 为每个示例生成运行目标
$(EXAMPLES):
	cd $(CARGO_PROJECT) && cargo run --bin $@

# 为每个示例生成 run-* 目标
$(addprefix run-, $(EXAMPLES)): run-%:
	cd $(CARGO_PROJECT) && cargo run --bin $*
