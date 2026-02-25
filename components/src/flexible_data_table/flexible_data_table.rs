//! # FlexibleDataTable
//!
//! A configurable data table widget with support for multiple cell types.
//!
//! ## Features
//! - Up to 10 columns
//! - Three cell types: DropDown, TextInput, ColorPicker
//! - DSL configuration for columns, rows, and hidden cells
//! - Add/remove rows dynamically
//! - Per-cell visibility control
//!
//! ## DSL Usage
//!
//! ```rust
//! <FlexibleDataTable> {
//!     width: Fill,
//!     height: Fit,
//!     initial_rows: 2,
//!     live_columns: [
//!         { name: "Status", cell_type: DropDown, width: 120.0, dropdown_labels: "Active,Inactive,Pending" },
//!         { name: "Description", cell_type: TextInput, width: 200.0 },
//!         { name: "Color", cell_type: ColorPicker, width: 150.0 },
//!     ],
//!     live_hidden_cells: [
//!         { row: 0, col: 1 },  // Hide cell at row 0, column 1
//!     ]
//! }
//! ```
//!
//! ## DSL Properties
//!
//! | Property | Type | Default | Description |
//! |----------|------|---------|-------------|
//! | `initial_rows` | i64 | 1 | Number of rows to create initially |
//! | `live_columns` | Array | default 3 cols | Column configurations |
//! | `live_hidden_cells` | Array | [] | Cells to hide |
//!
//! ## Column Configuration (LiveColumnConfig)
//!
//! | Field | Type | Default | Description |
//! |-------|------|---------|-------------|
//! | `name` | String | "" | Column header text |
//! | `cell_type` | CellType | TextInput | `DropDown`, `TextInput`, or `ColorPicker` |
//! | `width` | f64 | 150.0 | Column width in pixels |
//! | `dropdown_labels` | String | "" | Comma-separated options for DropDown cells |
//!
//! ## Hidden Cell Configuration (LiveHiddenCell)
//!
//! | Field | Type | Default | Description |
//! |-------|------|---------|-------------|
//! | `row` | i64 | 0 | Row index (0-based) |
//! | `col` | i64 | 0 | Column index (0-based) |
//!
//! ## Rust API
//!
//! ```rust
//! // Get table reference
//! let table = self.ui.flexible_data_table(id!(my_table));
//!
//! // Add a row
//! table.add_row(cx);
//!
//! // Set cell value
//! table.set_cell(cx, row_idx, col_idx, CellValue::Text("Hello".to_string()));
//!
//! // Hide a cell
//! table.add_hidden_cell(cx, row_idx, col_idx);
//!
//! // Check for changes
//! if let Some((row, col, value)) = table.cell_changed(&actions) {
//!     // Handle cell change
//! }
//!
//! // Check if add row was clicked
//! if table.add_row_clicked(&actions) {
//!     // Handle add row
//! }
//!
//! // Check if minus row was clicked
//! if let Some(removed_idx) = table.minus_row_clicked(&actions) {
//!     // Handle row removal
//! }
//! ```
//!
//! ## Actions
//!
//! The table emits `FlexibleDataTableAction`:
//! - `CellChanged(row, col, value)` - A cell value was changed
//! - `AddRowClicked` - The add row button was clicked
//! - `MinusRowClicked(row_idx)` - A row's minus button was clicked

use makepad_widgets::*;
use crate::color_picker::*;

