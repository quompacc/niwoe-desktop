//! Bounded unprivileged xdg-mime calls; used only by the one-shot worker.
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

pub(super) trait Backend {
    fn query(&mut self, mime: &str) -> Result<Option<String>, ()>;
    fn set(&mut self, desktop_id: &str, mimes: &[&str]) -> bool;
}

pub(super) struct XdgMime {
    deadline: Instant,
}

impl XdgMime {
    pub(super) fn new() -> Self {
        Self {
            deadline: Instant::now() + Duration::from_secs(30),
        }
    }

    fn run(&self, args: &[&str]) -> Result<Output, ()> {
        if Instant::now() >= self.deadline {
            return Err(());
        }
        let mut child = Command::new("xdg-mime")
            .env("LC_ALL", "C")
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| ())?;
        let deadline = self.deadline.min(Instant::now() + Duration::from_secs(5));
        loop {
            match child.try_wait() {
                Ok(Some(_)) => return child.wait_with_output().map_err(|_| ()),
                Ok(None) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(20))
                }
                _ => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(());
                }
            }
        }
    }
}

impl Backend for XdgMime {
    fn query(&mut self, mime: &str) -> Result<Option<String>, ()> {
        let output = self.run(&["query", "default", mime])?;
        if !output.status.success() || output.stdout.len() > 1024 {
            return Err(());
        }
        let text = std::str::from_utf8(&output.stdout).map_err(|_| ())?.trim();
        if text.is_empty() {
            return Ok(None);
        }
        if !text.ends_with(".desktop") || text.chars().any(char::is_control) {
            return Err(());
        }
        Ok(Some(text.to_owned()))
    }

    fn set(&mut self, desktop_id: &str, mimes: &[&str]) -> bool {
        let mut args = vec!["default", desktop_id];
        args.extend_from_slice(mimes);
        self.run(&args).is_ok_and(|out| out.status.success())
    }
}
