#![deny(warnings)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! Allows for embedding WIT [`Resolve`]s into application binaries.

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
    wasm: &'static [u8]
}

impl IncludedResolve {
    /// Creates a new resolve which will parse the given WIT package file.
    pub const fn new(wasm: &'static [u8]) -> Self {
        Self {
            inner: OnceBox::new(),
            wasm
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
            }
            else {
                panic!("Incorrect resolve type.")
            }
        })
    }
}