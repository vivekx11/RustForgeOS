# RustForge OS Architecture

## System Overview

RustForge OS is a comprehensive operating system written entirely in Rust, featuring:
- Custom x86_64 kernel
- Programming language (ForgeScript) with VM
- Full TCP/IP networking stack
- B-Tree database engine (ForgeDB)
- Distributed consensus (Raft)
- WebAssembly runtime
- Security sandbox

## Component Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Shell                                │
│              (Interactive CLI + REPL)                        │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
┌───────▼────────┐   ┌────────▼────────┐   ┌──────▼──────┐
│   ForgeScript  │   │    ForgeDB      │   │   Network   │
│   Language+VM  │   │   Database      │   │    Stack    │
└───────┬────────┘   └────────┬────────┘   └──────┬──────┘
        │                     │                     │
        │            ┌────────▼────────┐            │
        │            │   Distributed   │            │
        │            │  (Raft + KV)    │            │
        │            └────────┬────────┘            │
        │                     │                     │
┌───────▼─────────────────────▼─────────────────────▼──────┐
│                    Async Runtime                          │
│            (Executor + Thread Pool)                       │
└───────────────────────────────┬───────────────────────────┘
                                │
┌───────────────────────────────▼───────────────────────────┐
│                      Sandbox                              │
│         (Isolation + Capabilities + Syscall Filter)       │
└───────────────────────────────┬───────────────────────────┘
                                │
┌───────────────────────────────▼───────────────────────────┐
│                       Kernel                              │
│    (Memory, Interrupts, Scheduler, Drivers)               │
└───────────────────────────────┬───────────────────────────┘
                                │
┌───────────────────────────────▼───────────────────────────┐
│                    Hardware (x86_64)                      │
└───────────────────────────────────────────────────────────┘
```

## Layer Descriptions

### 1. Hardware Layer
- **Target**: x86_64 architecture
- **Boot**: BIOS/UEFI via bootloader crate
- **Devices**: VGA, Serial, Keyboard, Network card

### 2. Kernel Layer (`kernel/`)
**Responsibilities**:
- Boot sequence and initialization
- Memory management (paging, heap)
- Interrupt handling (IDT, PIC)
- Process scheduling
- Device drivers

**Key Files**:
- `main.rs`: Entry point, initialization
- `gdt.rs`: Global Descriptor Table
- `interrupts.rs`: Interrupt handlers
- `memory.rs`: Page tables, frame allocator
- `scheduler.rs`: Process scheduler
- `vga.rs`: VGA text mode output

**Unsafe Usage**:
- Hardware register access
- Memory-mapped I/O
- Page table manipulation
- Interrupt handler registration

### 3. Memory Allocator (`allocator/`)
**Responsibilities**:
- Heap allocation for kernel and userspace
- Memory block management
- Fragmentation handling

**Implementations**:
1. **Buddy Allocator** (feature: `buddy`)
   - Power-of-2 block sizes
   - O(log n) allocation
   - Automatic coalescing
   
2. **Linked List Allocator** (default)
   - Arbitrary sizes
   - First-fit strategy
   - Simpler implementation

**Interface**: `GlobalAlloc` trait

### 4. Sandbox Layer (`sandbox/`)
**Responsibilities**:
- Process isolation
- Capability-based permissions
- Syscall filtering
- Resource limits

**Security Model**:
```rust
Capability::READ | Capability::WRITE | Capability::EXECUTE
```

**Isolation Mechanisms**:
- Separate address spaces
- Syscall interception
- Resource quotas

### 5. Async Runtime (`runtime/`)
**Responsibilities**:
- Task scheduling
- Async I/O
- Work-stealing thread pool

**Components**:
- `Future`: Custom trait (no std)
- `Executor`: Task runner
- `Waker`: Task notification
- `ThreadPool`: Multi-threaded execution

**Design**:
```
Task Queue → Executor → Worker Threads
                ↓
            Work Stealing
```

### 6. ForgeScript Language (`lang/`)
**Pipeline**:
```
Source Code → Lexer → Parser → AST → Compiler → Bytecode → VM
```

**Components**:
1. **Lexer** (`lexer.rs`)
   - Token recognition with logos
   - Keywords, operators, literals
   
2. **Parser** (`parser.rs`)
   - Recursive descent
   - Produces AST
   
3. **AST** (`ast.rs`)
   - Expression and statement nodes
   - Type annotations
   
4. **Compiler** (`compiler.rs`)
   - AST → Bytecode
   - Optimization passes
   
5. **VM** (`vm.rs`)
   - Stack-based interpreter
   - Bytecode execution

**Type System**:
- `int`: 64-bit signed integer
- `float`: 64-bit floating point
- `string`: UTF-8 string
- `bool`: Boolean
- `array<T>`: Dynamic array

### 7. Networking Stack (`net/`)
**Layer Stack**:
```
Application (HTTP, DNS)
    ↓
Transport (TCP, UDP)
    ↓
Network (IP, ICMP)
    ↓
Link (Ethernet, ARP)
    ↓
Physical (Network Driver)
```

**Components**:
- `ethernet.rs`: Frame parsing
- `arp.rs`: Address resolution
- `ip.rs`: IPv4 packets
- `tcp.rs`: TCP state machine
- `udp.rs`: UDP datagrams
- `http.rs`: HTTP/1.1 server
- `dns.rs`: DNS resolver

**Socket API**:
```rust
let socket = Socket::new(SocketType::Stream);
socket.bind(addr);
socket.listen();
let conn = socket.accept();
```

### 8. ForgeDB Database (`db/`)
**Architecture**:
```
Query → Parser → Planner → Executor
                              ↓
                         Transaction
                              ↓
                    B-Tree + WAL + Buffer Pool
