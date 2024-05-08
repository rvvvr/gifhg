use gpui::{Element, ImageData, IntoElement, PaintQuad, Render, RenderOnce};
use image::{ImageBuffer, Rgb};

#[derive(IntoElement)]
pub struct Canvas {
    // a LOT of shit will need to go here.
}

impl Element for Canvas {
    type RequestLayoutState = ();

    type PrepaintState = ();

    fn id(&self) -> Option<gpui::ElementId> {
        todo!()
    }

    fn request_layout(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        cx: &mut gpui::WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        todo!()
    }

    fn prepaint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut gpui::WindowContext,
    ) -> Self::PrepaintState {
        todo!()
    }

    fn paint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut gpui::WindowContext,
    ) {
        let buffer = ImageBuffer::from_pixel(100, 100, Rgb::from([0u8, 0, 0]));
    }
}

impl RenderOnce for Canvas {
    fn render(self, cx: &mut gpui::WindowContext) -> impl IntoElement {
	self
    }
}
