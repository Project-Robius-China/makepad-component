use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpTableCell - Basic table cell with label
    // ============================================================
    pub MpTableCell = <View> {
        width: 120.0,
        height: 40.0,
        padding: { left: 12.0, right: 12.0 }
        align: { x: 0.0, y: 0.5 }

        label = <Label> {
            width: Fill,
            height: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{ font_size: 13.0 }
                color: (FOREGROUND)
            }
        }
    }

    // ============================================================
    // MpTableHeaderCell - Header cell with bold text and MUTED background
    // ============================================================
    pub MpTableHeaderCell = <View> {
        width: 120.0,
        height: 40.0,
        padding: { left: 12.0, right: 12.0 }
        align: { x: 0.0, y: 0.5 }

        show_bg: true
        draw_bg: {
            color: (MUTED)
        }

        label = <Label> {
            width: Fill,
            height: Fit,
            draw_text: {
                text_style: <THEME_FONT_BOLD>{ font_size: 13.0 }
                color: (FOREGROUND)
            }
        }
    }

    // ============================================================
    // MpTableRow - Row with hover/selected/stripe effects
    // ============================================================
    pub MpTableRow = <View> {
        width: Fill,
        height: 40.0,
        flow: Right,
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance hover: 0.0
            instance selected: 0.0
            instance stripe: 0.0
            instance bg_color: (TRANSPARENT)
            instance hover_color: (SECONDARY)
            instance selected_color: (PRIMARY)
            instance stripe_color: (MUTED)

            fn pixel(self) -> vec4 {
                let base = mix(self.bg_color, self.stripe_color, self.stripe);
                let c = mix(base, self.hover_color, self.hover * (1.0 - self.selected));
                let c = mix(c, self.selected_color, self.selected * 0.15);
                return c;
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
            selected = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { selected: 0.0 } }
                }
                on = {
                    from: { all: Snap }
                    apply: { draw_bg: { selected: 1.0 } }
                }
            }
        }
    }

    // ============================================================
    // MpTableHeader - Header row container
    // ============================================================
    pub MpTableHeader = <View> {
        width: Fill,
        height: 40.0,
        flow: Right,

        show_bg: true
        draw_bg: {
            color: (MUTED)
        }
    }

    // ============================================================
    // MpTable - Main container with bordered styling
    // ============================================================
    pub MpTable = {{MpTable}} {
        width: Fill,
        height: Fit,
        flow: Down,

        show_bg: true
        draw_bg: {
            instance radius: 6.0
            instance border_width: 1.0
            instance border_color: (BORDER)
            instance bg_color: (CARD)
            instance bordered: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                if self.bordered > 0.5 {
                    sdf.box(
                        self.border_width,
                        self.border_width,
                        self.rect_size.x - self.border_width * 2.0,
                        self.rect_size.y - self.border_width * 2.0,
                        self.radius
                    );
                    sdf.fill_keep(self.bg_color);
                    sdf.stroke(self.border_color, self.border_width);
                } else {
                    sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                    sdf.fill(self.bg_color);
                }

                return sdf.result;
            }
        }
    }
}

// ============================================================================
// MpTableAction - Actions emitted by the table
// ============================================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpTableAction {
    RowSelected(usize),
    RowClicked(usize),
    None,
}

// ============================================================================
// MpTable Widget Struct
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct MpTable {
    #[deref]
    view: View,

    #[live]
    bordered: bool,

    #[live]
    stripe: bool,

    #[rust]
    selected_row: Option<usize>,
}

// ============================================================================
// Widget Implementation
// ============================================================================

impl Widget for MpTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================================
// WidgetMatchEvent Implementation
// ============================================================================

impl WidgetMatchEvent for MpTable {
    fn handle_actions(&mut self, _cx: &mut Cx, _actions: &Actions, _scope: &mut Scope) {
        // Row click handling is delegated to individual MpTableRow components
        // which have their own hover/selected animators.
        // Users can listen for finger_up events on specific rows using:
        // table.view(ids!(row_name)).finger_up(actions)
    }
}

// ============================================================================
// Public Methods
// ============================================================================

impl MpTable {
    /// Set the selected row index
    pub fn set_selected_row(&mut self, row: Option<usize>) {
        self.selected_row = row;
    }

    /// Get the currently selected row index
    pub fn selected_row(&self) -> Option<usize> {
        self.selected_row
    }

    /// Check if stripe mode is enabled
    pub fn is_stripe(&self) -> bool {
        self.stripe
    }

    /// Check if bordered mode is enabled
    pub fn is_bordered(&self) -> bool {
        self.bordered
    }
}

// ============================================================================
// MpTableRef Methods
// ============================================================================

impl MpTableRef {
    /// Set the selected row index
    pub fn set_selected_row(&self, row: Option<usize>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_selected_row(row);
        }
    }

    /// Get the currently selected row index
    pub fn selected_row(&self) -> Option<usize> {
        if let Some(inner) = self.borrow() {
            inner.selected_row()
        } else {
            None
        }
    }

    /// Check if stripe mode is enabled
    pub fn is_stripe(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_stripe()
        } else {
            false
        }
    }

    /// Check if bordered mode is enabled
    pub fn is_bordered(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_bordered()
        } else {
            true
        }
    }
}