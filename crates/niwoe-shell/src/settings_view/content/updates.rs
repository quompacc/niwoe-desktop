fn build_updates_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Systemaktualisierungen",
        "Status der vorhandenen Paketintegration und ihrer lokalen Paketliste.",
        readonly_settings_rows(
            crate::updates::updates_rows(),
            settings_group_inner_width(ctx.content_w),
            Some(niwoe_ui::effect::Symbol::Download),
            ctx.pal,
        ),
    )
}
