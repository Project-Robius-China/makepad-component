pub mod colors;
pub mod colors_dark;
pub mod radius;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    crate::theme::colors::live_design(cx);
    crate::theme::colors_dark::live_design(cx);
    crate::theme::radius::live_design(cx);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

pub fn apply_theme(cx: &mut Cx, mode: ThemeMode) {
    match mode {
        ThemeMode::Light => {
            cx.link(live_id!(theme), live_id!(theme_desktop_light));
            cx.link(live_id!(theme_colors), live_id!(theme_colors_light));
        }
        ThemeMode::Dark => {
            cx.link(live_id!(theme), live_id!(theme_desktop_dark));
            cx.link(live_id!(theme_colors), live_id!(theme_colors_dark));
        }
    }
    cx.reload_ui_dsl();
}
