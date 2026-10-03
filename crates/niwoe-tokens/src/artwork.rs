//! Bounded static landscape artwork, separate from compositor glass.
pub struct Artwork;

impl Artwork {
    pub const MAX_WIDTH: u32 = 4096;
    pub const MAX_HEIGHT: u32 = 512;
    pub const CACHE_ENTRIES: usize = 4;
    pub const CACHE_BYTES: usize = 8 * 1024 * 1024;
    /// Shallow headers retain the eclipse/peak rather than the lower forest.
    pub const FOCAL_Y: f32 = 0.2;
}
