use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Week day label
    CalendarWeekLabel = <View> {
        width: 36.0,
        height: 28.0,
        align: { x: 0.5, y: 0.5 }

        label = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                color: (MUTED_FOREGROUND)
            }
        }
    }

    // Day cell button
    CalendarDayButton = <View> {
        width: 36.0,
        height: 36.0,
        align: { x: 0.5, y: 0.5 }

        show_bg: true
        draw_bg: {
            instance radius: 18.0
            instance hover: 0.0
            instance selected: 0.0
            instance is_today: 0.0
            instance is_other_month: 0.0
            instance color: (TRANSPARENT)
            instance color_hover: (SECONDARY)
            instance color_selected: (PRIMARY)
            instance color_today_border: (PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let center = self.rect_size * 0.5;

                sdf.circle(center.x, center.y, self.radius - 1.0);

                let bg_color = mix(self.color, self.color_hover, self.hover);
                let bg_color = mix(bg_color, self.color_selected, self.selected);

                sdf.fill_keep(bg_color);

                if self.is_today > 0.5 && self.selected < 0.5 {
                    sdf.stroke(self.color_today_border, 1.5);
                }

                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{ font_size: 13.0 }
                color: (FOREGROUND)
            }
        }
    }

    // Navigation button
    CalendarNavButton = <View> {
        width: 32.0,
        height: 32.0,
        align: { x: 0.5, y: 0.5 }
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance radius: 6.0
            instance hover: 0.0
            instance color: (TRANSPARENT)
            instance color_hover: (SECONDARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.radius);
                let bg_color = mix(self.color, self.color_hover, self.hover);
                sdf.fill(bg_color);
                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                text_style: <THEME_FONT_BOLD>{ font_size: 16.0 }
                color: (FOREGROUND)
            }
        }
    }

    // Month/Year selector button
    CalendarSelectorButton = <View> {
        width: Fit,
        height: 32.0,
        padding: { left: 8.0, right: 8.0 }
        align: { x: 0.5, y: 0.5 }
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance radius: 6.0
            instance hover: 0.0
            instance color: (TRANSPARENT)
            instance color_hover: (SECONDARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.radius);
                let bg_color = mix(self.color, self.color_hover, self.hover);
                sdf.fill(bg_color);
                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                text_style: <THEME_FONT_BOLD>{ font_size: 14.0 }
                color: (FOREGROUND)
            }
        }
    }

    // Month item for month view
    CalendarMonthButton = <View> {
        width: 70.0,
        height: 36.0,
        align: { x: 0.5, y: 0.5 }
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance radius: 6.0
            instance hover: 0.0
            instance selected: 0.0
            instance color: (TRANSPARENT)
            instance color_hover: (SECONDARY)
            instance color_selected: (PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.radius);
                let bg_color = mix(self.color, self.color_hover, self.hover);
                let bg_color = mix(bg_color, self.color_selected, self.selected);
                sdf.fill(bg_color);
                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{ font_size: 13.0 }
                color: (FOREGROUND)
            }
        }
    }

    // Year item for year view
    CalendarYearButton = <View> {
        width: 56.0,
        height: 36.0,
        align: { x: 0.5, y: 0.5 }
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance radius: 6.0
            instance hover: 0.0
            instance selected: 0.0
            instance color: (TRANSPARENT)
            instance color_hover: (SECONDARY)
            instance color_selected: (PRIMARY)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.radius);
                let bg_color = mix(self.color, self.color_hover, self.hover);
                let bg_color = mix(bg_color, self.color_selected, self.selected);
                sdf.fill(bg_color);
                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{ font_size: 13.0 }
                color: (FOREGROUND)
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

        show_bg: true
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

        // Header with navigation
        header = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8.0,
            align: { x: 0.5, y: 0.5 }

            prev_btn = <CalendarNavButton> {
                label = { text: "<" }
            }

            <View> { width: Fill, height: 1.0 }

            month_btn = <CalendarSelectorButton> {
                label = { text: "February" }
            }

            year_btn = <CalendarSelectorButton> {
                label = { text: "2026" }
            }

            <View> { width: Fill, height: 1.0 }

            next_btn = <CalendarNavButton> {
                label = { text: ">" }
            }
        }

        // Days view container
        days_view = <View> {
            width: Fit,
            height: Fit,
            flow: Down,
            spacing: 4.0,
            visible: true

            // Week header row
            week_header = <View> {
                width: Fit,
                height: Fit,
                flow: Right,
                spacing: 4.0,

                sun = <CalendarWeekLabel> { label = { text: "Su" } }
                mon = <CalendarWeekLabel> { label = { text: "Mo" } }
                tue = <CalendarWeekLabel> { label = { text: "Tu" } }
                wed = <CalendarWeekLabel> { label = { text: "We" } }
                thu = <CalendarWeekLabel> { label = { text: "Th" } }
                fri = <CalendarWeekLabel> { label = { text: "Fr" } }
                sat = <CalendarWeekLabel> { label = { text: "Sa" } }
            }

            // Week rows (5 weeks)
            week0 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 4.0
                d0 = <CalendarDayButton> {} d1 = <CalendarDayButton> {} d2 = <CalendarDayButton> {}
                d3 = <CalendarDayButton> {} d4 = <CalendarDayButton> {} d5 = <CalendarDayButton> {}
                d6 = <CalendarDayButton> {}
            }
            week1 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 4.0
                d0 = <CalendarDayButton> {} d1 = <CalendarDayButton> {} d2 = <CalendarDayButton> {}
                d3 = <CalendarDayButton> {} d4 = <CalendarDayButton> {} d5 = <CalendarDayButton> {}
                d6 = <CalendarDayButton> {}
            }
            week2 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 4.0
                d0 = <CalendarDayButton> {} d1 = <CalendarDayButton> {} d2 = <CalendarDayButton> {}
                d3 = <CalendarDayButton> {} d4 = <CalendarDayButton> {} d5 = <CalendarDayButton> {}
                d6 = <CalendarDayButton> {}
            }
            week3 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 4.0
                d0 = <CalendarDayButton> {} d1 = <CalendarDayButton> {} d2 = <CalendarDayButton> {}
                d3 = <CalendarDayButton> {} d4 = <CalendarDayButton> {} d5 = <CalendarDayButton> {}
                d6 = <CalendarDayButton> {}
            }
            week4 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 4.0
                d0 = <CalendarDayButton> {} d1 = <CalendarDayButton> {} d2 = <CalendarDayButton> {}
                d3 = <CalendarDayButton> {} d4 = <CalendarDayButton> {} d5 = <CalendarDayButton> {}
                d6 = <CalendarDayButton> {}
            }
        }

        // Months view container
        months_view = <View> {
            width: Fit,
            height: Fit,
            flow: Down,
            spacing: 8.0,
            visible: false

            row0 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 8.0
                m0 = <CalendarMonthButton> { label = { text: "Jan" } }
                m1 = <CalendarMonthButton> { label = { text: "Feb" } }
                m2 = <CalendarMonthButton> { label = { text: "Mar" } }
            }
            row1 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 8.0
                m3 = <CalendarMonthButton> { label = { text: "Apr" } }
                m4 = <CalendarMonthButton> { label = { text: "May" } }
                m5 = <CalendarMonthButton> { label = { text: "Jun" } }
            }
            row2 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 8.0
                m6 = <CalendarMonthButton> { label = { text: "Jul" } }
                m7 = <CalendarMonthButton> { label = { text: "Aug" } }
                m8 = <CalendarMonthButton> { label = { text: "Sep" } }
            }
            row3 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 8.0
                m9 = <CalendarMonthButton> { label = { text: "Oct" } }
                m10 = <CalendarMonthButton> { label = { text: "Nov" } }
                m11 = <CalendarMonthButton> { label = { text: "Dec" } }
            }
        }

        // Years view container
        years_view = <View> {
            width: Fit,
            height: Fit,
            flow: Down,
            spacing: 8.0,
            visible: false

            row0 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 8.0
                y0 = <CalendarYearButton> {} y1 = <CalendarYearButton> {}
                y2 = <CalendarYearButton> {} y3 = <CalendarYearButton> {}
            }
            row1 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 8.0
                y4 = <CalendarYearButton> {} y5 = <CalendarYearButton> {}
                y6 = <CalendarYearButton> {} y7 = <CalendarYearButton> {}
            }
            row2 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 8.0
                y8 = <CalendarYearButton> {} y9 = <CalendarYearButton> {}
                y10 = <CalendarYearButton> {} y11 = <CalendarYearButton> {}
            }
            row3 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 8.0
                y12 = <CalendarYearButton> {} y13 = <CalendarYearButton> {}
                y14 = <CalendarYearButton> {} y15 = <CalendarYearButton> {}
            }
            row4 = <View> {
                width: Fit, height: Fit, flow: Right, spacing: 8.0
                y16 = <CalendarYearButton> {} y17 = <CalendarYearButton> {}
                y18 = <CalendarYearButton> {} y19 = <CalendarYearButton> {}
            }
        }
    }
}

