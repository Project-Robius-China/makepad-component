use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use link::theme_colors::*;

    // ============================================================
    // MpAccordion - Collapsible panel component
    // ============================================================

    // Accordion item header
    pub MpAccordionHeaderBase = {{MpAccordionHeader}} {
        width: Fill
        height: Fit
        padding: { left: 16, right: 16, top: 12, bottom: 12 }
        flow: Right
        align: { y: 0.5 }
        spacing: 8
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance bg_color: #00000000
            instance bg_color_hover: #f8fafc
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let result_color = mix(self.bg_color, self.bg_color_hover, self.hover);
                return result_color;
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

        label = <Label> {
            width: Fill
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                color: (FOREGROUND)
            }
            text: "Accordion Title"
        }

        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance rotation: 0.0
                instance icon_color: #64748b

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // Rotate around center
                    sdf.rotate(self.rotation, c.x, c.y);

                    // Draw chevron (down arrow)
                    let size = 6.0;
                    sdf.move_to(c.x - size, c.y - size * 0.5);
                    sdf.line_to(c.x, c.y + size * 0.5);
                    sdf.line_to(c.x + size, c.y - size * 0.5);
                    sdf.stroke(self.icon_color, 1.5);

                    return sdf.result;
                }
            }
        }
    }

    // Accordion content wrapper
    MpAccordionContentBase = <View> {
        width: Fill
        height: Fit
        padding: { left: 16, right: 16, top: 0, bottom: 12 }
        flow: Down
    }

    // ============================================================
    // Accordion Item (header + content)
    // ============================================================

    pub MpAccordionItem = <View> {
        width: Fill
        height: Fit
        flow: Down

        show_bg: true
        draw_bg: {
            color: (CARD)
        }

        header = <MpAccordionHeaderBase> {}
        body = <MpAccordionContentBase> {}
    }

    // ============================================================
    // Accordion Container
    // ============================================================

    pub MpAccordion = <RoundedView> {
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
    // Variants
    // ============================================================

    // Bordered accordion (each item has border)
    pub MpAccordionBordered = <View> {
        width: Fill
        height: Fit
        flow: Down
        spacing: 8
    }

    pub MpAccordionItemBordered = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down

        draw_bg: {
            color: (CARD)
            border_radius: 6.0
            border_color: (BORDER)
        }

        header = <MpAccordionHeaderBase> {}
        body = <MpAccordionContentBase> {}
    }

    // Ghost accordion (no background)
    pub MpAccordionGhost = <View> {
        width: Fill
        height: Fit
        flow: Down
    }

    pub MpAccordionItemGhost = <View> {
        width: Fill
        height: Fit
        flow: Down

        header = <MpAccordionHeaderBase> {
            draw_bg: {
                bg_color: #00000000
                bg_color_hover: #0000000D
            }
        }
        body = <MpAccordionContentBase> {}
    }

    // Divider between items
    pub MpAccordionDivider = <View> {
        width: Fill
        height: 1
        show_bg: true
        draw_bg: {
            color: (BORDER)
        }
    }
}

// ============================================================
// Rust Implementation
// ============================================================

#[derive(Live, LiveHook, Widget)]
pub struct MpAccordionHeader {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,

    #[rust]
    expanded: bool,
}

impl Widget for MpAccordionHeader {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.animator_handle_event(cx, event);

        match event.hits(cx, self.view.area()) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerDown(_) => {
                cx.widget_action(self.widget_uid(), &scope.path, MpAccordionAction::Toggle);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpAccordionHeader {
    pub fn set_expanded(&mut self, _cx: &mut Cx, expanded: bool) {
        self.expanded = expanded;
        // TODO: Animate icon rotation
    }
}

impl MpAccordionHeaderRef {
    pub fn toggled(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let MpAccordionAction::Toggle = item.cast() {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpAccordionAction {
    Toggle,
    None,
}
