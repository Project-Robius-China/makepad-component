#![cfg(feature = "CommandPalette")]

use makepad_components::command_palette::{filter_and_rank_items, CommandPaletteItem};

fn sample_items() -> Vec<CommandPaletteItem> {
    vec![
        CommandPaletteItem::new(
            "open_file",
            "Open File",
            "File",
            "Ctrl+O",
            &["open", "file", "finder"],
        ),
        CommandPaletteItem::new(
            "open_folder",
            "Open Folder",
            "File",
            "Ctrl+K Ctrl+O",
            &["folder", "workspace"],
        ),
        CommandPaletteItem::new(
            "rename_symbol",
            "Rename Symbol",
            "Refactor",
            "F2",
            &["rename", "refactor"],
        ),
        CommandPaletteItem::new(
            "toggle_sidebar",
            "Toggle Sidebar",
            "View",
            "Ctrl+B",
            &["sidebar", "panel"],
        ),
    ]
}

#[test]
fn prefix_match_ranks_before_keyword_match() {
    let items = sample_items();
    let ranked = filter_and_rank_items(&items, "open");

    assert_eq!(ranked[0], 0);
    assert_eq!(ranked[1], 1);
}

#[test]
fn keyword_match_is_returned_when_title_does_not_match() {
    let items = sample_items();
    let ranked = filter_and_rank_items(&items, "panel");

    assert_eq!(ranked, vec![3]);
}

#[test]
fn empty_query_returns_all_items_in_original_order() {
    let items = sample_items();
    let ranked = filter_and_rank_items(&items, "");

    assert_eq!(ranked, vec![0, 1, 2, 3]);
}
