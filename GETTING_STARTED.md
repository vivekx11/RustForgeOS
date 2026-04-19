# Getting Started with RustForge OS

## Prerequisites

### Required Software
1. **Rust Nightly Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup toolchain install nightly
   rustup default nightly
   ```

2. **Rust Components**
   ```bash
   rustup component add rust-src
   rustup component add llvm-tools-preview
   ```

3. **Bootimage Tool**
   ```bash
   cargo install bootimage
   ```

4. **QEMU Emulator**
   - **Linux**: `sudo apt-get install qemu-system-x86`
   - **macOS**: `brew install qemu`
   - **Windows**: Download from https://www.qemu.org/download/#windows

### Optional Tools
- **GDB**: For debugging (`sudo apt-get install gdb`)
- **Make**: For using Makefile targets

## Quick Start

### 1. Clone and Setup
```bash
# If you haven't already, run the setup script
chmod +x scripts/setup.sh
./scripts/setup.sh
```

### 2. Build the Project
```bash
# Build all workspace crates
cargo build --workspace --release

# Or use Make
make build
```

### 3. Build the Kernel
```bash
# Create bootable kernel image
cd kernel
cargo bootimage --release

# Or use Make
make kernel
```

### 4. Run in QEMU
```bash
# Run with VGA output
make run

# Run with serial output (recommended for debugging)
make qemu
```

You should see:
```
RustForge OS v0.1.0
Initializing kernel...
[OK] GDT initialized
[OK] IDT initialized
[OK] Memory management initialized
[OK] Scheduler initialized
Kernel initialization complete!
Entering idle loop...
```

## Project Structure

```
RustForge-OS/
├── kernel/          # Bare metal kernel
├── allocator/       # Memory allocators
├── lang/            # ForgeScript language
├── net/             # Networking stack
├── db/              # ForgeDB database
├── sandbox/         # Security sandbox
├── distributed/     # Raft consensus
├── runtime/         # Async runtime
├── wasm/            # WASM runtime
├── monitor/         # Security monitor
├── shell/           # Interactive shell
├── common/          # Shared utilities
├── examples/        # ForgeScript examples
├── scripts/         # Build scripts
├── Cargo.toml       # Workspace manifest
├── Makefile         # Build targets
├── README.md        # Project overview
├── ROADMAP.md       # Development plan
└── ARCHITECTURE.md  # System design
```

## Development Workflow

### Building Components

```bash
# Build specific crate
cargo build -p kernel --release
cargo build -p lang --release
cargo build -p net --release

# Build everything
cargo build --workspace --release
```

### Running Tests

```bash
# Test specific crate
cargo test -p allocator
cargo test -p lang
cargo test -p net

# Test everything
cargo test --workspace

# Or use the test script
chmod +x scripts/test-all.sh
./scripts/test-all.sh
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --workspace -- -D warnings

# Generate documentation
cargo doc --workspace --no-deps --open
```

## Phase-by-Phase Development

### Phase 1: Project Structure ✅
**Status**: Complete

**What's Done**:
- Workspace setup
- All crate manifests
- Build configuration
- Common types and errors

### Phase 2: Kernel (Current)
**Goal**: Bootable kernel with memory management

**Steps**:
1. Verify kernel builds:
   ```bash
   cd kernel
   cargo build --release
   ```

2. Create bootable image:
   ```bash
   cargo bootimage --release
   ```

3. Test in QEMU:
   ```bash
   cargo run --release
   ```

**Expected Output**:
- Kernel boots
- VGA text appears
- Interrupts work (keyboard input)
- Memory allocator functional

**Files to Review**:
- `kernel/src/main.rs`: Entry point
- `kernel/src/gdt.rs`: GDT setup
- `kernel/src/interrupts.rs`: IDT and handlers
- `kernel/src/memory.rs`: Paging
- `allocator/src/buddy.rs`: Allocator

### Phase 3: Memory Allocator
**Goal**: Production-ready heap allocator

**Steps**:
1. Test buddy allocator:
   ```bash
   cargo test -p allocator --features buddy
   ```

2. Benchmark performance:
   ```bash
   cargo bench -p allocator
   ```

3. Integrate with kernel:
   - Modify `kernel/Cargo.toml` to enable buddy feature
   - Test allocation in kernel

### Phase 4: ForgeScript Language
**Goal**: Working language with VM

**Steps**:
1. Test lexer:
   ```bash
   cargo test -p lang -- lexer
   ```

2. Test parser:
   ```bash
   cargo test -p lang -- parser
   ```

3. Test VM:
   ```bash
   cargo test -p lang -- vm
   ```

4. Run example programs:
   ```bash
   # Once shell is ready
   cargo run -p shell
   > run examples/hello.forge
   ```

**Example Programs**:
- `examples/hello.forge`: Hello world
- `examples/fibonacci.forge`: Recursion
- `examples/array.forge`: Array operations

### Phase 5-11: Remaining Components
See [ROADMAP.md](ROADMAP.md) for detailed plans.

## Debugging

### Serial Output
```bash
# Run with serial output to stdio
qemu-system-x86_64 \
  -drive format=raw,file=target/x86_64-unknown-none/release/bootimage-kernel.bin \
  -serial stdio
