pub mod color_picker;
pub mod fold_header_dropdown;
use makepad_widgets::*;
pub use color_picker::*;
pub use fold_header_dropdown::*;

pub fn live_design(cx: &mut Cx) {
    color_picker::live_design(cx);
    fold_header_dropdown::live_design(cx);
}