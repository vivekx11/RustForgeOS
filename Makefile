# RustForge OS Makefile

.PHONY: all build test clean run kernel qemu help

all: build

help:
	@echo "RustForge OS Build System"
	@echo ""
	@echo "Targets:"
	@echo "  build    - Build all workspace crates"
	@echo "  kernel   - Build kernel and create bootable image"
	@echo "  test     - Run all tests"
	@echo "  run      - Run kernel in QEMU"
	@echo "  qemu     - Run kernel in QEMU with serial output"
	@echo "  clean    - Clean build artifacts"
	@echo "  fmt      - Format all code"
	@echo "  clippy   - Run clippy linter"
	@echo "  doc      - Generate documentation"

build:
	cargo build --workspace --release

kernel:
	cd kernel && cargo bootimage --release

test:
	cargo test --workspace

clean:
	cargo clean

run: kernel
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/bootimage-kernel.bin

qemu: kernel
	qemu-system-x86_64 \
		-drive format=raw,file=target/x86_64-unknown-none/release/bootimage-kernel.bin \
		-serial stdio \
		-display none

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace -- -D warnings

doc:
	cargo doc --workspace --no-deps --open

# Debug targets
debug-qemu: kernel
	qemu-system-x86_64 \
		-drive format=raw,file=target/x86_64-unknown-none/release/bootimage-kernel.bin \
		-serial stdio \
		-s -S

gdb:
	gdb target/x86_64-unknown-none/release/kernel \
		-ex "target remote :1234"