```

### GDB Debugging
```bash
# Terminal 1: Start QEMU with GDB server
make debug-qemu

# Terminal 2: Connect GDB
make gdb
```

GDB commands:
```gdb
(gdb) break kernel_main
(gdb) continue
(gdb) step
(gdb) print variable_name
(gdb) backtrace
```

### Logging
- **Kernel**: Use `serial_println!()` macro
- **Userspace**: Use `println!()` macro

Example:
```rust
serial_println!("Debug: value = {}", x);
```

## Common Issues

### Issue: "bootimage not found"
**Solution**:
```bash
cargo install bootimage
```

### Issue: "rust-src not found"
**Solution**:
```bash
rustup component add rust-src
```

### Issue: QEMU not found
**Solution**: Install QEMU for your platform (see Prerequisites)

### Issue: Build fails with "can't find crate"
**Solution**:
```bash
cargo clean
cargo build --workspace
```

### Issue: Kernel panics immediately
**Solution**: Check serial output for panic message
```bash
make qemu
```

## Next Steps

1. **Complete Phase 2**: Get kernel fully functional
   - Review `kernel/src/main.rs`
   - Test memory allocation
   - Verify interrupts work

2. **Start Phase 3**: Implement buddy allocator
   - Review `allocator/src/buddy.rs`
   - Write unit tests
   - Benchmark performance

3. **Begin Phase 4**: Build ForgeScript
   - Test lexer with example code
   - Implement parser tests
   - Write VM tests

4. **Read Documentation**:
   - [ARCHITECTURE.md](ARCHITECTURE.md): System design
   - [ROADMAP.md](ROADMAP.md): Development plan
   - [README.md](README.md): Project overview

## Learning Resources

### OS Development
- [OSDev Wiki](https://wiki.osdev.org/)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- "Operating Systems: Three Easy Pieces" (free online)

### Rust Embedded
- [Rust Embedded Book](https://rust-embedded.github.io/book/)
- [Rust Nomicon](https://doc.rust-lang.org/nomicon/) (unsafe Rust)

### Language Implementation
- "Crafting Interpreters" by Bob Nystrom (free online)
- [LLVM Tutorial](https://llvm.org/docs/tutorial/)

### Networking
- RFC 791 (IP), RFC 793 (TCP)
- "TCP/IP Illustrated" by Stevens
- [smoltcp documentation](https://docs.rs/smoltcp/)

### Databases
- "Database Internals" by Alex Petrov
- [SQLite source code](https://www.sqlite.org/src/doc/trunk/README.md)

### Distributed Systems
- [Raft paper](https://raft.github.io/raft.pdf)
- "Designing Data-Intensive Applications" by Martin Kleppmann

## Contributing

This is an educational project. Contributions welcome!

### Guidelines
1. Follow Rust style guide (`cargo fmt`)
2. Pass all tests (`cargo test --workspace`)
3. Document public APIs
4. Explain unsafe code
5. Add tests for new features

### Areas Needing Work
- [ ] Complete kernel implementation
- [ ] Buddy allocator optimization
- [ ] ForgeScript standard library
- [ ] TCP state machine
- [ ] B-Tree implementation
- [ ] Raft consensus
- [ ] WASM executor
- [ ] Documentation

## Getting Help

### Resources
- **Documentation**: Run `cargo doc --workspace --open`
- **Code Comments**: Read inline documentation
- **Architecture**: See [ARCHITECTURE.md](ARCHITECTURE.md)
- **Roadmap**: See [ROADMAP.md](ROADMAP.md)

### Community
- Open an issue on GitHub
- Check existing documentation
- Review example code

## License

MIT OR Apache-2.0

---

**Ready to start?** Run `make build` and then `make run`!
