fn build_users_content(ctx: &SettingsContentContext<'_>) -> Box<dyn Widget> {
    let row_w = settings_group_inner_width(ctx.content_w);
    let values = match ctx.user_accounts {
        crate::users::UserState::Loading => vec![(
            "Konten werden geladen".into(),
            "Die lokale Kontoliste wird gelesen.".into(),
        )],
        crate::users::UserState::Unavailable => vec![(
            "Kontoliste nicht verfügbar".into(),
            "Die lokalen Konten konnten nicht gelesen werden.".into(),
        )],
        crate::users::UserState::Ready(accounts) => accounts.rows(),
    };
    build_settings_group_page(
        ctx.content_w,
        ctx.content_h,
        "Lokale Konten",
        "Angemeldetes Konto und weitere lokale Benutzer.",
        readonly_settings_rows(values, row_w, Some(niwoe_ui::effect::Symbol::User), ctx.pal),
    )
}
