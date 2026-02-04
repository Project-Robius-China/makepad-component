use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use link::theme_colors::*;

    // Hue slider - rainbow gradient
    MpHueSlider = {{MpHueSlider}} {
        width: Fill,
        height: 20,

        draw_slider: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;
                let r = sz.y * 0.5;

                // Draw rounded rect
                sdf.box(0.0, 0.0, sz.x, sz.y, r);

                // Rainbow gradient based on hue
                let h = self.pos.x;
                let rgb = Pal::hsv2rgb(vec4(h, 1.0, 1.0, 1.0));
                sdf.fill(rgb);

                return sdf.result;
            }
        }

        draw_thumb: {
            instance hover: 0.0
            instance pressed: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                // Shadow
                sdf.circle(c.x + 1.0, c.y + 1.0, c.x - 2.0);
                sdf.fill(vec4(0.0, 0.0, 0.0, 0.2));

                // Main circle
                sdf.circle(c.x, c.y, c.x - 2.0);
                let color = mix(#ffffff, #f0f0f0, self.hover);
                let color = mix(color, #e0e0e0, self.pressed);
                sdf.fill(color);
                sdf.stroke(#333333, 2.0);

                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_thumb: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_thumb: { hover: 1.0 } }
                }
            }
            pressed = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: { draw_thumb: { pressed: 0.0 } }
                }
                on = {
                    from: { all: Snap }
                    apply: { draw_thumb: { pressed: 1.0 } }
                }
            }
        }
    }

    // Saturation-Value picker area
    MpSVPicker = {{MpSVPicker}} {
        width: Fill,
        height: 200,

        draw_picker: {
            instance hue: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;

                // Draw rounded rect
                sdf.box(0.0, 0.0, sz.x, sz.y, 4.0);

                // SV gradient: x = saturation, y = value (inverted)
                let s = self.pos.x;
                let v = 1.0 - self.pos.y;
                let rgb = Pal::hsv2rgb(vec4(self.hue, s, v, 1.0));
                sdf.fill(rgb);

                return sdf.result;
            }
        }

        draw_thumb: {
            instance hover: 0.0
            instance pressed: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;

                // Outer ring (white)
                sdf.circle(c.x, c.y, c.x - 1.0);
                sdf.stroke(#ffffff, 2.0);

                // Inner ring (dark for contrast)
                sdf.circle(c.x, c.y, c.x - 3.0);
                sdf.stroke(#333333, 1.0);

                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_thumb: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_thumb: { hover: 1.0 } }
                }
            }
            pressed = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: { draw_thumb: { pressed: 0.0 } }
                }
                on = {
                    from: { all: Snap }
                    apply: { draw_thumb: { pressed: 1.0 } }
                }
            }
        }
    }

    // Color preview swatch
    MpColorSwatch = {{MpColorSwatch}} {
        width: 40,
        height: 40,

        draw_swatch: {
            instance color: #ff0000
            instance border_color: #cccccc

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;

                // Checkerboard pattern for transparency
                let checker_size = 5.0;
                let cx = floor(self.pos.x * sz.x / checker_size);
                let cy = floor(self.pos.y * sz.y / checker_size);
                let checker = mod(cx + cy, 2.0);
                let checker_color = mix(#ffffff, #cccccc, checker);

                // Draw rounded rect
                sdf.box(1.0, 1.0, sz.x - 2.0, sz.y - 2.0, 4.0);
                sdf.fill(checker_color);

                // Overlay with actual color
                let final_color = mix(checker_color, self.color, self.color.w);
                sdf.fill(vec4(final_color.xyz, 1.0));

                // Border
                sdf.stroke(self.border_color, 1.0);

                return sdf.result;
            }
        }
    }

    // Preset color item
    MpPresetColor = {{MpPresetColor}} {
        width: 24,
        height: 24,

        draw_color: {
            instance color: #ff0000
            instance hover: 0.0
            instance selected: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;

                // Draw rounded rect
                sdf.box(1.0, 1.0, sz.x - 2.0, sz.y - 2.0, 4.0);
                sdf.fill(self.color);

                // Hover/selected border
                let border_alpha = max(self.hover * 0.5, self.selected);
                sdf.stroke(mix(self.color * 0.7, #3b82f6, self.selected), 2.0 * border_alpha);

                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_color: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_color: { hover: 1.0 } }
                }
            }
        }
    }

    // Main ColorPicker component
    pub MpColorPicker = {{MpColorPicker}} {
        width: 280,
        height: Fit,
        flow: Down,
        padding: 12,
        spacing: 12,

        show_bg: true,
        draw_bg: {
            color: #ffffff
            instance border_color: #e2e8f0
            instance radius: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;

                sdf.box(0.5, 0.5, sz.x - 1.0, sz.y - 1.0, self.radius);
                sdf.fill(self.color);
                sdf.stroke(self.border_color, 1.0);

                return sdf.result;
            }
        }

        sv_picker = <MpSVPicker> {}

        <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 12,
            align: { y: 0.5 }

            color_preview = <MpColorSwatch> {
                width: 48,
                height: 48,
            }

            <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 8,

                hue_slider = <MpHueSlider> {}

                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 4,
                    align: { y: 0.5 }

                    <Label> {
                        width: Fit,
                        text: "#"
                        draw_text: {
                            color: #64748b
                            text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                        }
                    }

                    hex_input = <TextInput> {
                        width: Fill,
                        height: 32,
                        padding: { left: 8, right: 8 }
                        text: "FF0000"
                        draw_bg: {
                            color: #f8fafc
                            instance border_color: #e2e8f0
                            instance radius: 4.0

                            fn pixel(self) -> vec4 {
                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                let sz = self.rect_size;
                                sdf.box(0.5, 0.5, sz.x - 1.0, sz.y - 1.0, self.radius);
                                sdf.fill(self.color);
                                sdf.stroke(self.border_color, 1.0);
                                return sdf.result;
                            }
                        }
                        draw_text: {
                            color: #0f172a
                            text_style: <THEME_FONT_REGULAR> { font_size: 12.0 }
                        }
                    }
                }
            }
        }

        // Preset colors section
        <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 8,

            <Label> {
                width: Fit,
                text: "Presets"
                draw_text: {
                    color: #64748b
                    text_style: <THEME_FONT_REGULAR> { font_size: 11.0 }
                }
            }

            preset_colors = <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 4,

                <MpPresetColor> { draw_color: { color: #ef4444 } }
                <MpPresetColor> { draw_color: { color: #f97316 } }
                <MpPresetColor> { draw_color: { color: #eab308 } }
                <MpPresetColor> { draw_color: { color: #22c55e } }
                <MpPresetColor> { draw_color: { color: #14b8a6 } }
                <MpPresetColor> { draw_color: { color: #3b82f6 } }
                <MpPresetColor> { draw_color: { color: #8b5cf6 } }
                <MpPresetColor> { draw_color: { color: #ec4899 } }
            }

            preset_colors_2 = <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 4,

                <MpPresetColor> { draw_color: { color: #000000 } }
                <MpPresetColor> { draw_color: { color: #374151 } }
                <MpPresetColor> { draw_color: { color: #6b7280 } }
                <MpPresetColor> { draw_color: { color: #9ca3af } }
                <MpPresetColor> { draw_color: { color: #d1d5db } }
                <MpPresetColor> { draw_color: { color: #e5e7eb } }
                <MpPresetColor> { draw_color: { color: #f3f4f6 } }
                <MpPresetColor> { draw_color: { color: #ffffff } }
            }
        }
    }
}

// ============================================================================
// Rust Implementation
// ============================================================================

/// HSV color representation
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Hsv {
    pub h: f32, // 0.0 - 1.0
    pub s: f32, // 0.0 - 1.0
    pub v: f32, // 0.0 - 1.0
}

impl Hsv {
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        Self { h, s, v }
    }

    pub fn to_rgb(&self) -> (f32, f32, f32) {
        let h = self.h * 6.0;
        let s = self.s;
        let v = self.v;

        let c = v * s;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = v - c;

        let (r, g, b) = match h as i32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        (r + m, g + m, b + m)
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let d = max - min;

        let h = if d == 0.0 {
            0.0
        } else if max == r {
            ((g - b) / d).rem_euclid(6.0) / 6.0
        } else if max == g {
            ((b - r) / d + 2.0) / 6.0
        } else {
            ((r - g) / d + 4.0) / 6.0
        };

        let s = if max == 0.0 { 0.0 } else { d / max };
        let v = max;

        Self { h, s, v }
    }

    pub fn to_hex(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!(
            "{:02X}{:02X}{:02X}",
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8
        )
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;

        Some(Self::from_rgb(r, g, b))
    }

    pub fn to_vec4(&self) -> Vec4 {
        let (r, g, b) = self.to_rgb();
        Vec4 { x: r, y: g, z: b, w: 1.0 }
    }
}

// ============================================================================
// MpHueSlider
// ============================================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpHueSliderAction {
    Changed(f32),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct MpHueSlider {
    #[redraw]
    #[live]
    draw_slider: DrawQuad,

    #[live]
    draw_thumb: DrawQuad,

    #[animator]
    animator: Animator,

    #[walk]
    walk: Walk,

    #[live(0.0)]
    hue: f32,

    #[rust]
    dragging: bool,

    #[rust]
    slider_area: Area,

    #[rust]
    thumb_area: Area,
}

impl Widget for MpHueSlider {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.slider_area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                if !self.dragging {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            Hit::FingerDown(fe) => {
                self.dragging = true;
                self.animator_play(cx, ids!(pressed.on));
                self.update_hue_from_position(cx, fe.abs, scope);
            }
            Hit::FingerMove(fe) => {
                if self.dragging {
                    self.update_hue_from_position(cx, fe.abs, scope);
                }
            }
            Hit::FingerUp(_) => {
                self.dragging = false;
                self.animator_play(cx, ids!(pressed.off));
                self.animator_play(cx, ids!(hover.off));
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        let thumb_size = rect.size.y;

        // Draw slider track
        self.draw_slider.draw_abs(cx, rect);
        self.slider_area = self.draw_slider.area();

        // Draw thumb
        let thumb_x = rect.pos.x + (rect.size.x - thumb_size) * self.hue as f64;
        let thumb_rect = Rect {
            pos: DVec2 { x: thumb_x, y: rect.pos.y },
            size: DVec2 { x: thumb_size, y: thumb_size },
        };
        self.draw_thumb.draw_abs(cx, thumb_rect);
        self.thumb_area = self.draw_thumb.area();

        DrawStep::done()
    }
}

impl MpHueSlider {
    fn update_hue_from_position(&mut self, cx: &mut Cx, pos: DVec2, scope: &mut Scope) {
        let rect = self.slider_area.rect(cx);
        let thumb_radius = rect.size.y / 2.0;
        let track_start = rect.pos.x + thumb_radius;
        let track_width = rect.size.x - rect.size.y;

        if track_width <= 0.0 {
            return;
        }

        let relative_x = (pos.x - track_start).clamp(0.0, track_width);
        let new_hue = (relative_x / track_width) as f32;

        if (new_hue - self.hue).abs() > f32::EPSILON {
            self.hue = new_hue;
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                MpHueSliderAction::Changed(self.hue),
            );
            self.redraw(cx);
        }
    }

    pub fn set_hue(&mut self, cx: &mut Cx, hue: f32) {
        self.hue = hue.clamp(0.0, 1.0);
        self.redraw(cx);
    }
}

impl MpHueSliderRef {
    pub fn set_hue(&self, cx: &mut Cx, hue: f32) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_hue(cx, hue);
        }
    }

    pub fn changed(&self, actions: &Actions) -> Option<f32> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let MpHueSliderAction::Changed(hue) = item.cast() {
                return Some(hue);
            }
        }
        None
    }
}

// ============================================================================
// MpSVPicker
// ============================================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpSVPickerAction {
    Changed { saturation: f32, value: f32 },
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct MpSVPicker {
    #[redraw]
    #[live]
    draw_picker: DrawQuad,

    #[live]
    draw_thumb: DrawQuad,

    #[animator]
    animator: Animator,

    #[walk]
    walk: Walk,

    #[live(0.0)]
    hue: f32,

    #[live(1.0)]
    saturation: f32,

    #[live(1.0)]
    value: f32,

    #[rust]
    dragging: bool,

    #[rust]
    picker_area: Area,

    #[rust]
    thumb_area: Area,
}

impl Widget for MpSVPicker {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.picker_area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Crosshair);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                if !self.dragging {
                    self.animator_play(cx, ids!(hover.off));
                }
            }
            Hit::FingerDown(fe) => {
                self.dragging = true;
                self.animator_play(cx, ids!(pressed.on));
                self.update_sv_from_position(cx, fe.abs, scope);
            }
            Hit::FingerMove(fe) => {
                if self.dragging {
                    self.update_sv_from_position(cx, fe.abs, scope);
                }
            }
            Hit::FingerUp(_) => {
                self.dragging = false;
                self.animator_play(cx, ids!(pressed.off));
                self.animator_play(cx, ids!(hover.off));
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        let thumb_size = 16.0;

        // Update hue in shader
        self.draw_picker.apply_over(cx, live! {
            hue: (self.hue)
        });

        // Draw picker area
        self.draw_picker.draw_abs(cx, rect);
        self.picker_area = self.draw_picker.area();

        // Draw thumb at current S/V position
        let thumb_x = rect.pos.x + rect.size.x * self.saturation as f64 - thumb_size / 2.0;
        let thumb_y = rect.pos.y + rect.size.y * (1.0 - self.value) as f64 - thumb_size / 2.0;

        let thumb_rect = Rect {
            pos: DVec2 { x: thumb_x, y: thumb_y },
            size: DVec2 { x: thumb_size, y: thumb_size },
        };
        self.draw_thumb.draw_abs(cx, thumb_rect);
        self.thumb_area = self.draw_thumb.area();

        DrawStep::done()
    }
}

impl MpSVPicker {
    fn update_sv_from_position(&mut self, cx: &mut Cx, pos: DVec2, scope: &mut Scope) {
        let rect = self.picker_area.rect(cx);

        let relative_x = (pos.x - rect.pos.x).clamp(0.0, rect.size.x);
        let relative_y = (pos.y - rect.pos.y).clamp(0.0, rect.size.y);

        let new_s = (relative_x / rect.size.x) as f32;
        let new_v = 1.0 - (relative_y / rect.size.y) as f32;

        if (new_s - self.saturation).abs() > f32::EPSILON
            || (new_v - self.value).abs() > f32::EPSILON
        {
            self.saturation = new_s;
            self.value = new_v;
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                MpSVPickerAction::Changed {
                    saturation: self.saturation,
                    value: self.value,
                },
            );
            self.redraw(cx);
        }
    }

    pub fn set_hue(&mut self, cx: &mut Cx, hue: f32) {
        self.hue = hue.clamp(0.0, 1.0);
        self.redraw(cx);
    }

    pub fn set_sv(&mut self, cx: &mut Cx, saturation: f32, value: f32) {
        self.saturation = saturation.clamp(0.0, 1.0);
        self.value = value.clamp(0.0, 1.0);
        self.redraw(cx);
    }
}

impl MpSVPickerRef {
    pub fn set_hue(&self, cx: &mut Cx, hue: f32) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_hue(cx, hue);
        }
    }

    pub fn set_sv(&self, cx: &mut Cx, saturation: f32, value: f32) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_sv(cx, saturation, value);
        }
    }

    pub fn changed(&self, actions: &Actions) -> Option<(f32, f32)> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let MpSVPickerAction::Changed { saturation, value } = item.cast() {
                return Some((saturation, value));
            }
        }
        None
    }
}

// ============================================================================
// MpColorSwatch
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct MpColorSwatch {
    #[redraw]
    #[live]
    draw_swatch: DrawQuad,

    #[walk]
    walk: Walk,

    #[rust]
    area: Area,
}

impl Widget for MpColorSwatch {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_swatch.draw_walk(cx, walk);
        self.area = self.draw_swatch.area();
        DrawStep::done()
    }
}

impl MpColorSwatch {
    pub fn set_color(&mut self, cx: &mut Cx, color: Vec4) {
        self.draw_swatch.apply_over(cx, live! {
            color: (color)
        });
        self.redraw(cx);
    }
}

impl MpColorSwatchRef {
    pub fn set_color(&self, cx: &mut Cx, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_color(cx, color);
        }
    }
}

// ============================================================================
// MpPresetColor
// ============================================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpPresetColorAction {
    Clicked(Vec4),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct MpPresetColor {
    #[redraw]
    #[live]
    draw_color: DrawQuad,

    #[animator]
    animator: Animator,

    #[walk]
    walk: Walk,

    #[live]
    color: Vec4,

    #[rust]
    area: Area,
}

impl Widget for MpPresetColor {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        MpPresetColorAction::Clicked(self.color),
                    );
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_color.draw_walk(cx, walk);
        self.area = self.draw_color.area();
        DrawStep::done()
    }
}

impl MpPresetColorRef {
    pub fn clicked(&self, actions: &Actions) -> Option<Vec4> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let MpPresetColorAction::Clicked(color) = item.cast() {
                return Some(color);
            }
        }
        None
    }
}

// ============================================================================
// MpColorPicker (Main Component)
// ============================================================================

#[derive(Clone, Debug, DefaultNone)]
pub enum MpColorPickerAction {
    Changed(Hsv),
    None,
}

#[derive(Live, LiveHook, Widget)]
pub struct MpColorPicker {
    #[deref]
    view: View,

    #[rust]
    hsv: Hsv,
}

impl Widget for MpColorPicker {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for MpColorPicker {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let sv_picker = self.view.mp_svpicker(ids!(sv_picker));
        let hue_slider = self.view.mp_hue_slider(ids!(hue_slider));
        let color_preview = self.view.mp_color_swatch(ids!(color_preview));
        let hex_input = self.view.text_input(ids!(hex_input));

        // Handle hue slider changes
        if let Some(hue) = hue_slider.changed(actions) {
            self.hsv.h = hue;
            sv_picker.set_hue(cx, hue);
            self.update_preview_and_input(cx, &color_preview, &hex_input);
            self.emit_change(cx, scope);
        }

        // Handle SV picker changes
        if let Some((s, v)) = sv_picker.changed(actions) {
            self.hsv.s = s;
            self.hsv.v = v;
            self.update_preview_and_input(cx, &color_preview, &hex_input);
            self.emit_change(cx, scope);
        }

        // Handle preset color clicks - iterate through all actions
        for action in actions {
            if let MpPresetColorAction::Clicked(color) = action.cast() {
                self.set_color_from_vec4(cx, color);
                self.emit_change(cx, scope);
                return;
            }
        }

        // Handle hex input changes
        if let Some(text) = hex_input.changed(actions) {
            if let Some(hsv) = Hsv::from_hex(&text) {
                self.hsv = hsv;
                hue_slider.set_hue(cx, hsv.h);
                sv_picker.set_hue(cx, hsv.h);
                sv_picker.set_sv(cx, hsv.s, hsv.v);
                color_preview.set_color(cx, hsv.to_vec4());
                self.emit_change(cx, scope);
            }
        }
    }
}

impl MpColorPicker {
    fn update_preview_and_input(
        &self,
        cx: &mut Cx,
        color_preview: &MpColorSwatchRef,
        hex_input: &TextInputRef,
    ) {
        color_preview.set_color(cx, self.hsv.to_vec4());
        hex_input.set_text(cx, &self.hsv.to_hex());
    }

    fn emit_change(&self, cx: &mut Cx, scope: &mut Scope) {
        cx.widget_action(
            self.widget_uid(),
            &scope.path,
            MpColorPickerAction::Changed(self.hsv),
        );
    }

    fn set_color_from_vec4(&mut self, cx: &mut Cx, color: Vec4) {
        let hsv = Hsv::from_rgb(color.x, color.y, color.z);
        self.set_color(cx, hsv);
    }

    pub fn set_color(&mut self, cx: &mut Cx, hsv: Hsv) {
        self.hsv = hsv;

        let sv_picker = self.view.mp_svpicker(ids!(sv_picker));
        let hue_slider = self.view.mp_hue_slider(ids!(hue_slider));
        let color_preview = self.view.mp_color_swatch(ids!(color_preview));
        let hex_input = self.view.text_input(ids!(hex_input));

        hue_slider.set_hue(cx, hsv.h);
        sv_picker.set_hue(cx, hsv.h);
        sv_picker.set_sv(cx, hsv.s, hsv.v);
        color_preview.set_color(cx, hsv.to_vec4());
        hex_input.set_text(cx, &hsv.to_hex());
    }

    pub fn color(&self) -> Hsv {
        self.hsv
    }

    pub fn hex(&self) -> String {
        self.hsv.to_hex()
    }
}

impl MpColorPickerRef {
    pub fn set_color(&self, cx: &mut Cx, hsv: Hsv) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_color(cx, hsv);
        }
    }

    pub fn set_hex(&self, cx: &mut Cx, hex: &str) {
        if let Some(hsv) = Hsv::from_hex(hex) {
            self.set_color(cx, hsv);
        }
    }

    pub fn color(&self) -> Option<Hsv> {
        self.borrow().map(|inner| inner.color())
    }

    pub fn hex(&self) -> Option<String> {
        self.borrow().map(|inner| inner.hex())
    }

    pub fn changed(&self, actions: &Actions) -> Option<Hsv> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let MpColorPickerAction::Changed(hsv) = item.cast() {
                return Some(hsv);
            }
        }
        None
    }
}
