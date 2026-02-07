use makepad_widgets::*;

live_design!{
    link widgets;
    use link::theme::*;
    
    pub FoldHeaderDropDownBase = {{FoldHeaderDropDown}} {}
    pub FoldHeaderDropDown = <FoldHeaderDropDownBase> {
        width: Fill, height: Fit,
        body_walk: { width: Fill, height: Fit}
        flow: Down,
        
        animator: {
            active = {
                default: on
                off = {
                    from: {all: Forward {duration: 0.2}}
                    ease: ExpDecay {d1: 0.96, d2: 0.97}
                    redraw: true
                    apply: {
                        opened: [{time: 0.0, value: 1.0}, {time: 1.0, value: 0.0}]
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.2}}
                    ease: ExpDecay {d1: 0.98, d2: 0.95}
                    redraw: true
                    apply: {
                        opened: [{time: 0.0, value: 0.0}, {time: 1.0, value: 1.0}]
                    }
                }
            }
        }
    }
    
}

#[derive(Live, LiveHook, Widget)]
pub struct FoldHeaderDropDown {
    #[rust] draw_state: DrawStateWrap<DrawState>,
    #[rust] rect_size: f64,
    #[rust] area: Area,
    #[find] #[redraw] #[live] header: WidgetRef,
    #[find] #[redraw] #[live] body: WidgetRef,
    #[animator] animator: Animator,

    #[live] opened: f64,
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    #[live] body_walk: Walk,
    #[live] draw_list: DrawList2d,
}

#[derive(Clone)]
enum DrawState {
    DrawHeader,
    DrawBody
}

impl Widget for FoldHeaderDropDown {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            if self.animator.is_track_animating(cx, ids!(active)) {
                self.area.redraw(cx);
            }
        };
        
        self.header.handle_event(cx,  event, scope);
        if self.body.visible() {
            self.body.handle_event(cx, event, scope);
        }
        if let Event::Actions(actions) = event{
            match actions.find_widget_action(self.header.widget(ids!(fold_button)).widget_uid()).cast() {
                FoldButtonAction::Opening => {
                    println!("opening {}", self.opened);
                    //self.animator_play(cx, ids!(active.on));
                    self.opened = 1.0;
                    self.redraw(cx);
                }
                FoldButtonAction::Closing => {
                    println!("closing {:?}", self.opened);
                    //self.animator_play(cx, ids!(active.off));
                    self.opened = 0.0;
                    self.redraw(cx);
                }
                _ => ()
            }
        }
    }

    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope:&mut Scope, walk: Walk) -> DrawStep {
        if self.draw_state.begin(cx, DrawState::DrawHeader) {
            cx.begin_turtle(walk, self.layout);
        }
        if let Some(DrawState::DrawHeader) = self.draw_state.get() {
            let walk = self.header.walk(cx);
            self.header.draw_walk(cx, scope, walk)?;
            if self.opened == 0.0 || self.opened == 1.0 {
                self.redraw(cx);
            }
            self.draw_state.set(DrawState::DrawBody);
        }
        if self.opened < 0.1 {
            self.body.set_visible(cx, false);
        } else {
            self.body.set_visible(cx, true);
        }
        println!("begore DrawState::DrawBody");
        if let Some(DrawState::DrawBody) = self.draw_state.get() {
            println!("DrawState::DrawBody");
            self.draw_list.begin_overlay_reuse(cx);
            let size = cx.current_pass_size();
            cx.begin_root_turtle(size, Layout::flow_overlay());
            let header_area = self.header.area().rect(cx);
            let body_pos = DVec2 {
                x: header_area.pos.x,
                y: header_area.pos.y + header_area.size.y
            };

            let mut walk = self.body.walk(cx);
            walk.abs_pos = Some(body_pos);
            walk.width = Size::Fixed(header_area.size.x);
            walk.height = Size::fit();
            walk.margin.top = (1.0 - self.opened) * self.rect_size * -1.0;
            
            let _ = self.body.draw_walk(cx, scope, walk);
            
            // Store body height for animation
            let body_rect = self.body.area().rect(cx);
            self.rect_size = body_rect.size.y;

            self.draw_state.end();

            cx.end_pass_sized_turtle();
            self.draw_list.end(cx);
        }
        DrawStep::done()
    }
}