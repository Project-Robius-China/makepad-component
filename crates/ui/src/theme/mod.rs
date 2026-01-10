pub mod colors;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    crate::theme::colors::live_design(cx);
}
