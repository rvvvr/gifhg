use gpui::{div, rgb, ParentElement, Render, Styled};

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
        div()
            .flex()
            .bg(rgb(0xabcdef))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child("hello world!")
    }
}
