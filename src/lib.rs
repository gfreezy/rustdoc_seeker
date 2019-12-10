//! # Example
//! ```
//! use std::fs;
//! use rustdoc_seeker::RustDoc;
//! let data = fs::read_to_string("search-index.js").unwrap();
//! let rustdoc: RustDoc = data.parse().unwrap();
//! let seeker = rustdoc.build();
//!
//! for i in seeker.search_regex("dedup.*") {
//!     println!("{}", i);
//! }
//! for i in seeker.search_edist("dedap", 1) {
//!     println!("{}", i);
//! }
//! ```
mod parser;
mod seeker;

pub use seeker::{DocItem, RustDoc, RustDocSeeker, TypeItem};
