//! Read-only list status. An unavailable query never becomes a confirmed empty list.
use super::{ConnectionProfile, WifiNetwork};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ListStatus {
    pub profiles_available: bool,
    pub wifi_available: bool,
}

pub(crate) struct NetworkLists {
    pub profiles: Vec<ConnectionProfile>,
    pub wifi: Vec<WifiNetwork>,
    pub status: ListStatus,
}

impl NetworkLists {
    pub(crate) fn poll() -> Self {
        #[cfg(target_os = "linux")]
        {
            let profiles = super::nmcli::run_nmcli(&super::nmcli::CONNECTION_LIST_ARGS);
            let wifi = super::nmcli::run_nmcli(&super::nmcli::WIFI_SCAN_ARGS);
            Self::from_reads(profiles.as_deref(), wifi.as_deref())
        }
        #[cfg(not(target_os = "linux"))]
        {
            // The existing ifconfig backend has a best-effort Vec boundary.
            // Retain its entries without claiming that an empty result proves absence.
            Self {
                profiles: super::list_saved_connections(),
                wifi: super::scan_wifi_networks(),
                status: ListStatus::default(),
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn from_reads(profiles: Option<&str>, wifi: Option<&str>) -> Self {
        let profiles = profiles.filter(|text| valid_rows(text, false));
        let wifi = wifi.filter(|text| valid_rows(text, true));
        Self {
            profiles: profiles
                .map(super::nmcli::parse_saved_connections)
                .unwrap_or_default(),
            wifi: wifi
                .map(super::nmcli::parse_wifi_networks)
                .unwrap_or_default(),
            status: ListStatus {
                profiles_available: profiles.is_some(),
                wifi_available: wifi.is_some(),
            },
        }
    }
}

#[cfg(target_os = "linux")]
fn valid_rows(text: &str, wifi: bool) -> bool {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .all(|line| {
            let fields = super::nmcli::parse_terse_fields(line.trim());
            if wifi {
                fields.len() >= 4
                    && matches!(fields[0].as_str(), "" | "*")
                    && fields[1].parse::<u8>().is_ok_and(|signal| signal <= 100)
            } else {
                fields.len() == 3 && !fields[0].is_empty() && !fields[1].is_empty()
            }
        })
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn failed_empty_and_partial_queries_remain_distinct() {
        let empty = NetworkLists::from_reads(Some("\n"), Some(""));
        assert!(empty.status.profiles_available && empty.status.wifi_available);
        assert!(empty.profiles.is_empty() && empty.wifi.is_empty());
        let unavailable = NetworkLists::from_reads(None, None);
        assert!(!unavailable.status.profiles_available && !unavailable.status.wifi_available);
        let partial = NetworkLists::from_reads(Some("Home:802-3-ethernet:eth0\n"), None);
        assert!(partial.status.profiles_available && !partial.status.wifi_available);
        assert_eq!(partial.profiles[0].name, "Home");
    }

    #[test]
    fn malformed_success_is_unknown_and_escaped_names_keep_their_identity() {
        for text in ["garbage", "Home:ethernet", "Home:ethernet:eth0\nbad"] {
            assert!(
                !NetworkLists::from_reads(Some(text), Some(""))
                    .status
                    .profiles_available
            );
        }
        for text in [
            "garbage",
            ":bad:WPA2:Home",
            ":101:WPA2:Home",
            "x:70:WPA2:Home",
        ] {
            assert!(
                !NetworkLists::from_reads(Some(""), Some(text))
                    .status
                    .wifi_available
            );
        }
        let lists = NetworkLists::from_reads(
            Some("Home\\:Office:802-3-ethernet:eth0\n"),
            Some("*:87:WPA2:Home\\:Office\n:70:--:\n"),
        );
        assert!(lists.status.profiles_available && lists.status.wifi_available);
        assert_eq!(lists.profiles[0].name, "Home:Office");
        assert_eq!(lists.wifi.len(), 1);
        assert_eq!(lists.wifi[0].ssid, "Home:Office");
    }
}
