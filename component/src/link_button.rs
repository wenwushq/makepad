#![allow(unused)]
use {
    crate::{
        makepad_platform::*,
        button_logic::*,
        frame::*,
        button::Button
    }
};

live_register!{
    import makepad_platform::shader::std::*;
    import crate::theme::*;
    
    LinkButton: {{LinkButton}} {
        button: {
            bg: {
                const THICKNESS: 0.8
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let offset_y = 1.0
                    sdf.move_to(0., self.rect_size.y - offset_y);
                    sdf.line_to(self.rect_size.x, self.rect_size.y - offset_y);
                    return sdf.stroke(mix(
                        COLOR_TEXT_DEFAULT,
                        COLOR_TEXT_META,
                        self.pressed
                    ), mix(0.0, THICKNESS, self.hover));
                }
            }
            label: {
                text_style:FONT_META{}
                fn get_color(self) -> vec4 {
                    return mix(
                        mix(
                            COLOR_TEXT_META,
                            COLOR_TEXT_DEFAULT,
                            self.hover
                        ),
                        COLOR_TEXT_META,
                        self.pressed
                    )
                }
            }
            
            walk: {
                width: Size::Fit,
                height: Size::Fit,
                margin: {left: 5.0, top: 0.0, right: 5.0}
            }
            
            layout: {
                padding: {left: 1.0, top: 1.0, right: 1.0, bottom: 1.0}
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct LinkButton {
    button: Button
}

impl LinkButton {
    
    pub fn handle_event(&mut self, cx: &mut Cx, event: &Event, dispatch_action: &mut dyn FnMut(&mut Cx, ButtonAction),) {
        self.button.handle_event(cx, event, dispatch_action)
    }
    
    pub fn draw_label(&mut self, cx: &mut Cx2d, label: &str) {
        self.button.draw_label(cx, &label)
    }
}
