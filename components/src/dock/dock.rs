use makepad_widgets::*;

#[derive(Live, LiveHook, LiveRegister)]
#[live_ignore]
pub enum MpDockSplitAxis {
    #[pick]
    Horizontal,
    Vertical,
}

#[derive(Live, LiveHook, LiveRegister, Clone, Copy, Debug, PartialEq)]
#[live_ignore]
pub enum MpDockPlacement {
    #[pick]
    Left,
    Right,
    Bottom,
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use link::theme_colors::*;

    pub MpDockPanel = {{MpDockPanel}} {
        width: Fill
        height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            color: (CARD)
            fn pixel(self) -> vec4 {
                return self.color;
            }
        }
    }

    pub MpDockSplitter = {{MpDockSplitter}} {
        width: Fill
        height: Fill
        flow: Right

        draw_splitter: {
            instance hover: 0.0
            instance down: 0.0
            instance is_vertical: 0.0
            instance handle_color: (BORDER)
            instance handle_color_hover: (PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                let line_w = 3.0;
                let rx = mix(c.x - line_w * 0.5, 0.0, self.is_vertical);
                let ry = mix(0.0, c.y - line_w * 0.5, self.is_vertical);
                let rw = mix(line_w, self.rect_size.x, self.is_vertical);
                let rh = mix(self.rect_size.y, line_w, self.is_vertical);
                sdf.rect(rx, ry, rw, rh);

                let hover_amount = max(self.hover, self.down);
                let col = mix(self.handle_color, self.handle_color_hover, hover_amount);
                sdf.fill(col);

                return sdf.result;
            }
        }

        handle_size: 10.0
        min_size: 50.0

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_splitter: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_splitter: { hover: 1.0 } }
                }
            }
            down = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_splitter: { down: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.05 } }
                    apply: { draw_splitter: { down: 1.0 } }
                }
            }
        }

        first = <View> { width: Fill, height: Fill }
        second = <View> { width: Fill, height: Fill }
    }

    // ============================================================
    // MpDockTab - Individual tab button for dock tab panel
    // ============================================================

    pub MpDockTab = {{MpDockTab}} {
        width: Fit
        height: Fit
        align: { x: 0.5, y: 0.5 }
        padding: { left: 12, right: 12, top: 6, bottom: 6 }

        text: ""

        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0
            instance bg_color: #00000000
            instance bg_color_hover: #0000000D
            instance bg_color_selected: (MUTED)
            instance border_color_selected: (PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let bg = mix(self.bg_color, self.bg_color_selected, self.selected);
                let bg_final = mix(bg, self.bg_color_hover, self.hover * (1.0 - self.selected));

                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                sdf.fill_keep(bg_final);

                // Bottom border for selected tab
                if self.selected > 0.5 {
                    sdf.rect(0.0, self.rect_size.y - 2.0, self.rect_size.x, 2.0);
                    sdf.fill(self.border_color_selected);
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
            text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }

            fn get_color(self) -> vec4 {
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
                    from: { all: Forward { duration: 0.1 } }
                    apply: {
                        draw_bg: { selected: 1.0 }
                        draw_text: { selected: 1.0 }
                    }
                }
            }
        }
    }

    // ============================================================
    // MpDockTabPanel - Tabbed container with tab bar and content
    // ============================================================

    pub MpDockTabPanel = {{MpDockTabPanel}} {
        width: Fill
        height: Fill
        flow: Down

        tab_bar = <View> {
            width: Fill
            height: Fit
            flow: Right
            padding: { left: 4, top: 2, bottom: 0 }
            spacing: 2
            show_bg: true
            draw_bg: {
                instance bar_color: (MUTED)
                instance border_color: (BORDER)
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                    sdf.fill_keep(self.bar_color);
                    // Bottom border
                    sdf.rect(0.0, self.rect_size.y - 1.0, self.rect_size.x, 1.0);
                    sdf.fill(self.border_color);
                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fill
        }
    }

    // ============================================================
    // MpDockResizeHandle - Thin draggable bar between dock regions
    // ============================================================

    pub MpDockResizeHandle = {{MpDockResizeHandle}} {
        width: 8, height: Fill
        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance down: 0.0
            instance handle_color: #00000000
            instance handle_color_hover: (PRIMARY)

            fn pixel(self) -> vec4 {
                let hover_amount = max(self.hover, self.down);
                let col = mix(self.handle_color, self.handle_color_hover, hover_amount);
                return col;
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

    // ============================================================
    // MpDockArea - Top-level dock layout with collapsible side docks
    // ============================================================

    pub MpDockArea = {{MpDockArea}} {
        width: Fill
        height: Fill
        flow: Down

        main_row = <View> {
            width: Fill
            height: Fill
            flow: Right

            left_dock = <View> {
                width: 240
                height: Fill
                visible: false
            }

            left_handle = <MpDockResizeHandle> {
                width: 8, height: Fill
                visible: false
                cursor: ColResize
            }

            center = <View> {
                width: Fill
                height: Fill
            }

            right_handle = <MpDockResizeHandle> {
                width: 8, height: Fill
                visible: false
                cursor: ColResize
            }

            right_dock = <View> {
                width: 240
                height: Fill
                visible: false
            }
        }

        bottom_handle = <MpDockResizeHandle> {
            width: Fill, height: 8
            visible: false
            cursor: RowResize
        }

        bottom_dock = <View> {
            width: Fill
            height: 200
            visible: false
        }
    }

    // ============================================================
    // MpDockToggle - Toggle button for collapsing/expanding docks
    // ============================================================

    pub MpDockToggle = {{MpDockToggle}} {
        width: 20, height: 48
        cursor: Hand
        show_bg: true

        draw_bg: {
            instance hover: 0.0
            instance expanded: 0.0
            instance direction: 0.0

            instance bg_color: (MUTED)
            instance bg_color_hover: #e2e8f0
            instance arrow_color: #64748b
            instance arrow_color_hover: #334155

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Background rounded rect
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
                let bg = mix(self.bg_color, self.bg_color_hover, self.hover);
                sdf.fill_keep(bg);

                // Arrow icon
                let cx = self.rect_size.x * 0.5;
                let cy = self.rect_size.y * 0.5;
                let s = 4.0;
                let arrow_col = mix(self.arrow_color, self.arrow_color_hover, self.hover);

                if self.direction < 0.5 {
                    // Left dock: collapsed=arrow right, expanded=arrow left
                    let flip = mix(1.0, -1.0, self.expanded);
                    sdf.move_to(cx - s * flip, cy - s);
                    sdf.line_to(cx + s * flip, cy);
                    sdf.line_to(cx - s * flip, cy + s);
                    sdf.close_path();
                    sdf.fill(arrow_col);
                } else if self.direction < 1.5 {
                    // Right dock: collapsed=arrow left, expanded=arrow right
                    let flip = mix(-1.0, 1.0, self.expanded);
                    sdf.move_to(cx - s * flip, cy - s);
                    sdf.line_to(cx + s * flip, cy);
                    sdf.line_to(cx - s * flip, cy + s);
                    sdf.close_path();
                    sdf.fill(arrow_col);
                } else {
                    // Bottom dock: collapsed=arrow up, expanded=arrow down
                    let flip = mix(-1.0, 1.0, self.expanded);
                    sdf.move_to(cx - s, cy - s * flip);
                    sdf.line_to(cx, cy + s * flip);
                    sdf.line_to(cx + s, cy - s * flip);
                    sdf.close_path();
                    sdf.fill(arrow_col);
                }

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
        }
    }
}

// ============================================================
// MpDockPanel
// ============================================================

#[derive(Live, LiveHook, Widget)]
pub struct MpDockPanel {
    #[deref] view: View,
}

impl Widget for MpDockPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================
// MpDockSplitter
// ============================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpDockSplitterAction {
    None,
    SplitRatioChanged { ratio: f64 },
}

#[derive(Clone)]
enum SplitterDrawState {
    DrawFirst,
    DrawSplit,
    DrawSecond,
}

#[derive(Live, Widget)]
pub struct MpDockSplitter {
    #[deref] view: View,
    #[live] axis: MpDockSplitAxis,
    #[live(0.5)] split_ratio: f64,
    #[live(50.0)] min_size: f64,
    #[live(10.0)] handle_size: f64,
    #[rust] dragging: bool,
    #[rust] drag_start_ratio: f64,
    #[rust] rect: Rect,
    #[rust] position: f64,
    #[rust] area_first: Area,
    #[rust] area_second: Area,
    #[animator] animator: Animator,
    #[redraw] #[live] draw_splitter: DrawQuad,
    #[rust] draw_state: DrawStateWrap<SplitterDrawState>,
}

impl LiveHook for MpDockSplitter {}

impl MpDockSplitter {
    fn clamp_ratio(&self, ratio: f64, container_dim: f64) -> f64 {
        let available = (container_dim - self.handle_size).max(0.0);
        if available <= 0.0 {
            return 0.5;
        }
        let min_ratio = self.min_size / available;
        let max_ratio = 1.0 - min_ratio;
        if min_ratio >= max_ratio {
            return 0.5;
        }
        ratio.clamp(min_ratio, max_ratio)
    }

    fn margin(&self) -> Margin {
        match self.axis {
            MpDockSplitAxis::Horizontal => Margin {
                left: 3.0,
                top: 0.0,
                right: 3.0,
                bottom: 0.0,
            },
            MpDockSplitAxis::Vertical => Margin {
                left: 0.0,
                top: 3.0,
                right: 0.0,
                bottom: 3.0,
            },
        }
    }
}

impl Widget for MpDockSplitter {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if self.animator_handle_event(cx, event).must_redraw() {
            self.draw_splitter.redraw(cx);
        }

        match event.hits_with_options(cx, self.draw_splitter.area(), HitOptions::new().with_margin(self.margin())) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, ids!(hover.on));
                match self.axis {
                    MpDockSplitAxis::Horizontal => cx.set_cursor(MouseCursor::ColResize),
                    MpDockSplitAxis::Vertical => cx.set_cursor(MouseCursor::RowResize),
                }
            }
            Hit::FingerHoverOut(_) => {
                if !self.dragging {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            Hit::FingerDown(_) => {
                self.dragging = true;
                self.drag_start_ratio = self.split_ratio;
                self.animator_play(cx, ids!(down.on));
                match self.axis {
                    MpDockSplitAxis::Horizontal => cx.set_cursor(MouseCursor::ColResize),
                    MpDockSplitAxis::Vertical => cx.set_cursor(MouseCursor::RowResize),
                }
            }
            Hit::FingerMove(fe) => {
                if self.dragging {
                    let container_dim = match self.axis {
                        MpDockSplitAxis::Horizontal => self.rect.size.x,
                        MpDockSplitAxis::Vertical => self.rect.size.y,
                    };
                    let available = (container_dim - self.handle_size).max(0.0);
                    if available > 0.0 {
                        let container_origin = match self.axis {
                            MpDockSplitAxis::Horizontal => self.rect.pos.x,
                            MpDockSplitAxis::Vertical => self.rect.pos.y,
                        };
                        let mouse_pos = match self.axis {
                            MpDockSplitAxis::Horizontal => fe.abs.x,
                            MpDockSplitAxis::Vertical => fe.abs.y,
                        };
                        let new_ratio = (mouse_pos - container_origin - self.handle_size * 0.5) / available;
                        let clamped = self.clamp_ratio(new_ratio, container_dim);
                        if (clamped - self.split_ratio).abs() > 0.001 {
                            self.split_ratio = clamped;
                            self.redraw(cx);
                            cx.widget_action(
                                self.widget_uid(),
                                &scope.path,
                                MpDockSplitterAction::SplitRatioChanged { ratio: self.split_ratio },
                            );
                        }
                    }
                }
            }
            Hit::FingerUp(f) => {
                self.dragging = false;
                if f.is_over && f.device.has_hovers() {
                    self.animator_play(cx, ids!(hover.on));
                } else {
                    self.animator_play(cx, ids!(hover.off));
                }
                self.animator_play(cx, ids!(down.off));
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.draw_state.begin(cx, SplitterDrawState::DrawFirst) {
            // Begin outer turtle with correct flow direction
            match self.axis {
                MpDockSplitAxis::Horizontal => {
                    cx.begin_turtle(walk, Layout::flow_right());
                }
                MpDockSplitAxis::Vertical => {
                    cx.begin_turtle(walk, Layout::flow_down());
                }
            }

            self.rect = cx.turtle().inner_rect();
            let container_dim = match self.axis {
                MpDockSplitAxis::Horizontal => self.rect.size.x,
                MpDockSplitAxis::Vertical => self.rect.size.y,
            };
            let available = (container_dim - self.handle_size).max(0.0);
            self.position = if available > 0.0 {
                (available * self.split_ratio).clamp(self.min_size, (available - self.min_size).max(self.min_size))
            } else {
                0.0
            };

            // Begin sub-turtle for first child
            let first_walk = match self.axis {
                MpDockSplitAxis::Horizontal => Walk::new(Size::Fixed(self.position), Size::fill()),
                MpDockSplitAxis::Vertical => Walk::new(Size::fill(), Size::Fixed(self.position)),
            };
            cx.begin_turtle(first_walk, Layout::flow_down());
        }

        if let Some(SplitterDrawState::DrawFirst) = self.draw_state.get() {
            // Draw first child inside the sub-turtle
            let first = self.view.view(ids!(first));
            first.draw_all(cx, scope);
            self.draw_state.set(SplitterDrawState::DrawSplit);
        }

        if let Some(SplitterDrawState::DrawSplit) = self.draw_state.get() {
            // End first child's sub-turtle
            cx.end_turtle_with_area(&mut self.area_first);

            // Draw the splitter handle
            match self.axis {
                MpDockSplitAxis::Horizontal => {
                    self.draw_splitter.apply_over(cx, live! { is_vertical: 0.0 });
                    self.draw_splitter.draw_walk(cx, Walk::new(Size::Fixed(self.handle_size), Size::fill()));
                }
                MpDockSplitAxis::Vertical => {
                    self.draw_splitter.apply_over(cx, live! { is_vertical: 1.0 });
                    self.draw_splitter.draw_walk(cx, Walk::new(Size::fill(), Size::Fixed(self.handle_size)));
                }
            }

            // Begin sub-turtle for second child
            cx.begin_turtle(Walk::default(), Layout::flow_down());
            self.draw_state.set(SplitterDrawState::DrawSecond);
        }

        if let Some(SplitterDrawState::DrawSecond) = self.draw_state.get() {
            // Draw second child inside the sub-turtle
            let second = self.view.view(ids!(second));
            second.draw_all(cx, scope);

            // End second child's sub-turtle and outer turtle
            cx.end_turtle_with_area(&mut self.area_second);
            cx.end_turtle();
            self.draw_state.end();
        }

        DrawStep::done()
    }
}

pub trait MpDockSplitterRefExt {
    fn set_split_ratio(&self, cx: &mut Cx, ratio: f64);
    fn split_ratio(&self) -> f64;
    fn ratio_changed(&self, actions: &Actions) -> Option<f64>;
}

impl MpDockSplitterRefExt for MpDockSplitterRef {
    fn set_split_ratio(&self, cx: &mut Cx, ratio: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.split_ratio = ratio.clamp(0.0, 1.0);
            inner.redraw(cx);
        }
    }

    fn split_ratio(&self) -> f64 {
        if let Some(inner) = self.borrow() {
            inner.split_ratio
        } else {
            0.5
        }
    }

    fn ratio_changed(&self, actions: &Actions) -> Option<f64> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let MpDockSplitterAction::SplitRatioChanged { ratio } = item.cast() {
                return Some(ratio);
            }
        }
        None
    }
}

