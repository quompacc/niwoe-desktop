//! Colour tokens. The `Color` primitive and the `Palette` now live in the
//! shared `niwoe-tokens` crate so the theme layer and the render layer
//! share one definition (no hand-synced mirror). Re-exported here under the
//! historical `style::Color` / `style::Palette` paths for all UI consumers.

pub use niwoe_tokens::{Color, Palette};