// Maximum number of columns supported
const MAX_COLUMNS: usize = 10;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_components::color_picker::color_picker::*;

    COLOR_PANEL = #2a2a2a
    COLOR_BORDER = #3a3a3a
    COLOR_TEXT = #e0e0e0
    COLOR_PRIMARY = #4a90e2
    COLOR_HOVER = #5aa0f2
    COLOR_HEADER = #333333
    COLOR_ROW_BG = #252525
    COLOR_ROW_ALT = #2d2d2d

    // Cell templates for different types
    FlexCell = <View> {
        width: 150, height: Fit

        dropdown_view = <View> {
            width: Fill, height: 30
            dropdown = <DropDown> {
                width: Fill, height: 30
                labels: []
                values: []
            }
        }
        text_input_view = <View> {
            width: Fill, height: 30
            text_input = <TextInput> {
                width: Fill, height: 30
                text: ""
                draw_text: {
                    color: #ffffff
                }
            }
        }

        // Color picker directly without wrapper - wrapper with fixed height clips popup
        color_picker = <MpColorPicker> {
            width: Fill, height: Fit
        }

    }

    // A single row with pre-defined column slots (up to 10)
    FlexTableRow = <View> {
        width: Fill, height: Fit
        flow: Right
        padding: 5
        spacing: 10
        show_bg: true
        draw_bg: {
            instance row_index: 0.0
            fn pixel(self) -> vec4 {
                let is_alt = mod(self.row_index, 2.0);
                return mix((COLOR_ROW_BG), (COLOR_ROW_ALT), is_alt);
            }
        }

        cell_0 = <FlexCell> {}
        cell_1 = <FlexCell> {}
        cell_2 = <FlexCell> {}
        cell_3 = <FlexCell> {}
        cell_4 = <FlexCell> {}
        cell_5 = <FlexCell> {}
        cell_6 = <FlexCell> {}
        cell_7 = <FlexCell> {}
        cell_8 = <FlexCell> {}
        cell_9 = <FlexCell> {}

        minus_btn = <Button> {
            width: 30, height: 30
            text: "-"
            draw_text: {
                text_style: {font_size: 16.0}
            }
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return mix(#e24a4a, #f25a5a, self.hover);
                }
            }
        }
    }

    // Header cell label
    FlexHeaderCell = <Label> {
        width: 150
        draw_text: {
            text_style: {font_size: 14.0},
            color: (COLOR_TEXT)
        }
    }

    pub FlexibleDataTable = {{FlexibleDataTable}} {
        width: Fill, height: Fit
        flow: Down
        spacing: 5
        show_bg: true
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_PANEL);
            }
        }
        padding: 10

        // Header row with pre-defined slots
        header = <View> {
            width: Fill, height: Fit
            flow: Right
            padding: 5
            spacing: 10
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return (COLOR_HEADER);
                }
            }

            header_0 = <FlexHeaderCell> {}
            header_1 = <FlexHeaderCell> {}
            header_2 = <FlexHeaderCell> {}
            header_3 = <FlexHeaderCell> {}
            header_4 = <FlexHeaderCell> {}
            header_5 = <FlexHeaderCell> {}
            header_6 = <FlexHeaderCell> {}
            header_7 = <FlexHeaderCell> {}
            header_8 = <FlexHeaderCell> {}
            header_9 = <FlexHeaderCell> {}

            // Empty header for the minus button column
            header_minus = <View> {
                width: 30, height: Fit
            }
        }

        // Rows using PortalList (needs fixed height for virtualization)
        rows_list = <PortalList> {
            width: Fill, height: 100
            flow: Down
            spacing: 2

            FlexTableRow = <FlexTableRow> {}
        }

        // Footer with Add Row button
        footer = <View> {
            width: Fill, height: Fit
            flow: Right
            padding: {top: 10}
            align: {x: 1.0, y: 0.5}

            add_row_btn = <Button> {
                width: 100, height: 30
                text: "+ Row"
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return mix((COLOR_PRIMARY), (COLOR_HOVER), self.hover);
                    }
                }
            }
        }
    }
}

/// Cell type for each column (Live-compatible for DSL)
#[derive(Live, LiveHook, LiveRegister, Clone, Debug, PartialEq)]
#[live_ignore]
pub enum CellType {
    #[pick]
    /// A dropdown with selectable options
    DropDown,
    /// A text input field
    TextInput,
    /// A color picker
    ColorPicker,
}

/// Live-compatible column configuration for DSL
#[derive(Live, LiveHook, LiveRegister, Clone, Debug)]
#[live_ignore]
pub struct LiveColumnConfig {
    /// Column header name
    #[live]
    pub name: String,
    /// Type of cell: DropDown, TextInput, or ColorPicker
    #[live]
    pub cell_type: CellType,
    /// Width of the column in pixels
    #[live(150.0)]
    pub width: f64,
    /// Options for dropdown cells (comma-separated labels)
    #[live]
    pub dropdown_labels: String,
}