// ============================================================================
// Date Types and Utilities
// ============================================================================

/// Represents a calendar date selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl CalendarDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    pub fn is_valid(&self) -> bool {
        self.year > 0 && self.month >= 1 && self.month <= 12 && self.day >= 1 && self.day <= 31
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalendarViewMode {
    #[default]
    Day,
    Month,
    Year,
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 30,
    }
}

fn weekday_of_first(year: i32, month: u32) -> u32 {
    // Zeller's congruence for Gregorian calendar
    let m = if month < 3 { month as i32 + 12 } else { month as i32 };
    let y = if month < 3 { year - 1 } else { year };
    let q: i32 = 1;
    let k = y % 100;
    let j = y / 100;

    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
    ((h + 6) % 7) as u32
}

fn get_today() -> (i32, u32, u32) {
    (2026, 2, 4)
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January", 2 => "February", 3 => "March", 4 => "April",
        5 => "May", 6 => "June", 7 => "July", 8 => "August",
        9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "",
    }
}

// ============================================================================
// Main Calendar Widget
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct MpCalendar {
    #[deref]
    view: View,

    #[rust]
    initialized: bool,
    #[rust]
    view_mode: CalendarViewMode,
    #[rust]
    current_year: i32,
    #[rust]
    current_month: u32,
    #[rust]
    selected_date: Option<CalendarDate>,
    #[rust]
    today: (i32, u32, u32),
    #[rust]
    year_page: i32,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpCalendarAction {
    DateSelected(CalendarDate),
    MonthChanged(i32, u32),
    None,
}

impl Widget for MpCalendar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.initialized {
            self.initialize();
        }

        // Update the UI before drawing
        self.update_header(cx);
        self.update_days_view(cx);
        self.update_months_view(cx);
        self.update_years_view(cx);

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for MpCalendar {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let uid = self.widget_uid();

        // Handle prev button
        if self.view.view(ids!(header.prev_btn)).finger_up(actions).is_some() {
            match self.view_mode {
                CalendarViewMode::Day => self.prev_month(),
                CalendarViewMode::Year => self.prev_year_page(),
                _ => {}
            }
            self.view.redraw(cx);
        }

        // Handle next button
        if self.view.view(ids!(header.next_btn)).finger_up(actions).is_some() {
            match self.view_mode {
                CalendarViewMode::Day => self.next_month(),
                CalendarViewMode::Year => self.next_year_page(),
                _ => {}
            }
            self.view.redraw(cx);
        }

        // Handle month button click
        if self.view.view(ids!(header.month_btn)).finger_up(actions).is_some() {
            self.view_mode = if self.view_mode == CalendarViewMode::Month {
                CalendarViewMode::Day
            } else {
                CalendarViewMode::Month
            };
            self.update_view_visibility(cx);
            self.view.redraw(cx);
        }

        // Handle year button click
        if self.view.view(ids!(header.year_btn)).finger_up(actions).is_some() {
            self.view_mode = if self.view_mode == CalendarViewMode::Year {
                CalendarViewMode::Day
            } else {
                CalendarViewMode::Year
            };
            self.update_view_visibility(cx);
            self.view.redraw(cx);
        }

        // Handle day clicks
        for week in 0..5u32 {
            for day in 0..7u32 {
                let day_ids: &[LiveId] = &[
                    LiveId::from_str("days_view"),
                    LiveId::from_str(&format!("week{}", week)),
                    LiveId::from_str(&format!("d{}", day)),
                ];
                if self.view.view(day_ids).finger_up(actions).is_some() {
                    if let Some(date) = self.get_day_date(week, day) {
                        self.selected_date = Some(date);
                        cx.widget_action(uid, &scope.path, MpCalendarAction::DateSelected(date));
                        self.view.redraw(cx);
                    }
                }
            }
        }

        // Handle month item clicks
        for m in 0..12u32 {
            let month_ids: &[LiveId] = &[
                LiveId::from_str("months_view"),
                LiveId::from_str(&format!("row{}", m / 3)),
                LiveId::from_str(&format!("m{}", m)),
            ];
            if self.view.view(month_ids).finger_up(actions).is_some() {
                self.current_month = m + 1;
                self.view_mode = CalendarViewMode::Day;
                self.update_view_visibility(cx);
                self.view.redraw(cx);
            }
        }

        // Handle year item clicks
        let start_year = self.today.0 - 10 + self.year_page * 20;
        for y in 0..20u32 {
            let year_ids: &[LiveId] = &[
                LiveId::from_str("years_view"),
                LiveId::from_str(&format!("row{}", y / 4)),
                LiveId::from_str(&format!("y{}", y)),
            ];
            if self.view.view(year_ids).finger_up(actions).is_some() {
                self.current_year = start_year + y as i32;
                self.view_mode = CalendarViewMode::Day;
                self.update_view_visibility(cx);
                self.view.redraw(cx);
            }
        }
    }
}

