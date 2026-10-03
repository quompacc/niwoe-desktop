use niwoe_ipc::RoomEntry;

#[derive(Default)]
pub(crate) struct ListUi {
    pub query: String,
    /// 0: all, 1: occupied, 2: empty. Active room is independent of occupancy.
    pub filter: usize,
    pub alphabetical: bool,
    pub list_view: bool,
    pub search_focus: bool,
    pub focus: Option<usize>,
}

impl ListUi {
    pub fn visible(
        &self,
        rooms: &[RoomEntry],
        counts: &[u16; niwoe_config::rooms::MAX_ROOMS],
    ) -> Vec<RoomEntry> {
        let query = self.query.to_lowercase();
        let mut result: Vec<_> = rooms
            .iter()
            .filter(|room| {
                let occupied = counts
                    .get(room.workspace.saturating_sub(1) as usize)
                    .is_some_and(|count| *count > 0);
                (self.filter == 0 || (self.filter == 1) == occupied)
                    && (room.name.to_lowercase().contains(&query)
                        || room.description.to_lowercase().contains(&query))
            })
            .cloned()
            .collect();
        if self.alphabetical {
            result.sort_by_key(|r| (r.name.to_lowercase(), r.id));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filter_and_sort_keep_stable_ids_and_use_real_occupancy() {
        let ui = super::super::RoomUi::default();
        let mut counts = [0; niwoe_config::rooms::MAX_ROOMS];
        counts[3] = 2;
        let list = ListUi {
            filter: 1,
            ..Default::default()
        };
        let result = list.visible(&ui.snapshot.rooms, &counts);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 4);
        let list = ListUi {
            query: "raum 4".into(),
            filter: 2,
            ..Default::default()
        };
        assert!(list.visible(&ui.snapshot.rooms, &counts).is_empty());
    }
}
