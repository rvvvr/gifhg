use gpui::{div, rgb, Global, Model, ParentElement, Render, Styled};

use crate::canvas::{context::CanvasContext, Canvas};

pub struct GifhgState {
    canvas: Model<CanvasContext>,
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
            .flex()
            .size_full()
	    .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(Canvas::new())
    }
}
