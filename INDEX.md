# RustForge OS - Complete Documentation Index

## 📖 Start Here

**New to the project?** Start with these documents in order:

1. **[README.md](README.md)** - Project overview, features, and goals
2. **[GETTING_STARTED.md](GETTING_STARTED.md)** - Setup instructions and first steps
3. **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Command cheat sheet
4. **[ROADMAP.md](ROADMAP.md)** - 12-phase development plan
5. **[ARCHITECTURE.md](ARCHITECTURE.md)** - System design and architecture

## 📚 Documentation Files

### Essential Reading

| Document | Purpose | When to Read |
|----------|---------|--------------|
| [README.md](README.md) | Project overview | First |
| [GETTING_STARTED.md](GETTING_STARTED.md) | Setup & usage | Before coding |
| [QUICK_REFERENCE.md](QUICK_REFERENCE.md) | Command reference | Keep handy |
| [ROADMAP.md](ROADMAP.md) | Development plan | Planning work |
| [ARCHITECTURE.md](ARCHITECTURE.md) | System design | Understanding system |

### Reference Documents

| Document | Purpose | When to Read |
|----------|---------|--------------|
| [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) | Phase 1 completion | Status check |
| [PROJECT_TREE.txt](PROJECT_TREE.txt) | File structure | Finding files |
| [INDEX.md](INDEX.md) | This file | Navigation |

## 🎯 By Role

### I'm a Systems Programmer
**Goal**: Understand the kernel and low-level components

**Read**:
1. [ARCHITECTURE.md](ARCHITECTURE.md) - System design
2. `kernel/src/main.rs` - Kernel entry point
3. `kernel/src/memory.rs` - Memory management
4. `allocator/src/buddy.rs` - Memory allocator

**Focus Areas**:
- Kernel implementation
- Memory management
- Interrupt handling
- Device drivers

### I'm a Language Designer
**Goal**: Understand ForgeScript implementation

**Read**:
1. [ARCHITECTURE.md](ARCHITECTURE.md) - Language section
2. `lang/src/lexer.rs` - Tokenization
3. `lang/src/parser.rs` - Parsing
4. `lang/src/compiler.rs` - Compilation
5. `lang/src/vm.rs` - Execution

**Focus Areas**:
- Language design
- Compiler implementation
- VM optimization
- Type system

### I'm a Network Engineer
**Goal**: Understand networking stack

**Read**:
1. [ARCHITECTURE.md](ARCHITECTURE.md) - Networking section
2. `net/src/ethernet.rs` - Link layer
3. `net/src/ip.rs` - Network layer
4. `net/src/tcp.rs` - Transport layer

**Focus Areas**:
- Protocol implementation
- Packet parsing
- Socket API
- HTTP server

### I'm a Database Developer
**Goal**: Understand ForgeDB

**Read**:
1. [ARCHITECTURE.md](ARCHITECTURE.md) - Database section
2. `db/src/btree.rs` - Storage engine
3. `db/src/wal.rs` - Crash recovery
4. `db/src/transaction.rs` - ACID properties

**Focus Areas**:
- B-Tree implementation
- Transaction management
- Query optimization
- Crash recovery

### I'm a Distributed Systems Engineer
**Goal**: Understand Raft consensus

