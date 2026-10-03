//! Widget abstractions for NIWOE UI.
//!
//! Contract:
//! - `paint` must stay allocation-free and side-effect free (`&self` only).
//! - `children` exposes a prebuilt tree assembled in setup/build phases.
//!   Heap allocation is allowed while building that tree, not while rendering.

pub mod base;
pub mod button;
pub mod component;
pub mod selection_row;
pub mod text_row;
pub mod tile;

pub use base::{Container, Widget};
pub use button::Button;
pub use component::{Component, ComponentKind, ComponentState};
pub use selection_row::{SelectionKind, SelectionRow};
pub use text_row::TextRow;
pub use tile::{Tile, TileSize};
