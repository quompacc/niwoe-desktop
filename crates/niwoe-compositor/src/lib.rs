pub mod backend;
pub mod cursor;
pub mod decoration;
pub mod grabs;
pub mod input;
pub mod protocols;
pub mod state;
pub mod wallpaper;
pub mod workspace;

// Root launcher shares the same user-override compatibility policy.
pub use niwoe_config::environment;
