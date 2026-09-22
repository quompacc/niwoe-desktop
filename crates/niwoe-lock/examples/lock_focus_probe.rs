//! Protocol regression probe. Run ONLY against the isolated smoke compositor.
//! Owns a synthetic lock; never authenticates or connects to the desktop socket.
use std::{thread, time::Duration};
use wayland_client::{
    delegate_noop,
    protocol::{wl_compositor, wl_keyboard, wl_output, wl_registry, wl_seat, wl_surface},
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols::{
    ext::session_lock::v1::client::{
        ext_session_lock_manager_v1 as manager, ext_session_lock_surface_v1 as lock_surface,
        ext_session_lock_v1 as lock,
    },
    xdg::shell::client::{xdg_popup, xdg_positioner, xdg_surface, xdg_toplevel, xdg_wm_base},
};

#[derive(Default)]
struct Probe {
    compositor: Option<wl_compositor::WlCompositor>,
    output: Option<wl_output::WlOutput>,
    manager: Option<manager::ExtSessionLockManagerV1>,
    wm: Option<xdg_wm_base::XdgWmBase>,
    seat: Option<wl_seat::WlSeat>,
    focus: Option<wl_surface::WlSurface>,
    allowed: Option<wl_surface::WlSurface>,
    enforce: bool,
    locked: bool,
    finished: bool,
    popup_done: bool,
    serial: u32,
}

impl Dispatch<wl_registry::WlRegistry, ()> for Probe {
    fn event(
        s: &mut Self,
        r: &wl_registry::WlRegistry,
        e: wl_registry::Event,
        _: &(),
        _: &Connection,
        q: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = e
        {
            match interface.as_str() {
                "wl_compositor" => s.compositor = Some(r.bind(name, version.min(4), q, ())),
                "wl_output" if s.output.is_none() => s.output = Some(r.bind(name, 1, q, ())),
                "wl_seat" => s.seat = Some(r.bind(name, version.min(5), q, ())),
                "xdg_wm_base" => s.wm = Some(r.bind(name, 1, q, ())),
                "ext_session_lock_manager_v1" => s.manager = Some(r.bind(name, 1, q, ())),
                _ => {}
            }
        }
    }
}
impl Dispatch<wl_seat::WlSeat, ()> for Probe {
    fn event(
        _: &mut Self,
        seat: &wl_seat::WlSeat,
        e: wl_seat::Event,
        _: &(),
        _: &Connection,
        q: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities { .. } = e {
            seat.get_keyboard(q, ());
        }
    }
}
impl Dispatch<wl_keyboard::WlKeyboard, ()> for Probe {
    fn event(
        s: &mut Self,
        _: &wl_keyboard::WlKeyboard,
        e: wl_keyboard::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match e {
            wl_keyboard::Event::Enter {
                surface, serial, ..
            } => {
                if s.enforce {
                    assert_eq!(
                        Some(&surface),
                        s.allowed.as_ref(),
                        "background window received keyboard focus while locked"
                    );
                }
                s.focus = Some(surface);
                s.serial = serial;
            }
            wl_keyboard::Event::Leave { .. } => s.focus = None,
            _ => {}
        }
    }
}
impl Dispatch<xdg_wm_base::XdgWmBase, ()> for Probe {
    fn event(
        _: &mut Self,
        wm: &xdg_wm_base::XdgWmBase,
        e: xdg_wm_base::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = e {
            wm.pong(serial);
        }
    }
}
impl Dispatch<xdg_surface::XdgSurface, ()> for Probe {
    fn event(
        _: &mut Self,
        surface: &xdg_surface::XdgSurface,
        e: xdg_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial } = e {
            surface.ack_configure(serial);
        }
    }
}
impl Dispatch<lock::ExtSessionLockV1, ()> for Probe {
    fn event(
        s: &mut Self,
        _: &lock::ExtSessionLockV1,
        e: lock::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match e {
            lock::Event::Locked => s.locked = true,
            lock::Event::Finished => s.finished = true,
            _ => {}
        }
    }
}
impl Dispatch<lock_surface::ExtSessionLockSurfaceV1, ()> for Probe {
    fn event(
        _: &mut Self,
        surface: &lock_surface::ExtSessionLockSurfaceV1,
        e: lock_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let lock_surface::Event::Configure { serial, .. } = e {
            surface.ack_configure(serial);
        }
    }
}
impl Dispatch<xdg_popup::XdgPopup, ()> for Probe {
    fn event(
        s: &mut Self,
        _: &xdg_popup::XdgPopup,
        e: xdg_popup::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let xdg_popup::Event::PopupDone = e {
            s.popup_done = true;
        }
    }
}
delegate_noop!(Probe: ignore wl_compositor::WlCompositor);
delegate_noop!(Probe: ignore wl_output::WlOutput);
delegate_noop!(Probe: ignore wl_surface::WlSurface);
delegate_noop!(Probe: ignore manager::ExtSessionLockManagerV1);
delegate_noop!(Probe: ignore xdg_toplevel::XdgToplevel);
delegate_noop!(Probe: ignore xdg_positioner::XdgPositioner);

fn window(
    s: &Probe,
    q: &QueueHandle<Probe>,
) -> (
    wl_surface::WlSurface,
    xdg_surface::XdgSurface,
    xdg_toplevel::XdgToplevel,
) {
    let surface = s.compositor.as_ref().unwrap().create_surface(q, ());
    let xdg = s.wm.as_ref().unwrap().get_xdg_surface(&surface, q, ());
    let top = xdg.get_toplevel(q, ());
    top.set_title("NIWOE isolated lock regression".into());
    surface.commit();
    (surface, xdg, top)
}

