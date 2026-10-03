use super::*;
use crate::audio::{AudioDevice, AudioServiceState};

fn audio(id: u32, volume: u8, muted: bool) -> AudioSnapshot {
    let mut snapshot = AudioSnapshot::unavailable();
    snapshot.service = AudioServiceState::Running;
    snapshot.default_output = Some(AudioDevice {
        id,
        name: "Test sink".into(),
        volume_percent: Some(volume),
        muted,
        is_default: true,
    });
    snapshot
}

#[test]
fn confirmation_requires_target_value_and_same_device() {
    let request = AudioRequest {
        device: 7,
        change: AudioChange::Volume(42),
    };
    assert!(request.confirmed(&audio(7, 42, false)));
    assert!(!request.confirmed(&audio(7, 41, false)));
    assert!(!request.confirmed(&audio(8, 42, false)));
    assert!(!request.confirmed(&AudioSnapshot::unavailable()));
    let request = AudioRequest {
        device: 7,
        change: AudioChange::Mute(true),
    };
    assert!(request.confirmed(&audio(7, 42, true)));
    assert!(!request.confirmed(&audio(7, 42, false)));
}

#[test]
fn job_keeps_pending_until_backend_returns_and_retains_failed_observation() {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut job = Job {
        receiver: Some(rx),
        status: Status::Pending,
    };
    assert_eq!(job.poll(), None);
    assert_eq!(job.status, Status::Pending);
    tx.send((17, false)).unwrap();
    assert_eq!(job.poll(), Some(Some(17)));
    assert_eq!(job.status, Status::Failed);
    assert_eq!(job.poll(), None);
}

#[test]
fn disconnected_worker_is_a_failure_not_permanent_pending() {
    let (tx, rx) = mpsc::sync_channel::<(u8, bool)>(1);
    let mut job = Job {
        receiver: Some(rx),
        status: Status::Pending,
    };
    drop(tx);
    assert_eq!(job.poll(), Some(None));
    assert_eq!(job.status, Status::Failed);
}

#[test]
fn successful_readback_clears_pending() {
    let (tx, rx) = mpsc::sync_channel(1);
    let mut job = Job {
        receiver: Some(rx),
        status: Status::Pending,
    };
    tx.send((PowerProfile::Eco, true)).unwrap();
    assert_eq!(job.poll(), Some(Some(PowerProfile::Eco)));
    assert_eq!(job.status, Status::Idle);
}

#[test]
fn rapid_slider_requests_keep_only_latest_and_mute_does_not_replace_it() {
    let mut mutations = DeckMutation::default();
    mutations.audio.status = Status::Pending;
    for volume in 1..=100 {
        mutations.audio(AudioRequest {
            device: 7,
            change: AudioChange::Volume(volume),
        });
    }
    mutations.audio(AudioRequest {
        device: 7,
        change: AudioChange::Mute(true),
    });
    assert_eq!(
        mutations.queued_volume,
        Some(AudioRequest {
            device: 7,
            change: AudioChange::Volume(100)
        })
    );
    assert_eq!(mutations.audio.status, Status::Pending);
}
