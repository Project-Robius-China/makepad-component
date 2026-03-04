use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub MpCommandPalette = {{MpCommandPalette}} {
        width: 420,
        height: 40,

        flow: Right,
        align: { y: 0.5 },
        padding: { left: 12, right: 12, top: 8, bottom: 8 },

        draw_bg: {
            color: #ffffff
        }

        draw_text: {
            color: #0f172a
            text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
        }

        panel_draw_bg: {
            color: #ffffff
        }

        item_draw_bg: {
            color: #00000000
        }

        item_selected_draw_bg: {
            color: #e2e8f0
        }

        item_disabled_draw_bg: {
            color: #f8fafc
        }

        item_title_draw_text: {
            color: #0f172a
            text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
        }

        item_meta_draw_text: {
            color: #64748b
            text_style: <THEME_FONT_REGULAR> { font_size: 11.0 }
        }

        placeholder: "Search commands..."
        max_visible_items: 8
        item_height: 48.0
        auto_open_on_focus: true
    }

    pub MpCommandPaletteCompact = <MpCommandPalette> {
        width: 320,
        height: 34,
        padding: { left: 10, right: 10, top: 6, bottom: 6 },
        item_height: 42.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandPaletteItem {
    pub id: String,
    pub title: String,
    pub group: String,
    pub shortcut: String,
    pub keywords: Vec<String>,
    pub enabled: bool,
}

impl CommandPaletteItem {
    pub fn new(id: &str, title: &str, group: &str, shortcut: &str, keywords: &[&str]) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            group: group.to_string(),
            shortcut: shortcut.to_string(),
            keywords: keywords.iter().map(|v| v.to_string()).collect(),
            enabled: true,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpCommandPaletteAction {
    None,
    Opened,
    Closed,
    QueryChanged(String),
    Executed(String, String, String),
}

pub fn filter_and_rank_items(items: &[CommandPaletteItem], query: &str) -> Vec<usize> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return items
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| if item.enabled { Some(idx) } else { None })
            .collect();
    }

    let needle = trimmed.to_lowercase();
    let mut scored: Vec<(usize, usize)> = Vec::new();

    for (idx, item) in items.iter().enumerate() {
        if !item.enabled {
            continue;
        }

        if let Some(score) = command_match_score(item, &needle) {
            scored.push((idx, score));
        }
    }

    scored.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    scored.into_iter().map(|(idx, _)| idx).collect()
}

fn command_match_score(item: &CommandPaletteItem, query: &str) -> Option<usize> {
    let title_lower = item.title.to_lowercase();
    if title_lower.starts_with(query) {
        return Some(0);
    }
    if let Some(pos) = title_lower.find(query) {
        return Some(100 + pos);
    }

    let group_lower = item.group.to_lowercase();
    if group_lower.starts_with(query) {
        return Some(200);
    }
    if let Some(pos) = group_lower.find(query) {
        return Some(300 + pos);
    }

    for (kw_idx, keyword) in item.keywords.iter().enumerate() {
        let keyword_lower = keyword.to_lowercase();
        if keyword_lower.starts_with(query) {
            return Some(400 + kw_idx);
        }
        if let Some(pos) = keyword_lower.find(query) {
            return Some(500 + kw_idx * 10 + pos);
        }
    }

    None
}

#[derive(Live, Widget)]
pub struct MpCommandPalette {
    #[redraw]
    #[live]
    draw_bg: DrawColor,
    #[live]
    draw_text: DrawText,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[live]
    panel_draw_bg: DrawColor,
    #[live]
    item_draw_bg: DrawColor,
    #[live]
    item_selected_draw_bg: DrawColor,
    #[live]
    item_disabled_draw_bg: DrawColor,
    #[live]
    item_title_draw_text: DrawText,
    #[live]
    item_meta_draw_text: DrawText,

    #[live]
    placeholder: String,
    #[live(8)]
    max_visible_items: i64,
    #[live(48.0)]
    item_height: f64,
    #[live(true)]
    auto_open_on_focus: bool,

    #[rust]
    opened: bool,
    #[rust]
    query: String,
    #[rust]
    items: Vec<CommandPaletteItem>,
    #[rust]
    filtered_indices: Vec<usize>,
    #[rust]
    selected_filtered_index: Option<usize>,

