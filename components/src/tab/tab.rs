use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use link::theme_colors::*;

    // ============================================================
    // MpTab - Individual tab item (clickable)
    // ============================================================

    // Base tab item - used inside TabBar
    MpTabBase = {{MpTab}} {
        width: Fit
        height: Fit
        align: { x: 0.5, y: 0.5 }
        padding: { left: 16, right: 16, top: 8, bottom: 8 }

        text: ""

        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0

            uniform border_radius: 0.0
            uniform border_width: 0.0

            uniform color: #00000000
            uniform color_hover: #0000000D
            uniform color_selected: #00000000

            uniform border_color: #00000000
            uniform border_color_selected: #00000000

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Apply selected first, then hover on top
                let bg = mix(self.color, self.color_selected, self.selected);
                let bg_final = mix(bg, self.color_hover, self.hover * (1.0 - self.selected));

                // Calculate box dimensions
                let bw = self.border_width;
                let box_w = self.rect_size.x - bw * 2.0;
                let box_h = self.rect_size.y - bw * 2.0;

                // Clamp radius to half of box height
                let max_r = box_h * 0.5;
                let r = min(self.border_radius, max_r);

                sdf.box(bw, bw, box_w, box_h, r);
                sdf.fill_keep(bg_final);

                if bw > 0.0 {
                    let border = mix(self.border_color, self.border_color_selected, self.selected);
                    sdf.stroke(border, bw);
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance hover: 0.0
            instance selected: 0.0

            uniform color: #64748b
            uniform color_hover: #334155
            uniform color_selected: #0f172a

            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }

            fn get_color(self) -> vec4 {
                // Apply selected first, then hover on top (only if not selected)
                let c = mix(self.color, self.color_selected, self.selected);
                return mix(c, self.color_hover, self.hover * (1.0 - self.selected));
            }
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: {
                        draw_bg: { hover: 0.0 }
                        draw_text: { hover: 0.0 }
                    }
                }
                on = {
                    cursor: Hand
                    from: { all: Forward { duration: 0.1 } }
                    apply: {
                        draw_bg: { hover: 1.0 }
                        draw_text: { hover: 1.0 }
                    }
                }
            }
            selected = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
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
        }
    }

    // ============================================================
    // Tab Variants
    // ============================================================

    // Default Tab style
    pub MpTab = <MpTabBase> {
        draw_bg: {
            color: #00000000
            color_hover: #f1f5f9
            color_selected: #ffffff

            border_width: 1.0
            border_radius: 6.0
            border_color: #00000000
            border_color_selected: #e2e8f0
        }
    }

    // Underline Tab - minimal with bottom indicator
    pub MpTabUnderline = <MpTabBase> {
        padding: { left: 12, right: 12, top: 8, bottom: 8 }

        draw_bg: {
            instance selected: 0.0

            uniform indicator_height: 2.0
            uniform indicator_color: #3b82f6

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Draw underline indicator when selected
                if self.selected > 0.5 {
                    sdf.rect(
                        0.0,
                        self.rect_size.y - self.indicator_height,
                        self.rect_size.x,
                        self.indicator_height
                    );
                    sdf.fill(self.indicator_color);
                }

                return sdf.result;
            }
        }

        draw_text: {
            instance hover: 0.0
            instance selected: 0.0

            uniform color: #64748b
            uniform color_hover: #334155
            uniform color_selected: #3b82f6

            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }

            fn get_color(self) -> vec4 {
                let c = mix(self.color, self.color_selected, self.selected);
                return mix(c, self.color_hover, self.hover * (1.0 - self.selected));
            }
        }
    }

    // Outline Tab - bordered outline with rounded corners
    pub MpTabOutline = <MpTabBase> {
        padding: { left: 16, right: 16, top: 6, bottom: 6 }

        draw_bg: {
            border_radius: 6.0
            border_width: 1.0

            color: #00000000
            color_hover: #f8fafc
            color_selected: #00000000

            border_color: #e2e8f0
            border_color_selected: #3b82f6
        }

        draw_text: {
            instance hover: 0.0
            instance selected: 0.0

            uniform color: #64748b
            uniform color_hover: #334155
            uniform color_selected: #3b82f6

            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }

            fn get_color(self) -> vec4 {
                let c = mix(self.color, self.color_selected, self.selected);
                return mix(c, self.color_hover, self.hover * (1.0 - self.selected));
            }
        }
    }

    // Pill Tab - 圆角矩形，默认透明，选中蓝底白字
    pub MpTabPill = <MpTabBase> {
        padding: { left: 16, right: 16, top: 8, bottom: 8 }

        draw_bg: {
            border_radius: 6.0
            border_width: 0.0

            color: #00000000
            color_hover: #dbeafe
            color_selected: #3b82f6

            border_color: #00000000
            border_color_selected: #00000000
        }

        draw_text: {
            color: #64748b
            color_hover: #1d4ed8
            color_selected: #ffffff
        }
    }

    // Segmented Tab - iOS style segmented control item
    pub MpTabSegmented = <MpTabBase> {
        padding: { left: 16, right: 16, top: 6, bottom: 6 }

        draw_bg: {
            border_radius: 4.0
            border_width: 0.0

            color: #00000000
            color_hover: #00000008
            color_selected: #ffffff
        }
    }

    // ============================================================
    // TabBar Containers
    // ============================================================

    // Base TabBar container
    MpTabBarBase = <View> {
        width: Fit
        height: Fit
        flow: Right
        align: { y: 0.5 }
    }

    // Default TabBar
    pub MpTabBar = <MpTabBarBase> {
        padding: 4
        spacing: 4
        show_bg: true
        draw_bg: {
            color: #f1f5f9
            instance radius: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.radius);
                sdf.fill(self.color);
                return sdf.result;
            }
        }
    }

    // Underline TabBar - with bottom border
    pub MpTabBarUnderline = <MpTabBarBase> {
        spacing: 0
        show_bg: true
        draw_bg: {
            uniform border_color: #e2e8f0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Bottom border line
                sdf.rect(0.0, self.rect_size.y - 1.0, self.rect_size.x, 1.0);
                sdf.fill(self.border_color);

                return sdf.result;
            }
        }
    }

    // Pill TabBar - transparent background
    pub MpTabBarPill = <MpTabBarBase> {
        spacing: 4
    }

    // Outline TabBar - transparent background
    pub MpTabBarOutline = <MpTabBarBase> {
        spacing: 8
    }

    // Segmented TabBar - with background container
    pub MpTabBarSegmented = <MpTabBarBase> {
        padding: 4
        spacing: 2
        show_bg: true
        draw_bg: {
            color: #f1f5f9
            instance radius: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0., 0., self.rect_size.x, self.rect_size.y, self.radius);
                sdf.fill(self.color);
                return sdf.result;
            }
        }
    }

    // ============================================================
    // Size Variants
    // ============================================================

    pub MpTabSmall = <MpTab> {
        padding: { left: 12, right: 12, top: 4, bottom: 4 }
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
        }
    }

    pub MpTabLarge = <MpTab> {
        padding: { left: 20, right: 20, top: 10, bottom: 10 }
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 16.0 }
        }
    }
}