```

**Components**:
1. **B-Tree** (`btree.rs`)
   - Order: 128
   - Insert, search, delete
   - Range queries
   
2. **WAL** (`wal.rs`)
   - Append-only log
   - Crash recovery
   - Checkpoint mechanism
   
3. **Query Parser** (`query.rs`)
   - SQL-like syntax
   - SELECT, INSERT, UPDATE, DELETE
   
4. **Transaction** (`transaction.rs`)
   - ACID properties
   - Serializable isolation
   - Two-phase locking

**Storage Layout**:
```
Page 0: Metadata
Page 1-N: B-Tree nodes
WAL: Append-only log file
```

### 9. Distributed Layer (`distributed/`)
**Raft Consensus**:
```
Leader Election → Log Replication → Commit → Apply
```

**Components**:
- `raft.rs`: Raft state machine
- `kv.rs`: Key-value store
- `node.rs`: Node management

**States**:
- Follower
- Candidate
- Leader

**RPCs**:
- RequestVote
- AppendEntries
- InstallSnapshot

### 10. WASM Runtime (`wasm/`)
**Execution Pipeline**:
```
.wasm Binary → Parser → Validator → Executor
```

**Components**:
- `parser.rs`: Binary format parsing
- `module.rs`: Module representation
- `executor.rs`: Instruction interpreter

**Supported**:
- Core WASM instructions
- Linear memory
- Function calls
- Import/export

### 11. Security Monitor (`monitor/`)
**Responsibilities**:
- Packet capture
- Protocol analysis
- Anomaly detection
- Logging

**Detection Rules**:
- Port scanning
- DDoS patterns
- Protocol violations
- Unusual traffic

### 12. Shell (`shell/`)
**Features**:
- Command execution
- ForgeScript REPL
- Database CLI
- Network utilities
- System information

**Built-in Commands**:
- `help`: Show commands
- `run <file>`: Execute ForgeScript
- `db <query>`: Database query
- `net <cmd>`: Network utilities
- `ps`: Process list
- `exit`: Quit shell

## Data Flow Examples

### Example 1: HTTP Request
```
1. Network card receives packet
2. Interrupt handler → Kernel
3. Ethernet frame parsed
4. IP packet extracted
5. TCP segment processed
6. HTTP request parsed
7. Handler invoked
8. Response generated
9. TCP/IP/Ethernet wrapping
10. Packet transmitted
```

### Example 2: ForgeScript Execution
```
1. User enters code in shell
2. Lexer tokenizes source
3. Parser builds AST
4. Compiler generates bytecode
5. VM executes bytecode
6. Sandbox enforces permissions
7. Result returned to shell
```

### Example 3: Database Transaction
```
1. SQL query received
2. Parser builds query plan
3. Transaction begins
4. Locks acquired
5. B-Tree operations
6. WAL entry written
7. Changes committed
8. Locks released
9. Result returned
```

### Example 4: Distributed Write
```
1. Client writes to leader
2. Leader appends to log
3. AppendEntries RPC to followers
4. Followers append to log
5. Majority acknowledge
6. Leader commits entry
7. State machine applies
8. Client receives confirmation
```

## Memory Layout

### Kernel Address Space
```
0x0000_0000_0000_0000 - 0x0000_7FFF_FFFF_FFFF: User space
0xFFFF_8000_0000_0000 - 0xFFFF_FFFF_FFFF_FFFF: Kernel space
  0xFFFF_8000_0000_0000: Kernel code
  0xFFFF_8000_0010_0000: Kernel data
  0xFFFF_8000_0020_0000: Kernel heap
  0xFFFF_FFFF_8000_0000: Physical memory mapping
```

### Heap Layout
```
Heap Start: 0x_4444_4444_0000
Heap Size:  100 KiB (expandable)
Allocator:  Buddy or Linked List
```

## Concurrency Model

### Kernel
- Spinlocks for short critical sections
- Interrupt disabling for atomicity
- No preemption (cooperative)

### Userspace
- Async/await via custom runtime
- Work-stealing thread pool
- Lock-free data structures (crossbeam)

## Error Handling

### Kernel
- Panic → Serial output → Halt
- No unwinding (panic=abort)

### Userspace
- Result<T, Error> for recoverable errors
- Panic for programming errors
- Error propagation with `?`

## Testing Strategy

### Unit Tests
```bash
cargo test -p <crate>
```

### Integration Tests
```bash
cargo test --workspace
```

### Kernel Tests
```bash
cargo test -p kernel
# Runs in QEMU with custom test framework
```

### Benchmarks
```bash
cargo bench -p <crate>
```

## Performance Considerations

### Kernel
- Zero-cost abstractions
- Inline hot paths
- Cache-friendly data structures

### Allocator
- Buddy: Fast for power-of-2
- Linked list: Slower but flexible

### Networking
- Zero-copy where possible
- Async I/O
- Buffer pooling

### Database
- B-Tree: O(log n) operations
- WAL: Sequential writes
- Buffer pool: LRU eviction

## Security Considerations

### Kernel
- Minimal unsafe code
- Documented invariants
- Fuzzing critical paths

### Sandbox
- Principle of least privilege
- Capability-based access
- Syscall filtering

### Network
- Input validation
- Rate limiting
- DDoS protection

### Database
- SQL injection prevention
- Transaction isolation
- Access control

## Future Enhancements

### Short Term
- Complete all Phase 2-12 implementations
- Comprehensive test coverage
- Performance optimization

### Medium Term
- SMP support
- Filesystem (ext2)
- USB drivers
- Graphics mode

### Long Term
- JIT compilation
- Container runtime
- Package manager
- Self-hosting compiler