    #[rust]
    item_areas: Vec<Area>,
    #[rust]
    panel_area: Area,

    #[rust(DrawList2d::new(cx))]
    draw_list: DrawList2d,
}

impl LiveHook for MpCommandPalette {}

impl MpCommandPalette {
    fn rebuild_filtered(&mut self) {
        self.filtered_indices = filter_and_rank_items(&self.items, &self.query);
        self.selected_filtered_index = self.first_enabled_filtered_index();
    }

    fn first_enabled_filtered_index(&self) -> Option<usize> {
        self.filtered_indices
            .iter()
            .enumerate()
            .find_map(|(fi, &idx)| {
                self.items
                    .get(idx)
                    .and_then(|item| if item.enabled { Some(fi) } else { None })
            })
    }

    fn selected_item_enabled(&self) -> bool {
        self.selected_filtered_index
            .and_then(|fi| self.filtered_indices.get(fi))
            .and_then(|&idx| self.items.get(idx))
            .map(|item| item.enabled)
            .unwrap_or(false)
    }

    fn open_panel(&mut self, cx: &mut Cx) {
        if self.opened {
            return;
        }

        self.opened = true;
        self.rebuild_filtered();
        self.draw_list.redraw(cx);
        self.redraw(cx);
        cx.sweep_lock(self.draw_bg.area());
    }

    fn close_panel(&mut self, cx: &mut Cx) {
        if !self.opened {
            return;
        }

        self.opened = false;
        self.selected_filtered_index = None;
        self.draw_list.redraw(cx);
        self.redraw(cx);
        cx.sweep_unlock(self.draw_bg.area());
    }

    fn move_selection(&mut self, delta: i32) {
        if self.filtered_indices.is_empty() {
            self.selected_filtered_index = None;
            return;
        }

        let len = self.filtered_indices.len() as i32;
        let mut cursor = self
            .selected_filtered_index
            .map(|idx| idx as i32)
            .unwrap_or_else(|| if delta >= 0 { -1 } else { 0 });

        for _ in 0..len {
            cursor += delta;
            if cursor < 0 {
                cursor = len - 1;
            }
            if cursor >= len {
                cursor = 0;
            }

            let filtered_idx = cursor as usize;
            if let Some(item_idx) = self.filtered_indices.get(filtered_idx) {
                if self
                    .items
                    .get(*item_idx)
                    .map(|i| i.enabled)
                    .unwrap_or(false)
                {
                    self.selected_filtered_index = Some(filtered_idx);
                    return;
                }
            }
        }
    }

    fn execute_filtered(
        &mut self,
        cx: &mut Cx,
        filtered_index: usize,
        uid: WidgetUid,
        path: &HeapLiveIdPath,
    ) {
        let Some(item_index) = self.filtered_indices.get(filtered_index).copied() else {
            return;
        };
        let Some(item) = self.items.get(item_index) else {
            return;
        };
        if !item.enabled {
            return;
        }

        cx.widget_action(
            uid,
            path,
            MpCommandPaletteAction::Executed(
                item.id.clone(),
                item.title.clone(),
                item.group.clone(),
            ),
        );
        self.close_panel(cx);
        cx.widget_action(uid, path, MpCommandPaletteAction::Closed);
    }

    fn row_meta_text(item: &CommandPaletteItem) -> String {
        match (item.group.is_empty(), item.shortcut.is_empty()) {
            (true, true) => String::new(),
            (false, true) => item.group.clone(),
            (true, false) => item.shortcut.clone(),
            (false, false) => format!("{}  ·  {}", item.group, item.shortcut),
        }
    }

