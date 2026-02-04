use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use link::theme_colors::*;

    // ============================================================
    // MpPageFlip - Page switching container
    // ============================================================

    // Base PageFlip - simple page container
    pub MpPageFlip = <PageFlip> {
        width: Fill
        height: Fill
    }

    // PageFlip with background
    pub MpPageFlipWithBg = <PageFlip> {
        width: Fill
        height: Fill
        show_bg: true
        draw_bg: {
            color: (BACKGROUND)
        }
    }

    // ============================================================
    // Page container styles - use these inside PageFlip
    // ============================================================

    // Basic page container
    pub MpPage = <View> {
        width: Fill
        height: Fill
        flow: Down
        padding: 16
    }

    // Page with centered content
    pub MpPageCentered = <View> {
        width: Fill
        height: Fill
        flow: Down
        align: { x: 0.5, y: 0.5 }
        padding: 16
    }

    // Page with background
    pub MpPageWithBg = <View> {
        width: Fill
        height: Fill
        flow: Down
        padding: 16
        show_bg: true
        draw_bg: {
            color: (CARD)
        }
    }

    // Page with rounded card style
    pub MpPageCard = <RoundedView> {
        width: Fill
        height: Fill
        flow: Down
        padding: 24
        margin: 16

        draw_bg: {
            color: (CARD)
            border_radius: 8.0
            border_width: 1.0
            border_color: (BORDER)
        }
    }

    // ============================================================
    // Pre-styled color pages for quick demos
    // ============================================================

    // Primary colored page
    pub MpPagePrimary = <View> {
        width: Fill
        height: Fill
        flow: Down
        align: { x: 0.5, y: 0.5 }
        padding: 16
        show_bg: true
        draw_bg: {
            color: (PRIMARY)
        }
    }

    // Secondary colored page
    pub MpPageSecondary = <View> {
        width: Fill
        height: Fill
        flow: Down
        align: { x: 0.5, y: 0.5 }
        padding: 16
        show_bg: true
        draw_bg: {
            color: (SECONDARY)
        }
    }

    // Muted colored page
    pub MpPageMuted = <View> {
        width: Fill
        height: Fill
        flow: Down
        align: { x: 0.5, y: 0.5 }
        padding: 16
        show_bg: true
        draw_bg: {
            color: (MUTED)
        }
    }

    // Accent colored page
    pub MpPageAccent = <View> {
        width: Fill
        height: Fill
        flow: Down
        align: { x: 0.5, y: 0.5 }
        padding: 16
        show_bg: true
        draw_bg: {
            color: (ACCENT)
        }
    }
}

// Re-export PageFlip types from makepad_widgets for convenience
pub use makepad_widgets::page_flip::PageFlipRef;
