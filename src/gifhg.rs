use gpui::{deferred, div, rgb, Global, InteractiveText, Model, ParentElement, Render, Styled, StyledText};

use crate::{canvas::{context::CanvasContext, Canvas}, ImportImage};

pub struct GifhgState {
    pub canvas: Model<CanvasContext>,
}

impl GifhgState {
    pub fn new(canvas: Model<CanvasContext>) -> Self {
	Self {
	    canvas,
	}
    }
}

impl Global for GifhgState {}

pub struct Gifhg {
    
}

impl Gifhg {
    pub fn new() -> Self {
	Self {
	    
	}
    }
}

impl Render for Gifhg {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
	println!("shmeep");
        div()
            .flex_col()
            .size_full()
	    .justify_center()
            .items_center()
            .text_xl()
            .bg(rgb(0xabcdef))
            .text_color(rgb(0xffffff))
            .child(
		div()
		    .bg(rgb(0xff0000))
		    .text_color(rgb(0xffffff))
		    .child(InteractiveText::new("shmeep", StyledText::new("shmeep")).on_click(vec![0..100000], |_, cx| {
			cx.dispatch_action(Box::new(ImportImage));
		    })))
            .child(Canvas::new().size_full())
    }
}
