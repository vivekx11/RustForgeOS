# RustForge OS - Quick Reference Card

## 🚀 Quick Start (5 Minutes)

```bash
# 1. Setup environment
./scripts/setup.sh

# 2. Build everything
make build

# 3. Run kernel in QEMU
make run
```

## 📦 Build Commands

| Command | Description |
|---------|-------------|
| `make build` | Build all workspace crates |
| `make kernel` | Build bootable kernel image |
| `make clean` | Clean build artifacts |
| `cargo build -p <crate>` | Build specific crate |
| `cargo build --release` | Release build (optimized) |

## 🧪 Testing Commands

| Command | Description |
|---------|-------------|
| `make test` | Run all tests |
| `cargo test -p <crate>` | Test specific crate |
| `./scripts/test-all.sh` | Comprehensive test suite |
| `cargo test -- --nocapture` | Show test output |
| `cargo bench -p <crate>` | Run benchmarks |

## 🏃 Run Commands

| Command | Description |
|---------|-------------|
| `make run` | Run kernel in QEMU (VGA) |
| `make qemu` | Run kernel in QEMU (serial) |
| `make debug-qemu` | Run with GDB server |
| `cargo run -p shell` | Run shell (when ready) |

## 🔍 Debug Commands

| Command | Description |
|---------|-------------|
| `make debug-qemu` | Start QEMU with GDB server |
| `make gdb` | Connect GDB to QEMU |
| `cargo run -- -serial stdio` | Show serial output |

## ✨ Code Quality

| Command | Description |
|---------|-------------|
| `make fmt` | Format all code |
| `make clippy` | Run linter |
| `make doc` | Generate documentation |
| `cargo fmt --check` | Check formatting |
| `cargo clippy -- -D warnings` | Strict linting |

## 📚 Documentation

| File | Description |
|------|-------------|
| `README.md` | Project overview |
| `ROADMAP.md` | 12-phase development plan |
| `ARCHITECTURE.md` | System design & architecture |
| `GETTING_STARTED.md` | Setup & usage guide |
| `PROJECT_SUMMARY.md` | Phase 1 completion summary |
| `PROJECT_TREE.txt` | Complete file structure |
| `QUICK_REFERENCE.md` | This file |

## 🎯 Project Structure

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
└── common/          # Shared utilities
```

## 🔧 Crate-Specific Commands

### Kernel
```bash
cd kernel
cargo build --release
cargo bootimage --release
cargo run --release
```

### Language
```bash
cargo test -p lang
cargo bench -p lang
# Test with example:
# cargo run -p shell < examples/hello.forge
```

### Allocator
```bash
cargo test -p allocator
cargo test -p allocator --features buddy
cargo bench -p allocator
```

### Networking
```bash
cargo test -p net
cargo test -p net --test integration -- --ignored
```

### Database
```bash
cargo test -p db
cargo bench -p db
```

## 🐛 Common Issues & Solutions

### Issue: "cargo not found"
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Issue: "bootimage not found"
```bash
cargo install bootimage
```

### Issue: "rust-src not found"
```bash
rustup component add rust-src
rustup component add llvm-tools-preview
```

### Issue: "QEMU not found"
```bash
# Linux
sudo apt-get install qemu-system-x86

# macOS
brew install qemu

