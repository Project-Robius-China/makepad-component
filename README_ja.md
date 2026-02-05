# Makepad Component

[![バージョン](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/Project-Robius-China/makepad-component)
[![ライセンス](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](LICENSE)

**[English](README.md) | [中文](README_zh.md)**

[Makepad](https://github.com/makepad/makepad) 向けのモダンなUIコンポーネントライブラリ。Longbridge の [gpui-component](https://github.com/longbridge/gpui-component) からインスピレーションを得ています。

![Makepad Component プレビュー](asserts/mc1.png)

## Makepad について

[Makepad](https://github.com/makepad/makepad) は Rust で書かれた次世代 UI フレームワークで、以下の特徴があります：

- **GPU アクセラレーション描画** - SDF（符号付き距離場）を使用したカスタムシェーダーベースの描画
- **クロスプラットフォーム** - デスクトップ（Windows、macOS、Linux）、モバイル（iOS、Android）、Web（WebAssembly）
- **ライブデザイン** - ホットリロード対応の DSL で高速な UI イテレーション
- **高パフォーマンス** - IDE やリアルタイムツールなど、要求の厳しいアプリケーション向けに設計

### プロダクション事例

| プロジェクト | 説明 |
|-------------|------|
| [Robrix](https://github.com/project-robius/robrix) | Makepad で構築された Matrix チャットクライアント |
| [Moly](https://github.com/moxin-org/moly) | AI モデル管理・推論ツール |
| [Makepad Studio](https://github.com/makepad/makepad) | Makepad IDE 自体 |

これらのプロジェクトは [Robius](https://github.com/project-robius) イニシアチブの下で開発され、クロスプラットフォーム Rust GUI 開発を推進しています。

## スクリーンショット

| コンポーネント | Slider 機能 |
|----------------|-------------|
| ![コンポーネント](asserts/mc1.png) | ![Slider](asserts/mc2.png) |

| その他のコンポーネント | フルデモ |
|------------------------|----------|
| ![その他](asserts/mc3.png) | ![デモ](asserts/mc4.png) |

## 機能

### コンポーネント (v0.1.0)

Accordion、Alert、Avatar、Badge、Button、Calendar、Card、Checkbox、Color Picker、Divider、Drawer、Dropdown、Input、Label、Layout、Link、List、Modal、Notification、Page Flip、Popover、Progress、Radio、Skeleton、Slider、Spinner、Space、Switch、Tab、Table、Text、Tooltip。

### Shell 連携

このクレートは `makepad_components::shell` 経由で `makepad-shell` を再エクスポートし、アプリメニュー、トレイアイコン、コンテキストメニューに利用できます。makepad-shell リポジトリ：https://github.com/Project-Robius-China/makepad-shell。完全な例は `examples/shell-demo` を参照してください。

## インストール

`Cargo.toml` に追加：

```toml
[dependencies]
makepad-components = { git = "https://github.com/Project-Robius-China/makepad-component", branch = "main", features = ["Button", "Checkbox", "Switch", "Slider"] }
```

### Feature Flags

すべての UI コンポーネントは Cargo feature で管理されています。必要なものだけ有効化してください。

```toml
makepad-components = { git = "https://github.com/Project-Robius-China/makepad-component", branch = "main", features = ["Modal", "Button", "Input", "Tooltip"] }
```

注意：
- `all` feature はありません。必要なコンポーネントを列挙してください。
- `Modal` は `Button` に依存します（自動で有効化されます）。

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

                    <MpButtonPrimary> { text: "プライマリボタン" }
                    <MpCheckbox> { text: "チェックして" }
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

## デモの実行

```bash
# リポジトリをクローン
git clone https://github.com/Project-Robius-China/makepad-component
cd makepad-component

# コンポーネントズーデモを実行
cargo run -p component-zoo

# makepad-shell デモを実行（メニュー/トレイ/コンテキストメニュー）
cargo run -p shell-demo
```

---

## AI 支援開発

本コンポーネントライブラリは、[makepad-skills](https://github.com/ZhangHanDong/makepad-skills) を使用して AI（Claude Code）と共同で構築されました。

makepad-skills は、Makepad 開発向けに設計された Claude Code スキルセットで、ウィジェット作成、シェーダープログラミング、プロダクションパターンをカバーしています。

---

## インスピレーション

本プロジェクトは Longbridge の [gpui-component](https://github.com/longbridge/gpui-component) からインスピレーションを得ています。gpui-component は GPUI フレームワーク（Zed エディタで使用）向けのコンポーネントライブラリです。gpui-component が GPUI をターゲットにしているのに対し、**makepad-component** は同様のデザイン原則とコンポーネントパターンを Makepad エコシステムにもたらします。

主な違い：
- **Makepad** は `live_design!` DSL を使用、GPUI は純粋な Rust アプローチ
- **Makepad** は組み込みのシェーダー/アニメーションシステムを持つ
- **Makepad** はより多くのプラットフォーム（モバイル/Web を含む）をサポート

## コントリビューション

> **注意：** 本コンポーネントライブラリはまだ開発初期段階です。皆さんと一緒に作り上げていきたいと思います！

コントリビューションを歓迎します！issue や pull request をお気軽にお送りください。

## ライセンス

以下のいずれかのライセンスの下で提供されます：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) または http://opensource.org/licenses/MIT)

お好きな方をお選びください。
