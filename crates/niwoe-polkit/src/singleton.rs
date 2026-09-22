//! Keep exactly one agent alive across shell/watchdog restarts.
//! The kernel releases the advisory lock on crash; never unlink its inode.
use std::{
    fs::{File, OpenOptions},
    io,
    os::{fd::AsRawFd, unix::fs::OpenOptionsExt},
    path::Path,
};

pub fn acquire() -> io::Result<Option<File>> {
    let directory = std::env::var_os("XDG_RUNTIME_DIR")
        .ok_or_else(|| io::Error::other("XDG_RUNTIME_DIR is required for the agent lock"))?;
    acquire_at(&Path::new(&directory).join("niwoe-polkit-agent.lock"))
}

fn acquire_at(path: &Path) -> io::Result<Option<File>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(path)?;
    // SAFETY: flock borrows the valid descriptor; File owns its lifetime.
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
        return Ok(Some(file));
    }
    let error = io::Error::last_os_error();
    if error.kind() == io::ErrorKind::WouldBlock {
        Ok(None)
    } else {
        Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_one_owner_and_release_allows_restart() {
        let directory = std::env::temp_dir().join(format!(
            "niwoe-agent-lock-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&directory).unwrap();
        let path = directory.join("agent.lock");
        let first = acquire_at(&path).unwrap().expect("first owner");
        assert!(acquire_at(&path).unwrap().is_none());
        drop(first);
        let next = acquire_at(&path).unwrap().expect("owner after restart");
        assert!(acquire_at(&path).unwrap().is_none());
        drop(next);
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(directory).unwrap();
    }
}
