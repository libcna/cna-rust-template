# CNA-Rust template

This repository is the functional 2D demonstration and starter project for
CNA-Rust. It runs its lifecycle and drawing through CNA C++; it does not use a
fake Rust frame loop.

The demo:

- initializes and shuts down a native CNA game;
- composes the durable per-game state and verifies retained, instance-local
  `GameServiceContainer` identity;
- receives XNA-shaped `GameTime` update/draw callbacks;
- obtains the native game graphics device;
- decodes `Content/logo.png` into a real native `Texture2D`;
- clears and draws through a real `SpriteBatch`;
- applies XNA's real `BlendState::AlphaBlend` through CNA's state-bearing
  `SpriteBatch::Begin` route;
- captures native keyboard, mouse, and player-one gamepad snapshots (`Escape`
  exits where input is available);
- moves and bounds the logo using the current viewport;
- prints renderer name/capabilities queried from CNA; and
- explicitly disposes resources, with `Drop` safely following.

There is no cube, fake `supports_3d`, fabricated renderer name, fake texture, or
XNB claim. `Texture2D::FromStream` is the raw encoded-image route; XNB
`ContentManager` support is future work.

## Current platform evidence

| Platform | Status |
|---|---|
| Linux x86-64 / CNA HEADLESS | Experimental runtime verified: 60 and 600 frames |
| Linux windowed/GPU | Planned |
| Windows | Planned |
| macOS | Planned |
| WebAssembly | Unsupported: no CNA WASM C ABI verified |
| Android | Unsupported: no native CNA app integration verified |

The 2026-08-23 headless test used CNA ABI 0.7 and required two temporary
build-only corrections outside the CNA checkout because CNA C++ added NanoVG
before the ABI 0.7 renderer table. See the CNA-Rust `NEXT.md` for exact evidence.

## Build and run

This development checkout intentionally uses an exact sibling path dependency:

```toml
cna = { package = "cna-rust", path = "../cna-rust/crates/cna" }
```

Neither crate is published yet. Supply an ABI 0.7 native library:

```bash
CNA_NATIVE_LIBRARY=/absolute/path/to/libcna_c_api.so cargo run
```

The current CNA-Rust loader supports Unix. A missing or mismatched library is a
clear error; the demo never falls back to fake behavior.

## Deterministic native tests

```bash
CNA_NATIVE_LIBRARY=/absolute/path/to/libcna_c_api.so \
  cargo run -- --smoke-test       # 60 successful draw frames

CNA_NATIVE_LIBRARY=/absolute/path/to/libcna_c_api.so \
  cargo run -- --stability-test   # 600 successful draw frames

CNA_NATIVE_LIBRARY=/absolute/path/to/libcna_c_api.so \
  cargo run -- --frames 120
```

A success message means CNA initialized, content loaded, `Update` and `Draw`
completed for the requested count, child resources were disposed, CNA shut
down, and the process exited zero.

## Generate a standalone project

The checked-in canary stays directly buildable. The generator parameterizes the
project directory, crate/package metadata, and binary name, and vendors a
supplied CNA-Rust checkout so the result has no developer absolute paths or
sibling-repository dependencies:

```bash
python3 tools/generate.py \
  --output /tmp \
  --project-name asteroid-demo \
  --crate-name asteroid-game \
  --binary-name asteroids \
  --author "Example Developer" \
  --description "A native CNA asteroid demo" \
  --cna-root ../cna-rust
```

The destination must not already exist; the generator refuses to overwrite it.
The vendored binding remains pinned to CNA ABI 0.7 and Rust 1.74.

## Requirements

- Rust 1.74 or newer;
- a CNA C API library matching experimental ABI 0.7;
- the native dependencies of that CNA build; and
- a display only when the selected CNA platform/renderer requires one.

## License

This template is licensed under the [MIT License](LICENSE).