impl Default for LiveColumnConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            cell_type: CellType::TextInput,
            width: 150.0,
            dropdown_labels: String::new(),
        }
    }
}

impl LiveColumnConfig {
    /// Get dropdown labels as a Vec<String>
    pub fn get_dropdown_labels(&self) -> Vec<String> {
        if self.dropdown_labels.is_empty() {
            Vec::new()
        } else {
            self.dropdown_labels.split(',').map(|s| s.trim().to_string()).collect()
        }
    }
    /// Create a new dropdown column
    pub fn dropdown(name: impl Into<String>, labels: Vec<String>, width: f64) -> Self {
        Self {
            name: name.into(),
            cell_type: CellType::DropDown,
            width,
            dropdown_labels: labels.join(","),
        }
    }

    /// Create a new text input column
    pub fn text_input(name: impl Into<String>, width: f64) -> Self {
        Self {
            name: name.into(),
            cell_type: CellType::TextInput,
            width,
            dropdown_labels: String::new(),
        }
    }

    /// Create a new color picker column
    pub fn color_picker(name: impl Into<String>, width: f64) -> Self {
        Self {
            name: name.into(),
            cell_type: CellType::ColorPicker,
            width,
            dropdown_labels: String::new(),
        }
    }
}

/// Live-compatible hidden cell configuration for DSL
#[derive(Live, LiveHook, LiveRegister, Clone, Debug)]
#[live_ignore]
pub struct LiveHiddenCell {
    /// Row index
    #[live(0)]
    pub row: i64,
    /// Column index
    #[live(0)]
    pub col: i64,
}

impl Default for LiveHiddenCell {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
}

impl LiveHiddenCell {
    /// Convert to (usize, usize) tuple
    pub fn to_tuple(&self) -> (usize, usize) {
        (self.row.max(0) as usize, self.col.max(0) as usize)
    }
}

/// Value held by a single cell
#[derive(Clone, Debug)]
pub enum CellValue {
    /// Selected index in dropdown
    DropDown(usize),
    /// Text value
    Text(String),
    /// Color value (RGBA)
    Color(Vec4),
}

impl Default for CellValue {
    fn default() -> Self {
        CellValue::Text(String::new())
    }
}

impl CellValue {
    /// Create default value for a cell type
    pub fn default_for_type(cell_type: &CellType) -> Self {
        match cell_type {
            CellType::DropDown => CellValue::DropDown(0),
            CellType::TextInput => CellValue::Text(String::new()),
            CellType::ColorPicker => CellValue::Color(vec4(0.29, 0.56, 0.89, 1.0)),
        }
    }

    /// Get as dropdown index
    pub fn as_dropdown(&self) -> Option<usize> {
        if let CellValue::DropDown(idx) = self {
            Some(*idx)
        } else {
            None
        }
    }