// ============================================================
// MpDockTab - Individual tab button
// ============================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpDockTabAction {
    Clicked,
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct MpDockTab {
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

impl Widget for MpDockTab {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);

        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
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
                self.draw_bg.apply_over(cx, live! { hover: 0.0 });
                self.draw_text.apply_over(cx, live! { hover: 0.0 });
                cx.widget_action(self.widget_uid(), &scope.path, MpDockTabAction::Clicked);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.selected {
            self.draw_bg.apply_over(cx, live! { selected: 1.0 });
            self.draw_text.apply_over(cx, live! { selected: 1.0 });
        }

        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
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

impl MpDockTab {
    pub fn set_selected(&mut self, cx: &mut Cx, selected: bool) {
        if self.selected != selected {
            self.selected = selected;
            let val = if selected { 1.0f64 } else { 0.0f64 };
            self.draw_bg.apply_over(cx, live! { selected: (val) });
            self.draw_text.apply_over(cx, live! { selected: (val) });
            self.redraw(cx);
        }
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

impl MpDockTabRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let MpDockTabAction::Clicked = item.cast() {
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

// ============================================================
// MpDockTabPanel - Tabbed container with tab switching
// ============================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpDockTabPanelAction {
    None,
    TabSelected { index: usize },
}

#[derive(Live, Widget)]
pub struct MpDockTabPanel {
    #[deref]
    view: View,

    #[rust]
    active_tab: usize,

    #[rust]
    tab_uids: Vec<WidgetUid>,
}

impl LiveHook for MpDockTabPanel {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.collect_tab_uids();
        self.sync_tab_visibility(cx);
    }
}

impl MpDockTabPanel {
    /// Collect widget UIDs of all MpDockTab children in the tab_bar
    fn collect_tab_uids(&mut self) {
        self.tab_uids.clear();
        let mut uids = Vec::new();
        let tab_bar_ref = self.view.widget(&[live_id!(tab_bar)]);
        if let Some(mut tab_bar_view) = tab_bar_ref.borrow_mut::<View>() {
            let count = tab_bar_view.child_count();
            for i in 0..count {
                if let Some(child) = tab_bar_view.child_at_index(i) {
                    if child.borrow::<MpDockTab>().is_some() {
                        uids.push(child.widget_uid());
                    }
                }
            }
            drop(tab_bar_view);
        }
        self.tab_uids = uids;
    }

    /// Synchronize tab selected states and content visibility based on active_tab
    fn sync_tab_visibility(&self, cx: &mut Cx) {
        // Update tab selected states
        let tab_bar_ref = self.view.widget(&[live_id!(tab_bar)]);
        if let Some(mut tab_bar_view) = tab_bar_ref.borrow_mut::<View>() {
            let count = tab_bar_view.child_count();
            let mut tab_index = 0usize;
            for i in 0..count {
                if let Some(child) = tab_bar_view.child_at_index(i) {
                    if let Some(mut tab) = child.borrow_mut::<MpDockTab>() {
                        tab.set_selected(cx, tab_index == self.active_tab);
                        tab_index += 1;
                    }
                }
            }
            drop(tab_bar_view);
        }

        // Update content visibility
        let content_ref = self.view.widget(&[live_id!(content)]);
        if let Some(mut content_view) = content_ref.borrow_mut::<View>() {
            let count = content_view.child_count();
            for i in 0..count {
                if let Some(child) = content_view.child_at_index(i) {
                    let visible = i == self.active_tab;
                    child.apply_over(cx, live! { visible: (visible) });
                }
            }
            drop(content_view);
        };
    }

    /// Set the active tab by index
    pub fn set_active_tab(&mut self, cx: &mut Cx, index: usize) {
        if self.active_tab != index {
            self.active_tab = index;
            self.sync_tab_visibility(cx);
            self.redraw(cx);
        }
    }

    /// Get the current active tab index
    pub fn active_tab(&self) -> usize {
        self.active_tab
    }
}

impl Widget for MpDockTabPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Check if any dock tab was clicked by matching against stored UIDs
        for (tab_index, tab_uid) in self.tab_uids.iter().enumerate() {
            if let Some(item) = actions.find_widget_action(*tab_uid) {
                if let MpDockTabAction::Clicked = item.cast() {
                    if self.active_tab != tab_index {
                        self.active_tab = tab_index;
                        self.sync_tab_visibility(cx);
                        self.redraw(cx);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            MpDockTabPanelAction::TabSelected { index: tab_index },
                        );
                    }
                    break;
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpDockTabPanelRef {
    pub fn set_active_tab(&self, cx: &mut Cx, index: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_active_tab(cx, index);
        }
    }

    pub fn active_tab(&self) -> usize {
        if let Some(inner) = self.borrow() {
            inner.active_tab()
        } else {
            0
        }
    }

    pub fn tab_selected(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let MpDockTabPanelAction::TabSelected { index } = item.cast() {
                return Some(index);
            }
        }
        None
    }
}

// ============================================================
// MpDockResizeHandle
// ============================================================

#[derive(Live, LiveHook, Widget)]
pub struct MpDockResizeHandle {
    #[deref] view: View,
    #[animator] animator: Animator,
}

impl Widget for MpDockResizeHandle {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        // Event handling is done by MpDockArea which checks hits on this handle's area
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================
// MpDockArea
// ============================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpDockAreaAction {
    None,
    DockToggled { placement: MpDockPlacement, visible: bool },
    DockResized { placement: MpDockPlacement, size: f64 },
}

#[derive(Live, Widget)]
pub struct MpDockArea {
    #[deref] view: View,

    // Dock sizes
    #[live(240.0)] left_width: f64,
    #[live(240.0)] right_width: f64,
    #[live(200.0)] bottom_height: f64,
    #[live(100.0)] min_dock_size: f64,
    #[live(800.0)] max_dock_size: f64,

    // Visibility state
    #[rust] left_visible: bool,
    #[rust] right_visible: bool,
    #[rust] bottom_visible: bool,

    // Resize drag state
    #[rust] dragging: Option<MpDockPlacement>,
    #[rust] drag_start_pos: f64,
    #[rust] drag_start_size: f64,
}

impl LiveHook for MpDockArea {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.sync_dock_layout(cx);
    }
}

impl MpDockArea {
    fn sync_dock_layout(&mut self, cx: &mut Cx) {
        // Left dock
        self.view.view(ids!(main_row.left_dock)).apply_over(cx, live! {
            visible: (self.left_visible),
            width: (self.left_width),
        });
        self.view.view(ids!(main_row.left_handle)).apply_over(cx, live! {
            visible: (self.left_visible),
        });

        // Right dock
        self.view.view(ids!(main_row.right_dock)).apply_over(cx, live! {
            visible: (self.right_visible),
            width: (self.right_width),
        });
        self.view.view(ids!(main_row.right_handle)).apply_over(cx, live! {
            visible: (self.right_visible),
        });

        // Bottom dock
        self.view.view(ids!(bottom_dock)).apply_over(cx, live! {
            visible: (self.bottom_visible),
            height: (self.bottom_height),
        });
        self.view.view(ids!(bottom_handle)).apply_over(cx, live! {
            visible: (self.bottom_visible),
        });
    }

    pub fn toggle_dock(&mut self, cx: &mut Cx, placement: MpDockPlacement) {
        match placement {
            MpDockPlacement::Left => self.left_visible = !self.left_visible,
            MpDockPlacement::Right => self.right_visible = !self.right_visible,
            MpDockPlacement::Bottom => self.bottom_visible = !self.bottom_visible,
        }
        self.sync_dock_layout(cx);
        self.redraw(cx);
    }

    pub fn set_dock_visible(&mut self, cx: &mut Cx, placement: MpDockPlacement, visible: bool) {
        match placement {
            MpDockPlacement::Left => self.left_visible = visible,
            MpDockPlacement::Right => self.right_visible = visible,
            MpDockPlacement::Bottom => self.bottom_visible = visible,
        }
        self.sync_dock_layout(cx);
        self.redraw(cx);
    }

    pub fn is_dock_visible(&self, placement: MpDockPlacement) -> bool {
        match placement {
            MpDockPlacement::Left => self.left_visible,
            MpDockPlacement::Right => self.right_visible,
            MpDockPlacement::Bottom => self.bottom_visible,
        }
    }

    fn clamp_dock_size(&self, size: f64) -> f64 {
        size.clamp(self.min_dock_size, self.max_dock_size)
    }
}

impl Widget for MpDockArea {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Handle resize drag on left_handle
        let left_handle_area = self.view.view(ids!(main_row.left_handle)).area();
        match event.hits(cx, left_handle_area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::ColResize);
            }
            Hit::FingerDown(fe) => {
                self.dragging = Some(MpDockPlacement::Left);
                self.drag_start_pos = fe.abs.x;
                self.drag_start_size = self.left_width;
                cx.set_cursor(MouseCursor::ColResize);
            }
            Hit::FingerMove(fe) => {
                if matches!(self.dragging, Some(MpDockPlacement::Left)) {
                    let delta = fe.abs.x - self.drag_start_pos;
                    let new_size = self.clamp_dock_size(self.drag_start_size + delta);
                    if (new_size - self.left_width).abs() > 1.0 {
                        self.left_width = new_size;
                        self.sync_dock_layout(cx);
                        self.redraw(cx);
                    }
                }
            }
            Hit::FingerUp(_) => {
                if matches!(self.dragging, Some(MpDockPlacement::Left)) {
                    self.dragging = None;
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        MpDockAreaAction::DockResized {
                            placement: MpDockPlacement::Left,
                            size: self.left_width,
                        },
                    );
                }
            }
            _ => {}
        }

        // Handle resize drag on right_handle (delta is reversed: drag left = bigger)
        let right_handle_area = self.view.view(ids!(main_row.right_handle)).area();
        match event.hits(cx, right_handle_area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::ColResize);
            }
            Hit::FingerDown(fe) => {
                self.dragging = Some(MpDockPlacement::Right);
                self.drag_start_pos = fe.abs.x;
                self.drag_start_size = self.right_width;
                cx.set_cursor(MouseCursor::ColResize);
            }
            Hit::FingerMove(fe) => {
                if matches!(self.dragging, Some(MpDockPlacement::Right)) {
                    let delta = self.drag_start_pos - fe.abs.x;
                    let new_size = self.clamp_dock_size(self.drag_start_size + delta);
                    if (new_size - self.right_width).abs() > 1.0 {
                        self.right_width = new_size;
                        self.sync_dock_layout(cx);
                        self.redraw(cx);
                    }
                }
            }
            Hit::FingerUp(_) => {
                if matches!(self.dragging, Some(MpDockPlacement::Right)) {
                    self.dragging = None;
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        MpDockAreaAction::DockResized {
                            placement: MpDockPlacement::Right,
                            size: self.right_width,
                        },
                    );
                }
            }
            _ => {}
        }

        // Handle resize drag on bottom_handle (vertical: drag up = bigger)
        let bottom_handle_area = self.view.view(ids!(bottom_handle)).area();
        match event.hits(cx, bottom_handle_area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::RowResize);
            }
            Hit::FingerDown(fe) => {
                self.dragging = Some(MpDockPlacement::Bottom);
                self.drag_start_pos = fe.abs.y;
                self.drag_start_size = self.bottom_height;
                cx.set_cursor(MouseCursor::RowResize);
            }
            Hit::FingerMove(fe) => {
                if matches!(self.dragging, Some(MpDockPlacement::Bottom)) {
                    let delta = self.drag_start_pos - fe.abs.y;
                    let new_size = self.clamp_dock_size(self.drag_start_size + delta);
                    if (new_size - self.bottom_height).abs() > 1.0 {
                        self.bottom_height = new_size;
                        self.sync_dock_layout(cx);
                        self.redraw(cx);
                    }
                }
            }
            Hit::FingerUp(_) => {
                if matches!(self.dragging, Some(MpDockPlacement::Bottom)) {
                    self.dragging = None;
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        MpDockAreaAction::DockResized {
                            placement: MpDockPlacement::Bottom,
                            size: self.bottom_height,
                        },
                    );
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpDockAreaRef {
    pub fn toggle_dock(&self, cx: &mut Cx, placement: MpDockPlacement) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.toggle_dock(cx, placement);
        }
    }

    pub fn set_dock_visible(&self, cx: &mut Cx, placement: MpDockPlacement, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_dock_visible(cx, placement, visible);
        }
    }

    pub fn is_dock_visible(&self, placement: MpDockPlacement) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_dock_visible(placement)
        } else {
            false
        }
    }
}

