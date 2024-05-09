use std::sync::Arc;

use gpui::{deferred, div, px, Corners, Element, ElementId, Empty, ImageData, Interactivity, IntoElement, LayoutId, Model, PaintQuad, Render, RenderOnce, Style};
use image::{Bgra, ImageBuffer, Rgb};

pub mod context;


pub struct Canvas {
    // a LOT of shit will need to go here.
    id: ElementId,
}

impl Canvas {
    pub fn new() -> Self {
	println!("shmop");
	Self {
	    id: ElementId::from("shmeep"),
	}
    }
}

impl Element for Canvas {
    type RequestLayoutState = ();

    type PrepaintState = ();

    fn id(&self) -> Option<gpui::ElementId> {
	println!("shmoop");
        Some(self.id.clone())
    }

    fn request_layout(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        cx: &mut gpui::WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
	println!("shmerp");
        (cx.request_layout(&Style::default(), []), ())
    }

    fn prepaint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut gpui::WindowContext,
    ) -> Self::PrepaintState {
        println!("shmourt");
    }

    fn paint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut gpui::WindowContext,
    ) {
        let buffer = ImageBuffer::from_pixel(1000, 1000, Bgra::from([0u8, 0, 0, 0xFF]));
	cx.paint_image(bounds, Corners::all(px(300.)), Arc::new(ImageData::new(buffer)), false).unwrap();
    }
}

impl IntoElement for Canvas {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
