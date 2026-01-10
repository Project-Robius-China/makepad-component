use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Progress bar component
    pub MpProgress = {{MpProgress}} {
        width: Fill,
        height: 8,

        draw_bg: {
            instance progress: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;
                let r = sz.y * 0.5;

                // Draw track (background capsule)
                sdf.circle(r, r, r);
                sdf.rect(r, 0.0, sz.x - sz.y, sz.y);
                sdf.circle(sz.x - r, r, r);

                let track_color = #e2e8f0;
                let fill_color = #3b82f6;

                sdf.fill(track_color);

                // Draw fill using position check
                let fill_end = sz.x * self.progress;
                let px = self.pos.x * sz.x;

                // If current pixel is within progress range, draw fill color
                let in_fill = step(px, fill_end);

                // Re-draw capsule shape for fill area
                let sdf2 = Sdf2d::viewport(self.pos * self.rect_size);
                sdf2.circle(r, r, r);
                sdf2.rect(r, 0.0, sz.x - sz.y, sz.y);
                sdf2.circle(sz.x - r, r, r);
                sdf2.fill(fill_color);

                // Blend based on whether we're in fill region
                let result = mix(sdf.result, sdf2.result, in_fill * sdf2.result.w);
                return result;
            }
        }
    }

    // Progress variants
    pub MpProgressSuccess = {{MpProgress}} {
        width: Fill,
        height: 8,
        draw_bg: {
            instance progress: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;
                let r = sz.y * 0.5;
                sdf.circle(r, r, r);
                sdf.rect(r, 0.0, sz.x - sz.y, sz.y);
                sdf.circle(sz.x - r, r, r);
                sdf.fill(#e2e8f0);

                let fill_end = sz.x * self.progress;
                let px = self.pos.x * sz.x;
                let in_fill = step(px, fill_end);

                let sdf2 = Sdf2d::viewport(self.pos * self.rect_size);
                sdf2.circle(r, r, r);
                sdf2.rect(r, 0.0, sz.x - sz.y, sz.y);
                sdf2.circle(sz.x - r, r, r);
                sdf2.fill(#22c55e);

                return mix(sdf.result, sdf2.result, in_fill * sdf2.result.w);
            }
        }
    }

    pub MpProgressWarning = {{MpProgress}} {
        width: Fill,
        height: 8,
        draw_bg: {
            instance progress: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;
                let r = sz.y * 0.5;
                sdf.circle(r, r, r);
                sdf.rect(r, 0.0, sz.x - sz.y, sz.y);
                sdf.circle(sz.x - r, r, r);
                sdf.fill(#e2e8f0);

                let fill_end = sz.x * self.progress;
                let px = self.pos.x * sz.x;
                let in_fill = step(px, fill_end);

                let sdf2 = Sdf2d::viewport(self.pos * self.rect_size);
                sdf2.circle(r, r, r);
                sdf2.rect(r, 0.0, sz.x - sz.y, sz.y);
                sdf2.circle(sz.x - r, r, r);
                sdf2.fill(#f59e0b);

                return mix(sdf.result, sdf2.result, in_fill * sdf2.result.w);
            }
        }
    }

    pub MpProgressDanger = {{MpProgress}} {
        width: Fill,
        height: 8,
        draw_bg: {
            instance progress: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let sz = self.rect_size;
                let r = sz.y * 0.5;
                sdf.circle(r, r, r);
                sdf.rect(r, 0.0, sz.x - sz.y, sz.y);
                sdf.circle(sz.x - r, r, r);
                sdf.fill(#e2e8f0);

                let fill_end = sz.x * self.progress;
                let px = self.pos.x * sz.x;
                let in_fill = step(px, fill_end);

                let sdf2 = Sdf2d::viewport(self.pos * self.rect_size);
                sdf2.circle(r, r, r);
                sdf2.rect(r, 0.0, sz.x - sz.y, sz.y);
                sdf2.circle(sz.x - r, r, r);
                sdf2.fill(#dc2626);

                return mix(sdf.result, sdf2.result, in_fill * sdf2.result.w);
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MpProgress {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,

    #[walk]
    walk: Walk,

    #[live(0.0)]
    value: f64,
}

impl Widget for MpProgress {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // Set progress via apply_over
        let progress = (self.value / 100.0).clamp(0.0, 1.0);
        self.draw_bg.apply_over(cx, live! {
            progress: (progress)
        });

        self.draw_bg.draw_walk(cx, walk);
        DrawStep::done()
    }
}

impl MpProgress {
    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, cx: &mut Cx, value: f64) {
        self.value = value.clamp(0.0, 100.0);
        self.redraw(cx);
    }
}

impl MpProgressRef {
    pub fn value(&self) -> f64 {
        if let Some(inner) = self.borrow() {
            inner.value
        } else {
            0.0
        }
    }

    pub fn set_value(&self, cx: &mut Cx, value: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_value(cx, value);
        }
    }
}
