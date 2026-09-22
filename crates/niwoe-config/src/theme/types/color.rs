//! The `Color` primitive (hex parse/serialize + helpers) now lives in the
//! shared `niwoe-tokens` crate so the theme layer and the render layer
//! share one definition. Re-exported here so existing `theme::Color` /
//! `niwoe_config::Color` paths keep resolving.

pub use niwoe_tokens::Color;