    /// Get as text
    pub fn as_text(&self) -> Option<&str> {
        if let CellValue::Text(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Get as color
    pub fn as_color(&self) -> Option<Vec4> {
        if let CellValue::Color(c) = self {
            Some(*c)
        } else {
            None
        }
    }
}

/// A single row of data
#[derive(Clone, Debug)]
pub struct FlexRow {
    /// Cell values indexed by column
    pub cells: Vec<CellValue>,
}

impl FlexRow {
    /// Create a new row with default values for each column
    pub fn new(columns: &[LiveColumnConfig]) -> Self {
        let cells = columns
            .iter()
            .map(|col| CellValue::default_for_type(&col.cell_type))
            .collect();
        Self { cells }
    }

    /// Get cell value at column index
    pub fn get(&self, col: usize) -> Option<&CellValue> {
        self.cells.get(col)
    }

    /// Set cell value at column index
    pub fn set(&mut self, col: usize, value: CellValue) {
        if col < self.cells.len() {
            self.cells[col] = value;
        }
    }
}

/// Action emitted by FlexibleDataTable
#[derive(Clone, Debug, DefaultNone)]
pub enum FlexibleDataTableAction {
    /// A cell value changed (row_index, column_index, new_value)
    CellChanged(usize, usize, CellValue),
    /// Add row button was clicked
    AddRowClicked,
    /// Minus row button was clicked (contains the row index that was removed)
    MinusRowClicked(usize),
    /// No action
    None,
}

/// A flexible data table widget with configurable columns and cell types
#[derive(Live, Widget)]
pub struct FlexibleDataTable {
    #[deref]
    view: View,

    /// Column configurations (set via DSL)
    #[live]
    live_columns: Vec<LiveColumnConfig>,

    /// Number of initial rows to create
    #[live(1)]
    initial_rows: i64,

    /// Hidden cells configuration (set via DSL)
    #[live]
    live_hidden_cells: Vec<LiveHiddenCell>,

    /// Row data
    #[rust]
    rows: Vec<FlexRow>,
}

impl LiveHook for FlexibleDataTable {
    //fn after_new_from_doc(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        // Only initialize once
        if self.live_columns.is_empty() {
            self.live_columns = vec![
                LiveColumnConfig {
                    name: "Name".to_string(),
                    cell_type: CellType::DropDown,
                    width: 100.0,
                    dropdown_labels: "one,two,three".to_string(),
                },
                LiveColumnConfig {
                    name: "Text".to_string(),
                    cell_type: CellType::TextInput,
                    width: 100.0,
                    dropdown_labels: String::new(),
                },
                LiveColumnConfig {
                    name: "Color".to_string(),
                    cell_type: CellType::ColorPicker,
                    width: 150.0,
                    dropdown_labels: String::new(),
                },
            ];
        }

        // Create initial rows
        for _ in 0..self.initial_rows.max(0) as usize {
            self.rows.push(FlexRow::new(&self.live_columns));
        }
        
    }
}

// Static cell IDs for accessing cells
const CELL_IDS: [&[LiveId]; MAX_COLUMNS] = [
    ids!(cell_0), ids!(cell_1), ids!(cell_2), ids!(cell_3), ids!(cell_4),
    ids!(cell_5), ids!(cell_6), ids!(cell_7), ids!(cell_8), ids!(cell_9),
];

const HEADER_IDS: [&[LiveId]; MAX_COLUMNS] = [
    ids!(header_0), ids!(header_1), ids!(header_2), ids!(header_3), ids!(header_4),
    ids!(header_5), ids!(header_6), ids!(header_7), ids!(header_8), ids!(header_9),
];

impl Widget for FlexibleDataTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Setup headers first
        self.setup_headers(cx.cx);

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.rows.len());

                while let Some(row_idx) = list.next_visible_item(cx) {
                    if row_idx < self.rows.len() {
                        let row_item = list.item(cx, row_idx, live_id!(FlexTableRow));

                        // Set row background alternating color
                        row_item.apply_over(cx.cx, live! {
                            draw_bg: { row_index: (row_idx as f32) }
                        });

                        // Draw each cell based on column config
                        self.draw_row_cells(cx.cx, &row_item, row_idx);

                        row_item.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl WidgetMatchEvent for FlexibleDataTable {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // Handle Add Row button click
        if self.view.button(ids!(add_row_btn)).clicked(actions) {
            self.add_row();
            self.redraw(cx);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                FlexibleDataTableAction::AddRowClicked,
            );
        }

        // Handle cell changes and minus button in the portal list
        let list_widget = self.view.portal_list(ids!(rows_list));

        // Collect row to remove (only one per action cycle)
        let mut row_to_remove: Option<usize> = None;

        for (row_idx, row_widget) in list_widget.items_with_actions(actions) {
            if row_idx >= self.rows.len() {
                continue;
            }

            // Check if minus button was clicked for this row
            if row_widget.button(ids!(minus_btn)).clicked(actions) {
                row_to_remove = Some(row_idx);
            }

            // Check each column for changes
            for (col_idx, col_config) in self.live_columns.iter().enumerate() {
                if col_idx >= MAX_COLUMNS {
                    break;
                }

                let cell = row_widget.view(CELL_IDS[col_idx]);

                match col_config.cell_type {
                    CellType::DropDown => {
                        let dropdown = cell.drop_down(ids!(dropdown));
                        if let Some(selected) = dropdown.changed(actions) {
                            self.rows[row_idx].set(col_idx, CellValue::DropDown(selected));
                            cx.widget_action(
                                self.widget_uid(),
                                &scope.path,
                                FlexibleDataTableAction::CellChanged(
                                    row_idx,
                                    col_idx,
                                    CellValue::DropDown(selected),
                                ),
                            );
                            cx.redraw_all();
                        }
                    }
                    CellType::TextInput => {
                        let text_input = cell.text_input(ids!(text_input));
                        if let Some(text) = text_input.changed(actions) {
                            self.rows[row_idx].set(col_idx, CellValue::Text(text.clone()));
                            cx.widget_action(
                                self.widget_uid(),
                                &scope.path,
                                FlexibleDataTableAction::CellChanged(
                                    row_idx,
                                    col_idx,
                                    CellValue::Text(text),
                                ),
                            );
                            cx.redraw_all();
                        }
                    }
                    CellType::ColorPicker => {
                        // Access color picker directly (not through wrapper view)
                        let color_picker = cell.mp_color_picker(ids!(color_picker));
                        if let Some(hsv) = color_picker.changed(actions) {
                            let color = hsv.to_vec4();
                            self.rows[row_idx].set(col_idx, CellValue::Color(color));
                            cx.widget_action(
                                self.widget_uid(),
                                &scope.path,
                                FlexibleDataTableAction::CellChanged(
                                    row_idx,
                                    col_idx,
                                    CellValue::Color(color),
                                ),
                            );
                            cx.redraw_all();
                        }
                    }
                }
            }
        }

        // Remove the row after the loop to avoid borrow issues
        if let Some(row_idx) = row_to_remove {
            self.rows.remove(row_idx);
            self.redraw(cx);
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                FlexibleDataTableAction::MinusRowClicked(row_idx),
            );
        }
    }
}

