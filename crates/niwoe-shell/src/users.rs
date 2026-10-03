//! Read-only local user accounts for the Settings "Benutzer" page.
//!
//! Pure `/etc/passwd` parser (unit-tested) plus a best-effort `gather()`.
//! Only human accounts (UID in the login range) are listed; system and
//! `nobody` accounts are filtered out.

use std::fs;

#[derive(Debug, Clone, Default)]
pub(crate) enum UserState {
    #[default]
    Loading,
    Ready(UserAccounts),
    Unavailable,
}

const UNKNOWN: &str = "—";
const MIN_HUMAN_UID: u32 = 1000;
const MAX_HUMAN_UID: u32 = 60_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalUser {
    pub name: String,
    pub uid: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UserAccounts {
    pub current: String,
    pub accounts: Vec<LocalUser>,
}

impl UserAccounts {
    pub(crate) fn gather() -> UserState {
        let current = std::env::var("USER")
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| UNKNOWN.to_string());
        state_from_source(current, fs::read_to_string("/etc/passwd"))
    }

    /// Label/value rows for the settings view: the current login first, then
    /// one row per local account (the active one is marked).
    pub fn rows(&self) -> Vec<(String, String)> {
        let mut rows = vec![("Angemeldet als".to_string(), self.current.clone())];
        if self.accounts.is_empty() {
            rows.push(("Konten".to_string(), "keine lokalen Benutzer".to_string()));
        } else {
            for user in &self.accounts {
                let status = if user.name == self.current {
                    "Angemeldetes Konto"
                } else {
                    "Lokales Konto"
                };
                rows.push((user.name.clone(), status.to_string()));
            }
        }
        rows
    }
}

fn state_from_source(current: String, source: std::io::Result<String>) -> UserState {
    match source {
        Ok(passwd) => UserState::Ready(UserAccounts {
            current,
            accounts: parse_local_users(&passwd),
        }),
        Err(_) => UserState::Unavailable,
    }
}

fn parse_local_users(passwd: &str) -> Vec<LocalUser> {
    passwd
        .lines()
        .filter_map(|line| {
            let mut fields = line.split(':');
            let name = fields.next()?;
            let _password = fields.next()?;
            let uid: u32 = fields.next()?.parse().ok()?;
            if !name.is_empty() && (MIN_HUMAN_UID..MAX_HUMAN_UID).contains(&uid) {
                Some(LocalUser {
                    name: name.to_string(),
                    uid,
                })
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unreadable_accounts_are_distinct_from_a_successfully_empty_list() {
        assert!(matches!(
            state_from_source(
                "user".into(),
                Err(std::io::ErrorKind::PermissionDenied.into())
            ),
            UserState::Unavailable
        ));
        let UserState::Ready(accounts) = state_from_source("user".into(), Ok(String::new())) else {
            panic!("empty successful source must be ready")
        };
        assert!(accounts.accounts.is_empty());
        assert_eq!(accounts.current, "user");
    }

    #[test]
    fn parse_filters_to_human_uids() {
        let passwd = "root:x:0:0:root:/root:/bin/bash\n\
            daemon:x:1:1::/usr/sbin:/usr/sbin/nologin\n\
            eduard:x:1000:1000:Eduard:/home/eduard:/bin/bash\n\
            nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin\n\
            bob:x:1001:1001::/home/bob:/bin/zsh\n";
        assert_eq!(
            parse_local_users(passwd),
            vec![
                LocalUser {
                    name: "eduard".to_string(),
                    uid: 1000
                },
                LocalUser {
                    name: "bob".to_string(),
                    uid: 1001
                },
            ]
        );
    }

    #[test]
    fn parse_skips_malformed_lines() {
        assert!(parse_local_users("garbage\n").is_empty());
        assert!(parse_local_users("x:x:notanumber:1::/:/bin/sh\n").is_empty());
        assert!(parse_local_users("").is_empty());
    }

    #[test]
    fn rows_put_current_first_and_mark_active() {
        let accounts = UserAccounts {
            current: "eduard".to_string(),
            accounts: vec![
                LocalUser {
                    name: "eduard".to_string(),
                    uid: 1000,
                },
                LocalUser {
                    name: "bob".to_string(),
                    uid: 1001,
                },
            ],
        };
        let rows = accounts.rows();
        assert_eq!(
            rows[0],
            ("Angemeldet als".to_string(), "eduard".to_string())
        );
        assert!(rows
            .iter()
            .any(|(l, v)| l == "eduard" && v == "Angemeldetes Konto"));
        assert!(rows.iter().any(|(l, v)| l == "bob" && v == "Lokales Konto"));
    }

    #[test]
    fn rows_handle_no_accounts() {
        let accounts = UserAccounts {
            current: "x".to_string(),
            accounts: vec![],
        };
        let rows = accounts.rows();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1].1, "keine lokalen Benutzer");
    }
}
