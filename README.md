# include_wit

[![Crates.io](https://img.shields.io/crates/v/include_wit.svg)](https://crates.io/crates/include_wit)
[![Docs.rs](https://docs.rs/include_wit/badge.svg)](https://docs.rs/include_wit)
[![Unsafe Forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

`include_wit` allows for embedding `wit_parser::Resolve` instances into an application binary.
It exposes a single macro which parses a WIT file or directory, and generates a WASM binary to include in
the source code. This WASM binary is then parsed at runtime upon first access.

## Usage

The following is a simple example of how to use `include_wit`. The full example may be found in [the examples folder](/crates/include_wit/examples/).

```rust
// Embed the WIT folder into this application
let resolve = include_wit::include_wit!("wit");

// Print all interfaces in the resolve
for x in &resolve.interfaces {
    println!("{x:?}");
}
```

## Optional features

**relative_path** (requires nightly) - Makes all included WIT paths relative to the file where the macro is invoked.

**track_path** (requires nightly) - Tracks the included WIT files for changes, causing recompilation automatically when they are edited.