//! Versioned, bounded layout intent. Never contains live window IDs or images.
use crate::rooms::{AppReference, RoomId, RoomLayout};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub mod matching;
pub mod store;
#[cfg(test)]
mod tests;

pub const VERSION: u32 = 1;
pub const MAX_WINDOWS: usize = 128;
pub const MAX_BYTES: u64 = 256 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Geometry {
    /// Logical coordinates; use i64 arithmetic for untrusted/offscreen input.
    pub fn fit(&self, area: &Self) -> Self {
        let width = self.width.clamp(1, area.width.max(1));
        let height = self.height.clamp(1, area.height.max(1));
        let x = (self.x as i64).clamp(
            area.x as i64,
            area.x as i64 + (area.width - width).max(0) as i64,
        );
        let y = (self.y as i64).clamp(
            area.y as i64,
            area.y as i64 + (area.height - height).max(0) as i64,
        );
        Self {
            x: x as i32,
            y: y as i32,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputHint {
    pub name: String,
    pub workarea: Geometry,
    pub scale_millis: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    /// Snapshot-local identity, unrelated to compositor object IDs.
    pub key: u32,
    pub app: AppReference,
    pub desktop_id: Option<String>,
    /// A title is only an additional discriminator, never an executable/path.
    pub title: String,
    pub file: Option<String>,
    pub floating: bool,
    pub geometry: Geometry,
    pub output: Option<OutputHint>,
}

/// Postorder flat tree prevents deserializer recursion on hostile data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Node {
    Window {
        key: u32,
    },
    Split {
        horizontal: bool,
        ratio_millis: u16,
        left: usize,
        right: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub revision: u64,
    pub schema_version: u32,
    pub room_id: RoomId,
    pub mode: RoomLayout,
    pub entries: Vec<Entry>,
    pub tree: Vec<Node>,
}

impl Snapshot {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != VERSION {
            return Err("Nicht unterstützte Layoutversion".into());
        }
        if self.room_id.0 == 0
            || self.entries.len() > MAX_WINDOWS
            || self.tree.len() > MAX_WINDOWS * 2
        {
            return Err("Layoutgrenze überschritten".into());
        }
        let text =
            |s: &str, max| !s.is_empty() && s.len() <= max && !s.chars().any(char::is_control);
        let mut keys = BTreeSet::new();
        for e in &self.entries {
            let app = match &e.app {
                AppReference::Native(s) | AppReference::Xwayland(s) => s,
            };
            if !keys.insert(e.key)
                || !text(app, 255)
                || e.title.len() > 4096
                || e.desktop_id
                    .as_ref()
                    .is_some_and(|s| !text(s, 255) || s.contains('/') || !s.ends_with(".desktop"))
                || e.file
                    .as_ref()
                    .is_some_and(|s| !text(s, 4096) || !std::path::Path::new(s).is_absolute())
                || e.output.as_ref().is_some_and(|o| {
                    !text(&o.name, 255)
                        || o.scale_millis == 0
                        || o.workarea.width <= 0
                        || o.workarea.height <= 0
                })
            {
                return Err("Ungültiger Layouteintrag".into());
            }
        }
        let mut leaves = BTreeSet::new();
        let mut parents = vec![0; self.tree.len()];
        for (i, node) in self.tree.iter().enumerate() {
            match node {
                Node::Window { key }
                    if keys.contains(key)
                        && leaves.insert(*key)
                        && self.entries.iter().any(|e| e.key == *key && !e.floating) => {}
                Node::Split {
                    ratio_millis,
                    left,
                    right,
                    ..
                } if (100..=900).contains(ratio_millis)
                    && *left < i
                    && *right < i
                    && left != right =>
                {
                    parents[*left] += 1;
                    parents[*right] += 1;
                }
                _ => return Err("Ungültiger Layoutbaum".into()),
            }
        }
        if parents
            .iter()
            .enumerate()
            .any(|(i, p)| *p != usize::from(i + 1 < parents.len()))
        {
            return Err("Layoutbaum enthält geteilte oder unverbundene Knoten".into());
        }
        Ok(())
    }
}
