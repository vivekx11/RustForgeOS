# RustForge OS Development Roadmap

## ✅ Phase 1: Project Structure (COMPLETE)
- [x] Workspace Cargo.toml with all crates
- [x] Individual crate Cargo.toml files
- [x] Build configuration (.cargo/config.toml)
- [x] Rust toolchain setup
- [x] Target specification (x86_64-unknown-none)
- [x] Common types and error handling

## 🔄 Phase 2: Kernel (IN PROGRESS)
### Core Components
- [x] Boot sequence with bootloader
- [x] GDT (Global Descriptor Table) setup
- [x] IDT (Interrupt Descriptor Table) with handlers
- [x] VGA text mode output
- [x] Serial port debugging
- [x] Page table management
- [x] Frame allocator
- [x] Basic scheduler structure

### Testing & Verification
```bash
cd kernel
cargo build --release
cargo bootimage --release
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/bootimage-kernel.bin
```

### Unsafe Blocks Explained
1. **GDT/IDT Setup**: Direct hardware register manipulation
2. **Memory Management**: Raw pointer operations for page tables
3. **Interrupt Handlers**: x86-interrupt calling convention
4. **VGA Buffer**: Memory-mapped I/O at 0xb8000

## 📋 Phase 3: Memory Allocator
### Components
- [x] Buddy allocator structure
- [x] Linked-list allocator (fallback)
- [x] GlobalAlloc trait implementation
- [ ] Heap initialization in kernel
- [ ] Unit tests for allocator
- [ ] Benchmarks

### Testing
```bash
cargo test -p allocator
cargo bench -p allocator
```

### Design Decisions
- **Buddy Allocator**: O(log n) allocation, good for power-of-2 sizes
- **Linked List**: Fallback for arbitrary sizes
- **Feature Flag**: Switch between implementations

## 📋 Phase 4: ForgeScript Language
### Components
- [x] Lexer with logos
- [x] Token definitions
- [x] Recursive descent parser
- [x] AST node definitions
- [x] Bytecode instruction set
- [x] Stack-based VM
- [x] Bytecode compiler
- [ ] Function calls and closures
- [ ] Standard library functions
- [ ] REPL

### Testing
```bash
cargo test -p lang
# Test example programs
echo 'let x = 5 + 3; x * 2;' | cargo run -p shell
```

### Language Features
- **Types**: int, float, string, bool, array
- **Control Flow**: if/else, while, for
- **Functions**: First-class, closures (TODO)
- **VM**: Stack-based, bytecode interpreter

## 📋 Phase 5: Networking Stack
### Layers (Bottom-Up)
- [ ] Ethernet frame parsing/generation
- [ ] ARP (Address Resolution Protocol)
- [ ] IPv4 packet handling
- [ ] ICMP (ping)
- [ ] UDP sockets
- [ ] TCP state machine
- [ ] HTTP/1.1 server
- [ ] DNS resolver

### Testing
```bash
cargo test -p net
# Integration test with TAP device
sudo cargo test -p net --test integration -- --ignored
```

