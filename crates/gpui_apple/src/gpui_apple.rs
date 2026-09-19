//! Shared Apple platform support for GPUI.
//!
//! The architecture moves the Metal renderer and its GPU resource management
//! here, out of `gpui_macos`, and `gpui_macos` re-exports a handful of names
//! from it. On this base that move did not happen: `crates/gpui_macos/src/`
//! still holds `metal_atlas.rs` and `metal_renderer.rs`, and this crate is the
//! shell the manifests expect - the workspace declares it, `gpui_macos` depends
//! on it, and it is what `gpui_macos`'s features forward to.
//!
//! It exists so the workspace resolves and so completing the move is a file
//! move when someone with a macOS host does it; nothing here is ported yet.
//! See `maintenance-report.md` section 21.

#![cfg(target_os = "macos")]
