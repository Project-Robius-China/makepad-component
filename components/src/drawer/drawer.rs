use makepad_widgets::*;

#[derive(Live, LiveHook, LiveRegister)]
#[live_ignore]
pub enum MpDrawerPlacement {
    #[pick]
    Right,
    Left,
    Top,
    Bottom,
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use link::theme_colors::*;
    use crate::theme::radius::*;

    // ============================================================
    // MpDrawer - Drawer component
    // ============================================================

    // Drawer backdrop (mask)
    MpDrawerBackdrop = <View> {
        width: Fill
        height: Fill

        show_bg: true
        draw_bg: {
            color: #x00000066

            fn pixel(self) -> vec4 {
                return self.color;
            }
        }
    }

    // Base drawer panel
    MpDrawerBase = {{MpDrawer}} {
        fixed: false
        width: Fit
        height: Fit
        flow: Down

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance border_radius: (RADIUS_NONE)
            instance border_color: (BORDER)
            instance shadow_color: #x00000026
            instance shadow_offset_x: 0.0
            instance shadow_offset_y: 0.0
            instance shadow_blur: 24.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let shadow_x = self.shadow_offset_x;
                let shadow_y = self.shadow_offset_y;

                // Shadow
                sdf.box(
                    shadow_x,
                    shadow_y,
                    self.rect_size.x - abs(shadow_x),
                    self.rect_size.y - abs(shadow_y),
                    self.border_radius
                );
                sdf.blur = self.shadow_blur;
                sdf.fill(self.shadow_color);
                sdf.blur = 0.0;

                // Main panel
                sdf.box(
                    0.5,
                    0.5,
                    self.rect_size.x - 1.0,
                    self.rect_size.y - 1.0,
                    self.border_radius
                );
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, 1.0);