impl MpCalendar {
    fn initialize(&mut self) {
        self.today = get_today();
        self.current_year = self.today.0;
        self.current_month = self.today.1;
        self.year_page = 0;
        self.initialized = true;
    }

    fn prev_month(&mut self) {
        if self.current_month == 1 {
            self.current_month = 12;
            self.current_year -= 1;
        } else {
            self.current_month -= 1;
        }
    }

    fn next_month(&mut self) {
        if self.current_month == 12 {
            self.current_month = 1;
            self.current_year += 1;
        } else {
            self.current_month += 1;
        }
    }

    fn prev_year_page(&mut self) {
        if self.year_page > -5 {
            self.year_page -= 1;
        }
    }

    fn next_year_page(&mut self) {
        if self.year_page < 5 {
            self.year_page += 1;
        }
    }

    fn update_view_visibility(&mut self, cx: &mut Cx) {
        self.view.view(ids!(days_view)).set_visible(cx, self.view_mode == CalendarViewMode::Day);
        self.view.view(ids!(months_view)).set_visible(cx, self.view_mode == CalendarViewMode::Month);
        self.view.view(ids!(years_view)).set_visible(cx, self.view_mode == CalendarViewMode::Year);
        self.view.redraw(cx);
    }

    fn update_header(&mut self, cx: &mut Cx) {
        let month_text = month_name(self.current_month);
        let year_text = self.current_year.to_string();

        self.view.label(ids!(header.month_btn.label)).set_text(cx, month_text);
        self.view.label(ids!(header.year_btn.label)).set_text(cx, &year_text);
    }

