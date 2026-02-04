use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use link::theme_colors::*;

    // ============================================================
    // MpList - List container and item components
    // ============================================================

    // List container
    pub MpList = <View> {
        width: Fill
        height: Fit
        flow: Down
    }

    // List with dividers (using RoundedView for proper border support)
    pub MpListDivided = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down

        draw_bg: {
            color: (CARD)
            border_radius: 8.0
            border_color: (BORDER)
        }
    }

    // ============================================================
    // List Items
    // ============================================================

    // Basic list item
    pub MpListItem = <View> {
        width: Fill
        height: Fit
        padding: { left: 16, right: 16, top: 12, bottom: 12 }
        flow: Right
        align: { y: 0.5 }
        spacing: 12
    }

    // List item with hover effect
    pub MpListItemHover = <View> {
        width: Fill
        height: Fit
        padding: { left: 16, right: 16, top: 12, bottom: 12 }
        flow: Right
        align: { y: 0.5 }
        spacing: 12
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance bg_color: #00000000
            instance bg_color_hover: #f8fafc
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                return mix(self.bg_color, self.bg_color_hover, self.hover);
            }
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
        }
    }

    // List item with active state
    pub MpListItemActive = <View> {
        width: Fill
        height: Fit
        padding: { left: 16, right: 16, top: 12, bottom: 12 }
        flow: Right
        align: { y: 0.5 }
        spacing: 12
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance bg_color: #00000000
            instance bg_color_hover: #f8fafc
            instance bg_color_active: #eff6ff
            instance hover: 0.0
            instance active: 0.0

            fn pixel(self) -> vec4 {
                let base = mix(self.bg_color, self.bg_color_active, self.active);
                return mix(base, self.bg_color_hover, self.hover * (1.0 - self.active));
            }
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
            active = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { active: 0.0 } }
                }
                on = {
                    from: { all: Snap }
                    apply: { draw_bg: { active: 1.0 } }
                }
            }
        }
    }

    // ============================================================
    // List Item Components
    // ============================================================

    // List item leading (icon/avatar area)
    pub MpListItemLeading = <View> {
        width: Fit
        height: Fit
        align: { x: 0.5, y: 0.5 }
    }

    // List item content (title + description)
    pub MpListItemContent = <View> {
        width: Fill
        height: Fit
        flow: Down
        spacing: 2
    }

    // List item title
    pub MpListItemTitle = <Label> {
        width: Fill
        height: Fit
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
            color: (FOREGROUND)
        }
    }

    // List item description/subtitle
    pub MpListItemDescription = <Label> {
        width: Fill
        height: Fit
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
            color: (MUTED_FOREGROUND)
        }
    }

    // List item trailing (action area)
    pub MpListItemTrailing = <View> {
        width: Fit
        height: Fit
        align: { x: 0.5, y: 0.5 }
    }

    // ============================================================
    // List Divider
    // ============================================================

    pub MpListDivider = <View> {
        width: Fill
        height: 1
        margin: { left: 16, right: 16 }
        show_bg: true
        draw_bg: {
            color: (BORDER)
        }
    }

    pub MpListDividerFull = <View> {
        width: Fill
        height: 1
        show_bg: true
        draw_bg: {
            color: (BORDER)
        }
    }

    // ============================================================
    // List Section Header
    // ============================================================

    pub MpListSectionHeader = <View> {
        width: Fill
        height: Fit
        padding: { left: 16, right: 16, top: 8, bottom: 8 }

        show_bg: true
        draw_bg: {
            color: (MUTED)
        }

        <Label> {
            width: Fill
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 12.0 }
                color: (MUTED_FOREGROUND)
            }
        }
    }

    // ============================================================
    // Compact List Item
    // ============================================================

    pub MpListItemCompact = <View> {
        width: Fill
        height: Fit
        padding: { left: 12, right: 12, top: 8, bottom: 8 }
        flow: Right
        align: { y: 0.5 }
        spacing: 8
    }

    // ============================================================
    // Large List Item
    // ============================================================

    pub MpListItemLarge = <View> {
        width: Fill
        height: Fit
        padding: { left: 20, right: 20, top: 16, bottom: 16 }
        flow: Right
        align: { y: 0.5 }
        spacing: 16
    }
}