impl FlexibleDataTable {
    /// Setup header labels and visibility
    fn setup_headers(&mut self, cx: &mut Cx) {
        for i in 0..MAX_COLUMNS {
            let header = self.view.label(HEADER_IDS[i]);
            if i < self.live_columns.len() {
                header.set_text(cx, &self.live_columns[i].name);
                header.set_visible(cx, true);
                header.apply_over(cx, live! {
                    width: (self.live_columns[i].width)
                });
            } else {
                header.set_visible(cx, false);
            }
        }
    }

    /// Check if a cell is in the hidden list
    fn is_cell_hidden(&self, row_idx: usize, col_idx: usize) -> bool {
        self.live_hidden_cells.iter().any(|hc| hc.row as usize == row_idx && hc.col as usize == col_idx)
    }

    /// Draw cells for a single row
    fn draw_row_cells(&self, cx: &mut Cx, row_widget: &WidgetRef, row_idx: usize) {
        let row_data = &self.rows[row_idx];

        for col_idx in 0..MAX_COLUMNS {
            let cell = row_widget.view(CELL_IDS[col_idx]);

            if col_idx < self.live_columns.len() {
                let col_config = &self.live_columns[col_idx];
                let cell_value = row_data.get(col_idx);

                // Set cell width and make visible
                cell.set_visible(cx, true);
                cell.apply_over(cx, live! {
                    width: (col_config.width)
                });

                // Check if this cell should have its view hidden
                let is_hidden = self.is_cell_hidden(row_idx, col_idx);

                // Configure and show the appropriate widget type
                let dropdown = cell.drop_down(ids!(dropdown));
                let text_input = cell.text_input(ids!(text_input));
                let color_picker = cell.mp_color_picker(ids!(color_picker));
                let dropdown_view = cell.view(ids!(dropdown_view));
                let text_input_view = cell.view(ids!(text_input_view));

                if is_hidden {
                    // Hide all views for this cell
                    dropdown_view.set_visible(cx, false);
                    text_input_view.set_visible(cx, false);
                    color_picker.set_visible(cx, false);
                } else {
                    match col_config.cell_type {
                        CellType::DropDown => {
                            dropdown_view.set_visible(cx, true);
                            text_input_view.set_visible(cx, false);
                            color_picker.set_visible(cx, false);

                            dropdown.set_labels(cx, col_config.get_dropdown_labels());
                            if let Some(CellValue::DropDown(idx)) = cell_value {
                                dropdown.set_selected_item(cx, *idx);
                            }
                        }
                        CellType::TextInput => {
                            dropdown_view.set_visible(cx, false);
                            text_input_view.set_visible(cx, true);
                            color_picker.set_visible(cx, false);

                            if let Some(CellValue::Text(text)) = cell_value {
                                text_input.set_text(cx, text);
                            }
                        }
                        CellType::ColorPicker => {
                            dropdown_view.set_visible(cx, false);
                            text_input_view.set_visible(cx, false);
                            color_picker.set_visible(cx, true);

                            if let Some(CellValue::Color(color)) = cell_value {
                                let hsv = Hsv::from_rgb(color.x, color.y, color.z, color.w);
                                color_picker.set_color(cx, hsv);
                            }
                        }
                    }
                }
            } else {
                // Hide unused cells
                cell.set_visible(cx, false);
            }
        }
    }

