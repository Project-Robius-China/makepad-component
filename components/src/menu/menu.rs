use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use link::theme_colors::*;

    // Menu container
    pub MpMenu = <View> {
        width: 220
        height: Fit
        flow: Down
        spacing: 2
        padding: { left: 4, right: 4, top: 4, bottom: 4 }

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance border_color: (BORDER)
            instance border_width: 1.0
            instance radius: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    self.radius
                );
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, self.border_width);
                return sdf.result;
            }
        }
    }

    // Inline menu (no outer card)
    pub MpMenuInline = <MpMenu> {
        show_bg: false
    }

    // Section title inside menu
    pub MpMenuSection = <View> {
        width: Fill
        height: Fit
        padding: { left: 12, right: 12, top: 8, bottom: 4 }

        label = <Label> {
            width: Fill
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 11.0 }
                color: (MUTED_FOREGROUND)
            }
            text: "SECTION"
        }
    }

    // Divider inside menu
    pub MpMenuDivider = <View> {
        width: Fill
        height: 1
        margin: { left: 8, right: 8, top: 4, bottom: 4 }
        show_bg: true
        draw_bg: { color: (BORDER) }
    }

    // Interactive menu item
    pub MpMenuItem = {{MpMenuItem}} {
        width: Fill
        height: Fit
        align: { x: 0.0, y: 0.5 }
        padding: { left: 14, right: 12, top: 8, bottom: 8 }

        text: "Menu Item"
        selected: false
        disabled: false

        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0
            instance disabled: 0.0

            uniform radius: 6.0
            uniform color: #00000000
            uniform color_hover: (SECONDARY)
            uniform color_selected: (PRIMARY)
            uniform color_disabled: #00000000
            uniform indicator_color: (PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let bg = mix(self.color, self.color_hover, self.hover);
                let bg = mix(bg, self.color_selected, self.selected);
                let bg = mix(bg, self.color_disabled, self.disabled);

                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.radius);
                sdf.fill_keep(bg);

                let indicator = self.selected * (1.0 - self.disabled);
                if indicator > 0.001 {
                    sdf.box(4.0, 4.0, 3.0, self.rect_size.y - 8.0, 1.5);
                    sdf.fill(self.indicator_color);
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance hover: 0.0
            instance selected: 0.0
            instance disabled: 0.0

            uniform color: (FOREGROUND)
            uniform color_hover: (FOREGROUND)
            uniform color_selected: (PRIMARY_FOREGROUND)
            uniform color_disabled: (MUTED_FOREGROUND)

            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }

            fn get_color(self) -> vec4 {
                let c = mix(self.color, self.color_hover, self.hover);
                let c = mix(c, self.color_selected, self.selected);
                return mix(c, self.color_disabled, self.disabled);
            }
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.12 } }
                    apply: {
                        draw_bg: { hover: 0.0 }
                        draw_text: { hover: 0.0 }
                    }
                }
                on = {
                    from: { all: Forward { duration: 0.08 } }
                    apply: {
                        draw_bg: { hover: 1.0 }
                        draw_text: { hover: 1.0 }
                    }
                }
            }
            selected_state = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.12 } }
                    apply: {
                        draw_bg: { selected: 0.0 }
                        draw_text: { selected: 0.0 }
                    }
                }
                on = {
                    from: { all: Snap }
                    apply: {
                        draw_bg: { selected: 1.0 }
                        draw_text: { selected: 1.0 }
                    }
                }
            }
            disabled_state = {
                default: off
                off = {
                    from: { all: Snap }
                    apply: {
                        draw_bg: { disabled: 0.0 }
                        draw_text: { disabled: 0.0 }
                    }
                }
                on = {
                    from: { all: Snap }
                    apply: {
                        draw_bg: { disabled: 1.0 }
                        draw_text: { disabled: 1.0 }
                    }
                }
            }
        }
    }

    pub MpMenuItemDanger = <MpMenuItem> {
        draw_text: {
            color: (DANGER)
            color_hover: (DANGER)
            color_selected: (PRIMARY_FOREGROUND)
        }
    }

    pub MpMenuItemDisabled = <MpMenuItem> {
        disabled: true
        draw_bg: {
            disabled: 1.0
            color_hover: #00000000
            color_selected: #00000000
            indicator_color: #00000000
        }
        draw_text: {
            disabled: 1.0
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpMenuItemAction {
    Clicked(String),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct MpMenuItem {
    #[animator]
    animator: Animator,

    #[redraw]
    #[live]
    draw_bg: DrawQuad,

    #[live]
    draw_text: DrawText,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live]
    text: ArcStringMut,

    #[live]
    selected: bool,

    #[live]
    disabled: bool,
}

impl Widget for MpMenuItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        if self.disabled {
            return;
        }

        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerDown(fe) if fe.is_primary_hit() => {
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerUp(fe) if fe.is_primary_hit() => {
                if fe.is_over {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        MpMenuItemAction::Clicked(self.text.as_ref().to_string()),
                    );
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}

impl MpMenuItem {
    fn sync_visual_state(&mut self, cx: &mut Cx) {
        self.animator_toggle(
            cx,
            self.selected,
            Animate::No,
            ids!(selected_state.on),
            ids!(selected_state.off),
        );
        self.animator_toggle(
            cx,
            self.disabled,
            Animate::No,
            ids!(disabled_state.on),
            ids!(disabled_state.off),
        );
    }

    pub fn set_selected(&mut self, cx: &mut Cx, selected: bool) {
        if self.selected != selected {
            self.selected = selected;
            self.sync_visual_state(cx);
            self.redraw(cx);
        }
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_disabled(&mut self, cx: &mut Cx, disabled: bool) {
        if self.disabled != disabled {
            self.disabled = disabled;
            self.sync_visual_state(cx);
            self.redraw(cx);
        }
    }

    pub fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.text.as_mut_empty().push_str(text);
        self.redraw(cx);
    }
}

impl MpMenuItemRef {
    pub fn clicked(&self, actions: &Actions) -> Option<String> {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            if let MpMenuItemAction::Clicked(label) = action.cast::<MpMenuItemAction>() {
                return Some(label);
            }
        }
        None
    }

    pub fn set_selected(&self, cx: &mut Cx, selected: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected(cx, selected);
        }
    }

    pub fn is_selected(&self) -> bool {
        if let Some(inner) = self.borrow() {
            return inner.is_selected();
        }
        false
    }

    pub fn set_disabled(&self, cx: &mut Cx, disabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_disabled(cx, disabled);
        }
    }

    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(cx, text);
        }
    }
}
