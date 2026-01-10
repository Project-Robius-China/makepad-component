pub mod button;
pub mod checkbox;
pub mod switch;
pub mod divider;
pub mod radio;
pub mod progress;
pub mod slider;

pub use button::*;
pub use checkbox::*;
pub use switch::*;
pub use divider::*;
pub use radio::*;
pub use progress::*;
pub use slider::*;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    crate::widgets::button::live_design(cx);
    crate::widgets::checkbox::live_design(cx);
    crate::widgets::switch::live_design(cx);
    crate::widgets::divider::live_design(cx);
    crate::widgets::radio::live_design(cx);
    crate::widgets::progress::live_design(cx);
    crate::widgets::slider::live_design(cx);
}
