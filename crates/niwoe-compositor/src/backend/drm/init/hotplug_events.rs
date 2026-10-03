// Linux connector changes must wake an idle compositor without a page flip.
fn register_hotplug_event_source(
    event_loop: &mut EventLoop<NiwoeState>,
    seat: &str,
    gpu_path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    use smithay::backend::udev::{UdevBackend, UdevEvent};

    let monitor = UdevBackend::new(seat)?;
    let device_id = monitor
        .device_list()
        .find_map(|(id, path)| (path == gpu_path).then_some(id))
        .ok_or("selected DRM device absent from udev monitor")?;
    event_loop.handle().insert_source(monitor, move |event, _, state| {
        if matches!(event, UdevEvent::Changed { device_id: changed } if changed == device_id) {
            // Do not throttle explicit changes: an unplug directly after a
            // plug must not get lost while the remaining displays are idle.
            scan_drm_connectors_for_h5b(state, "udev", true);
        }
    })?;
    Ok(())
}
