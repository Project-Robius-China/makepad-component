pub extern crate makepad_widgets;
pub mod shell;

pub mod theme;
#[cfg(feature = "Accordion")]
#[path ="./accordion/accordion.rs"]
pub mod accordion;

#[cfg(feature = "Alert")]
#[path ="./alert/alert.rs"]
pub mod alert;

#[cfg(feature = "Avatar")]
#[path ="./avatar/avatar.rs"]
pub mod avatar;

#[cfg(feature = "Badge")]
#[path ="./badge/badge.rs"]
pub mod badge;

#[cfg(feature = "Button")]
#[path ="./button/button.rs"]
pub mod button;

#[cfg(feature = "Card")]
#[path ="./card/card.rs"]
pub mod card;

#[cfg(feature = "Checkbox")]
#[path ="./checkbox/checkbox.rs"]
pub mod checkbox;

#[cfg(feature = "Calendar")]
#[path ="./calendar/calendar.rs"]
pub mod calendar;

#[cfg(feature = "ColorPicker")]
#[path ="./color_picker/color_picker.rs"]
pub mod color_picker;
#[cfg(feature = "ColorPicker")]
#[path ="./fold_header_dropdown/fold_header_dropdown.rs"]
pub mod fold_header_dropdown;

#[cfg(feature = "Divider")]
#[path ="./divider/divider.rs"]
pub mod divider;

#[cfg(feature = "Dropdown")]
#[path ="./dropdown/dropdown.rs"]
pub mod dropdown;

#[cfg(feature = "Drawer")]
#[path ="./drawer/drawer.rs"]
pub mod drawer;

#[cfg(feature = "Input")]
#[path ="./input/input.rs"]
pub mod input;

#[cfg(feature = "Label")]
#[path ="./label/label.rs"]
pub mod label;

#[cfg(feature = "Layout")]
#[path ="./layout/layout.rs"]
pub mod layout;

#[cfg(feature = "Link")]
#[path ="./link/link.rs"]
pub mod link;

#[cfg(feature = "List")]
#[path ="./list/list.rs"]
pub mod list;

#[cfg(feature = "Menu")]
#[path ="./menu/menu.rs"]
pub mod menu;

#[cfg(feature = "Modal")]
#[path ="./modal/modal.rs"]
pub mod modal;

#[cfg(feature = "Notification")]
#[path ="./notification/notification.rs"]
pub mod notification;

#[cfg(feature = "PageFlip")]
#[path ="./page_flip/page_flip.rs"]
pub mod page_flip;

#[cfg(feature = "Popover")]
#[path ="./popover/popover.rs"]
pub mod popover;

#[cfg(feature = "Progress")]
#[path ="./progress/progress.rs"]
pub mod progress;

#[cfg(feature = "Radio")]
#[path ="./radio/radio.rs"]
pub mod radio;

#[cfg(feature = "Skeleton")]
#[path ="./skeleton/skeleton.rs"]
pub mod skeleton;

#[cfg(feature = "Slider")]
#[path ="./slider/slider.rs"]
pub mod slider;

#[cfg(feature = "Spinner")]
#[path ="./spinner/spinner.rs"]
pub mod spinner;

#[cfg(feature = "Switch")]
#[path ="./switch/switch.rs"]
pub mod switch;

#[cfg(feature = "Tab")]
#[path ="./tab/tab.rs"]
pub mod tab;

#[cfg(feature = "Text")]
#[path ="./text/text.rs"]
pub mod text;

#[cfg(feature = "Tooltip")]
#[path ="./tooltip/tooltip.rs"]
pub mod tooltip;

#[cfg(feature = "Table")]
#[path ="./table/table.rs"]
pub mod table;

#[cfg(feature = "Space")]
#[path ="./space/space.rs"]
pub mod space;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    crate::theme::live_design(cx);
    #[cfg(feature = "Accordion")]
    accordion::live_design(cx);
    #[cfg(feature = "Alert")]
    alert::live_design(cx);
    #[cfg(feature = "Avatar")]
    avatar::live_design(cx);
    #[cfg(feature = "Badge")]
    badge::live_design(cx);
    #[cfg(feature = "Button")]
    button::live_design(cx);
    #[cfg(feature = "Card")]
    card::live_design(cx);
    #[cfg(feature = "Checkbox")]
    checkbox::live_design(cx);
    #[cfg(feature = "Calendar")]
    calendar::live_design(cx);
    #[cfg(feature = "ColorPicker")]
    fold_header_dropdown::live_design(cx);
    #[cfg(feature = "ColorPicker")]
    color_picker::live_design(cx);
    #[cfg(feature = "Divider")]
    divider::live_design(cx);
    #[cfg(feature = "Dropdown")]
    dropdown::live_design(cx);
    #[cfg(feature = "Drawer")]
    drawer::live_design(cx);
    #[cfg(feature = "Input")]
    input::live_design(cx);
    #[cfg(feature = "Label")]
    label::live_design(cx);
    #[cfg(feature = "Layout")]
    layout::live_design(cx);
    #[cfg(feature = "Link")]
    link::live_design(cx);
    #[cfg(feature = "List")]
    list::live_design(cx);
    #[cfg(feature = "Menu")]
    menu::live_design(cx);
    #[cfg(feature = "Modal")]
    modal::live_design(cx);
    #[cfg(feature = "Notification")]
    notification::live_design(cx);
    #[cfg(feature = "PageFlip")]
    page_flip::live_design(cx);
    #[cfg(feature = "Popover")]
    popover::live_design(cx);
    #[cfg(feature = "Progress")]
    progress::live_design(cx);
    #[cfg(feature = "Radio")]
    radio::live_design(cx);
    #[cfg(feature = "Skeleton")]
    skeleton::live_design(cx);
    #[cfg(feature = "Slider")]
    slider::live_design(cx);
    #[cfg(feature = "Spinner")]
    spinner::live_design(cx);
    #[cfg(feature = "Switch")]
    switch::live_design(cx);
    #[cfg(feature = "Tab")]
    tab::live_design(cx);
    #[cfg(feature = "Text")]
    text::live_design(cx);
    #[cfg(feature = "Tooltip")]
    tooltip::live_design(cx);
    #[cfg(feature = "Space")]
    space::live_design(cx);
    #[cfg(feature = "Table")]
    table::live_design(cx);
}
