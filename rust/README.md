# Rust Package

Generated Protocol Buffer bindings for Rust using [quick-protobuf](https://github.com/idanarye/rust-protobuf).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
lupyd-protos = { path = "path/to/rust" }
```

## Build

```bash
cargo build
```

Proto files are compiled automatically via build.rs.

## Usage

```rust
use lupyd_protos::user::{User, FullUser};
use lupyd_protos::auth::UserTokens;
```