// ============================================================
// MpDockToggle - Toggle button for collapsing/expanding docks
// ============================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpDockToggleAction {
    None,
    Toggled,
}

#[derive(Live, Widget)]
pub struct MpDockToggle {
    #[deref] view: View,
    #[live] placement: MpDockPlacement,
    #[live] expanded: bool,
    #[animator] animator: Animator,
}

impl LiveHook for MpDockToggle {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.update_shader_state(cx);
    }
}

impl MpDockToggle {
    fn update_shader_state(&mut self, cx: &mut Cx) {
        let direction = match self.placement {
            MpDockPlacement::Left => 0.0f64,
            MpDockPlacement::Right => 1.0f64,
            MpDockPlacement::Bottom => 2.0f64,
        };
        let expanded_val = if self.expanded { 1.0f64 } else { 0.0f64 };
        self.view.apply_over(cx, live! {
            draw_bg: {
                direction: (direction),
                expanded: (expanded_val),
            }
        });
    }

    pub fn set_expanded(&mut self, cx: &mut Cx, expanded: bool) {
        self.expanded = expanded;
        self.update_shader_state(cx);
        self.redraw(cx);
    }

    pub fn is_expanded(&self) -> bool {
        self.expanded
    }
}

impl Widget for MpDockToggle {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        match event.hits(cx, self.view.area()) {
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerDown(_) => {
                self.expanded = !self.expanded;
                self.update_shader_state(cx);
                cx.widget_action(self.widget_uid(), &scope.path, MpDockToggleAction::Toggled);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

pub trait MpDockToggleRefExt {
    fn set_expanded(&self, cx: &mut Cx, expanded: bool);
    fn is_expanded(&self) -> bool;
    fn toggled(&self, actions: &Actions) -> bool;
}

impl MpDockToggleRefExt for MpDockToggleRef {
    fn set_expanded(&self, cx: &mut Cx, expanded: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_expanded(cx, expanded);
        }
    }

    fn is_expanded(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_expanded()
        } else {
            false
        }
    }

    fn toggled(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast(), MpDockToggleAction::Toggled)
        } else {
            false
        }
    }
}