// ============================================================
// Rust Implementation
// ============================================================

#[derive(Live, LiveHook, Widget)]
pub struct MpTab {
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

    #[animator]
    animator: Animator,

    #[rust]
    selected: bool,
}

impl Widget for MpTab {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);

        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                // Only play hover animation if not selected
                if !self.selected {
                    self.animator_play(cx, ids!(hover.on));
                }
            }
            Hit::FingerHoverOut(_) => {
                if !self.selected {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            Hit::FingerDown(_) => {
                // Reset hover state directly to prevent interference
                self.draw_bg.apply_over(cx, live! { hover: 0.0 });
                self.draw_text.apply_over(cx, live! { hover: 0.0 });
                cx.widget_action(self.widget_uid(), &scope.path, MpTabAction::Clicked);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // Debug: force set selected value before draw
        if self.selected {
            self.draw_bg.apply_over(cx, live! { selected: 1.0 });
            self.draw_text.apply_over(cx, live! { selected: 1.0 });
        }

        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        DrawStep::done()
    }

    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }

    fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.text.as_mut_empty().push_str(text);
        self.redraw(cx);
    }
}

impl MpTab {
    pub fn set_selected(&mut self, cx: &mut Cx, selected: bool) {
        if self.selected != selected {
            self.selected = selected;
            // Directly apply the selected state to shader variables
            let val = if selected { 1.0f64 } else { 0.0f64 };
            self.draw_bg.apply_over(cx, live! {
                selected: (val)
            });
            self.draw_text.apply_over(cx, live! {
                selected: (val)
            });
            self.redraw(cx);
        }
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

impl MpTabRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let MpTabAction::Clicked = item.cast() {
                return true;
            }
        }
        false
    }

    pub fn set_selected(&self, cx: &mut Cx, selected: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected(cx, selected);
        }
    }

    pub fn is_selected(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_selected()
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpTabAction {
    Clicked,
    None,
}