    fn draw_panel(&mut self, cx: &mut Cx2d) {
        self.item_areas.clear();

        let trigger_rect = self.draw_bg.area().rect(cx);

        self.draw_list.begin_overlay_reuse(cx);
        let pass_size = cx.current_pass_size();
        cx.begin_root_turtle(pass_size, Layout::flow_down());

        let panel_walk =
            Walk::new(Size::Fixed(trigger_rect.size.x), Size::fit()).with_abs_pos(dvec2(
                trigger_rect.pos.x,
                trigger_rect.pos.y + trigger_rect.size.y + 4.0,
            ));

        let panel_layout = Layout::flow_down().with_padding(Padding {
            left: 6.0,
            top: 6.0,
            right: 6.0,
            bottom: 6.0,
        });

        self.panel_draw_bg.begin(cx, panel_walk, panel_layout);

        let row_layout = Layout::flow_down().with_padding(Padding {
            left: 10.0,
            top: 7.0,
            right: 10.0,
            bottom: 7.0,
        });

        let visible_count = self
            .filtered_indices
            .len()
            .min(self.max_visible_items.max(1) as usize);

        if visible_count == 0 {
            self.item_draw_bg.begin(
                cx,
                Walk::new(Size::fill(), Size::Fixed(self.item_height.max(36.0))),
                row_layout,
            );
            self.item_title_draw_text.draw_walk(
                cx,
                Walk::fit(),
                Align::default(),
                "No matching commands",
            );
            self.item_meta_draw_text.draw_walk(
                cx,
                Walk::fit(),
                Align::default(),
                "Try another keyword",
            );
            self.item_draw_bg.end(cx);
        } else {
            for (fi, &item_idx) in self.filtered_indices.iter().take(visible_count).enumerate() {
                let item = &self.items[item_idx];
                let is_selected = self.selected_filtered_index == Some(fi);
                let row_walk = Walk::new(Size::fill(), Size::Fixed(self.item_height.max(36.0)));

                if !item.enabled {
                    self.item_disabled_draw_bg.begin(cx, row_walk, row_layout);
                } else if is_selected {
                    self.item_selected_draw_bg.begin(cx, row_walk, row_layout);
                } else {
                    self.item_draw_bg.begin(cx, row_walk, row_layout);
                }

                self.item_title_draw_text
                    .draw_walk(cx, Walk::fit(), Align::default(), &item.title);

                let meta_text = Self::row_meta_text(item);
                if !meta_text.is_empty() {
                    self.item_meta_draw_text.draw_walk(
                        cx,
                        Walk::fit(),
                        Align::default(),
                        &meta_text,
                    );
                }

                let row_area = if !item.enabled {
                    self.item_disabled_draw_bg.area()
                } else if is_selected {
                    self.item_selected_draw_bg.area()
                } else {
                    self.item_draw_bg.area()
                };
                self.item_areas.push(row_area);

                if !item.enabled {
                    self.item_disabled_draw_bg.end(cx);
                } else if is_selected {
                    self.item_selected_draw_bg.end(cx);
                } else {
                    self.item_draw_bg.end(cx);
                }
            }

            if self.filtered_indices.len() > visible_count {
                let extra = self.filtered_indices.len() - visible_count;
                self.item_draw_bg
                    .begin(cx, Walk::new(Size::fill(), Size::Fixed(28.0)), row_layout);
                self.item_meta_draw_text.draw_walk(
                    cx,
                    Walk::fit(),
                    Align::default(),
                    &format!("{} more results...", extra),
                );
                self.item_draw_bg.end(cx);
            }
        }

        self.panel_draw_bg.end(cx);
        self.panel_area = self.panel_draw_bg.area();

        cx.end_pass_sized_turtle();
        self.draw_list.end(cx);
    }
}

impl Widget for MpCommandPalette {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.opened {
            if let Event::MouseDown(e) = event {
                let in_trigger = self.draw_bg.area().clipped_rect(cx).contains(e.abs);
                let in_panel = self.panel_area.clipped_rect(cx).contains(e.abs);
                if !in_trigger && !in_panel {
                    self.close_panel(cx);
                    cx.widget_action(uid, &scope.path, MpCommandPaletteAction::Closed);
                    return;
                }
            }

            if let Event::MouseMove(e) = event {
                for (fi, area) in self.item_areas.iter().enumerate() {
                    if area.clipped_rect(cx).contains(e.abs) {
                        self.selected_filtered_index = Some(fi);
                        self.draw_list.redraw(cx);
                        break;
                    }
                }
            }

            if let Event::MouseUp(e) = event {
                for (fi, area) in self.item_areas.iter().enumerate() {
                    if area.clipped_rect(cx).contains(e.abs) {
                        self.execute_filtered(cx, fi, uid, &scope.path);
                        return;
                    }
                }
            }