                return sdf.result;
            }
        }

        header = <View> {
            width: Fill
            height: Fit
            padding: { left: 24, right: 24, top: 20, bottom: 16 }
            flow: Right
            align: { y: 0.5 }

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 18.0 }
                    color: (FOREGROUND)
                    wrap: Word
                }
                text: "Drawer Title"
            }

            close = <View> {
                width: 24
                height: 24
                cursor: Hand
                align: { x: 0.5, y: 0.5 }

                show_bg: true
                draw_bg: {
                    instance icon_color: (MUTED_FOREGROUND)
                    instance hover: 0.0
                    instance down: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let c = self.rect_size * 0.5;
                        let size = 6.0;

                        let hover_amount = max(self.hover, self.down * 0.6);
                        let final_color = mix(self.icon_color, (FOREGROUND), hover_amount);

                        // X mark
                        sdf.move_to(c.x - size, c.y - size);
                        sdf.line_to(c.x + size, c.y + size);
                        sdf.stroke(final_color, 1.5);

                        sdf.move_to(c.x + size, c.y - size);
                        sdf.line_to(c.x - size, c.y + size);
                        sdf.stroke(final_color, 1.5);

                        return sdf.result;
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
                    down = {
                        default: off
                        off = {
                            from: { all: Forward { duration: 0.1 } }
                            apply: { draw_bg: { down: 0.0 } }
                        }
                        on = {
                            from: { all: Forward { duration: 0.05 } }
                            apply: { draw_bg: { down: 1.0 } }
                        }
                    }
                }
            }
        }

        body = <ScrollYView> {
            width: Fill
            height: Fit
            padding: { left: 24, right: 24, top: 0, bottom: 16 }
            flow: Down
            spacing: 8

            <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                    color: (MUTED_FOREGROUND)
                    wrap: Word
                }
                text: "Drawer content goes here."
            }
        }

        footer = <View> {
            width: Fill
            height: Fit
            padding: { left: 24, right: 24, top: 16, bottom: 20 }
            flow: Right
            spacing: 8
            align: { x: 1.0, y: 0.5 }
        }
    }

    // ============================================================
    // Drawer Placement Variants
    // ============================================================

    pub MpDrawerRight = <MpDrawerBase> {
        fixed: true
        width: 320
        height: Fill
        draw_bg: { shadow_offset_x: -8.0 }
    }

    pub MpDrawerLeft = <MpDrawerBase> {
        fixed: true
        width: 320
        height: Fill
        draw_bg: { shadow_offset_x: 8.0 }
    }

    pub MpDrawerTop = <MpDrawerBase> {
        width: Fill
        height: Fit
        draw_bg: { shadow_offset_y: 8.0 }
    }

    pub MpDrawerBottom = <MpDrawerBase> {
        width: Fill
        height: Fit
        draw_bg: { shadow_offset_y: -8.0 }
    }

    pub MpDrawer = <MpDrawerRight> {}

    // ============================================================
    // Drawer Containers (with backdrop)
    // ============================================================

    pub MpDrawerContainerRight = <View> {
        width: Fill
        height: Fill
        flow: Overlay
        align: { x: 1.0, y: 0.0 }

        backdrop = <MpDrawerBackdrop> {}
        drawer = <MpDrawerRight> {}
    }

    pub MpDrawerContainerLeft = <View> {
        width: Fill
        height: Fill
        flow: Overlay
        align: { x: 0.0, y: 0.0 }

        backdrop = <MpDrawerBackdrop> {}
        drawer = <MpDrawerLeft> {}
    }

    pub MpDrawerContainerTop = <View> {
        width: Fill
        height: Fill
        flow: Overlay
        align: { x: 0.0, y: 0.0 }

        backdrop = <MpDrawerBackdrop> {}
        drawer = <MpDrawerTop> {}
    }

    pub MpDrawerContainerBottom = <View> {
        width: Fill
        height: Fill
        flow: Overlay
        align: { x: 0.0, y: 1.0 }

        backdrop = <MpDrawerBackdrop> {}
        drawer = <MpDrawerBottom> {}
    }

    // ============================================================
    // Drawer Layout Helpers
    // ============================================================

    pub MpDrawerHeader = <View> {
        width: Fill
        height: Fit
        padding: { left: 24, right: 24, top: 20, bottom: 16 }
        flow: Right
        align: { y: 0.5 }
    }

    pub MpDrawerBody = <ScrollYView> {
        width: Fill
        height: Fit
        padding: { left: 24, right: 24, top: 0, bottom: 16 }
        flow: Down
        spacing: 8
    }

    pub MpDrawerFooter = <View> {
        width: Fill
        height: Fit
        padding: { left: 24, right: 24, top: 16, bottom: 20 }
        flow: Right
        spacing: 8
        align: { x: 1.0, y: 0.5 }
    }

    pub MpDrawerDivider = <View> {
        width: Fill
        height: 1
        show_bg: true
        draw_bg: {
            color: (BORDER)
        }
    }

    // ============================================================
    // Interactive Drawer Widget
    // ============================================================

    pub MpDrawerWidget = {{MpDrawerWidget}} {
        width: Fill
        height: Fill
        flow: Overlay
        visible: false

        placement: Right
        container = <MpDrawerContainerRight> {}
    }

    pub MpDrawerWidgetLeft = <MpDrawerWidget> {
        placement: Left
        container = <MpDrawerContainerLeft> {}
    }

    pub MpDrawerWidgetTop = <MpDrawerWidget> {
        placement: Top
        max_height: 420
        container = <MpDrawerContainerTop> {}
    }

    pub MpDrawerWidgetBottom = <MpDrawerWidget> {
        placement: Bottom
        max_height: 420
        container = <MpDrawerContainerBottom> {}
    }
}

#[derive(Live, Widget)]
pub struct MpDrawer {
    #[deref]
    view: View,
    #[live(true)]
    fixed: bool,
}

/// Drawer actions
#[derive(Clone, Debug, DefaultNone)]
pub enum MpDrawerAction {
    None,
    Opened,
    Closed,
    CloseRequested,
}

impl LiveHook for MpDrawer {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.sync_fixed_layout(cx);
    }
}

/// Interactive drawer widget with open/close functionality
#[derive(Live, LiveHook, Widget)]
pub struct MpDrawerWidget {
    #[deref]
    view: View,

    #[live]
    visible: bool,

    #[live]
    placement: MpDrawerPlacement,
    #[live(0.0)]
    max_height: f64,

    #[rust]
    progress: f64,
    #[rust]
    animating: bool,
    #[rust]
    anim_from: f64,
    #[rust]
    anim_to: f64,
    #[rust]
    anim_start: f64,
    #[rust]
    next_frame: NextFrame,
    #[rust]
    drawer_size: Vec2,
    #[rust]
    pass_size: Vec2,
    #[rust]
    scroll_enabled: bool,
    #[rust]
    height_clamped: bool,
}