    fn update_days_view(&mut self, cx: &mut Cx) {
        let first_weekday = weekday_of_first(self.current_year, self.current_month);
        let days_count = days_in_month(self.current_year, self.current_month);

        let (prev_year, prev_month) = if self.current_month == 1 {
            (self.current_year - 1, 12)
        } else {
            (self.current_year, self.current_month - 1)
        };
        let prev_month_days = days_in_month(prev_year, prev_month);

        for week in 0..5 {
            for weekday in 0..7 {
                let day_index = week * 7 + weekday;
                let (year, month, day, is_current_month) = if day_index < first_weekday {
                    let d = prev_month_days - (first_weekday - day_index - 1);
                    (prev_year, prev_month, d, false)
                } else if day_index - first_weekday >= days_count {
                    let d = day_index - first_weekday - days_count + 1;
                    let (ny, nm) = if self.current_month == 12 {
                        (self.current_year + 1, 1)
                    } else {
                        (self.current_year, self.current_month + 1)
                    };
                    (ny, nm, d, false)
                } else {
                    let d = day_index - first_weekday + 1;
                    (self.current_year, self.current_month, d, true)
                };

                let day_ids: &[LiveId] = &[
                    LiveId::from_str("days_view"),
                    LiveId::from_str(&format!("week{}", week)),
                    LiveId::from_str(&format!("d{}", weekday)),
                ];
                let label_ids: &[LiveId] = &[
                    LiveId::from_str("days_view"),
                    LiveId::from_str(&format!("week{}", week)),
                    LiveId::from_str(&format!("d{}", weekday)),
                    LiveId::from_str("label"),
                ];

                self.view.label(label_ids).set_text(cx, &day.to_string());

                let is_today = (year, month, day) == self.today;
                let is_selected = self.selected_date.map_or(false, |d| {
                    d.year == year && d.month == month && d.day == day
                });

                // Update visual state
                self.view.view(day_ids).apply_over(cx, live! {
                    draw_bg: {
                        selected: (if is_selected { 1.0 } else { 0.0 }),
                        is_today: (if is_today { 1.0 } else { 0.0 }),
                        is_other_month: (if !is_current_month { 1.0 } else { 0.0 }),
                    }
                });
            }
        }
    }