            if let Event::TextInput(te) = event {
                if !te.input.is_empty() {
                    self.query.push_str(&te.input);
                    self.rebuild_filtered();
                    self.draw_list.redraw(cx);
                    self.redraw(cx);
                    cx.widget_action(
                        uid,
                        &scope.path,
                        MpCommandPaletteAction::QueryChanged(self.query.clone()),
                    );
                }
            }

            if let Event::KeyDown(ke) = event {
                match ke.key_code {
                    KeyCode::Escape => {
                        self.close_panel(cx);
                        cx.widget_action(uid, &scope.path, MpCommandPaletteAction::Closed);
                        return;
                    }
                    KeyCode::ArrowDown => {
                        self.move_selection(1);
                        self.draw_list.redraw(cx);
                    }
                    KeyCode::ArrowUp => {
                        self.move_selection(-1);
                        self.draw_list.redraw(cx);
                    }
                    KeyCode::ReturnKey => {
                        if let Some(fi) = self.selected_filtered_index {
                            if self.selected_item_enabled() {
                                self.execute_filtered(cx, fi, uid, &scope.path);
                            }
                        }
                        return;
                    }
                    KeyCode::Backspace => {
                        if !self.query.is_empty() {
                            self.query.pop();
                            self.rebuild_filtered();
                            self.draw_list.redraw(cx);
                            self.redraw(cx);
                            cx.widget_action(
                                uid,
                                &scope.path,
                                MpCommandPaletteAction::QueryChanged(self.query.clone()),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        match event.hits_with_sweep_area(cx, self.draw_bg.area(), self.draw_bg.area()) {
            Hit::FingerDown(_) => {
                cx.set_key_focus(self.draw_bg.area());
                if !self.opened {
                    self.open_panel(cx);
                    cx.widget_action(uid, &scope.path, MpCommandPaletteAction::Opened);
                }
            }
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
            }
            Hit::KeyFocus(_) => {
                if self.auto_open_on_focus && !self.opened {
                    self.open_panel(cx);
                    cx.widget_action(uid, &scope.path, MpCommandPaletteAction::Opened);
                }
            }
            Hit::KeyFocusLost(_) => {
                if self.opened {
                    self.close_panel(cx);
                    cx.widget_action(uid, &scope.path, MpCommandPaletteAction::Closed);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let display = if self.query.is_empty() {
            self.placeholder.clone()
        } else {
            self.query.clone()
        };

        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text.draw_walk(
            cx,
            Walk::new(Size::fill(), Size::fit()),
            Align::default(),
            &display,
        );
        self.draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), "⌘K");
        self.draw_bg.end(cx);

        if self.opened {
            self.draw_panel(cx);
        }

        DrawStep::done()
    }
}

impl MpCommandPaletteRef {
    pub fn set_items(&self, cx: &mut Cx, items: Vec<CommandPaletteItem>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.items = items;
            inner.rebuild_filtered();
            inner.redraw(cx);
        }
    }

    pub fn clear_query(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.query.clear();
            inner.rebuild_filtered();
            inner.redraw(cx);
        }
    }

    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open_panel(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close_panel(cx);
        }
    }

    pub fn query_changed(&self, actions: &Actions) -> Option<String> {
        if let Some(inner) = self.borrow() {
            if let Some(action) = actions.find_widget_action(inner.widget_uid()) {
                if let MpCommandPaletteAction::QueryChanged(query) = action.cast() {
                    return Some(query);
                }
            }
        }
        None
    }

    pub fn executed(&self, actions: &Actions) -> Option<(String, String, String)> {
        if let Some(inner) = self.borrow() {
            if let Some(action) = actions.find_widget_action(inner.widget_uid()) {
                if let MpCommandPaletteAction::Executed(id, title, group) = action.cast() {
                    return Some((id, title, group));
                }
            }
        }
        None
    }

    pub fn opened(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            if let Some(action) = actions.find_widget_action(inner.widget_uid()) {
                return matches!(
                    action.cast::<MpCommandPaletteAction>(),
                    MpCommandPaletteAction::Opened
                );
            }
        }
        false
    }

    pub fn closed(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            if let Some(action) = actions.find_widget_action(inner.widget_uid()) {
                return matches!(
                    action.cast::<MpCommandPaletteAction>(),
                    MpCommandPaletteAction::Closed
                );
            }
        }
        false
    }
}
