use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Calendar Day Cell
    CalendarDayCell = {{CalendarDayCell}} {
        width: 36.0,
        height: 36.0,
        align: { x: 0.5, y: 0.5 }

        draw_bg: {
            instance radius: 18.0
            instance hover: 0.0
            instance selected: 0.0
            instance in_range: 0.0
            instance is_today: 0.0
            instance disabled: 0.0
            instance color: (TRANSPARENT)
            instance color_hover: (SECONDARY)
            instance color_selected: (PRIMARY)
            instance color_in_range: (SECONDARY)
            instance color_today_border: (PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let center = self.rect_size * 0.5;

                sdf.circle(center.x, center.y, self.radius);

                let bg_color = mix(self.color, self.color_hover, self.hover);
                let bg_color = mix(bg_color, self.color_in_range, self.in_range);
                let bg_color = mix(bg_color, self.color_selected, self.selected);

                sdf.fill_keep(bg_color);

                if self.is_today > 0.5 && self.selected < 0.5 {
                    sdf.stroke(self.color_today_border, 1.5);
                }

                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 13.0 }
            instance color: (FOREGROUND)
            instance color_disabled: (MUTED_FOREGROUND)
            instance color_selected: (PRIMARY_FOREGROUND)
            instance disabled: 0.0
            instance selected: 0.0

            fn get_color(self) -> vec4 {
                if self.selected > 0.5 {
                    return self.color_selected;
                }
                if self.disabled > 0.5 {
                    return self.color_disabled;
                }
                return self.color;
            }
        }

        text: ""

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
        }
    }

    // Calendar Week Header
    CalendarWeekHeader = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 4.0,
        align: { x: 0.5, y: 0.5 }
    }

    CalendarWeekLabel = <Label> {
        width: 36.0,
        height: 24.0,
        align: { x: 0.5, y: 0.5 }
        draw_text: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
            color: (MUTED_FOREGROUND)
        }
    }

    // Calendar Navigation Button
    CalendarNavButton = {{CalendarNavButton}} {
        width: 32.0,
        height: 32.0,
        align: { x: 0.5, y: 0.5 }

        draw_bg: {
            instance radius: 6.0
            instance hover: 0.0
            instance color: (TRANSPARENT)
            instance color_hover: (SECONDARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.radius
                );
                let bg_color = mix(self.color, self.color_hover, self.hover);
                sdf.fill(bg_color);
                return sdf.result;
            }
        }

        draw_icon: {
            instance color: (FOREGROUND)
            fn get_color(self) -> vec4 {
                return self.color;
            }
        }

        icon_walk: { width: 16.0, height: 16.0 }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
        }
    }

    // Month/Year Selector Button
    CalendarSelectorButton = {{CalendarSelectorButton}} {
        width: Fit,
        height: 32.0,
        padding: { left: 8.0, right: 8.0 }
        align: { x: 0.5, y: 0.5 }

        draw_bg: {
            instance radius: 6.0
            instance hover: 0.0
            instance selected: 0.0
            instance color: (TRANSPARENT)
            instance color_hover: (SECONDARY)
            instance color_selected: (SECONDARY_ACTIVE)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.radius
                );
                let bg_color = mix(self.color, self.color_hover, self.hover);
                let bg_color = mix(bg_color, self.color_selected, self.selected);
                sdf.fill(bg_color);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_BOLD>{ font_size: 14.0 }
            color: (FOREGROUND)
        }

        text: ""

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
        }
    }

    // Month Grid Item
    CalendarMonthItem = {{CalendarMonthItem}} {
        width: 70.0,
        height: 36.0,
        align: { x: 0.5, y: 0.5 }

        draw_bg: {
            instance radius: 6.0
            instance hover: 0.0
            instance selected: 0.0
            instance color: (TRANSPARENT)
            instance color_hover: (SECONDARY)
            instance color_selected: (PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.radius
                );
                let bg_color = mix(self.color, self.color_hover, self.hover);
                let bg_color = mix(bg_color, self.color_selected, self.selected);
                sdf.fill(bg_color);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 13.0 }
            instance color: (FOREGROUND)
            instance color_selected: (PRIMARY_FOREGROUND)
            instance selected: 0.0

            fn get_color(self) -> vec4 {
                if self.selected > 0.5 {
                    return self.color_selected;
                }
                return self.color;
            }
        }

        text: ""

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
        }
    }

    // Year Grid Item
    CalendarYearItem = {{CalendarYearItem}} {
        width: 56.0,
        height: 36.0,
        align: { x: 0.5, y: 0.5 }

        draw_bg: {
            instance radius: 6.0
            instance hover: 0.0
            instance selected: 0.0
            instance color: (TRANSPARENT)
            instance color_hover: (SECONDARY)
            instance color_selected: (PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.radius
                );
                let bg_color = mix(self.color, self.color_hover, self.hover);
                let bg_color = mix(bg_color, self.color_selected, self.selected);
                sdf.fill(bg_color);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 13.0 }
            instance color: (FOREGROUND)
            instance color_selected: (PRIMARY_FOREGROUND)
            instance selected: 0.0

            fn get_color(self) -> vec4 {
                if self.selected > 0.5 {
                    return self.color_selected;
                }
                return self.color;
            }
        }

        text: ""

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
        }
    }

    // Main Calendar Component
    pub MpCalendar = {{MpCalendar}} {
        width: Fit,
        height: Fit,
        flow: Down,
        padding: 16.0,
        spacing: 8.0,

        draw_bg: {
            instance radius: 8.0
            instance border_width: 1.0
            instance border_color: (BORDER)
            instance color: (CARD)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    self.radius
                );
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, self.border_width);
                return sdf.result;
            }
        }

        show_bg: true
    }
}