# Windows
# Download from https://www.qemu.org/download/#windows
```

### Issue: Build fails
```bash
cargo clean
cargo build --workspace
```

## 📊 Project Statistics

| Metric | Value |
|--------|-------|
| Total Lines of Code | ~5,000 |
| Rust Source Files | 70+ |
| Workspace Crates | 12 |
| Documentation Files | 6 |
| Example Programs | 3 |

## 🎓 Learning Path

### Week 1-2: Kernel
1. Read `kernel/src/main.rs`
2. Study `kernel/src/gdt.rs`
3. Understand `kernel/src/interrupts.rs`
4. Review `kernel/src/memory.rs`
5. Test in QEMU

### Week 3-4: Language
1. Study `lang/src/lexer.rs`
2. Review `lang/src/parser.rs`
3. Understand `lang/src/compiler.rs`
4. Explore `lang/src/vm.rs`
5. Run examples

### Week 5-6: Networking
1. Read `net/src/ethernet.rs`
2. Study `net/src/ip.rs`
3. Review TCP/UDP
4. Test with packets

### Week 7-8: Database
1. Study `db/src/btree.rs`
2. Understand WAL
3. Review transactions
4. Test queries

### Week 9-10: Distributed
1. Read Raft paper
2. Study `distributed/src/raft.rs`
3. Test consensus
4. Multi-node setup

### Week 11-12: Integration
1. Connect all components
2. Build shell
3. End-to-end demo
4. Documentation

## 🔗 External Resources

### OS Development
- [OSDev Wiki](https://wiki.osdev.org/)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)

### Language Implementation
- [Crafting Interpreters](https://craftinginterpreters.com/)
- [LLVM Tutorial](https://llvm.org/docs/tutorial/)

### Networking
- [RFC 791 (IP)](https://tools.ietf.org/html/rfc791)
- [RFC 793 (TCP)](https://tools.ietf.org/html/rfc793)
- [smoltcp docs](https://docs.rs/smoltcp/)

### Databases
- [Database Internals](https://www.databass.dev/)
- [SQLite Architecture](https://www.sqlite.org/arch.html)

### Distributed Systems
- [Raft Paper](https://raft.github.io/raft.pdf)
- [Designing Data-Intensive Applications](https://dataintensive.net/)

## 💡 Tips & Tricks

### Fast Iteration
```bash
# Use cargo watch for auto-rebuild
cargo install cargo-watch
cargo watch -x "build -p kernel"
```

### Better Error Messages
```bash
# Use cargo-expand to see macro expansions
cargo install cargo-expand
cargo expand -p lang
```

### Profiling
```bash
# Use cargo-flamegraph
cargo install flamegraph
cargo flamegraph --bench vm_benchmark
```

### Documentation
```bash
# Generate and open docs
cargo doc --workspace --no-deps --open

# Document private items
cargo doc --workspace --document-private-items
```

## 🎯 Phase Checklist

### Phase 1: Project Structure ✅
- [x] Workspace setup
- [x] All crates defined
- [x] Build system configured
- [x] Documentation written

### Phase 2: Kernel (Current)
- [ ] Kernel boots in QEMU
- [ ] VGA output works
- [ ] Interrupts functional
- [ ] Memory allocator tested

### Phase 3: Allocator
- [ ] Buddy allocator complete
- [ ] Unit tests passing
- [ ] Benchmarks run
- [ ] Integrated with kernel

### Phase 4: Language
- [ ] Lexer tested
- [ ] Parser tested
- [ ] VM tested
- [ ] Examples run

### Phase 5-12: See ROADMAP.md

## 🚨 Important Notes

### Safety
- Every `unsafe` block is documented
- Safety invariants are explained
- Review before modifying

### Testing
- Write tests for new features
- Aim for >80% coverage
- Test edge cases

### Documentation
- Document public APIs
- Explain design decisions
- Add examples

### Performance
- Profile before optimizing
- Benchmark changes
- Document tradeoffs

## 📞 Getting Help

1. **Read Documentation**: Check README, ROADMAP, ARCHITECTURE
2. **Review Code**: Read inline comments and documentation
3. **Run Tests**: See what's working
4. **Check Examples**: Look at example programs
5. **Debug**: Use serial output and GDB

## 🎉 Success Indicators

### Kernel Works
```
RustForge OS v0.1.0
Initializing kernel...
[OK] GDT initialized
[OK] IDT initialized
[OK] Memory management initialized
[OK] Scheduler initialized
Kernel initialization complete!
```

### Tests Pass
```
running 42 tests
test result: ok. 42 passed; 0 failed
```

### Code Quality
```
cargo fmt --check    # No changes needed
cargo clippy         # No warnings
```

---

**Quick Start**: `./scripts/setup.sh && make build && make run`

**Next Steps**: See [GETTING_STARTED.md](GETTING_STARTED.md)

**Full Plan**: See [ROADMAP.md](ROADMAP.md)
