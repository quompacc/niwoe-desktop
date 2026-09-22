//! Corner-radius scale. The `Radius` token now lives in the shared
//! `niwoe-tokens` crate; re-exported here under the historical
//! `style::Radius` path. The UI `Theme` bundle still defaults to
//! `Radius::METRO` (square); the shell rounds with `Radius::DEFAULT`.

pub use niwoe_tokens::Radius;