    /// Set the column configuration
    pub fn set_columns(&mut self, columns: Vec<LiveColumnConfig>) {
        self.live_columns = columns;
        // Clear existing rows since column structure changed
        self.rows.clear();
    }

    /// Get the column configuration
    pub fn get_columns(&self) -> &[LiveColumnConfig] {
        &self.live_columns
    }

    /// Add a new empty row
    pub fn add_row(&mut self) {
        let row = FlexRow::new(&self.live_columns);
        self.rows.push(row);
    }

    /// Add a row with pre-set values
    pub fn add_row_with_values(&mut self, values: Vec<CellValue>) {
        let mut row = FlexRow::new(&self.live_columns);
        for (i, value) in values.into_iter().enumerate() {
            row.set(i, value);
        }
        self.rows.push(row);
    }

    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Get all rows
    pub fn get_rows(&self) -> &[FlexRow] {
        &self.rows
    }

    /// Get a specific row
    pub fn get_row(&self, row_idx: usize) -> Option<&FlexRow> {
        self.rows.get(row_idx)
    }

    /// Set a cell value
    pub fn set_cell(&mut self, row_idx: usize, col_idx: usize, value: CellValue) {
        if let Some(row) = self.rows.get_mut(row_idx) {
            row.set(col_idx, value);
        }
    }

    /// Get a cell value
    pub fn get_cell(&self, row_idx: usize, col_idx: usize) -> Option<&CellValue> {
        self.rows.get(row_idx).and_then(|row| row.get(col_idx))
    }

    /// Clear all rows
    pub fn clear(&mut self) {
        self.rows.clear();
    }

    /// Delete a specific row
    pub fn delete_row(&mut self, row_idx: usize) {
        if row_idx < self.rows.len() {
            self.rows.remove(row_idx);
        }
    }

    /// Update dropdown options for a specific column
    pub fn set_dropdown_options(&mut self, col_idx: usize, labels: Vec<String>) {
        if let Some(col) = self.live_columns.get_mut(col_idx) {
            col.dropdown_labels = labels.join(",");
        }
    }

    /// Set the list of hidden cells (replaces existing list)
    pub fn set_hidden_cells(&mut self, cells: Vec<LiveHiddenCell>) {
        self.live_hidden_cells = cells;
    }

    /// Get the list of hidden cells
    pub fn get_hidden_cells(&self) -> &[LiveHiddenCell] {
        &self.live_hidden_cells
    }

    /// Add a cell to the hidden list
    pub fn add_hidden_cell(&mut self, row_idx: usize, col_idx: usize) {
        if !self.is_cell_hidden(row_idx, col_idx) {
            self.live_hidden_cells.push(LiveHiddenCell {
                row: row_idx as i64,
                col: col_idx as i64,
            });
        }
    }

    /// Remove a cell from the hidden list
    pub fn remove_hidden_cell(&mut self, row_idx: usize, col_idx: usize) {
        self.live_hidden_cells.retain(|hc| !(hc.row as usize == row_idx && hc.col as usize == col_idx));
    }

    /// Clear all hidden cells
    pub fn clear_hidden_cells(&mut self) {
        self.live_hidden_cells.clear();
    }
}