**Read**:
1. [Raft Paper](https://raft.github.io/raft.pdf)
2. [ARCHITECTURE.md](ARCHITECTURE.md) - Distributed section
3. `distributed/src/raft.rs` - Consensus
4. `distributed/src/kv.rs` - Replicated store

**Focus Areas**:
- Consensus algorithm
- Leader election
- Log replication
- Fault tolerance

### I'm a Security Researcher
**Goal**: Understand security model

**Read**:
1. [ARCHITECTURE.md](ARCHITECTURE.md) - Security section
2. `sandbox/src/capability.rs` - Permissions
3. `sandbox/src/isolation.rs` - Process isolation
4. `monitor/src/sniffer.rs` - Network monitoring

**Focus Areas**:
- Capability system
- Sandboxing
- Syscall filtering
- Anomaly detection

## 🔍 By Topic

### Getting Started
- [GETTING_STARTED.md](GETTING_STARTED.md) - Complete setup guide
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Command reference
- `scripts/setup.sh` - Automated setup

### Architecture & Design
- [ARCHITECTURE.md](ARCHITECTURE.md) - Complete system design
- [ROADMAP.md](ROADMAP.md) - Development phases
- [README.md](README.md) - High-level overview

### Implementation
- [PROJECT_TREE.txt](PROJECT_TREE.txt) - File structure
- [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) - What's implemented
- Individual crate `src/` directories

### Building & Testing
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Build commands
- `Makefile` - Build targets
- `scripts/test-all.sh` - Test runner

### Examples
- `examples/hello.forge` - Hello world
- `examples/fibonacci.forge` - Recursion
- `examples/array.forge` - Arrays

## 📦 By Component

### Kernel
**Files**:
- `kernel/src/main.rs` - Entry point
- `kernel/src/gdt.rs` - GDT setup
- `kernel/src/interrupts.rs` - Interrupt handling
- `kernel/src/memory.rs` - Memory management
- `kernel/src/scheduler.rs` - Process scheduling
- `kernel/src/vga.rs` - VGA output
- `kernel/src/serial.rs` - Serial debugging

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - Kernel section
- [ROADMAP.md](ROADMAP.md) - Phase 2

### Memory Allocator
**Files**:
- `allocator/src/lib.rs` - Main interface
- `allocator/src/buddy.rs` - Buddy allocator
- `allocator/src/linked_list.rs` - Linked list allocator

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - Allocator section
- [ROADMAP.md](ROADMAP.md) - Phase 3

### ForgeScript Language
**Files**:
- `lang/src/lexer.rs` - Lexer
- `lang/src/parser.rs` - Parser
- `lang/src/ast.rs` - AST
- `lang/src/compiler.rs` - Compiler
- `lang/src/vm.rs` - Virtual machine
- `lang/src/bytecode.rs` - Bytecode

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - Language section
- [ROADMAP.md](ROADMAP.md) - Phase 4
- `examples/*.forge` - Example programs

### Networking Stack
**Files**:
- `net/src/ethernet.rs` - Ethernet
- `net/src/arp.rs` - ARP
- `net/src/ip.rs` - IPv4
- `net/src/tcp.rs` - TCP
- `net/src/udp.rs` - UDP
- `net/src/http.rs` - HTTP
- `net/src/dns.rs` - DNS

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - Networking section
- [ROADMAP.md](ROADMAP.md) - Phase 5

### ForgeDB Database
**Files**:
- `db/src/btree.rs` - B-Tree
- `db/src/wal.rs` - Write-Ahead Log
- `db/src/query.rs` - Query parser
- `db/src/transaction.rs` - Transactions
- `db/src/storage.rs` - Storage

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - Database section
- [ROADMAP.md](ROADMAP.md) - Phase 6

### Security Sandbox
**Files**:
- `sandbox/src/capability.rs` - Capabilities
- `sandbox/src/isolation.rs` - Isolation
- `sandbox/src/syscall.rs` - Syscall filtering

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - Security section
- [ROADMAP.md](ROADMAP.md) - Phase 8

### Distributed Layer
**Files**:
- `distributed/src/raft.rs` - Raft consensus
- `distributed/src/kv.rs` - Key-value store
- `distributed/src/node.rs` - Node management

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - Distributed section
- [ROADMAP.md](ROADMAP.md) - Phase 9

### Async Runtime
**Files**:
- `runtime/src/executor.rs` - Executor
- `runtime/src/future.rs` - Future trait
- `runtime/src/task.rs` - Task
- `runtime/src/waker.rs` - Waker
- `runtime/src/thread_pool.rs` - Thread pool

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - Runtime section
- [ROADMAP.md](ROADMAP.md) - Phase 7

### WASM Runtime
**Files**:
- `wasm/src/parser.rs` - Parser
- `wasm/src/module.rs` - Module
- `wasm/src/executor.rs` - Executor

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - WASM section
- [ROADMAP.md](ROADMAP.md) - Phase 10

### Security Monitor
**Files**:
- `monitor/src/sniffer.rs` - Packet sniffer
- `monitor/src/anomaly.rs` - Anomaly detection

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - Monitor section
- [ROADMAP.md](ROADMAP.md) - Phase 11

### Shell
**Files**:
- `shell/src/main.rs` - Interactive shell

**Documentation**:
- [ARCHITECTURE.md](ARCHITECTURE.md) - Shell section
- [ROADMAP.md](ROADMAP.md) - Phase 12

### Common Utilities
**Files**:
- `common/src/error.rs` - Error types
- `common/src/types.rs` - Common types
- `common/src/sync.rs` - Synchronization

## 🎓 Learning Paths

### Path 1: Complete Beginner
1. Read [README.md](README.md)
2. Follow [GETTING_STARTED.md](GETTING_STARTED.md)
3. Run `make build && make run`
4. Read [ARCHITECTURE.md](ARCHITECTURE.md)
5. Study `kernel/src/main.rs`
6. Follow [ROADMAP.md](ROADMAP.md) Phase 2

### Path 2: Experienced Systems Programmer
1. Skim [README.md](README.md)
2. Read [ARCHITECTURE.md](ARCHITECTURE.md)
3. Review [PROJECT_TREE.txt](PROJECT_TREE.txt)
4. Study kernel implementation
5. Start contributing

### Path 3: Language Enthusiast
1. Read [README.md](README.md)
2. Study `lang/` directory
3. Read [ARCHITECTURE.md](ARCHITECTURE.md) language section
4. Run example programs
5. Extend ForgeScript

### Path 4: Network Engineer
1. Read [README.md](README.md)
2. Study `net/` directory
3. Read [ARCHITECTURE.md](ARCHITECTURE.md) networking section
4. Implement protocols
5. Test with packets

## 🔗 External Resources

### OS Development
- [OSDev Wiki](https://wiki.osdev.org/)
- [Writing an OS in Rust](https://os.phil-opp.com/)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)
- [Rust Nomicon](https://doc.rust-lang.org/nomicon/)

### Language Implementation
- [Crafting Interpreters](https://craftinginterpreters.com/)
- [LLVM Tutorial](https://llvm.org/docs/tutorial/)
- [Engineering a Compiler](https://www.elsevier.com/books/engineering-a-compiler/cooper/978-0-12-088478-0)

### Networking
- [RFC 791 (IP)](https://tools.ietf.org/html/rfc791)
- [RFC 793 (TCP)](https://tools.ietf.org/html/rfc793)
- [TCP/IP Illustrated](https://www.amazon.com/TCP-Illustrated-Vol-Addison-Wesley-Professional/dp/0201633469)
- [smoltcp docs](https://docs.rs/smoltcp/)

### Databases
- [Database Internals](https://www.databass.dev/)
- [SQLite Architecture](https://www.sqlite.org/arch.html)
- [PostgreSQL Docs](https://www.postgresql.org/docs/)

### Distributed Systems
- [Raft Paper](https://raft.github.io/raft.pdf)
- [Designing Data-Intensive Applications](https://dataintensive.net/)
- [Distributed Systems](https://www.distributed-systems.net/)

## 🎯 Quick Links

### Most Important Files
1. [README.md](README.md) - Start here
2. [GETTING_STARTED.md](GETTING_STARTED.md) - Setup guide
3. [ARCHITECTURE.md](ARCHITECTURE.md) - System design
4. [ROADMAP.md](ROADMAP.md) - Development plan
5. `kernel/src/main.rs` - Kernel entry point

### Most Useful Commands
```bash
make build          # Build everything
make run            # Run kernel
make test           # Run tests
make doc            # Generate docs
```

### Most Common Tasks
- **Setup**: `./scripts/setup.sh`
- **Build**: `make build`
- **Test**: `make test`
- **Run**: `make run`
- **Debug**: `make debug-qemu`

## 📊 Project Status

### Phase 1: Complete ✅
- Project structure
- All crates defined
- Build system
- Documentation

### Phase 2: In Progress 🔄
- Kernel implementation
- Memory allocator
- Testing

### Phase 3-12: Planned 📋
- See [ROADMAP.md](ROADMAP.md)

## 🤝 Contributing

### Before Contributing
1. Read [README.md](README.md)
2. Review [ARCHITECTURE.md](ARCHITECTURE.md)
3. Check [ROADMAP.md](ROADMAP.md)
4. Run tests: `make test`

### Contribution Areas
- Kernel implementation
- Language features
- Network protocols
- Database optimization
- Documentation
- Tests

## 📞 Getting Help

### Documentation
1. Check this index
2. Read relevant documentation
3. Review code comments
4. Study examples

### Debugging
1. Use serial output
2. Run with GDB
3. Check test output
4. Review logs

## 🎉 Success Checklist

- [ ] Read README.md
- [ ] Setup environment
- [ ] Build project
- [ ] Run kernel in QEMU
- [ ] Understand architecture
- [ ] Review roadmap
- [ ] Start contributing

---

**Quick Start**: [GETTING_STARTED.md](GETTING_STARTED.md)

**Commands**: [QUICK_REFERENCE.md](QUICK_REFERENCE.md)

**Design**: [ARCHITECTURE.md](ARCHITECTURE.md)

**Plan**: [ROADMAP.md](ROADMAP.md)
