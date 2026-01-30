use makepad_components::modal::{MpModalAction, MpModalWidgetWidgetRefExt};
use makepad_components::shell::*;
use makepad_widgets::*;
use makepad_widgets::makepad_platform::CxOsOp;

const CMD_COPY: u64 = 1;
const CMD_PASTE: u64 = 2;
const CMD_TOGGLE_GRID: u64 = 3;
const CMD_ZOOM_TO_FIT: u64 = 4;
const CMD_CLOSE_TO_TRAY: u64 = 5;
const CMD_ABOUT: u64 = 100;
const CMD_PREFERENCES: u64 = 101;
const CMD_QUIT: u64 = 102;
const APP_VERSION: &str = "0.1.0";

#[derive(Clone, Debug)]
struct ShellCommandAction(CommandId);

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_components::modal::*;
    use makepad_components::theme::colors::*;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {inner_size: vec2(1280, 800), title: "Makepad Component Shell Demo"},
                caption_bar = {
                    caption_label = {
                        label = {
                            margin: {left: 65},
                            align: {x: 0.5},
                            text: "Makepad Component Shell",
                        }
                    }
                    windows_buttons = {
                        // Note: these are the background colors of the buttons used in Windows:
                        // * idle: Clear, for all three buttons.
                        // * hover: #E9E9E9 for minimize and maximize, #E81123 for close.
                        // * down: either darker (on light mode) or lighter (on dark mode).
                        //
                        // However, the DesktopButton widget doesn't support drawing a background color yet,
                        // so these colors are the colors of the icon itself, not the background highlight.
                        // When it supports that, we will keep the icon color always black,
                        // and change the background color instead based on the above colors.
                        min   = { draw_bg: {color: #0, color_hover: #9, color_down: #3} }
                        max   = { draw_bg: {color: #0, color_hover: #9, color_down: #3} }
                        close = { draw_bg: {color: #0, color_hover: #E81123, color_down: #FF0015} }
                    }
                    draw_bg: {color: #F3F3F3},
                }
                
                body = <View> {
                    padding: 0,
                    
                    <View> {
                        width: Fill, height: Fill,
                        flow: Overlay,
                        show_bg: true,
                        draw_bg: {
                            color: #2b2b2b
                        }
    
                        <View> {
                            width: Fill, height: Fill,
                            spacing: 16,
                            flow: Down,
                            align: { x: 0.5, y: 0.5 },
                            <Label> {
                                draw_text: {color: #fff}
                                text: "App Menu / Tray / Context Menu demo (macOS)"
                            }
        
                            status_label = <Label> {
                                draw_text: {color: #9}
                                text: "Last command: (none)"
                            }
                        }
                        
                        // About modal overlay
                        about_modal = <MpModalWidget> {
                            backdrop = {
                                show_bg: false
                            }
                            content = {
                                dialog = <MpModal> {
                                    width: 400,
                                    header = {
                                        title = { text: "About Makepad Shell Demo" }
                                    }
                                    body = <View> {
                                        width: Fill,
                                        height: Fit,
                                        padding: { left: 24, right: 24, top: 0, bottom: 16 },
                                        flow: Down,
                                        spacing: 8,
                                        about_version = <Label> {
                                            width: Fill,
                                            height: Fit,
                                            draw_text: {
                                                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                                                wrap: Word
                                                color: (MUTED_FOREGROUND)
                                            }
                                            text: "Version:"
                                        }
                                    }
                                }
                            }
                        }
                    }
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
    #[rust]
    show_grid: bool,
    #[rust]
    last_command: Option<CommandId>,
    #[rust]
    tray: Option<TrayHandle>,
    #[rust]
    app_menu_installed: bool,
    #[rust]
    close_to_tray: bool,
    #[rust]
    hidden_via_tray: bool,
    #[rust]
    last_event: Option<String>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        makepad_components::live_design(cx);
    }
}

impl App {
    fn install_app_menu(&mut self) {
        #[cfg(target_os = "macos")]
        {
            let menu_bar = build_menu_bar(self.show_grid);
            let result = AppMenu::set(menu_bar, |cmd| {
                Cx::post_action(ShellCommandAction(cmd));
                log!("app menu invoked: {}", cmd.as_u64());
            });
            if result.is_ok() {
                self.app_menu_installed = true;
            }
        }
    }

    fn install_tray(&mut self) {
        #[cfg(target_os = "macos")]
        {
            if self.tray.is_some() {
                return;
            }

            let icon = tray_icon(self.show_grid);
            let menu = build_tray_menu(self.show_grid, self.close_to_tray);
            let model = TrayModel::new(icon, menu).with_tooltip("Shell demo tray");

            let result = Tray::create(
                model,
                |cmd| {
                    Cx::post_action(ShellCommandAction(cmd));
                    log!("tray menu invoked: {}", cmd.as_u64());
                },
                || {
                    log!("tray activated");
                },
            );

            if let Ok(handle) = result {
                self.tray = Some(handle);
            }
        }
    }

    fn open_context_menu(&mut self, _cx: &mut Cx, e: &MouseDownEvent) {
        let menu = build_context_menu(self.show_grid);
        #[cfg(target_os = "macos")]
        {
            let (ns_view, ns_event) = macos_context();
            let _ = ContextMenu::popup_macos(
                menu,
                MenuAnchor::Window {
                    x: e.abs.x as f32,
                    y: e.abs.y as f32,
                },
                MenuTrigger::MouseRight,
                ns_view,
                ns_event,
                |cmd| Cx::post_action(ShellCommandAction(cmd)),
            );
        }
    }

    fn apply_command(&mut self, cx: &mut Cx, cmd: CommandId) {
        self.last_event = Some(format!("Command {}", cmd.as_u64()));
        self.last_command = Some(cmd);
        match cmd.as_u64() {
            CMD_TOGGLE_GRID => {
                self.show_grid = !self.show_grid;
                if let Some(handle) = self.tray.as_mut() {
                    let _ = handle.update_menu(build_tray_menu(self.show_grid, self.close_to_tray));
                    let _ = handle.update_icon(tray_icon(self.show_grid));
                }
                self.install_app_menu();
            }
            CMD_CLOSE_TO_TRAY => {
                self.close_to_tray = !self.close_to_tray;
                if let Some(handle) = self.tray.as_mut() {
                    let _ = handle.update_menu(build_tray_menu(self.show_grid, self.close_to_tray));
                }
            }
            CMD_ABOUT => {
                let text = format!("Version: {}", APP_VERSION);
                self.ui.label(ids!(about_version)).set_text(cx, &text);
                self.ui.mp_modal_widget(ids!(about_modal)).open(cx);
                self.last_event = Some(format!("About (v{})", APP_VERSION));
            }
            CMD_QUIT => {
                cx.quit();
            }
            _ => {}
        }
        self.update_status_label(cx);
    }

    fn update_status_label(&mut self, cx: &mut Cx) {
        let text = if let Some(msg) = &self.last_event {
            format!("{msg} (show_grid: {}, close_to_tray: {})", self.show_grid, self.close_to_tray)
        } else {
            "Last command: (none)".to_string()
        };
        self.ui.label(ids!(status_label)).set_text(cx, &text);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        self.close_to_tray = true;
        self.install_app_menu();
        self.install_tray();
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        if let Event::WindowCloseRequested(ev) = event {
            if self.close_to_tray {
                ev.accept_close.set(false);
                self.hidden_via_tray = true;
                cx.push_unique_platform_op(CxOsOp::HideWindow(ev.window_id));
            }
        }
        if let Event::WindowGotFocus(_) = event {
            if self.hidden_via_tray {
                self.hidden_via_tray = false;
                self.last_event = Some("Restored from Dock/Tray".to_string());
                self.update_status_label(cx);
            }
        }
        self.install_app_menu();
        if let Event::MouseDown(e) = event {
            if e.button.is_secondary() {
                self.open_context_menu(cx, e);
            }
        }
        self.install_tray();
        if let Event::Actions(actions) = event {
            for action in actions {
                if let Some(cmd) = action.downcast_ref::<ShellCommandAction>() {
                    self.apply_command(cx, cmd.0);
                }
                if let MpModalAction::CloseRequested = action.as_widget_action().cast() {
                    self.ui.mp_modal_widget(ids!(about_modal)).close(cx);
                }
            }
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

fn build_context_menu(show_grid: bool) -> MenuModel {
    let mut toggle_grid = CommandItem::new(CommandId::new(CMD_TOGGLE_GRID).unwrap(), "Show Grid");
    toggle_grid.checked = show_grid;

    MenuModel::new(vec![
        MenuItem::Command(CommandItem::new(CommandId::new(CMD_COPY).unwrap(), "Copy")),
        MenuItem::Command(CommandItem::new(CommandId::new(CMD_PASTE).unwrap(), "Paste")),
        MenuItem::Separator,
        MenuItem::Command(toggle_grid),
        MenuItem::Submenu(Submenu::new(
            "View",
            vec![MenuItem::Command(CommandItem::new(
                CommandId::new(CMD_ZOOM_TO_FIT).unwrap(),
                "Zoom to Fit",
            ))],
        )),
    ])
}

fn build_menu_bar(show_grid: bool) -> MenuBarModel {
    let about = CommandItem::new(CommandId::new(CMD_ABOUT).unwrap(), "About Shell Demo");
    let prefs = CommandItem::new(CommandId::new(CMD_PREFERENCES).unwrap(), "Preferences…")
        .with_role(MenuItemRole::Preferences);
    let quit = CommandItem::new(CommandId::new(CMD_QUIT).unwrap(), "Quit Shell Demo")
        .with_role(MenuItemRole::Quit);

    let mut grid_toggle =
        CommandItem::new(CommandId::new(CMD_TOGGLE_GRID).unwrap(), "Show Grid");
    grid_toggle.checked = show_grid;

    let app_menu = TopMenu::new(
        "Shell Demo",
        vec![
            MenuItem::Command(about),
            MenuItem::Separator,
            MenuItem::Command(prefs),
            MenuItem::Separator,
            MenuItem::Command(quit),
        ],
    )
    .with_role(TopMenuRole::App);

    let file_menu = TopMenu::new(
        "File",
        vec![
            MenuItem::Command(CommandItem::new(CommandId::new(CMD_COPY).unwrap(), "Copy")),
            MenuItem::Command(CommandItem::new(CommandId::new(CMD_PASTE).unwrap(), "Paste")),
        ],
    )
    .with_role(TopMenuRole::File);

    let view_menu = TopMenu::new(
        "View",
        vec![
            MenuItem::Command(grid_toggle),
            MenuItem::Command(CommandItem::new(
                CommandId::new(CMD_ZOOM_TO_FIT).unwrap(),
                "Zoom to Fit",
            )),
        ],
    )
    .with_role(TopMenuRole::View);

    MenuBarModel::new(vec![app_menu, file_menu, view_menu])
}

fn build_tray_menu(show_grid: bool, close_to_tray: bool) -> MenuModel {
    let mut toggle_grid = CommandItem::new(CommandId::new(CMD_TOGGLE_GRID).unwrap(), "Show Grid");
    toggle_grid.checked = show_grid;
    toggle_grid.shortcut = Some(Shortcut {
        mods: Modifiers {
            meta: true,
            ..Modifiers::default()
        },
        key: Key::Char('g'),
    });

    let mut close_to_tray_item =
        CommandItem::new(CommandId::new(CMD_CLOSE_TO_TRAY).unwrap(), "Close to Tray");
    close_to_tray_item.checked = close_to_tray;

    MenuModel::new(vec![
        MenuItem::Command(CommandItem::new(CommandId::new(CMD_ABOUT).unwrap(), "About Shell Demo")),
        MenuItem::Command(toggle_grid),
        MenuItem::Command(close_to_tray_item),
        MenuItem::Separator,
        MenuItem::Command(
            CommandItem::new(CommandId::new(CMD_QUIT).unwrap(), "Quit").with_role(MenuItemRole::Quit),
        ),
    ])
}

fn tray_icon(show_grid: bool) -> TrayIcon {
    if show_grid {
        let bytes: &[u8] = include_bytes!("../assets/tray_alt.png");
        TrayIcon::from_png_bytes(bytes.to_vec()).with_template(true)
    } else {
        let bytes: &[u8] = include_bytes!("../assets/tray.png");
        TrayIcon::from_png_bytes(bytes.to_vec()).with_template(true)
    }
}

#[cfg(target_os = "macos")]
fn macos_context() -> (*mut core::ffi::c_void, *mut core::ffi::c_void) {
    use makepad_widgets::makepad_platform::os::apple::apple_sys::{
        class, msg_send, nil, sel, sel_impl, ObjcId,
    };
    use makepad_widgets::makepad_platform::os::apple::macos::macos_app::with_macos_app;

    let view: ObjcId = with_macos_app(|app| {
        app.cocoa_windows
            .first()
            .map(|(_, view)| *view)
            .unwrap_or(nil)
    });

    let ns_app: ObjcId = unsafe { msg_send![class!(NSApplication), sharedApplication] };
    let event: ObjcId = unsafe { msg_send![ns_app, currentEvent] };

    let view_ptr = if view == nil {
        core::ptr::null_mut()
    } else {
        view as *mut core::ffi::c_void
    };

    let event_ptr = if event == nil {
        core::ptr::null_mut()
    } else {
        event as *mut core::ffi::c_void
    };

    (view_ptr, event_ptr)
}
