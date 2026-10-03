//! Retain client output intent across live layout changes. No frame work or cache.
use smithay::desktop::layer_map_for_output;

use super::{select_layer_output_info, select_layer_recovery_output_info, NiwoeState, OutputInfo};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Assignment {
    Primary,
    Current,
}

impl Assignment {
    pub(super) fn target<'a>(
        self,
        infos: &'a [OutputInfo],
        current: &str,
    ) -> Option<&'a OutputInfo> {
        match self {
            Self::Primary => select_layer_output_info(infos, None),
            Self::Current => select_layer_recovery_output_info(infos, Some(current)),
        }
        .map(|(info, _)| info)
    }
}

impl NiwoeState {
    pub(super) fn reassign_layer_output_targets(&mut self) {
        // Preserve map insertion order within each layer. Neither a surface nor
        // its role/focus identity is recreated when its output changes.
        for source in self.outputs.clone() {
            let source_name = source.name();
            let changes: Vec<_> = layer_map_for_output(&source)
                .layers()
                .filter_map(|layer| {
                    let assignment = layer
                        .user_data()
                        .get::<Assignment>()
                        .copied()
                        .unwrap_or(Assignment::Current);
                    let info = assignment.target(self.output_registry.list(), &source_name)?;
                    if info.name == source_name {
                        return None;
                    }
                    let target = self
                        .outputs
                        .iter()
                        .find(|output| output.name() == info.name)?;
                    Some((layer.clone(), target.clone()))
                })
                .collect();
            for (layer, target) in changes {
                layer_map_for_output(&source).unmap_layer(&layer);
                let result = layer_map_for_output(&target).map_layer(&layer);
                if let Err(error) = result {
                    // Keep the original live layer if a destination rejects it.
                    let restored = layer_map_for_output(&source).map_layer(&layer);
                    tracing::warn!(
                        from = %source_name, to = %target.name(), %error,
                        restored = restored.is_ok(), "layer output reassignment failed"
                    );
                    continue;
                }
                self.mark_output_dirty_by_name(&source_name, "layer-output-reassigned");
                self.mark_output_dirty_by_name(&target.name(), "layer-output-reassigned");
                tracing::debug!(
                    from = %source_name, to = %target.name(),
                    namespace = %layer.namespace(), "layer followed primary output"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{OutputGeometry, OutputId};
    use smithay::utils::Transform;

    fn info(id: u32, name: &str, primary: bool) -> OutputInfo {
        OutputInfo {
            id: OutputId(id),
            name: name.into(),
            geometry: OutputGeometry {
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
            },
            scale: 1.0,
            transform: Transform::Normal,
            refresh_millihz: Some(60_000),
            primary,
        }
    }

    #[test]
    fn primary_changes_move_implicit_layers_and_preserve_explicit_output_intent() {
        for second_primary in [false, true, false] {
            let infos = [
                info(1, "first", !second_primary),
                info(2, "second", second_primary),
            ];
            let primary = if second_primary { "second" } else { "first" };
            for current in ["first", "second"] {
                assert_eq!(
                    Assignment::Primary.target(&infos, current).unwrap().name,
                    primary
                );
                assert_eq!(
                    Assignment::Current.target(&infos, current).unwrap().name,
                    current
                );
            }
        }
    }

    #[test]
    fn missing_output_uses_live_fallback_and_empty_registry_has_no_target() {
        let infos = [info(2, "survivor", true)];
        for assignment in [Assignment::Primary, Assignment::Current] {
            assert_eq!(
                assignment.target(&infos, "removed").unwrap().name,
                "survivor"
            );
            assert!(assignment.target(&[], "removed").is_none());
        }
    }
}