// ============================================================================
// Date Types
// ============================================================================

/// Represents a calendar date selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarDate {
    /// Single date selection
    Single(Option<(i32, u32, u32)>), // (year, month, day)
    /// Date range selection
    Range(Option<(i32, u32, u32)>, Option<(i32, u32, u32)>),
}

impl Default for CalendarDate {
    fn default() -> Self {
        Self::Single(None)
    }
}

impl CalendarDate {
    /// Create a single date
    pub fn single(year: i32, month: u32, day: u32) -> Self {
        Self::Single(Some((year, month, day)))
    }

    /// Create a date range
    pub fn range(start: (i32, u32, u32), end: (i32, u32, u32)) -> Self {
        Self::Range(Some(start), Some(end))
    }

    /// Check if date is set
    pub fn is_some(&self) -> bool {
        match self {
            Self::Single(Some(_)) | Self::Range(Some(_), _) => true,
            _ => false,
        }
    }

    /// Check if date selection is complete
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Single(Some(_)) => true,
            Self::Range(Some(_), Some(_)) => true,
            _ => false,
        }
    }

    /// Get start date
    pub fn start(&self) -> Option<(i32, u32, u32)> {
        match self {
            Self::Single(d) => *d,
            Self::Range(start, _) => *start,
        }
    }

    /// Get end date (for range)
    pub fn end(&self) -> Option<(i32, u32, u32)> {
        match self {
            Self::Range(_, end) => *end,
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }

    #[allow(dead_code)]
    fn is_active(&self, year: i32, month: u32, day: u32) -> bool {
        let date = (year, month, day);
        match self {
            Self::Single(d) => *d == Some(date),
            Self::Range(start, end) => *start == Some(date) || *end == Some(date),
        }
    }

    #[allow(dead_code)]
    fn is_in_range(&self, year: i32, month: u32, day: u32) -> bool {
        match self {
            Self::Range(Some(start), Some(end)) => {
                let date = (year, month, day);
                date > *start && date < *end
            }
            _ => false,
        }
    }
}

// ============================================================================
// View Mode
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalendarViewMode {
    #[default]
    Day,
    Month,
    Year,
}

// ============================================================================
// Date Utilities
// ============================================================================

#[allow(dead_code)]
fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[allow(dead_code)]
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 30,
    }
}

#[allow(dead_code)]
fn weekday_of_first(year: i32, month: u32) -> u32 {
    // Zeller's congruence for Gregorian calendar
    let m = if month < 3 { month as i32 + 12 } else { month as i32 };
    let y = if month < 3 { year - 1 } else { year };
    let q: i32 = 1; // first day
    let k = y % 100;
    let j = y / 100;

    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
    // Convert to Sunday = 0
    ((h + 6) % 7) as u32
}

/// Get current date (year, month, day)
fn get_today() -> (i32, u32, u32) {
    // Use a simple approach - in real app you'd use chrono or similar
    // For now, default to a reasonable date
    (2026, 2, 4)
}

#[allow(dead_code)]
fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

fn month_name_short(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
}

#[allow(dead_code)]
const WEEK_DAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

