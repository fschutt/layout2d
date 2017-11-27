//! Library for UI development
//!
//! This library features a `UIScreen<T>` where T is your data that you want to
//! store in the UI. You can for example, have a function callback in the UI.
//!
//! This library doesn't do any drawing, only layout.

#[cfg(feature = "use_simd")]
pub extern crate simd;

// Forked from https://github.com/SimonSapin/rust-forest/blob/master/rctree/lib.rs
// because the crate is not available anymore on crates.io. MIT License Simon Sapin
pub mod rctree;
pub mod rect;
pub mod ui_screen;
pub mod node_data;

pub use rctree::NodeRef as NodeRef;
pub use rect::Rect;
pub use ui_screen::UiScreen;
pub use node_data::{NodeData, FlexDirection};

