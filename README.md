# CNA Rust Template

> **Status: In progress - NOT YET FUNCTIONAL**


A multi-platform Rust game template using the CNA framework (XNA 4.0 compatible API).

## Features
- **Desktop**: Windows, Linux, macOS
- **Web**: Browser support (via WebAssembly)
- **Mobile**: Android support (via `cargo-apk`)
- **Adaptive Graphics**: Automatic 3D Cube (HiDef) or 2D Logo (Reach) rendering.
- **Renderer Banner**: In-game banner showing the active renderer.

## Prerequisites
- [Rust](https://www.rust-lang.org/) (latest stable)
- (Web) `wasm-pack` or `cargo-wasm`
- (Android) `cargo-apk`

## Getting Started

### Desktop
Run the game directly:
```bash
cargo run
```

### Web (WebAssembly)
Build for the browser:
```bash
wasm-pack build --target web
```
Then serve the `index.html`.

### Android
Build and run on a connected device:
```bash
cargo apk run
```

## Project Structure
- `src/`: Rust source code.
- `Content/`: Game assets (textures, etc.).
- `Cargo.toml`: Project configuration and dependencies.

## Smoke Test
Run a quick automated test to verify the engine loop:
```bash
cargo run -- --smoke-test
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
