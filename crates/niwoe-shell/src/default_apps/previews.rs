use super::*;

impl MimeAppIndex {
    /// One bounded batch on the existing Settings worker, never during paint.
    /// The snapshot owns at most 9 × (24 candidates + 1 current handler) icons.
    pub(crate) fn prepare_previews(
        &mut self,
        config: &(String, String),
        current: &HashMap<DefaultAppCategory, String>,
    ) {
        let mut names = std::collections::BTreeSet::new();
        for category in DefaultAppCategory::ALL {
            for app in self
                .apps_for_mime(category.representative_mime())
                .into_iter()
                .take(MAX_APPS_PER_CATEGORY)
                .chain(current.get(category).and_then(|id| self.lookup(id)))
            {
                if let Some(name) = app.icon.as_ref() {
                    names.insert(name.clone());
                }
            }
        }
        let side = niwoe_tokens::Controls::SYMBOL_SIZE;
        let batch = crate::icons::IconCache::load_batch(
            &config.0,
            &config.1,
            &names.into_iter().collect::<Vec<_>>(),
            &[side],
        );
        self.previews = batch
            .into_iter()
            .filter_map(|(name, _, image)| {
                let pixmap = crate::icons::icon_image_to_pixmap(&image?)?;
                Some((name, std::sync::Arc::new(pixmap)))
            })
            .collect();
    }
}
