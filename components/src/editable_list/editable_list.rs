use makepad_widgets::*;

use crate::{button::MpButtonWidgetRefExt, drawer::MpDrawerWidgetWidgetExt};

live_design! {
    use link::theme::*;
    use link::theme_colors::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::drawer::*;
    use crate::button::*;

    // ============================================================
    // MpDynamicList - Generic dynamic list for any widget type
    // ============================================================

    pub MpDynamicList = {{MpDynamicList}} {
        width: Fill
        height: Fit
        flow: RightWrap
        //spacing: 8.0
        padding: 8.0

        // Default template - override this with your widget type
        template: <View> {
            width: Fit
            height: Fit
        }

        // Plus button template for adding new items
        plus_template: <View> {
            width: Fit
            height: Fit
            plus_button = <Button> {
                width: 40
                height: 40
                text: "+"
            }
        }
    }

    // ============================================================
    // MpEditableList - Dynamic list with right drawer
    // ============================================================

    pub MpEditableList = {{MpEditableList}} {
        width: Fill,
        height: 200
        flow: Overlay

        // Main content area with dynamic list
        content = <View> {
            width: Fill
            height: Fill
            flow: Down

            list = <MpDynamicList> {
                width: Fill
                height: Fit
                flow: RightWrap

                margin: {left: 20}

                // Template for child items
                template: <View> {
                    width: 100
                    height: 40.0
                    padding: {left: 6, right: 6}
                    show_bg: false
                    <RoundedView> {
                        width: Fill, height: Fill
                        padding: 0
                        show_bg: true
                        draw_bg: {
                            color: (PRIMARY)
                            border_radius: 8.0
                        }
                        item_label = <Label> {
                            width: Fit
                            height: Fit
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                color: (PRIMARY_FOREGROUND)
                            }
                        }
                    }
                }
                plus_template: <View> {
                    plus_button = <Button> {
                        text: "+"
                    }
                }
                setting_template: <View> {
                    setting_button = <Button> {
                        text: "*"
                    }
                }
            }
        }

        // Right drawer for item options
        drawer = <MpDrawerWidget> {
            container = {
                drawer = {
                    width: 320

                    header = {
                        title = {
                            text: "Options"
                        }
                    }

                    body = {
                        spacing: 12

                        configure_btn = <MpButtonPrimary> {
                            width: Fill, height: 44
                            text: "Configure"
                        }

                        edit_buttons = <View> {
                            width: Fill, height: Fit
                            flow: Down
                            spacing: 12

                            move_left_btn = <MpButtonPrimary> {
                                width: Fill, height: 44
                                text: "Move Left"
                            }

                            move_right_btn = <MpButtonPrimary> {
                                width: Fill, height: 44
                                text: "Move Right"
                            }

                            delete_btn = <MpButtonDanger> {
                                width: Fill, height: 44
                                text: "Delete"
                            }
                        }
                    }

                    footer = {}
                }
            }
        }
    }
}

// ============================================================
// MpDynamicList - Generic list for any child widget type
// ============================================================

#[derive(Live, Widget)]
pub struct MpDynamicList {
    #[redraw]
    #[rust]
    area: Area,
    /// Template for creating child widgets
    #[live]
    template: Option<LivePtr>,
    #[live]
    setting_template: Option<LivePtr>,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    /// Dynamic children vector
    #[rust]
    pub children: Vec<WidgetRef>,
    #[rust]
    setting_children: Vec<WidgetRef>,
    #[live]
    plus_template: Option<LivePtr>,
    #[rust]
    plus: WidgetRef,
}

impl LiveHook for MpDynamicList {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.plus = WidgetRef::new_from_ptr(cx, self.plus_template);
    }
}