// ============================================================================
// Calendar Day Cell Widget
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct CalendarDayCell {
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
    area: Area,

    // Cell state
    #[rust]
    pub day: u32,
    #[rust]
    pub month: u32,
    #[rust]
    pub year: i32,
    #[rust]
    pub is_current_month: bool,
    #[rust]
    pub is_selected: bool,
    #[rust]
    pub is_in_range: bool,
    #[rust]
    pub is_today: bool,
    #[rust]
    pub is_disabled: bool,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum CalendarDayCellAction {
    Clicked(i32, u32, u32), // year, month, day
    None,
}

impl Widget for CalendarDayCell {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        if self.is_disabled {
            return;
        }

        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.widget_action(
                        uid,
                        &scope.path,
                        CalendarDayCellAction::Clicked(self.year, self.month, self.day),
                    );
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // Update shader uniforms
        self.draw_bg.apply_over(cx, live! {
            selected: (if self.is_selected { 1.0 } else { 0.0 }),
            in_range: (if self.is_in_range { 1.0 } else { 0.0 }),
            is_today: (if self.is_today { 1.0 } else { 0.0 }),
            disabled: (if self.is_disabled || !self.is_current_month { 1.0 } else { 0.0 }),
        });

        self.draw_text.apply_over(cx, live! {
            selected: (if self.is_selected { 1.0 } else { 0.0 }),
            disabled: (if self.is_disabled || !self.is_current_month { 1.0 } else { 0.0 }),
        });

        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

impl CalendarDayCell {
    pub fn set_day(&mut self, year: i32, month: u32, day: u32, is_current_month: bool) {
        self.year = year;
        self.month = month;
        self.day = day;
        self.is_current_month = is_current_month;
        self.text.as_mut_empty().push_str(&day.to_string());
    }
}

// ============================================================================
// Calendar Navigation Button Widget
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct CalendarNavButton {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_icon: DrawIcon,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[live]
    icon_walk: Walk,

    #[animator]
    animator: Animator,

    #[rust]
    area: Area,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum CalendarNavButtonAction {
    Clicked,
    None,
}

impl Widget for CalendarNavButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.widget_action(uid, &scope.path, CalendarNavButtonAction::Clicked);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_icon.draw_walk(cx, self.icon_walk);
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

// ============================================================================
// Calendar Selector Button Widget
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct CalendarSelectorButton {
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
    area: Area,

    #[rust]
    pub is_selected: bool,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum CalendarSelectorButtonAction {
    Clicked,
    None,
}

impl Widget for CalendarSelectorButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.widget_action(uid, &scope.path, CalendarSelectorButtonAction::Clicked);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.apply_over(cx, live! {
            selected: (if self.is_selected { 1.0 } else { 0.0 }),
        });

        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

impl CalendarSelectorButton {
    pub fn set_text(&mut self, text: &str) {
        self.text.as_mut_empty().push_str(text);
    }
}

// ============================================================================
// Calendar Month Item Widget
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct CalendarMonthItem {
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
    area: Area,

    #[rust]
    pub month: u32,
    #[rust]
    pub is_selected: bool,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum CalendarMonthItemAction {
    Clicked(u32), // month
    None,
}

impl Widget for CalendarMonthItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.widget_action(uid, &scope.path, CalendarMonthItemAction::Clicked(self.month));
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.apply_over(cx, live! {
            selected: (if self.is_selected { 1.0 } else { 0.0 }),
        });
        self.draw_text.apply_over(cx, live! {
            selected: (if self.is_selected { 1.0 } else { 0.0 }),
        });

        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

impl CalendarMonthItem {
    pub fn set_month(&mut self, month: u32) {
        self.month = month;
        self.text.as_mut_empty().push_str(month_name_short(month));
    }
}

// ============================================================================
// Calendar Year Item Widget
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct CalendarYearItem {
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
    area: Area,

    #[rust]
    pub year: i32,
    #[rust]
    pub is_selected: bool,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum CalendarYearItemAction {
    Clicked(i32), // year
    None,
}

impl Widget for CalendarYearItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.widget_action(uid, &scope.path, CalendarYearItemAction::Clicked(self.year));
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.apply_over(cx, live! {
            selected: (if self.is_selected { 1.0 } else { 0.0 }),
        });
        self.draw_text.apply_over(cx, live! {
            selected: (if self.is_selected { 1.0 } else { 0.0 }),
        });

        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

impl CalendarYearItem {
    pub fn set_year(&mut self, year: i32) {
        self.year = year;
        self.text.as_mut_empty().push_str(&year.to_string());
    }
}

// ============================================================================
// Main Calendar Widget
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct MpCalendar {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[rust]
    area: Area,

    // Calendar state
    #[rust]
    view_mode: CalendarViewMode,
    #[rust]
    current_year: i32,
    #[rust]
    current_month: u32,
    #[rust]
    selected_date: CalendarDate,
    #[rust]
    today: (i32, u32, u32),
    #[rust]
    year_page: i32,
    #[rust]
    year_range_start: i32,
    #[rust]
    year_range_end: i32,

    // Selection mode
    #[rust]
    range_mode: bool,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpCalendarAction {
    DateSelected(CalendarDate),
    None,
}

impl Widget for MpCalendar {
    fn handle_event(&mut self, cx: &mut Cx, _event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        // Handle day cell clicks
        for action in cx.capture_actions(|_cx| {
            // Process child widget actions
        }) {
            // Handle CalendarDayCellAction
            if let CalendarDayCellAction::Clicked(year, month, day) = action.as_widget_action().cast() {
                self.handle_day_click(year, month, day);
                cx.widget_action(uid, &scope.path, MpCalendarAction::DateSelected(self.selected_date));
                self.redraw(cx);
            }
            // Handle CalendarMonthItemAction
            if let CalendarMonthItemAction::Clicked(month) = action.as_widget_action().cast() {
                self.current_month = month;
                self.view_mode = CalendarViewMode::Day;
                self.redraw(cx);
            }
            // Handle CalendarYearItemAction
            if let CalendarYearItemAction::Clicked(year) = action.as_widget_action().cast() {
                self.current_year = year;
                self.view_mode = CalendarViewMode::Day;
                self.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Initialize if needed
        if self.current_year == 0 {
            self.initialize();
        }

        self.draw_bg.begin(cx, walk, self.layout);

        // Draw header
        self.draw_header(cx, scope);

        // Draw content based on view mode
        match self.view_mode {
            CalendarViewMode::Day => self.draw_days_view(cx, scope),
            CalendarViewMode::Month => self.draw_months_view(cx, scope),
            CalendarViewMode::Year => self.draw_years_view(cx, scope),
        }

        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

impl MpCalendar {
    fn initialize(&mut self) {
        self.today = get_today();
        self.current_year = self.today.0;
        self.current_month = self.today.1;
        self.year_range_start = self.today.0 - 50;
        self.year_range_end = self.today.0 + 50;
        self.year_page = (self.current_year - self.year_range_start) / 20;
    }

    fn handle_day_click(&mut self, year: i32, month: u32, day: u32) {
        if self.range_mode {
            match self.selected_date {
                CalendarDate::Range(None, None) | CalendarDate::Single(_) | CalendarDate::Range(None, Some(_)) => {
                    self.selected_date = CalendarDate::Range(Some((year, month, day)), None);
                }
                CalendarDate::Range(Some(start), None) => {
                    let date = (year, month, day);
                    if date < start {
                        self.selected_date = CalendarDate::Range(Some(date), None);
                    } else {
                        self.selected_date = CalendarDate::Range(Some(start), Some(date));
                    }
                }
                CalendarDate::Range(Some(_), Some(_)) => {
                    self.selected_date = CalendarDate::Range(Some((year, month, day)), None);
                }
            }
        } else {
            self.selected_date = CalendarDate::Single(Some((year, month, day)));
        }
    }

    #[allow(dead_code)]
    fn prev_month(&mut self) {
        if self.current_month == 1 {
            self.current_month = 12;
            self.current_year -= 1;
        } else {
            self.current_month -= 1;
        }
    }

    #[allow(dead_code)]
    fn next_month(&mut self) {
        if self.current_month == 12 {
            self.current_month = 1;
            self.current_year += 1;
        } else {
            self.current_month += 1;
        }
    }

    #[allow(dead_code)]
    fn prev_year_page(&mut self) {
        if self.year_page > 0 {
            self.year_page -= 1;
        }
    }

    #[allow(dead_code)]

    fn next_year_page(&mut self) {
        let max_page = (self.year_range_end - self.year_range_start) / 20;
        if self.year_page < max_page {
            self.year_page += 1;
        }
    }

    fn draw_header(&mut self, _cx: &mut Cx2d, _scope: &mut Scope) {
        // Header is drawn via DSL layout
    }

    fn draw_days_view(&mut self, _cx: &mut Cx2d, _scope: &mut Scope) {
        // Days view is drawn via DSL layout
    }

    fn draw_months_view(&mut self, _cx: &mut Cx2d, _scope: &mut Scope) {
        // Months view is drawn via DSL layout
    }

    fn draw_years_view(&mut self, _cx: &mut Cx2d, _scope: &mut Scope) {
        // Years view is drawn via DSL layout
    }

    // Public API
    pub fn set_date(&mut self, date: CalendarDate) {
        self.selected_date = date;
        if let Some((year, month, _)) = date.start() {
            self.current_year = year;
            self.current_month = month;
        }
    }

    pub fn date(&self) -> CalendarDate {
        self.selected_date
    }

    pub fn set_range_mode(&mut self, enabled: bool) {
        self.range_mode = enabled;
        if enabled {
            self.selected_date = CalendarDate::Range(None, None);
        } else {
            self.selected_date = CalendarDate::Single(None);
        }
    }

    pub fn set_view_mode(&mut self, mode: CalendarViewMode) {
        self.view_mode = mode;
    }
}

impl MpCalendarRef {
    pub fn set_date(&self, date: CalendarDate) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_date(date);
        }
    }

    pub fn date(&self) -> CalendarDate {
        if let Some(inner) = self.borrow() {
            inner.date()
        } else {
            CalendarDate::default()
        }
    }

    pub fn set_range_mode(&self, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_range_mode(enabled);
        }
    }
}
