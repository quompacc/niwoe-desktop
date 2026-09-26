//! Real protocol client for isolated P07 assignment tests, never a desktop app.
// guard:allow-file: synthetic test client with a plain SHM buffer
use std::{fs::File, io::Write, os::fd::AsFd, path::PathBuf, thread, time::Duration};
use wayland_client::{
    delegate_noop,
    protocol::{
        wl_buffer, wl_compositor, wl_keyboard, wl_registry, wl_seat, wl_shm, wl_shm_pool,
        wl_surface,
    },
    Connection, Dispatch, QueueHandle,
};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

#[derive(Default)]
struct Probe {
    compositor: Option<wl_compositor::WlCompositor>,
    wm: Option<xdg_wm_base::XdgWmBase>,
    shm: Option<wl_shm::WlShm>,
    enters: u32,
    focused: bool,
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
                "xdg_wm_base" => s.wm = Some(r.bind(name, 1, q, ())),
                "wl_shm" => s.shm = Some(r.bind(name, 1, q, ())),
                "wl_seat" => {
                    let seat: wl_seat::WlSeat = r.bind(name, version.min(5), q, ());
                    seat.get_keyboard(q, ());
                }
                _ => {}
            }
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
            wl_keyboard::Event::Enter { .. } => {
                s.enters += 1;
                s.focused = true;
            }
            wl_keyboard::Event::Leave { .. } => s.focused = false,
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
delegate_noop!(Probe: ignore wl_compositor::WlCompositor);
delegate_noop!(Probe: ignore wl_surface::WlSurface);
delegate_noop!(Probe: ignore wl_seat::WlSeat);
delegate_noop!(Probe: ignore wl_shm::WlShm);
delegate_noop!(Probe: ignore wl_shm_pool::WlShmPool);
delegate_noop!(Probe: ignore wl_buffer::WlBuffer);
delegate_noop!(Probe: ignore xdg_toplevel::XdgToplevel);

fn main() {
    let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap();
    assert!(
        runtime.starts_with("/tmp/niwoe-p01."),
        "isolated test profile required"
    );
    let args: Vec<_> = std::env::args().collect();
    let directory = PathBuf::from(&args[1]);
    let c = Connection::connect_to_env().unwrap();
    let mut queue = c.new_event_queue();
    let q = queue.handle();
    c.display().get_registry(&q, ());
    let mut s = Probe::default();
    queue.roundtrip(&mut s).unwrap();
    let surface = s.compositor.as_ref().unwrap().create_surface(&q, ());
    let xdg = s.wm.as_ref().unwrap().get_xdg_surface(&surface, &q, ());
    let top = xdg.get_toplevel(&q, ());
    top.set_title(args[2].clone());
    if args[3] != "-" {
        top.set_app_id(args[3].clone());
    }
    surface.commit();
    queue.roundtrip(&mut s).unwrap();
    let file = directory.join("buffer");
    let mut data = File::create(&file).unwrap();
    data.write_all(&vec![0x44; 320 * 180 * 4]).unwrap();
    let data = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(file)
        .unwrap();
    let pool = s
        .shm
        .as_ref()
        .unwrap()
        .create_pool(data.as_fd(), 320 * 180 * 4, &q, ());
    let buffer = pool.create_buffer(0, 320, 180, 320 * 4, wl_shm::Format::Xrgb8888, &q, ());
    surface.attach(Some(&buffer), 0, 0);
    surface.commit();
    let mut previous = String::new();
    let mut dialogs = Vec::new();
    loop {
        let command = std::fs::read_to_string(directory.join("command")).unwrap_or_default();
        if command != previous {
            if let Some(id) = command.strip_prefix("app ") {
                top.set_app_id(id.into());
            }
            if command == "minimize" {
                top.set_minimized();
            }
            if let Some(id) = command.strip_prefix("dialog ") {
                let child = s.compositor.as_ref().unwrap().create_surface(&q, ());
                let xdg = s.wm.as_ref().unwrap().get_xdg_surface(&child, &q, ());
                let dialog = xdg.get_toplevel(&q, ());
                dialog.set_title(format!("{} Dialog", args[2]));
                dialog.set_app_id(id.into());
                dialog.set_parent(Some(&top));
                child.commit();
                queue.roundtrip(&mut s).unwrap();
                child.attach(Some(&buffer), 0, 0);
                child.commit();
                dialogs.push((child, xdg, dialog));
            }
            previous = command;
        }
        queue.roundtrip(&mut s).unwrap();
        std::fs::write(
            directory.join("state.tmp"),
            format!("{{\"enters\":{},\"focused\":{}}}", s.enters, s.focused),
        )
        .unwrap();
        std::fs::rename(directory.join("state.tmp"), directory.join("state.json")).unwrap();
        thread::sleep(Duration::from_millis(30));
    }
}