fn lock_cycle() {
    let socket = std::env::var("WAYLAND_DISPLAY").unwrap();
    let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap();
    assert!(
        runtime.starts_with("/tmp/niwoe-p01.") && socket.starts_with(&format!("{runtime}/")),
        "probe requires an isolated smoke socket"
    );
    let c = Connection::connect_to_env().unwrap();
    let mut queue = c.new_event_queue();
    let q = queue.handle();
    c.display().get_registry(&q, ());
    let mut s = Probe::default();
    for _ in 0..3 {
        queue.roundtrip(&mut s).unwrap();
    }
    let first = window(&s, &q);
    queue.roundtrip(&mut s).unwrap();
    assert_eq!(
        s.focus.as_ref(),
        Some(&first.0),
        "unlocked window must receive focus"
    );

    let locker = s.manager.as_ref().unwrap().lock(&q, ());
    // The request after lock is in the same dispatch batch: acquisition must
    // revoke application focus even before the first cleared frame confirms it.
    s.enforce = true;
    let pending_window = window(&s, &q);
    queue.roundtrip(&mut s).unwrap();
    assert!(
        s.focus.is_none(),
        "pending acquisition retained application focus"
    );
    let surface = s.compositor.as_ref().unwrap().create_surface(&q, ());
    let lock_surface = locker.get_lock_surface(&surface, s.output.as_ref().unwrap(), &q, ());
    s.allowed = Some(surface.clone());
    for _ in 0..100 {
        queue.roundtrip(&mut s).unwrap();
        if s.locked && s.focus.as_ref() == Some(&surface) {
            break;
        }
        assert!(!s.finished);
        thread::sleep(Duration::from_millis(10));
    }
    assert!(s.locked);
    assert_eq!(s.focus.as_ref(), Some(&surface));
    let dialog = window(&s, &q);
    queue.roundtrip(&mut s).unwrap();
    assert_eq!(s.focus.as_ref(), Some(&surface), "dialog stole lock focus");
    dialog.2.destroy();
    dialog.1.destroy();
    dialog.0.destroy();
    queue.roundtrip(&mut s).unwrap();
    assert_eq!(
        s.focus.as_ref(),
        Some(&surface),
        "dialog destruction cleared lock focus"
    );
    println!(
        "PASS: pending acquisition, new dialog and dialog destruction preserve lock isolation"
    );

    let popup_surface = s.compositor.as_ref().unwrap().create_surface(&q, ());
    let popup_xdg =
        s.wm.as_ref()
            .unwrap()
            .get_xdg_surface(&popup_surface, &q, ());
    let positioner = s.wm.as_ref().unwrap().create_positioner(&q, ());
    positioner.set_size(10, 10);
    positioner.set_anchor_rect(0, 0, 10, 10);
    let popup = popup_xdg.get_popup(Some(&first.1), &positioner, &q, ());
    popup.grab(s.seat.as_ref().unwrap(), s.serial);
    popup_surface.commit();
    queue.roundtrip(&mut s).unwrap();
    assert!(s.popup_done, "popup grab was not dismissed while locked");
    assert_eq!(s.focus.as_ref(), Some(&surface));
    popup.destroy();
    popup_xdg.destroy();
    popup_surface.destroy();
    positioner.destroy();
    println!("PASS: popup grab cannot override lock focus");

    locker.unlock_and_destroy();
    s.enforce = false;
    queue.roundtrip(&mut s).unwrap();
    assert!(s.focus.is_none(), "unlock must clear the lock focus");
    lock_surface.destroy();
    surface.destroy();
    let after = window(&s, &q);
    queue.roundtrip(&mut s).unwrap();
    assert_eq!(
        s.focus.as_ref(),
        Some(&after.0),
        "normal focus did not recover after unlock"
    );
    // Keep the original windows alive throughout the test to catch restoration
    // to the wrong (background) surface rather than just dangling focus.
    drop((first, pending_window, after));
    println!("PASS: authenticated protocol owner unlock restores normal focus eligibility");

    s.locked = false;
    let _lost_locker = s.manager.as_ref().unwrap().lock(&q, ());
    for _ in 0..100 {
        queue.roundtrip(&mut s).unwrap();
        if s.locked {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    assert!(s.locked);
    // Drop the owning connection without an unlock request. A new application
    // must not regain keyboard access and a replacement lock must be rejected.
}

fn main() {
    lock_cycle();
    let c = Connection::connect_to_env().unwrap();
    let mut queue = c.new_event_queue();
    let q = queue.handle();
    c.display().get_registry(&q, ());
    let mut s = Probe {
        enforce: true,
        ..Probe::default()
    };
    for _ in 0..3 {
        queue.roundtrip(&mut s).unwrap();
    }
    let _background = window(&s, &q);
    let _replacement = s.manager.as_ref().unwrap().lock(&q, ());
    for _ in 0..3 {
        queue.roundtrip(&mut s).unwrap();
    }
    assert!(s.focus.is_none());
    assert!(
        s.finished && !s.locked,
        "locker loss must remain fail-closed"
    );
    println!("PASS: locker disconnect denies background focus and replacement lock");
}
