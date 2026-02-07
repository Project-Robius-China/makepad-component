# Makepad Component

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/Project-Robius-China/makepad-component)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](LICENSE)

**[中文](README_zh.md) | [日本語](README_ja.md)**

A modern UI component library for [Makepad](https://github.com/makepad/makepad), inspired by Longbridge's [gpui-component](https://github.com/longbridge/gpui-component).

![Makepad Component Preview](asserts/mc1.png)

## About Makepad

[Makepad](https://github.com/makepad/makepad) is a next-generation UI framework written in Rust, featuring:

- **GPU-accelerated rendering** - Custom shader-based drawing with SDF (Signed Distance Field)
- **Cross-platform** - Desktop (Windows, macOS, Linux), Mobile (iOS, Android), and Web (WebAssembly)
- **Live design** - Hot-reload DSL for rapid UI iteration
- **High performance** - Designed for demanding applications like IDEs and real-time tools

### Production Applications

| Project | Description |
|---------|-------------|
| [Robrix](https://github.com/project-robius/robrix) | A Matrix chat client built with Makepad |
| [Moly](https://github.com/moxin-org/moly) | AI model manager and inference tool |
| [Makepad Studio](https://github.com/makepad/makepad) | The Makepad IDE itself |

These projects are developed under the [Robius](https://github.com/project-robius) initiative, advancing cross-platform Rust GUI development.

## Screenshots

| Components | Slider Features |
|------------|-----------------|
| ![Components](asserts/mc1.png) | ![Slider](asserts/mc2.png) |

| More Components | Full Demo |
|-----------------|-----------|
| ![More](asserts/mc3.png) | ![Demo](asserts/mc4.png) |

### Web Demo (WebAssembly)

![Web Demo](assets/component-zoo-web.png)

## Features

### Components (v0.1.0)

Accordion, Alert, Avatar, Badge, Button, Calendar, Card, Checkbox, Color Picker, Divider, Drawer, Dropdown, Input, Label, Layout, Link, List, Modal, Notification, Page Flip, Popover, Progress, Radio, Skeleton, Slider, Spinner, Space, Switch, Tab, Table, Text, Tooltip.

### Shell Integration

This crate re-exports `makepad-shell` via `makepad_components::shell` for app menus, tray icons, and context menus. The shell layer lives at https://github.com/Project-Robius-China/makepad-shell. See `examples/shell-demo` for a full example.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
makepad-components = { git = "https://github.com/Project-Robius-China/makepad-component", branch = "main", features = ["Button", "Checkbox", "Switch", "Slider"] }
```

### Feature Flags

All UI components are behind Cargo features. Enable only what you use:

```toml
makepad-components = { git = "https://github.com/Project-Robius-China/makepad-component", branch = "main", features = ["Modal", "Button", "Input", "Tooltip"] }
```

Notes:
- There is no `all` feature; list the components you want.
- `Modal` depends on `Button` (it is enabled automatically).

### Makepad Version Compatibility

`makepad-components` re-exports `makepad-widgets`, so the safest option is to use that single source in your app:

```rust
use makepad_components::makepad_widgets::*;
```

If your project also depends on `makepad-widgets` directly, make sure all crates resolve to the exact same Makepad source/revision. Otherwise Rust will treat them as different types and compilation will fail.

Recommended workspace setup:

```toml
[workspace.dependencies]
makepad-components = { git = "https://github.com/Project-Robius-China/makepad-component", rev = "YOUR_REV" }
makepad-widgets    = { git = "https://github.com/Project-Robius-China/makepad", rev = "SAME_MAKEPAD_REV" }
```

If a transitive dependency still pulls another Makepad revision, pin it with `patch`:

```toml
[patch."https://github.com/Project-Robius-China/makepad"]
makepad-widgets = { git = "https://github.com/Project-Robius-China/makepad", rev = "SAME_MAKEPAD_REV" }
```

To diagnose duplicates:

```bash
cargo tree -d
```

If duplicate `makepad-*` crates appear, unify them to one revision.

## Usage

```rust
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_components::button::*;
    use makepad_components::checkbox::*;
    use makepad_components::slider::*;
    use makepad_components::switch::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {
                    flow: Down, spacing: 20, padding: 20

                    <MpButtonPrimary> { text: "Primary Button" }
                    <MpCheckbox> { text: "Check me" }
                    <MpSwitch> {}
                    <MpSlider> { value: 50.0, min: 0.0, max: 100.0 }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        cx.link(live_id!(theme), live_id!(theme_desktop_light));
        cx.link(live_id!(theme_colors), live_id!(theme_colors_light));
        makepad_components::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
```

## Running the Demo

### Desktop

```bash
# Clone the repository
git clone https://github.com/Project-Robius-China/makepad-component
cd makepad-component

# Run the component zoo demo
cargo run -p component-zoo

# Run the makepad-shell demo (menus, tray, context menu)
cargo run -p shell-demo
```

### Web (WebAssembly)

```bash
# Install cargo-makepad (if not installed)
cargo install --force --git https://github.com/makepad/makepad.git --branch rik cargo-makepad

# Install wasm toolchain
cargo makepad wasm install-toolchain

# Build for web
cargo makepad wasm build -p component-zoo --release

# Serve locally (requires Python 3)
python3 serve_wasm.py 8080
# Open http://localhost:8080 in your browser
```

---

## AI-Assisted Development

This component library was built collaboratively with AI (Claude Code) using [makepad-skills](https://github.com/ZhangHanDong/makepad-skills).

makepad-skills is a comprehensive set of Claude Code skills designed for Makepad development, covering widget creation, shader programming, and production-ready patterns.

---

## Inspiration

This project draws inspiration from Longbridge's [gpui-component](https://github.com/longbridge/gpui-component), a component library for the GPUI framework (used in Zed editor). While gpui-component targets GPUI, **makepad-component** brings similar design principles and component patterns to the Makepad ecosystem.

Key differences:
- **Makepad** uses `live_design!` DSL vs GPUI's Rust-only approach
- **Makepad** has built-in shader/animation system
- **Makepad** targets more platforms (including mobile/web)

## Contributing

> **Note:** This component library is still in early development and needs your help to grow! We welcome contributors to build it together.

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
