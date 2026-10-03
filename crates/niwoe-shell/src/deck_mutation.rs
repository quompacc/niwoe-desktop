//! Bounded background mutations. No persistent worker or idle backend polling.
use std::sync::mpsc::{self, Receiver, TryRecvError};

use crate::{audio::AudioSnapshot, power_profile::PowerProfile};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Status {
    #[default]
    Idle,
    Pending,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AudioChange {
    Volume(u8),
    Mute(bool),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AudioRequest {
    pub device: u32,
    pub change: AudioChange,
}

impl AudioRequest {
    fn confirmed(self, snapshot: &AudioSnapshot) -> bool {
        snapshot.default_output.as_ref().is_some_and(|device| {
            device.id == self.device
                && match self.change {
                    AudioChange::Volume(value) => device.volume_percent == Some(value),
                    AudioChange::Mute(value) => device.muted == value,
                }
        })
    }

    fn run(self) -> (AudioSnapshot, bool) {
        let before = AudioSnapshot::poll();
        if !before
            .default_output
            .as_ref()
            .is_some_and(|d| d.id == self.device)
        {
            return (before, false);
        }
        if !self.confirmed(&before) {
            match self.change {
                AudioChange::Volume(value) => crate::audio::set_default_sink_volume(value),
                AudioChange::Mute(_) => crate::audio::toggle_default_sink_mute(),
            }
        }
        let observed = AudioSnapshot::poll();
        let confirmed = self.confirmed(&observed);
        (observed, confirmed)
    }
}

/// One worker plus at most one replacement volume request. Other requests do
/// not queue while busy; controls communicate that via their pending state.
pub(crate) struct Job<T> {
    receiver: Option<Receiver<(T, bool)>>,
    pub status: Status,
}

impl<T> Default for Job<T> {
    fn default() -> Self {
        Self {
            receiver: None,
            status: Status::Idle,
        }
    }
}

impl<T: Send + 'static> Job<T> {
    fn start(&mut self, work: impl FnOnce() -> (T, bool) + Send + 'static) {
        let (tx, rx) = mpsc::sync_channel(1);
        match std::thread::Builder::new()
            .name("deck-mutation".into())
            .spawn(move || {
                let _ = tx.send(work());
            }) {
            Ok(_) => {
                self.receiver = Some(rx);
                self.status = Status::Pending;
            }
            Err(_) => self.status = Status::Failed,
        }
    }

    pub fn poll(&mut self) -> Option<Option<T>> {
        match self.receiver.as_ref()?.try_recv() {
            Ok((observed, confirmed)) => {
                self.receiver = None;
                self.status = if confirmed {
                    Status::Idle
                } else {
                    Status::Failed
                };
                Some(Some(observed))
            }
            Err(TryRecvError::Disconnected) => {
                self.receiver = None;
                self.status = Status::Failed;
                Some(None)
            }
            Err(TryRecvError::Empty) => None,
        }
    }
}

#[derive(Default)]
pub(crate) struct DeckMutation {
    pub audio: Job<AudioSnapshot>,
    pub power: Job<Option<PowerProfile>>,
    queued_volume: Option<AudioRequest>,
    pub requested_volume: Option<u8>,
}

impl DeckMutation {
    pub fn audio(&mut self, request: AudioRequest) {
        if let AudioChange::Volume(value) = request.change {
            self.requested_volume = Some(value);
        }
        if self.audio.status == Status::Pending {
            if matches!(request.change, AudioChange::Volume(_)) {
                self.queued_volume = Some(request);
            }
        } else {
            self.audio.start(move || request.run());
        }
    }

    pub fn poll_audio(&mut self) -> Option<Option<AudioSnapshot>> {
        let completed = self.audio.poll()?;
        if let Some(request) = self.queued_volume.take() {
            self.audio.start(move || request.run());
        } else {
            self.requested_volume = None;
        }
        Some(completed)
    }

    pub fn power(&mut self, profile: PowerProfile) {
        if self.power.status != Status::Pending {
            self.power.start(move || {
                let accepted = crate::power_profile::set(profile);
                let observed = crate::power_profile::current();
                (observed, accepted && observed == Some(profile))
            });
        }
    }

    pub fn pending(&self) -> bool {
        self.audio.status == Status::Pending || self.power.status == Status::Pending
    }
}

#[cfg(test)]
#[path = "deck_mutation_tests.rs"]
mod tests;