    fn update_months_view(&mut self, cx: &mut Cx) {
        for m in 0..12u32 {
            let month_ids: &[LiveId] = &[
                LiveId::from_str("months_view"),
                LiveId::from_str(&format!("row{}", m / 3)),
                LiveId::from_str(&format!("m{}", m)),
            ];
            let is_selected = m + 1 == self.current_month;

            self.view.view(month_ids).apply_over(cx, live! {
                draw_bg: {
                    selected: (if is_selected { 1.0 } else { 0.0 }),
                }
            });
        }
    }

    fn update_years_view(&mut self, cx: &mut Cx) {
        let start_year = self.today.0 - 10 + self.year_page * 20;

        for y in 0..20u32 {
            let year = start_year + y as i32;
            let year_ids: &[LiveId] = &[
                LiveId::from_str("years_view"),
                LiveId::from_str(&format!("row{}", y / 4)),
                LiveId::from_str(&format!("y{}", y)),
            ];
            let label_ids: &[LiveId] = &[
                LiveId::from_str("years_view"),
                LiveId::from_str(&format!("row{}", y / 4)),
                LiveId::from_str(&format!("y{}", y)),
                LiveId::from_str("label"),
            ];

            self.view.label(label_ids).set_text(cx, &year.to_string());

            let is_selected = year == self.current_year;
            self.view.view(year_ids).apply_over(cx, live! {
                draw_bg: {
                    selected: (if is_selected { 1.0 } else { 0.0 }),
                }
            });
        }
    }

    fn get_day_date(&self, week: u32, weekday: u32) -> Option<CalendarDate> {
        let first_weekday = weekday_of_first(self.current_year, self.current_month);
        let days_count = days_in_month(self.current_year, self.current_month);

        let (prev_year, prev_month) = if self.current_month == 1 {
            (self.current_year - 1, 12)
        } else {
            (self.current_year, self.current_month - 1)
        };
        let prev_month_days = days_in_month(prev_year, prev_month);

        let day_index = week * 7 + weekday;
        let (year, month, day) = if day_index < first_weekday {
            let d = prev_month_days - (first_weekday - day_index - 1);
            (prev_year, prev_month, d)
        } else if day_index - first_weekday >= days_count {
            let d = day_index - first_weekday - days_count + 1;
            let (ny, nm) = if self.current_month == 12 {
                (self.current_year + 1, 1)
            } else {
                (self.current_year, self.current_month + 1)
            };
            (ny, nm, d)
        } else {
            let d = day_index - first_weekday + 1;
            (self.current_year, self.current_month, d)
        };

        Some(CalendarDate::new(year, month, day))
    }

    // Public API
    pub fn set_date(&mut self, date: CalendarDate) {
        self.selected_date = Some(date);
        self.current_year = date.year;
        self.current_month = date.month;
    }

    pub fn date(&self) -> Option<CalendarDate> {
        self.selected_date
    }

    pub fn set_year_month(&mut self, year: i32, month: u32) {
        self.current_year = year;
        self.current_month = month;
    }
}

impl MpCalendarRef {
    pub fn set_date(&self, date: CalendarDate) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_date(date);
        }
    }

    pub fn date(&self) -> Option<CalendarDate> {
        if let Some(inner) = self.borrow() {
            inner.date()
        } else {
            None
        }
    }
}