impl Widget for MpDynamicList {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);

        // Draw each child with its setting widget on top
        for (child_ref, setting_ref) in self.children.iter_mut().zip(self.setting_children.iter_mut()) {
            // Use the child's walk for the container (preserves margin, size)
            let child_walk = child_ref.walk(cx);
            let container_walk = Walk {
                abs_pos: child_walk.abs_pos,
                margin: child_walk.margin,
                width: child_walk.width,
                height: child_walk.height,
                metrics: child_walk.metrics,
            };
            cx.begin_turtle(container_walk, Layout::flow_overlay());
            // Draw the child first (with no margin since container has it)
            let inner_walk = Walk {
                abs_pos: None,
                margin: Margin::default(),
                width: Size::fill(),
                height: Size::fill(),
                metrics: Metrics::default(),
            };
            let _ = child_ref.draw_walk(cx, scope, inner_walk);
            //let _ = child_ref.draw(cx, scope);
            // Draw the setting on top (fills the same space)
            let _ = setting_ref.draw(cx, scope);
            cx.end_turtle();
        }

        let _ = self.plus.draw(cx, scope);
        cx.end_turtle_with_area(&mut self.area);

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Handle settings first since they're drawn on top
        for setting_ref in self.setting_children.iter_mut() {
            setting_ref.handle_event(cx, event, scope);
        }
        for widget_ref in self.children.iter_mut() {
            widget_ref.handle_event(cx, event, scope);
        }
        self.plus.handle_event(cx, event, scope);
        self.match_event(cx, event);
    }
}
impl MatchEvent for MpDynamicList {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.plus.button(ids!(plus_button)).clicked(actions) {
            cx.widget_action(
                self.widget_uid(),
                &Scope::empty().path,
                DynamicListAction::Add,
            );
        }
        for (idx, setting_ref) in self.setting_children.iter().enumerate() {
            if setting_ref.button(ids!(setting_button)).clicked(actions) {
                cx.widget_action(
                    self.widget_uid(),
                    &Scope::empty().path,
                    DynamicListAction::Configure(idx),
                );
                break;
            }
        }
    }
}
impl MpDynamicList {
    /// Get or create a child widget at the given index, then populate it with the closure.
    /// If the index is beyond current children count, new widgets are created
    /// from the template to fill up to and including the index.
    ///
    /// Inside the closure, you can cast the WidgetRef to a specific type using
    /// the generated `as_*` methods (e.g., `widget.as_my_widget()`).
    pub fn set_child<F>(&mut self, cx: &mut Cx, idx: usize, f: F)
    where
        F: FnOnce(&mut Cx, WidgetRef),
    {
        while self.children.len() <= idx {
            let widget = WidgetRef::new_from_ptr(cx, self.template);
            self.children.push(widget);
            let widget = WidgetRef::new_from_ptr(cx, self.setting_template);
            self.setting_children.push(widget);
        }
        f(cx, self.children[idx].clone());
        self.redraw(cx);
    }

    /// Get a child widget at the given index, if it exists
    pub fn get_child(&self, idx: usize) -> Option<WidgetRef> {
        self.children.get(idx).cloned()
    }

    /// Get the number of children
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Clear all children
    pub fn clear(&mut self, cx: &mut Cx) {
        self.children.clear();
        self.setting_children.clear();
        self.redraw(cx);
    }

    /// Remove a child at the given index
    pub fn remove_child(&mut self, _cx: &mut Cx, idx: usize) -> Option<WidgetRef> {
        if idx < self.children.len() {
            self.setting_children.remove(idx);
            Some(self.children.remove(idx))
        } else {
            None
        }
    }
    pub fn set_plus<F>(&mut self, cx: &mut Cx, f: F)
    where
        F: FnOnce(&mut Cx, WidgetRef),
    {
        f(cx, self.plus.clone());
    }
}

