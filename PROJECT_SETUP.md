# 🚀 UniPDF Project Structure

## Overview
```
UniPDF/
├── src/
│   └── main.rs              # CLI entry point (Phase 0.1.0)
├── tests/
│   └── integration_test.rs  # End-to-end tests
├── test_files/
│   └── sample.txt           # Test fixtures
├── docs/
│   ├── ROADMAP.md          # Detailed phase plans
│   ├── CONTRIBUTING.md     # Development guide
│   └── ARCHITECTURE.md     # System design
├── Cargo.toml              # Rust project manifest
├── CHANGELOG.md            # Progress tracking
├── README.md               # Main documentation
├── LICENSE-MIT             # MIT license
├── LICENSE-APACHE          # Apache 2.0 license
└── .gitignore              # Git ignore rules
```

## Current Phase: 0.1.0 - Foundation & Text Support

### What's Done ✅
- [x] Project structure initialized
- [x] CLI skeleton with clap
- [x] Documentation (README, ROADMAP, CHANGELOG)
- [x] Dual licensing (MIT/Apache-2.0)
- [x] Test infrastructure
- [x] Sample test files

### What's Next 🔨
- [ ] Implement text file reader
- [ ] Implement PDF generator using printpdf
- [ ] Add pagination logic
- [ ] Embed Liberation Mono font
- [ ] Add proper error handling
- [ ] Write unit tests

## Quick Start

### Build the project
```powershell
cargo build
```

### Run the CLI
```powershell
cargo run -- convert test_files/sample.txt output.pdf
```

### Run tests
```powershell
cargo test
```

### Check code quality
```powershell
cargo clippy
cargo fmt --check
```

## Next Steps

1. **Read the ROADMAP**: See [docs/ROADMAP.md](docs/ROADMAP.md) for detailed phase plans
2. **Check CHANGELOG**: See [CHANGELOG.md](CHANGELOG.md) for current progress
3. **Start coding**: Begin with Phase 0.1.0 deliverables
4. **Write tests**: Add tests as you implement features
5. **Update CHANGELOG**: Mark tasks as complete in CHANGELOG.md

## Phase Progression

Each phase is designed to be:
- **Incremental**: Build on previous work
- **Testable**: Each phase has clear acceptance criteria
- **Documented**: Update CHANGELOG.md as you complete tasks
- **Reviewable**: Commit after each logical unit

## Documentation

- **README.md**: Quick overview and features
- **ROADMAP.md**: Detailed implementation plans for each phase
- **CHANGELOG.md**: Track progress and completed work
- **ARCHITECTURE.md**: System design and technical details
- **CONTRIBUTING.md**: Development workflow and conventions

## License

Dual-licensed under MIT or Apache-2.0 (user's choice).
