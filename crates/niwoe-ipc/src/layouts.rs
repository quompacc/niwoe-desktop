use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case")]
pub enum LayoutAction {
    Save {
        room_id: u64,
        expected_revision: u64,
    },
    Restore {
        room_id: u64,
        relaunch: bool,
    },
    Cancel {
        room_id: u64,
    },
    Status {
        room_id: u64,
    },
    /// Explicit user binding; never infer documents from window titles.
    SetFile {
        room_id: u64,
        expected_revision: u64,
        key: u32,
        path: Option<String>,
    },
    Prepared {
        run: u64,
        key: u32,
        launch: Option<LayoutLaunch>,
        error: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutLaunch {
    pub desktop_id: String,
    pub program: String,
    pub args: Vec<String>,
    pub terminal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutResult {
    pub key: u32,
    pub label: String,
    pub message: String,
    pub file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "notice", rename_all = "kebab-case")]
pub enum LayoutNotice {
    Status {
        room_id: u64,
        running: bool,
        revision: u64,
        message: String,
        results: Vec<LayoutResult>,
    },
    Prepare {
        run: u64,
        key: u32,
        app: crate::AppReference,
        desktop_id: Option<String>,
        file: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_restore_and_cancel_keep_correlation_on_the_wire() {
        for action in [
            LayoutAction::Save {
                room_id: 42,
                expected_revision: 0,
            },
            LayoutAction::Restore {
                room_id: 42,
                relaunch: false,
            },
            LayoutAction::Restore {
                room_id: 42,
                relaunch: true,
            },
            LayoutAction::Cancel { room_id: 42 },
            LayoutAction::SetFile {
                room_id: 42,
                expected_revision: 1,
                key: 7,
                path: Some("/tmp/literal document.txt".into()),
            },
        ] {
            let command = crate::ShellCommand::Layout {
                request_id: "explicit-1".into(),
                action,
            };
            let encoded = crate::encode_command(&command).unwrap();
            assert_eq!(
                crate::decode_command(std::str::from_utf8(&encoded).unwrap()).unwrap(),
                command
            );
        }
        let event = crate::ShellEvent::Layout {
            request_id: "explicit-1".into(),
            notice: LayoutNotice::Status {
                revision: 1,
                room_id: 42,
                running: false,
                message: "Abgebrochen".into(),
                results: vec![],
            },
        };
        let encoded = crate::encode_event(&event).unwrap();
        assert_eq!(
            crate::decode_event(std::str::from_utf8(&encoded).unwrap()).unwrap(),
            event
        );
    }
}
