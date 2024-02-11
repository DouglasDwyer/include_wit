#![deny(warnings)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![cfg_attr(feature = "track_path", feature(track_path))]
#![cfg_attr(feature = "relative_path", feature(proc_macro_span))]

//! The procedural macro implementation for the `include_wit` crate.

use litrs::*;
use proc_macro::*;
use quote::quote;
use std::fs::*;
use std::path::*;
use wit_component::*;
use wit_parser::*;

/// Parses the provided WIT file or directory and produces a `Resolve` which may be queried at runtime.
#[proc_macro]
pub fn include_wit(x: TokenStream) -> TokenStream {
    let input = x.into_iter().map(Into::into).collect::<Vec<TokenTree>>();
    assert!(input.len() == 1, "Wrong number of arguments.");
    let x = StringLit::try_from(&input[0])
        .expect("Could not parse argument as path string.")
        .into_value();

    #[allow(unused)]
    let mut parent_dir_path = None;
    #[cfg(feature = "relative_path")]
    {
        let mut path = Span::call_site().source_file().path();
        path.pop();
        parent_dir_path = Some(path);
    }

    let resolved_path = resolve_path(&x, parent_dir_path).expect("Could not resolve path.");

    #[cfg(feature = "track_path")]
    tracked_path::path(resolved_path.display().to_string());

    let (resolve, package) = parse_wit(&resolved_path);
    let encoded_wasm = encode(Some(true), &resolve, package).expect("Could not encode WIT binary.");
    let byte_literal = proc_macro2::Literal::byte_string(&encoded_wasm);

    quote! {
        {
            use ::include_wit::*;
            static RESOLVE: IncludedResolve = IncludedResolve::new(#byte_literal);
            &RESOLVE
        }
    }
    .into()
}

/// Canonicalizes the path provided by the user.
fn resolve_path(path: &str, parent_dir_path: Option<PathBuf>) -> std::io::Result<PathBuf> {
    let mut path = PathBuf::from(path);
    if let Some(p) = parent_dir_path {
        if !path.is_absolute() {
            path = p.join(path);
        }
    }
    canonicalize(&path)
}

/// Parses the WIT information (either a single file or collection of files) at the provided path.
fn parse_wit(path: &Path) -> (Resolve, PackageId) {
    let mut resolve = Resolve::default();
    let id = if path.is_dir() {
        resolve
            .push_dir(path)
            .expect("Could not parse WIT directory.")
            .0
    } else {
        let contents =
            std::fs::read(path).unwrap_or_else(|_| panic!("Failed to read file {path:?}"));
        let text = match std::str::from_utf8(&contents) {
            Ok(s) => s,
            Err(_) => panic!("input file is not valid utf-8"),
        };
        let pkg = UnresolvedPackage::parse(path, text).expect("Failed to parse package.");
        resolve
            .push(pkg)
            .expect("Failed to add package to resolution.")
    };
    (resolve, id)
}
