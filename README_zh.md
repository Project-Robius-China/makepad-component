# Makepad Component

[![版本](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/Project-Robius-China/makepad-component)
[![许可证](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](LICENSE)

**[English](README.md) | [日本語](README_ja.md)**

一个现代化的 [Makepad](https://github.com/makepad/makepad) UI 组件库，灵感来源于 longbridge [gpui-component](https://github.com/longbridge/gpui-component)。

![Makepad Component 预览1](asserts/mc1.png)

## 关于 Makepad

[Makepad](https://github.com/makepad/makepad) 是一个用 Rust 编写的下一代 UI 框架，具有以下特点：

- **GPU 加速渲染** - 基于自定义着色器的 SDF（有向距离场）绘制
- **跨平台** - 桌面端（Windows、macOS、Linux）、移动端（iOS、Android）和 Web（WebAssembly）
- **实时设计** - 支持热重载的 DSL，实现快速 UI 迭代
- **高性能** - 专为 IDE 和实时工具等高要求应用设计

### 生产级应用

| 项目 | 描述 |
|------|------|
| [Robrix](https://github.com/project-robius/robrix) | 使用 Makepad 构建的 Matrix 聊天客户端 |
| [Moly](https://github.com/moxin-org/moly) | AI 模型管理和推理工具 |
| [Makepad Studio](https://github.com/makepad/makepad) | Makepad IDE 本身 |

这些项目在 [Robius](https://github.com/project-robius) 计划下开发，推动跨平台 Rust GUI 开发的发展。

## 截图

| 组件展示 | Slider 功能 |
|----------|-------------|
| ![组件](asserts/mc1.png) | ![Slider](asserts/mc2.png) |

| 更多组件 | 完整演示 |
|----------|----------|
| ![更多](asserts/mc3.png) | ![演示](asserts/mc4.png) |

## 功能特性

### 组件 (v0.1.0)

Accordion、Alert、Avatar、Badge、Button、Calendar、Card、Checkbox、Color Picker、Divider、Drawer、Dropdown、Input、Label、Layout、Link、List、Modal、Notification、Page Flip、Popover、Progress、Radio、Skeleton、Slider、Spinner、Space、Switch、Tab、Table、Text、Tooltip。

### Shell 集成

本库通过 `makepad_components::shell` 重新导出 `makepad-shell`，用于应用菜单、托盘图标和右键菜单。makepad-shell 仓库：https://github.com/Project-Robius-China/makepad-shell。完整示例见 `examples/shell-demo`。

## 安装

添加到你的 `Cargo.toml`：

```toml
[dependencies]
makepad-components = { git = "https://github.com/Project-Robius-China/makepad-component", branch = "main", features = ["Button", "Checkbox", "Switch", "Slider"] }
```

### Feature Flags

所有组件都在 Cargo features 下，按需开启：

```toml
makepad-components = { git = "https://github.com/Project-Robius-China/makepad-component", branch = "main", features = ["Modal", "Button", "Input", "Tooltip"] }
```

说明：
- 没有 `all` feature，需要手动列出。
- `Modal` 依赖 `Button`（会自动启用）。

## 使用方法

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

                    <MpButtonPrimary> { text: "主要按钮" }
                    <MpCheckbox> { text: "选中我" }
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

## 运行演示

```bash
# 克隆仓库
git clone https://github.com/Project-Robius-China/makepad-component
cd makepad-component

# 运行组件动物园演示
cargo run -p component-zoo

# 运行 makepad-shell 演示（菜单/托盘/右键菜单）
cargo run -p shell-demo
```

---

## AI 辅助开发

本组件库基于 [makepad-skills](https://github.com/ZhangHanDong/makepad-skills) 与 AI（Claude Code）共同实现。

makepad-skills 是一套专为 Makepad 开发设计的 Claude Code 技能集，涵盖组件创建、着色器编写、生产级模式等最佳实践。

---

## 灵感来源

本项目的灵感来源于 longbridge [gpui-component](https://github.com/longbridge/gpui-component)，这是一个为 GPUI 框架（用于 Zed 编辑器）设计的组件库。虽然 gpui-component 面向 GPUI，但 **makepad-component** 将类似的设计原则和组件模式带入了 Makepad 生态系统。

主要区别：
- **Makepad** 使用 `live_design!` DSL，而 GPUI 采用纯 Rust 方式
- **Makepad** 内置着色器/动画系统
- **Makepad** 支持更多平台（包括移动端/Web）

## 贡献

> **注意：** 本组件库还未完善，需要大家一起共建！欢迎社区成员参与贡献。

欢迎贡献！请随时提交 issues 和 pull requests。

## 许可证

根据以下任一许可证授权：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) 或 http://opensource.org/licenses/MIT)

任选其一。