### Crate Suggestions
- **smoltcp**: Reference implementation (study, don't copy)
- **etherparse**: Packet parsing utilities
- Custom implementation for learning

## 📋 Phase 6: ForgeDB Database
### Components
- [ ] B-Tree node structure
- [ ] B-Tree insert/delete/search
- [ ] Page management
- [ ] Write-Ahead Log (WAL)
- [ ] SQL-like query parser
- [ ] Query planner
- [ ] Transaction manager (ACID)
- [ ] Lock manager
- [ ] Buffer pool

### Testing
```bash
cargo test -p db
# Benchmark
cargo bench -p db
```

### Design Decisions
- **B-Tree Order**: 128 (tune based on page size)
- **Page Size**: 4KB (matches OS page size)
- **WAL**: Append-only log for crash recovery
- **Isolation**: Serializable (simplest to implement)

## 📋 Phase 7: Async Runtime + Thread Pool
### Components
- [ ] Future trait (custom, no std)
- [ ] Waker implementation
- [ ] Task queue
- [ ] Executor (single-threaded)
- [ ] Multi-threaded executor
- [ ] Work-stealing scheduler
- [ ] Async I/O primitives

### Testing
```bash
cargo test -p runtime
cargo bench -p runtime
```

### Crate Suggestions
- **crossbeam**: Lock-free data structures
- Study tokio/async-std source code

## 📋 Phase 8: Security Sandbox
### Components
- [ ] Capability system
- [ ] Permission bits
- [ ] Syscall filter table
- [ ] Process isolation (namespace concept)
- [ ] Resource limits
- [ ] Safe ForgeScript executor

### Testing
```bash
cargo test -p sandbox
# Test malicious scripts
cargo test -p sandbox --test exploit_attempts
```

### Security Model
- **Capabilities**: Read, Write, Execute, Network, FileSystem
- **Principle of Least Privilege**: Default deny
- **Sandboxed Execution**: ForgeScript runs in isolated context

## 📋 Phase 9: Distributed Layer
### Components
- [ ] Raft state machine
- [ ] Leader election
- [ ] Log replication
- [ ] Snapshot mechanism
- [ ] Node discovery
- [ ] Key-value store
- [ ] RPC over custom network stack

### Testing
```bash
cargo test -p distributed
# Multi-node test
cargo test -p distributed --test cluster -- --ignored
```

### Raft Implementation
- **Election Timeout**: 150-300ms
- **Heartbeat**: 50ms
- **Log Compaction**: Snapshot every 1000 entries

## 📋 Phase 10: WASM Runtime
### Components
- [ ] WASM binary parser
- [ ] Module validation
- [ ] Instruction decoder
- [ ] Stack machine executor
- [ ] Memory management
- [ ] Import/export handling
- [ ] WASI support (subset)

### Testing
```bash
cargo test -p wasm
# Test with real WASM modules
cargo test -p wasm --test wasm_modules
```

### Crate Suggestions
- **wasmparser**: Binary format parsing
- **wasmtime**: Reference (study architecture)
- Custom interpreter for learning

## 📋 Phase 11: Security Monitor
### Components
- [ ] Packet capture interface
- [ ] Protocol dissectors
- [ ] Anomaly detection rules
- [ ] Logging to ForgeDB
- [ ] Alert system
- [ ] Statistics dashboard

### Testing
```bash
cargo test -p monitor
```

## 📋 Phase 12: Integration + Shell
### Components
- [ ] Command parser
- [ ] Built-in commands (ls, cat, etc.)
- [ ] ForgeScript REPL
- [ ] Database CLI
- [ ] Network utilities (ping, curl)
- [ ] System information
- [ ] Process management

### Testing
```bash
cargo run -p shell
# End-to-end demo
./scripts/demo.sh
```

### Demo Script
1. Boot kernel
2. Run ForgeScript program
3. Store data in ForgeDB
4. Query database
5. Start HTTP server
6. Make network request
7. Show distributed replication
8. Run WASM module
9. Monitor network traffic

## 🎯 Milestones

### Milestone 1: Bootable Kernel (Week 1-2)
- Kernel boots in QEMU
- VGA output works
- Interrupts handled
- Memory management functional

### Milestone 2: Language Runtime (Week 3-4)
- ForgeScript compiles and runs
- VM executes bytecode
- Basic programs work

### Milestone 3: Networking (Week 5-6)
- TCP/IP stack functional
- HTTP server responds
- DNS resolution works

### Milestone 4: Database (Week 7-8)
- B-Tree operations work
- WAL recovery tested
- Transactions ACID-compliant

### Milestone 5: Distributed (Week 9-10)
- Raft consensus works
- Multi-node cluster
- KV store replicated

### Milestone 6: Full Integration (Week 11-12)
- All components integrated
- End-to-end demo
- Documentation complete

## 📚 Learning Resources

### OS Development
- [OSDev Wiki](https://wiki.osdev.org/)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- "Operating Systems: Three Easy Pieces"

### Language Implementation
- "Crafting Interpreters" by Bob Nystrom
- "Engineering a Compiler"
- LLVM documentation

### Networking
- RFC 791 (IP), RFC 793 (TCP)
- "TCP/IP Illustrated" by Stevens
- smoltcp source code

### Databases
- "Database Internals" by Alex Petrov
- SQLite source code
- PostgreSQL documentation

### Distributed Systems
- Raft paper (raft.github.io)
- "Designing Data-Intensive Applications"
- etcd/Consul source code

## 🔧 Development Commands

### Build Everything
```bash
cargo build --workspace --release
```

### Test Everything
```bash
cargo test --workspace
```

### Run Kernel in QEMU
```bash
cd kernel
cargo run --release
```

### Format Code
```bash
cargo fmt --all
```

### Lint
```bash
cargo clippy --workspace -- -D warnings
```

### Documentation
```bash
cargo doc --workspace --no-deps --open
```

## 🐛 Debugging

### QEMU with GDB
```bash
qemu-system-x86_64 -drive format=raw,file=kernel.bin -s -S
# In another terminal
gdb target/x86_64-unknown-none/release/kernel
(gdb) target remote :1234
(gdb) continue
```

### Serial Output
```bash
qemu-system-x86_64 -drive format=raw,file=kernel.bin -serial stdio
```

### Logging
- Kernel: Serial port (COM1)
- Userspace: VGA buffer
- Network: Packet dumps to ForgeDB

## 📊 Performance Goals

- **Boot Time**: < 1 second
- **ForgeScript**: > 1M ops/sec
- **Database**: > 10K transactions/sec
- **Network**: > 100 Mbps throughput
- **Raft**: < 100ms leader election

## 🚀 Future Enhancements

- [ ] SMP (multi-core) support
- [ ] USB driver
- [ ] Filesystem (ext2/FAT32)
- [ ] Graphics mode (framebuffer)
- [ ] JIT compiler for ForgeScript
- [ ] WASM JIT with Cranelift
- [ ] TLS/SSL support
- [ ] Container runtime
- [ ] Package manager

## 📝 Notes

- **Safety**: Document every `unsafe` block
- **Testing**: Aim for >80% code coverage
- **Documentation**: Every public API documented
- **Performance**: Profile before optimizing
- **Incremental**: Each phase should compile and run
