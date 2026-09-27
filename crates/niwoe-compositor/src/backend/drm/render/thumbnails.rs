// Capture only the requested toplevel, never the composed output or Hub.
// One thumbnail-sized render per queued request; no frame timer or history.
fn process_thumbnail_requests(
    state: &mut NiwoeState,
    renderer: &mut GlesRenderer,
    _out: &super::DrmOutput,
    _out_size: (u32, u32),
) {
    use smithay::{
        backend::{
            allocator::Fourcc,
            renderer::{
                element::{Element, RenderElement},
                Bind, ExportMem, Frame as _, Offscreen, Renderer,
            },
        },
        utils::{Rectangle, Scale, Size, Transform},
    };
    if state.lock_manager.is_locked_or_pending() {
        state.pending_thumbnail_requests.clear();
        return;
    }
    for req in std::mem::take(&mut state.pending_thumbnail_requests) {
        let Some((_, window)) = state.workspaces.find_element_workspace(|w| {
            crate::state::window_list_entry(w).is_some_and(|(id, _)| id == req.window_id)
        }) else {
            continue;
        };
        let geometry = window.geometry();
        if geometry.size.w <= 0 || geometry.size.h <= 0 {
            continue;
        }
        let factor = (f64::from(req.max_width) / f64::from(geometry.size.w))
            .min(f64::from(req.max_height) / f64::from(geometry.size.h))
            .min(1.0);
        let width = (f64::from(geometry.size.w) * factor).floor().max(1.0) as u32;
        let height = (f64::from(geometry.size.h) * factor).floor().max(1.0) as u32;
        let scale = Scale::from(factor);
        let mut elements = Vec::new();
        render_window_toplevel_elements(
            renderer,
            window,
            (-geometry.loc.x, -geometry.loc.y).into(),
            scale,
            None,
            None,
            &mut elements,
        );
        if elements.is_empty() {
            continue;
        }
        let pixels: Option<Vec<u8>> = (|| {
            let mut texture = <GlesRenderer as Offscreen<GlesTexture>>::create_buffer(
                renderer,
                Fourcc::Xrgb8888,
                Size::from((width as i32, height as i32)),
            )
            .ok()?;
            let mut target = renderer.bind(&mut texture).ok()?;
            let size = Size::from((width as i32, height as i32));
            let mut frame = renderer.render(&mut target, size, Transform::Normal).ok()?;
            // guard:allow: transparent capture target initialization, not UI styling.
            frame
                .clear([0.0, 0.0, 0.0, 0.0].into(), &[Rectangle::from_size(size)])
                .ok()?;
            for element in elements.iter().rev() {
                let dst = element.geometry(scale);
                element
                    .draw(
                        &mut frame,
                        element.src(),
                        dst,
                        &[Rectangle::from_size(dst.size)],
                        &[],
                        None,
                    )
                    .ok()?;
            }
            drop(frame);
            let mapping = renderer
                .copy_framebuffer(
                    &target,
                    Rectangle::from_size(Size::from((width as i32, height as i32))),
                    Fourcc::Xrgb8888,
                )
                .ok()?;
            Some(renderer.map_texture(&mapping).ok()?.to_vec())
        })();
        if let Some(pixels) = pixels {
            if let Some(path) = write_thumbnail(&pixels) {
                state
                    .thumbnail_files
                    .push_back(std::path::PathBuf::from(&path));
                while state.thumbnail_files.len() > 16 {
                    if let Some(old) = state.thumbnail_files.pop_front() {
                        let _ = std::fs::remove_file(old);
                    }
                }
                state
                    .ipc
                    .broadcast(&niwoe_ipc::ShellEvent::WindowThumbnail {
                        request_id: req.request_id,
                        id: req.window_id,
                        path,
                        width,
                        height,
                    });
            }
        }
    }
}

fn write_thumbnail(pixels: &[u8]) -> Option<String> {
    use std::{
        io::Write,
        os::unix::fs::OpenOptionsExt,
        sync::atomic::{AtomicU64, Ordering},
    };
    static SERIAL: AtomicU64 = AtomicU64::new(0);
    let runtime = std::env::var_os("XDG_RUNTIME_DIR")?;
    let path = std::path::PathBuf::from(runtime).join(format!(
        "niwoe-thumb-{}-{}.xrgb",
        std::process::id(),
        SERIAL.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&path)
        .ok()?;
    if file.write_all(pixels).is_err() {
        let _ = std::fs::remove_file(path);
        return None;
    }
    Some(path.to_string_lossy().into_owned())
}
