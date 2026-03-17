# Contributing to Protlin™

**Official Contributor Guidelines** | **Version 1.0.0** | **Copyright © 2026 Moude AI LLC and Moude Corp**

---

## Legal Notice

⚠️ **IMPORTANT**: By contributing to Protlin, you agree to the Contributor License Agreement (CLA) outlined in this document. All contributions become the exclusive property of Moude AI LLC and Moude Corp and are subject to the proprietary license.

---

## Contributor License Agreement (CLA)

By submitting any contribution to this project, you hereby:

1. **Grant Exclusive Rights**: Transfer all rights, title, and interest in your contribution to Moude AI LLC and Moude Corp
2. **Certify Authorship**: Confirm that you are the original author or have permission to submit the contribution
3. **Waive Claims**: Waive any future claims to ownership, royalties, or attribution for contributed code
4. **Accept Terms**: Agree that contributions are subject to the proprietary license

---

## Code of Conduct

### Professional Standards

- Maintain professional and respectful communication
- Provide constructive, technical feedback
- Focus on code quality and language design
- Respect intellectual property and licensing terms

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in Issues
2. Create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Protlin version and OS
   - Code sample if applicable

### Suggesting Features

1. Check existing issues and discussions
2. Create a new issue with:
   - Clear description of the feature
   - Use cases and benefits
   - Possible implementation approach

### Pull Requests

1. Fork the repository
2. Create a new branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Test your changes: `cargo test && cargo build`
5. Commit with clear messages: `git commit -m "Add feature: description"`
6. Push to your fork: `git push origin feature/your-feature-name`
7. Open a Pull Request

## Development Setup

### Prerequisites
- Rust (latest stable)
- Cargo
- Git

### Building
```bash
git clone https://github.com/Arthurc1Moude/Protlin.git
cd Protlin
cargo build
```

### Running Tests
```bash
cargo test
```

### Running Examples
```bash
cargo run examples/hello.prot
cargo run examples/graphics_demo.prot
```

## Project Structure

```
Protlin/
├── src/
│   ├── main.rs          # Entry point
│   ├── lexer.rs         # Tokenization (66K lines)
│   ├── parser.rs        # AST generation (307K lines)
│   ├── ast.rs           # AST definitions (40K lines)
│   ├── interpreter.rs   # Execution engine (186K lines)
│   ├── builtins.rs      # Built-in functions (318K lines)
│   ├── environment.rs   # Variable scope (76K lines)
│   ├── types.rs         # Type system (4K lines)
│   ├── error.rs         # Error handling (2K lines)
│   └── graphics.rs      # Graphics system (23K lines)
├── examples/            # Demo programs
├── docs/                # Documentation
└── Cargo.toml          # Dependencies
```

## Coding Guidelines

### Rust Code Style
- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Add comments for complex logic
- Write descriptive variable names
- Keep functions focused and small

### Protlin Language Style
- Use clear, descriptive names
- Add comments for complex examples
- Follow existing example patterns
- Test your .prot files before submitting

## Areas for Contribution

### High Priority
- [ ] Complete import/export system with code execution
- [ ] Native library loading
- [ ] Web URL downloading for imports
- [ ] Standard library functions
- [ ] Error messages improvement

### Medium Priority
- [ ] REPL (Read-Eval-Print Loop)
- [ ] Debugger
- [ ] More example programs
- [ ] Performance optimizations
- [ ] Documentation improvements

### Low Priority
- [ ] Language Server Protocol (LSP)
- [ ] VS Code extension
- [ ] Syntax highlighting for other editors
- [ ] Package manager
- [ ] Online playground

## Testing

### Manual Testing
```bash
# Test basic functionality
cargo run examples/hello.prot

# Test graphics
cargo run examples/graphics_demo.prot

# Test theme system
cargo run examples/theme_demo.prot
```

### Adding Tests
- Add unit tests in the same file as the code
- Add integration tests in `tests/` directory
- Test edge cases and error conditions

## Documentation

### Code Documentation
- Add doc comments for public functions: `///`
- Explain complex algorithms
- Include examples in doc comments

### User Documentation
- Update README.md for major features
- Add examples for new features
- Update GETTING_STARTED.md for beginner features

## Commit Messages

Format:
```
<type>: <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

Examples:
```
feat: Add import from web URLs

Implemented HTTP/HTTPS import functionality for loading
external Protlin libraries from web URLs.

Closes #123
```

```
fix: Correct theme detection on Linux

Fixed OS theme detection to properly read gsettings
when dark-light library returns Default mode.

Fixes #456
```

## Review Process

1. Maintainer reviews PR within 1-2 weeks
2. Address feedback and requested changes
3. Once approved, PR will be merged
4. Your contribution will be credited in release notes

## Getting Help

- Open an issue for questions
- Join discussions in GitHub Discussions
- Check existing documentation
- Ask in pull request comments

## Recognition

Contributors will be:
- Listed in release notes
- Credited in README.md (for significant contributions)
- Thanked in commit messages

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Protlin! 🚀