/// Extension methods for FlexibleDataTableRef
#[allow(dead_code)]
impl FlexibleDataTableRef {
    /// Set the column configuration and redraw
    pub fn set_columns(&self, cx: &mut Cx, columns: Vec<LiveColumnConfig>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_columns(columns);
            inner.redraw(cx);
        }
    }

    /// Get the column configuration
    pub fn get_columns(&self) -> Vec<LiveColumnConfig> {
        if let Some(inner) = self.borrow() {
            inner.get_columns().to_vec()
        } else {
            Vec::new()
        }
    }

    /// Add a new empty row and redraw
    pub fn add_row(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_row();
            inner.redraw(cx);
        }
    }

    /// Add a row with pre-set values and redraw
    pub fn add_row_with_values(&self, cx: &mut Cx, values: Vec<CellValue>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_row_with_values(values);
            inner.redraw(cx);
        }
    }

    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        if let Some(inner) = self.borrow() {
            inner.row_count()
        } else {
            0
        }
    }

    /// Get all rows data
    pub fn get_rows(&self) -> Vec<FlexRow> {
        if let Some(inner) = self.borrow() {
            inner.get_rows().to_vec()
        } else {
            Vec::new()
        }
    }

    /// Get a specific row
    pub fn get_row(&self, row_idx: usize) -> Option<FlexRow> {
        if let Some(inner) = self.borrow() {
            inner.get_row(row_idx).cloned()
        } else {
            None
        }
    }

    /// Set a cell value and redraw
    pub fn set_cell(&self, cx: &mut Cx, row_idx: usize, col_idx: usize, value: CellValue) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_cell(row_idx, col_idx, value);
            inner.redraw(cx);
        }
    }

    /// Get a cell value
    pub fn get_cell(&self, row_idx: usize, col_idx: usize) -> Option<CellValue> {
        if let Some(inner) = self.borrow() {
            inner.get_cell(row_idx, col_idx).cloned()
        } else {
            None
        }
    }

    /// Clear all rows and redraw
    pub fn clear(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear();
            inner.redraw(cx);
        }
    }

    /// Delete a specific row and redraw
    pub fn delete_row(&self, cx: &mut Cx, row_idx: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.delete_row(row_idx);
            inner.redraw(cx);
        }
    }

    /// Update dropdown options for a column and redraw
    pub fn set_dropdown_options(&self, cx: &mut Cx, col_idx: usize, labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_dropdown_options(col_idx, labels);
            inner.redraw(cx);
        }
    }

    /// Check if add_row_btn was clicked
    pub fn add_row_clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast(), FlexibleDataTableAction::AddRowClicked)
        } else {
            false
        }
    }

    /// Check if minus_row_btn was clicked, returns the removed row index
    pub fn minus_row_clicked(&self, actions: &Actions) -> Option<usize> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let FlexibleDataTableAction::MinusRowClicked(idx) = item.cast() {
                return Some(idx);
            }
        }
        None
    }

    /// Check if any cell changed, returns (row_index, col_index, new_value)
    pub fn cell_changed(&self, actions: &Actions) -> Option<(usize, usize, CellValue)> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let FlexibleDataTableAction::CellChanged(row, col, value) = item.cast() {
                return Some((row, col, value));
            }
        }
        None
    }

    /// Set the list of hidden cells and redraw
    pub fn set_hidden_cells(&self, cx: &mut Cx, cells: Vec<LiveHiddenCell>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_hidden_cells(cells);
            inner.redraw(cx);
        }
    }

    /// Get the list of hidden cells
    pub fn get_hidden_cells(&self) -> Vec<LiveHiddenCell> {
        if let Some(inner) = self.borrow() {
            inner.get_hidden_cells().to_vec()
        } else {
            Vec::new()
        }
    }

    /// Add a cell to the hidden list and redraw
    pub fn add_hidden_cell(&self, cx: &mut Cx, row_idx: usize, col_idx: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_hidden_cell(row_idx, col_idx);
            inner.redraw(cx);
        }
    }

    /// Remove a cell from the hidden list and redraw
    pub fn remove_hidden_cell(&self, cx: &mut Cx, row_idx: usize, col_idx: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.remove_hidden_cell(row_idx, col_idx);
            inner.redraw(cx);
        }
    }

    /// Clear all hidden cells and redraw
    pub fn clear_hidden_cells(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear_hidden_cells();
            inner.redraw(cx);
        }
    }
}
