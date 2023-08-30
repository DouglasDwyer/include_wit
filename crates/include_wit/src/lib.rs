#![deny(warnings)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! `include_wit` allows for embedding [`wit_parser::Resolve`] instances into an application binary.
//! It exposes a single macro which parses a WIT file or directory, and generates a WASM binary to include in
//! the source code. This WASM binary is then parsed at runtime upon first access.
//!
//! ## Usage
//!
//! The following is a simple example of how to use `include_wit`. The full example may be found in [the examples folder](/crates/include_wit/examples/).
//!
//! ```rust
//! // Embed the WIT folder into this application
//! let resolve = include_wit::include_wit!("wit");
//!
//! // Print all interfaces in the resolve
//! for x in &resolve.interfaces {
//!     println!("{x:?}");
//! }
//! ```
//!
//! ## Optional features
//!
//! **relative_path** (requires nightly) - Makes all included WIT paths relative to the file where the macro is invoked.
//!
//! **track_path** (requires nightly) - Tracks the included WIT files for changes, causing recompilation automatically when they are edited.

pub use include_wit_macro::include_wit;
use once_cell::race::*;
use std::ops::*;
use wit_component::*;
use wit_parser::*;

/// A [`Resolve`] that has been embedded into a crate.
pub struct IncludedResolve {
    /// The fully-loaded resolve.
    inner: OnceBox<Resolve>,
    /// The raw bytes of the resolve.
    wasm: &'static [u8],
}

impl IncludedResolve {
    /// Creates a new resolve which will parse the given WIT package file.
    pub const fn new(wasm: &'static [u8]) -> Self {
        Self {
            inner: OnceBox::new(),
            wasm,
        }
    }
}

impl Deref for IncludedResolve {
    type Target = Resolve;

    fn deref(&self) -> &Self::Target {
        self.inner.get_or_init(|| {
            let resolve = decode(self.wasm).expect("Could not decode resolve.");
            if let DecodedWasm::WitPackage(resolve, _) = resolve {
                Box::new(resolve)
            } else {
                panic!("Incorrect resolve type.")
            }
        })
    }
}