impl Widget for MpDrawerWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if let Event::NextFrame(ne) = event {
            if ne.set.contains(&self.next_frame) {
                self.tick_animation(cx, ne.time);
            }
        }

        if !self.visible {
            return;
        }

        self.view.handle_event(cx, event, scope);

        let backdrop = self.view.view(ids!(container.backdrop));
        if let Hit::FingerUp(fe) = event.hits(cx, backdrop.area()) {
            if fe.is_over {
                cx.widget_action(self.widget_uid(), &scope.path, MpDrawerAction::CloseRequested);
            }
        }

        let close_btn = self.view.view(ids!(container.drawer.header.close));
        if let Hit::FingerUp(fe) = event.hits(cx, close_btn.area()) {
            if fe.is_over {
                cx.widget_action(self.widget_uid(), &scope.path, MpDrawerAction::CloseRequested);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.pass_size = cx.current_pass_size().into();
        self.apply_animation_state(cx);
        let result = self.view.draw_walk(cx, scope, walk);
        self.update_drawer_size(cx);
        self.block_background_scroll(cx);
        result
    }
}

impl MpDrawerWidget {
    /// Open the drawer
    pub fn open(&mut self, cx: &mut Cx) {
        self.visible = true;
        self.height_clamped = false;
        self.scroll_enabled = false;
        self.drawer_size = Vec2::default();
        self.start_animation(cx, 1.0);
        self.redraw(cx);
    }

    /// Close the drawer
    pub fn close(&mut self, cx: &mut Cx) {
        self.start_animation(cx, 0.0);
        self.redraw(cx);
    }

    /// Check if drawer is visible
    pub fn is_open(&self) -> bool {
        self.visible
    }

    fn start_animation(&mut self, cx: &mut Cx, target: f64) {
        let target = target.clamp(0.0, 1.0);
        if (self.progress - target).abs() < 0.0001 {
            if target <= 0.0 {
                self.visible = false;
            }
            return;
        }
        self.anim_from = self.progress;
        self.anim_to = target;
        self.anim_start = 0.0;
        self.animating = true;
        self.next_frame = cx.new_next_frame();
    }

    fn tick_animation(&mut self, cx: &mut Cx, time: f64) {
        if !self.animating {
            return;
        }
        if self.anim_start == 0.0 {
            self.anim_start = time;
        }
        let duration = 0.2;
        let mut t = (time - self.anim_start) / duration;
        if t >= 1.0 {
            t = 1.0;
        }
        let eased = t * t * (3.0 - 2.0 * t);
        self.progress = self.anim_from + (self.anim_to - self.anim_from) * eased;

        if t < 1.0 {
            self.next_frame = cx.new_next_frame();
        } else {
            self.animating = false;
            if self.progress <= 0.0 {
                self.visible = false;
                cx.unblock_scrolling();
            }
        }
        self.redraw(cx);
    }

