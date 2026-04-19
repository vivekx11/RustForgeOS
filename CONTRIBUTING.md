# Contributing to RustForge OS

Thank you for your interest in contributing to RustForge OS! 🎉

## Getting Started

1. **Fork the repository**
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/RustForgeOS.git
   cd RustForgeOS
   ```
3. **Setup environment**:
   ```bash
   ./scripts/setup.sh
   ```
4. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Before You Start
- Read [ARCHITECTURE.md](ARCHITECTURE.md) to understand the system design
- Check [ROADMAP.md](ROADMAP.md) to see what's planned
- Look at existing issues or create a new one

### Making Changes

1. **Write Code**:
   - Follow Rust style guide
   - Add tests for new features
   - Document public APIs
   - Explain unsafe code

2. **Test Your Changes**:
   ```bash
   # Format code
   cargo fmt --all
   
   # Run linter
   cargo clippy --workspace -- -D warnings
   
   # Run tests
   cargo test --workspace
   
   # Build everything
   cargo build --workspace --release
   ```

3. **Commit Your Changes**:
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```
   
   Use conventional commits:
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation changes
   - `test:` - Test additions/changes
   - `refactor:` - Code refactoring
   - `perf:` - Performance improvements

4. **Push and Create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a Pull Request on GitHub.

## Code Guidelines

### Rust Style
- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Keep functions small and focused
- Prefer explicit over implicit

### Documentation
- Document all public APIs
- Add examples where helpful
- Explain complex algorithms
- Document safety requirements for unsafe code

### Testing
- Write unit tests for new functions
- Add integration tests for features
- Test edge cases
- Aim for >80% code coverage

### Unsafe Code
- Minimize unsafe usage
- Document safety invariants
- Explain why unsafe is necessary
- Add safety comments

## Areas to Contribute

### High Priority
- [ ] Complete kernel implementation (Phase 2)
- [ ] Buddy allocator optimization
- [ ] ForgeScript standard library
- [ ] TCP state machine
- [ ] B-Tree implementation

### Medium Priority
- [ ] HTTP server
- [ ] DNS resolver
- [ ] Raft consensus
- [ ] WASM executor
- [ ] Security monitor

### Documentation
- [ ] API documentation
- [ ] Tutorial articles
- [ ] Example programs
- [ ] Architecture diagrams

### Testing
- [ ] Unit tests
- [ ] Integration tests
- [ ] Benchmarks
- [ ] Fuzzing

## Pull Request Process

1. **Update Documentation**: If you change APIs, update docs
2. **Add Tests**: New features need tests
3. **Pass CI**: All checks must pass
4. **Get Review**: Wait for maintainer review
5. **Address Feedback**: Make requested changes
6. **Merge**: Maintainer will merge when ready

## Code Review

We review PRs for:
- **Correctness**: Does it work?
- **Style**: Does it follow guidelines?
- **Tests**: Are there adequate tests?
- **Documentation**: Is it documented?
- **Performance**: Is it efficient?

## Questions?

- Open an issue for questions
- Check existing documentation
- Review similar code in the project

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to RustForge OS!** 🚀