impl MpDynamicListRef {
    /// Get or create a child widget at the given index, then populate it with the closure.
    ///
    /// Inside the closure, you can cast the WidgetRef to a specific type using
    /// the generated `as_*` methods (e.g., `widget.as_my_widget()`).
    pub fn set_child<F>(&self, cx: &mut Cx, idx: usize, f: F)
    where
        F: FnOnce(&mut Cx, WidgetRef),
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_child(cx, idx, f);
        }
    }

    /// Get a child widget at the given index, if it exists
    pub fn get_child(&self, idx: usize) -> Option<WidgetRef> {
        self.borrow().and_then(|inner| inner.get_child(idx))
    }

    /// Get the number of children
    pub fn child_count(&self) -> usize {
        self.borrow().map_or(0, |inner| inner.child_count())
    }

    /// Clear all children
    pub fn clear(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear(cx);
        }
    }

    /// Remove a child at the given index
    pub fn remove_child(&self, cx: &mut Cx, idx: usize) -> Option<WidgetRef> {
        self.borrow_mut()
            .and_then(|mut inner| inner.remove_child(cx, idx))
    }

    pub fn set_plus<F>(&self, cx: &mut Cx, f: F)
    where
        F: FnOnce(&mut Cx, WidgetRef),
    {
        self.borrow_mut().map(|mut inner| inner.set_plus(cx, f));
    }

    /// Check if the Add action was requested
    pub fn add_requested(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                if let DynamicListAction::Add = item.cast() {
                    return true;
                }
            }
        }
        false
    }

    /// Check if Configure action was requested and return the index
    pub fn configure_requested(&self, actions: &Actions) -> Option<usize> {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                if let DynamicListAction::Configure(idx) = item.cast() {
                    return Some(idx);
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum DynamicListAction {
    None,
    Configure(usize),
    Add,
}

// ============================================================
// MpEditableList - Dynamic list with right drawer
// ============================================================

/// Callback type for setting up child widgets
pub type SetChildFn<T> = Box<dyn Fn(&mut Cx, usize, WidgetRef, &T)>;

#[derive(Live, Widget)]
pub struct MpEditableList {
    #[deref]
    view: View,

    #[rust]
    selected_index: Option<usize>,

    #[rust]
    initialized: bool,
}

impl LiveHook for MpEditableList {
    fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if !self.initialized {
            self.initialized = true;
            // Pre-initialize drawer content to avoid first-open lag
            // This forces widget instantiation before first use
            let _ = self.view.view(ids!(drawer.container.drawer));
        }
    }
}

impl Widget for MpEditableList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.view
            .view(ids!(drawer.container.drawer))
            .handle_event(cx, event, scope);
        self.match_event(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MatchEvent for MpEditableList {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Collect what actions need to be performed (to avoid borrow conflicts)
        let mut close_drawer_requested = false;
        let mut add_requested = false;
        let mut configure_idx: Option<usize> = None;
        let mut configure_clicked = false;
        let mut move_left_clicked = false;
        let mut move_right_clicked = false;
        let mut delete_clicked = false;

        // Check actions while holding borrows
        {
            let drawer = self.view.mp_drawer_widget(ids!(drawer));
            if drawer.close_requested(actions) {
                close_drawer_requested = true;
            }

            let list = self.view.mp_dynamic_list(ids!(content.list));
            if list.add_requested(actions) {
                add_requested = true;
            }
            if let Some(idx) = list.configure_requested(actions) {
                configure_idx = Some(idx);
                println!("configure idx: {}", idx);
            }

            if drawer.mp_button(ids!(body.configure_btn)).clicked(actions) {
                configure_clicked = true;
            }
            if drawer
                .mp_button(ids!(body.edit_buttons.move_left_btn))
                .clicked(actions)
            {
                move_left_clicked = true;
            }
            if drawer
                .mp_button(ids!(body.edit_buttons.move_right_btn))
                .clicked(actions)
            {
                move_right_clicked = true;
            }
            if drawer
                .mp_button(ids!(body.edit_buttons.delete_btn))
                .clicked(actions)
            {
                delete_clicked = true;
            }
        }
        // Borrows dropped, now execute actions

        if close_drawer_requested {
            self.close_drawer(cx);
        }

        if add_requested {
            self.open_drawer_for_add(cx);
        }

        if let Some(idx) = configure_idx {
            self.selected_index = Some(idx);
            self.open_drawer_for_item(cx, idx);
        }

        if configure_clicked {
            println!("configure clicked");
            cx.widget_action(
                self.widget_uid(),
                &Scope::empty().path,
                EditableListAction::Configure(self.selected_index),
            );
            self.close_drawer(cx);
        }

        if move_left_clicked {
            if let Some(idx) = self.selected_index {
                self.move_item_left(cx, idx);
                cx.widget_action(
                    self.widget_uid(),
                    &Scope::empty().path,
                    EditableListAction::MoveLeft(idx),
                );
            }
            self.close_drawer(cx);
        }

        if move_right_clicked {
            if let Some(idx) = self.selected_index {
                self.move_item_right(cx, idx);
                cx.widget_action(
                    self.widget_uid(),
                    &Scope::empty().path,
                    EditableListAction::MoveRight(idx),
                );
            }
            self.close_drawer(cx);
        }

        if delete_clicked {
            if let Some(idx) = self.selected_index {
                self.delete_item(cx, idx);
                cx.widget_action(
                    self.widget_uid(),
                    &Scope::empty().path,
                    EditableListAction::Delete(idx),
                );
            }
            self.close_drawer(cx);
        }
    }
}

impl MpEditableList {
    /// Open the drawer
    pub fn open_drawer(&mut self, cx: &mut Cx) {
        self.view.mp_drawer_widget(ids!(drawer)).open(cx);
    }

    /// Open the drawer for adding a new item (hides edit buttons)
    pub fn open_drawer_for_add(&mut self, cx: &mut Cx) {
        self.selected_index = None;
        let drawer_view = self.view.view(ids!(drawer.container.drawer));
        drawer_view
            .label(ids!(header.title))
            .set_text(cx, "Add Item");
        drawer_view
            .view(ids!(body.edit_buttons))
            .set_visible(cx, false);
        self.open_drawer(cx);
    }

    /// Open the drawer with a specific item selected (shows edit buttons)
    pub fn open_drawer_for_item(&mut self, cx: &mut Cx, index: usize) {
        self.selected_index = Some(index);
        let drawer_view = self.view.view(ids!(drawer.container.drawer));
        drawer_view
            .label(ids!(header.title))
            .set_text(cx, &format!("Item {}", index));
        drawer_view
            .view(ids!(body.edit_buttons))
            .set_visible(cx, true);
        self.open_drawer(cx);
    }

    /// Close the drawer
    pub fn close_drawer(&mut self, cx: &mut Cx) {
        self.view.mp_drawer_widget(ids!(drawer)).close(cx);
        self.selected_index = None;
    }

    /// Check if the drawer is open
    pub fn is_drawer_open(&self) -> bool {
        self.view.mp_drawer_widget(ids!(drawer)).is_open()
    }

    /// Get the inner dynamic list
    pub fn list(&self) -> MpDynamicListRef {
        self.view.mp_dynamic_list(ids!(content.list))
    }

    /// Move an item left (swap with previous item)
    pub fn move_item_left(&mut self, cx: &mut Cx, index: usize) {
        if index > 0 {
            let did_swap = {
                if let Some(mut list) = self.view.mp_dynamic_list(ids!(content.list)).borrow_mut() {
                    if index < list.children.len() {
                        list.children.swap(index, index - 1);
                        list.setting_children.swap(index, index - 1);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            if did_swap {
                self.view.redraw(cx);
            }
        }
    }

    /// Move an item right (swap with next item)
    pub fn move_item_right(&mut self, cx: &mut Cx, index: usize) {
        let did_swap = {
            if let Some(mut list) = self.view.mp_dynamic_list(ids!(content.list)).borrow_mut() {
                if index + 1 < list.children.len() {
                    list.children.swap(index, index + 1);
                    list.setting_children.swap(index, index + 1);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if did_swap {
            self.view.redraw(cx);
        }
    }

    /// Delete an item at the given index
    pub fn delete_item(&mut self, cx: &mut Cx, index: usize) {
        let did_delete = {
            if let Some(mut list) = self.view.mp_dynamic_list(ids!(content.list)).borrow_mut() {
                if index < list.children.len() {
                    list.setting_children.remove(index);
                    list.children.remove(index);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        if did_delete {
            self.view.redraw(cx);
        }
    }
}

impl MpEditableListRef {
    /// Open the drawer
    pub fn open_drawer(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open_drawer(cx);
        }
    }

    /// Open the drawer for adding a new item (hides edit buttons)
    pub fn open_drawer_for_add(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open_drawer_for_add(cx);
        }
    }

    /// Open the drawer with a specific item selected (shows edit buttons)
    pub fn open_drawer_for_item(&self, cx: &mut Cx, index: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open_drawer_for_item(cx, index);
        }
    }

    /// Close the drawer
    pub fn close_drawer(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close_drawer(cx);
        }
    }

    /// Check if the drawer is open
    pub fn is_drawer_open(&self) -> bool {
        self.borrow().map_or(false, |inner| inner.is_drawer_open())
    }

    /// Get the inner dynamic list
    pub fn list(&self) -> MpDynamicListRef {
        if let Some(inner) = self.borrow() {
            inner.list()
        } else {
            MpDynamicListRef::default()
        }
    }

    /// Move an item left (swap with previous item)
    pub fn move_item_left(&self, cx: &mut Cx, index: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.move_item_left(cx, index);
        }
    }

    /// Move an item right (swap with next item)
    pub fn move_item_right(&self, cx: &mut Cx, index: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.move_item_right(cx, index);
        }
    }

    /// Delete an item at the given index
    pub fn delete_item(&self, cx: &mut Cx, index: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.delete_item(cx, index);
        }
    }

    /// Handle a EditableListEditAction with a closure to set up child widgets
    /// The closure receives (cx, index, widget_ref, data) for Add/Configure actions
    pub fn handle_edit_action<T, F>(&self, cx: &mut Cx, action: &EditableListEditAction<T>, f: F)
    where
        F: FnOnce(&mut Cx, usize, WidgetRef, &T),
    {
        match action {
            EditableListEditAction::Add(data) => {
                let list = self.list();
                let idx = list.child_count();
                list.set_child(cx, idx, |cx, widget| {
                    f(cx, idx, widget, data);
                });
            }
            EditableListEditAction::Configure(Some(idx), data) => {
                let list = self.list();
                if let Some(widget) = list.get_child(*idx) {
                    f(cx, *idx, widget, data);
                }
            }
            EditableListEditAction::Delete(idx) => {
                self.delete_item(cx, *idx);
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum EditableListAction {
    None,
    Configure(Option<usize>),
    MoveLeft(usize),
    MoveRight(usize),
    Delete(usize),
}

#[derive(Clone, Debug)]
pub enum EditableListEditAction<T> {
    None,
    Configure(Option<usize>, T),
    Add(T),
    Delete(usize),
}

impl<T> Default for EditableListEditAction<T> {
    fn default() -> Self {
        EditableListEditAction::None
    }
}