    fn apply_animation_state(&mut self, cx: &mut Cx) {
        let mut size = self.drawer_size;
        let drawer_walk = self.view.view(ids!(container.drawer)).walk(cx);
        let height_is_fixed = drawer_walk.height.is_fixed() || drawer_walk.height.is_fill();
        if size.x <= 0.0 || size.y <= 0.0 {
            if size.x <= 0.0 {
                if let Size::Fixed(w) = drawer_walk.width {
                    size.x = w as f32;
                } else if matches!(self.placement, MpDrawerPlacement::Right | MpDrawerPlacement::Left) {
                    size.x = 320.0;
                }
            }
            if size.y <= 0.0 {
                if let Size::Fixed(h) = drawer_walk.height {
                    size.y = h as f32;
                } else if matches!(self.placement, MpDrawerPlacement::Top | MpDrawerPlacement::Bottom) {
                    size.y = 240.0;
                }
            }
        }

        let mut max_height = if self.max_height > 0.0 { self.max_height } else { 0.0 };
        if max_height > 0.0 && self.pass_size.y > 0.0 {
            max_height = max_height.min(self.pass_size.y as f64);
        }

        let is_vertical = matches!(self.placement, MpDrawerPlacement::Top | MpDrawerPlacement::Bottom);
        let mut limit_height = false;
        if is_vertical && !height_is_fixed && max_height > 0.0 {
            if self.height_clamped {
                limit_height = true;
            } else if self.drawer_size.y > 0.0 && (self.drawer_size.y as f64) > max_height {
                limit_height = true;
            }
        }

        if limit_height {
            self.view
                .view(ids!(container.drawer))
                .apply_over(cx, live! { walk: { height: (max_height) } });
        } else if self.height_clamped {
            self.view
                .view(ids!(container.drawer))
                .apply_over(cx, live! { walk: { height: Fit } });
        }
        self.height_clamped = limit_height;

        let use_scroll = height_is_fixed || limit_height;
        if use_scroll != self.scroll_enabled {
            self.scroll_enabled = use_scroll;
            if use_scroll {
                self.view.view(ids!(container.drawer.body)).apply_over(cx, live! {
                    walk: { height: Fill }
                    scroll_bars: {
                        show_scroll_x: false,
                        show_scroll_y: true,
                        scroll_bar_y: { drag_scrolling: true }
                    }
                });
            } else {
                self.view.view(ids!(container.drawer.body)).apply_over(cx, live! {
                    walk: { height: Fit }
                    scroll_bars: { show_scroll_x: false, show_scroll_y: false }
                });
            }
        }

        let progress = self.progress.clamp(0.0, 1.0) as f32;
        let backdrop_alpha = 0.4 * progress;
        let backdrop_color = vec4(0.0, 0.0, 0.0, backdrop_alpha);

        let drawer = self.view.view(ids!(container.drawer));
        let offset = 1.0 - progress;
        if limit_height && size.y <= 0.0 {
            size.y = (max_height as f32).max(1.0);
        }
        match self.placement {
            MpDrawerPlacement::Right => {
                drawer.apply_over(cx, live! { walk: { margin: { right: (-size.x * offset) } } });
            }
            MpDrawerPlacement::Left => {
                drawer.apply_over(cx, live! { walk: { margin: { left: (-size.x * offset) } } });
            }
            MpDrawerPlacement::Top => {
                drawer.apply_over(cx, live! { walk: { margin: { top: (-size.y * offset) } } });
            }
            MpDrawerPlacement::Bottom => {
                drawer.apply_over(cx, live! { walk: { margin: { bottom: (-size.y * offset) } } });
            }
        }

        self.view.view(ids!(container.backdrop)).apply_over(cx, live! {
            draw_bg: { color: (backdrop_color) }
        });
    }

    fn update_drawer_size(&mut self, cx: &mut Cx) {
        let drawer_area = self.view.view(ids!(container.drawer)).area();
        let rect = drawer_area.rect(cx);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            self.drawer_size = rect.size.into();
        }
    }

    fn block_background_scroll(&mut self, cx: &mut Cx) {
        let body_area = self.view.view(ids!(container.drawer.body)).area();
        if self.visible {
            cx.block_scrolling_except_within(body_area);
        } else {
            cx.unblock_scrolling();
        }
    }
}

impl MpDrawerWidgetRef {
    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    pub fn is_open(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_open()
        } else {
            false
        }
    }

    pub fn close_requested(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                return matches!(item.cast::<MpDrawerAction>(), MpDrawerAction::CloseRequested);
            }
        }
        false
    }
}

impl Widget for MpDrawer {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpDrawer {
    fn sync_fixed_layout(&mut self, cx: &mut Cx) {
        if self.fixed && matches!(self.view.walk.height, Size::Fit { .. }) {
            self.view.walk.height = Size::fill();
        }

        let fixed_height = self.view.walk.height.is_fixed() || self.view.walk.height.is_fill();

        if fixed_height {
            self.view(ids!(body)).apply_over(cx, live! {
                walk: { height: Fill }
                scroll_bars: {
                    show_scroll_x: false,
                    show_scroll_y: true,
                    scroll_bar_y: { drag_scrolling: true }
                }
            });
        } else {
            self.view.walk.height = Size::fit();
            self.view(ids!(body)).apply_over(cx, live! {
                walk: { height: Fit }
                scroll_bars: {
                    show_scroll_x: false,
                    show_scroll_y: false
                }
            });
        }

        self.view.redraw(cx);
    }
}
