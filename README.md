# RustForge OS

[![Rust CI](https://github.com/vivekx11/RustForgeOS/workflows/Rust%20CI/badge.svg)](https://github.com/vivekx11/RustForgeOS/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-x86__64-blue.svg)](https://en.wikipedia.org/wiki/X86-64)

A comprehensive operating system written in Rust, featuring a custom kernel, programming language, networking stack, database engine, and distributed systems layer.

**🎉 Phase 1 Complete!** - Full project structure with 12 integrated crates and comprehensive documentation.

## 🏗️ Architecture

```
RustForge OS
├── kernel          → Custom x86_64 kernel (bare metal)
├── allocator       → Buddy & linked-list memory allocators
├── lang            → ForgeScript language + VM
├── net             → Full TCP/IP networking stack
├── db              → ForgeDB (B-Tree database with ACID)
├── sandbox         → Security isolation & capabilities
├── distributed     → Raft consensus + replicated KV store
├── runtime         → Custom async executor + thread pool
├── wasm            → WebAssembly runtime
├── monitor         → Network packet sniffer & security
├── shell           → Interactive shell
└── common          → Shared types and utilities
```

## 📦 Components

### 1. **Kernel** (`kernel/`)
- Bare metal x86_64 kernel (no_std)
- GDT (Global Descriptor Table) setup
- IDT (Interrupt Descriptor Table) with interrupt handlers
- Page table management and memory mapping
- Simple round-robin process scheduler
- VGA text mode output
- Serial port debugging

### 2. **Memory Allocator** (`allocator/`)
- Buddy allocator system (power-of-2 blocks with merging)
- Linked-list allocator (fallback)
- GlobalAlloc implementation for kernel heap
- Configurable via features

### 3. **ForgeScript Language** (`lang/`)
- Lexer with token recognition
- Recursive descent parser
- AST (Abstract Syntax Tree) representation
- Bytecode compiler
- Stack-based virtual machine
- Types: int, float, string, bool, array
- Control flow: if/else, loops, functions

### 4. **Networking Stack** (`net/`)
- Ethernet frame parsing
- ARP (Address Resolution Protocol)
- IPv4 implementation
- TCP/UDP protocols
- HTTP server
- DNS resolver

### 5. **ForgeDB Database** (`db/`)
- B-Tree based storage engine
- Write-Ahead Logging (WAL) for crash recovery
- SQL-like query parser
- ACID transaction support
- Async I/O via custom runtime

### 6. **Security Sandbox** (`sandbox/`)
- Process isolation
- Syscall filtering
- Capability-based permissions
- Safe ForgeScript execution environment

### 7. **Distributed Layer** (`distributed/`)
- Raft consensus algorithm
- Leader election
- Replicated key-value store
- Node discovery
- Works over custom networking stack

### 8. **Async Runtime** (`runtime/`)
- Custom Future executor (no tokio)
- Work-stealing thread pool scheduler
- Used by networking and database layers

### 9. **WASM Runtime** (`wasm/`)
- WebAssembly bytecode parser
- WASM instruction executor
- Integrated into OS for native .wasm execution

### 10. **Security Monitor** (`monitor/`)
- Network packet sniffer
- Anomaly detection
- Logging to ForgeDB

### 11. **Shell** (`shell/`)
- Interactive command-line interface
- Integrates all components
- Scripting support via ForgeScript

## 🚀 Building

### Prerequisites
- Rust nightly toolchain
- `cargo-xbuild` or `cargo build-std`
- QEMU (for testing)
- `bootimage` tool

### Build Commands

```bash
# Build the entire workspace
cargo build --release

# Build kernel specifically
cd kernel
cargo build --release

# Create bootable kernel image
cargo bootimage --release

# Run in QEMU
cargo run --release
```

### Testing

```bash
# Run all tests
cargo test

# Test specific component
cargo test -p lang
cargo test -p allocator
cargo test -p net
```

## 📖 Development Phases

### ✅ Phase 1: Project Structure
- Workspace setup with all crates
- Dependency management
- Build configuration

### 🔄 Phase 2: Kernel (In Progress)
- Boot sequence
- GDT/IDT setup
- Memory management
- Interrupt handling

### 📋 Phase 3-11: Remaining Components
See [ROADMAP.md](ROADMAP.md) for detailed implementation plan.

## 🛠️ Key Technologies

- **bootloader**: Bootloader for x86_64 kernels
- **x86_64**: x86_64 architecture support
- **logos**: Fast lexer generator
- **spin**: Spinlock-based synchronization
- **crossbeam**: Lock-free concurrent data structures
- **serde/bincode**: Serialization

## 🔒 Safety

This project uses `unsafe` Rust extensively for:
- Hardware interaction (memory-mapped I/O)
- Page table manipulation
- Interrupt handlers
- Raw pointer operations

Every `unsafe` block is documented with safety invariants.

## 📝 License

MIT OR Apache-2.0

## 🤝 Contributing

This is an educational mega-project. Contributions welcome!

## 🎯 Goals

1. **Educational**: Learn OS development, language design, networking, databases
2. **Integrated**: All components work together in one system
3. **Production-Quality**: Real implementations, not toy examples
4. **Safe**: Leverage Rust's safety where possible
5. **Documented**: Every design decision explained

## 📚 Resources

- [OSDev Wiki](https://wiki.osdev.org/)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)
- [Raft Paper](https://raft.github.io/raft.pdf)
