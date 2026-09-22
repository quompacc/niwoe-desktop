//! Track the authority's unique bus owner. No polling while registered.
use std::{future::poll_fn, time::Duration};

use zbus::export::futures_core::Stream;

use super::{
    unix_session_subject, AgentService, AuthorityProxy, AGENT_OBJECT_PATH, POLKIT_BUS_NAME,
};

pub(super) async fn maintain(
    conn: &zbus::Connection,
    service: &AgentService,
    session_id: &str,
    locale: &str,
    ready: std::sync::mpsc::Sender<Result<(), String>>,
) -> zbus::Result<()> {
    let authority = AuthorityProxy::new(conn).await?;
    // Subscribe before activation/query/registration so a restart in between
    // cannot be missed. Duplicate events for the initial owner are harmless.
    let mut changes = Box::pin(authority.inner().receive_owner_changed().await?);
    let bus = zbus::fdo::DBusProxy::new(conn).await?;
    let initial_owner = match bus.get_name_owner(POLKIT_BUS_NAME.try_into()?).await {
        Ok(owner) => owner,
        Err(zbus::fdo::Error::NameHasNoOwner(_)) => {
            bus.start_service_by_name(POLKIT_BUS_NAME.try_into()?, 0)
                .await?;
            bus.get_name_owner(POLKIT_BUS_NAME.try_into()?).await?
        }
        Err(error) => return Err(error.into()),
    };
    let mut owner = Some(initial_owner.to_string());
    let mut registered = false;
    let mut ready = Some(ready);
    let mut retry_delay = Duration::from_secs(1);

    loop {
        if let Some(name) = owner.as_deref().filter(|_| !registered) {
            // Bind to this exact authority instance. Never accidentally send a
            // retry intended for the previous daemon to a replacement daemon.
            let proxy = AuthorityProxy::builder(conn)
                .destination(name)?
                .build()
                .await?;
            let subject = unix_session_subject(session_id);
            let result = tokio::time::timeout(
                Duration::from_secs(5),
                proxy.register_authentication_agent(&subject, locale, AGENT_OBJECT_PATH),
            )
            .await;
            match result {
                Ok(Ok(())) => {
                    registered = true;
                    retry_delay = Duration::from_secs(1);
                    if let Some(ready) = ready.take() {
                        let _ = ready.send(Ok(()));
                    }
                    tracing::info!(owner = name, session_id, "polkit: agent registered");
                }
                result => tracing::warn!(
                    ?result,
                    owner = name,
                    "polkit: registration failed; retrying"
                ),
            }
        }

        tokio::select! {
            change = poll_fn(|cx| changes.as_mut().poll_next(cx)) => {
                let Some(change) = change else {
                    return Err(zbus::Error::Failure("polkit owner stream ended".into()));
                };
                let next = change.map(|name| name.to_string());
                if next != owner {
                    service.cancel_pending();
                    owner = next;
                    registered = false;
                    retry_delay = Duration::from_secs(1);
                    tracing::info!(?owner, "polkit: authority owner changed");
                }
            }
            // Sleep only after a failed registration, never in steady state
            // or while the service is absent. Bound retry load during errors.
            _ = tokio::time::sleep(retry_delay), if owner.is_some() && !registered => {
                retry_delay = (retry_delay * 2).min(Duration::from_secs(30));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };
    use tokio::sync::{mpsc, oneshot};
    use zbus::zvariant::OwnedValue;

    struct TestAuthority(mpsc::UnboundedSender<()>);

    #[zbus::interface(name = "org.freedesktop.PolicyKit1.Authority")]
    impl TestAuthority {
        fn register_authentication_agent(
            &self,
            subject: (String, HashMap<String, OwnedValue>),
            locale: String,
            object_path: String,
        ) -> zbus::fdo::Result<()> {
            assert_eq!(subject.0, "unix-session");
            assert_eq!(locale, "C");
            assert_eq!(object_path, AGENT_OBJECT_PATH);
            self.0.send(()).unwrap();
            Ok(())
        }
    }

    async fn authority(tx: mpsc::UnboundedSender<()>) -> zbus::Connection {
        zbus::connection::Builder::session()
            .unwrap()
            .serve_at("/org/freedesktop/PolicyKit1/Authority", TestAuthority(tx))
            .unwrap()
            .name(POLKIT_BUS_NAME)
            .unwrap()
            .build()
            .await
            .unwrap()
    }

    #[tokio::test]
    #[ignore = "run with dbus-run-session: isolated mock authority, never system bus"]
    async fn authority_restart_reregisters_and_cancels_pending_auth() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        let (calls_tx, mut calls) = mpsc::unbounded_channel();
        let first = authority(calls_tx.clone()).await;
        let conn = zbus::Connection::session().await.unwrap();
        let (tx, events) = super::super::cchannel::channel();
        let service = AgentService {
            tx,
            pending: Arc::new(Mutex::new(HashMap::new())),
        };
        let observer = service.clone();
        let (ready, ready_rx) = std::sync::mpsc::channel();
        let task = tokio::spawn(async move {
            let result = maintain(&conn, &service, "test-session", "C", ready).await;
            panic!("registration loop unexpectedly ended: {result:?}");
        });
        tokio::time::timeout(Duration::from_secs(5), calls.recv())
            .await
            .unwrap()
            .unwrap();
        // Allow the registration response to reach the agent before restarting.
        tokio::time::timeout(Duration::from_secs(5), async {
            while ready_rx.try_recv().is_err() {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await
        .unwrap();
        let (cancel, cancelled) = oneshot::channel();
        observer
            .pending
            .lock()
            .unwrap()
            .insert("stale-cookie".into(), cancel);
        first.release_name(POLKIT_BUS_NAME).await.unwrap();
        tokio::time::timeout(Duration::from_secs(5), cancelled)
            .await
            .unwrap()
            .unwrap();
        assert!(observer.pending.lock().unwrap().is_empty());
        assert!(
            matches!(events.try_recv().unwrap(), super::super::DbusEvent::Cancel { cookie } if cookie == "stale-cookie")
        );
        let second = authority(calls_tx.clone()).await;
        tokio::time::timeout(Duration::from_secs(5), calls.recv())
            .await
            .unwrap()
            .unwrap();
        second.release_name(POLKIT_BUS_NAME).await.unwrap();
        let _third = authority(calls_tx).await;
        tokio::time::timeout(Duration::from_secs(5), calls.recv())
            .await
            .unwrap()
            .unwrap();
        // A registered agent must not repeatedly re-register in the idle state.
        assert!(
            tokio::time::timeout(Duration::from_millis(1200), calls.recv())
                .await
                .is_err()
        );
        task.abort();
        let _ = task.await;
    }
}
