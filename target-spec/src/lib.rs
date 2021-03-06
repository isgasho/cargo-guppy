// Copyright (c) The cargo-guppy Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Evaluate `Cargo.toml` target specifications against platform triples.
//!
//! Cargo supports
//! [platform-specific dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies).
//! These dependencies can be specified in one of two ways:
//!
//! ```toml
//! # 1. As Rust-like `#[cfg]` syntax.
//! [target.'cfg(all(unix, target_arch = "x86_64"))'.dependencies]
//! native = { path = "native/x86_64" }
//!
//! # 2. Listing out the full target triple.
//! [target.x86_64-pc-windows-gnu.dependencies]
//! winhttp = "0.4.0"
//! ```
//!
//! `target-spec` provides the `eval` API which can be used to figure out whether such a
//! dependency will be included on a particular platform.
//!
//! ```rust
//! use target_spec::eval;
//!
//! // Evaluate Rust-like `#[cfg]` syntax.
//! let cfg_target = "cfg(all(unix, target_arch = \"x86_64\"))";
//! assert_eq!(eval(cfg_target, "x86_64-unknown-linux-gnu"), Ok(Some(true)));
//! assert_eq!(eval(cfg_target, "i686-unknown-linux-gnu"), Ok(Some(false)));
//! assert_eq!(eval(cfg_target, "x86_64-pc-windows-msvc"), Ok(Some(false)));
//!
//! // Evaluate a full target-triple.
//! assert_eq!(eval("x86_64-unknown-linux-gnu", "x86_64-unknown-linux-gnu"), Ok(Some(true)));
//! assert_eq!(eval("x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc"), Ok(Some(false)));
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]

mod evaluator;
mod parser;
mod platform;
#[cfg(feature = "proptest09")]
mod proptest;

pub use evaluator::*;
pub use parser::*;
pub use platform::*;
