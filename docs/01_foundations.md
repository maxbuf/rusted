## Installation

- [https://rust-lang.org/tools/install](https://rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- `rustup`: Rust version manager
- `cargo`: create projects, manage dependencies

```bash
cargo --version
```

## Creating and running projects

- Create a new project: `cargo new <name>`
- Run a project (after changing into project dir): `cargo run`
- Run without debug output: `cargo run -q`

## Disabling inlay type hints

```json
// settings.json
""rust-analyzer.inlayHints.parameterHints.enable"": false,
""rust-analyzer.inlayHints.typeHints.enable"": false,"
```

## Credits

- [https://www.udemy.com/course/rust-the-complete-developers-guide/](https://www.udemy.com/course/rust-the-complete-developers-guide/)
