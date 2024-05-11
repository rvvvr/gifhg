use std::sync::Arc;

use gpui::{deferred, div, px, Bounds, Context, Corners, Element, ElementId, Empty, ImageData, Interactivity, IntoElement, LayoutId, Model, PaintQuad, Point, Refineable, Render, RenderOnce, Style, StyleRefinement, Styled};
use image::{Bgra, ImageBuffer, Rgb};

use crate::gifhg::GifhgState;

use self::layers::LayerKind;

pub mod context;
pub mod layers;


pub struct Canvas {
    // a LOT of shit will need to go here.
    id: ElementId,
    style: StyleRefinement,
}

impl Canvas {
    pub fn new() -> Self {
	println!("shmop");
	Self {
	    id: ElementId::from("shmeep"),
	    style: StyleRefinement::default(),
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
	let mut style = Style::default();
	style.refine(&self.style);
        (cx.request_layout(&style, []), ())
    }

    fn prepaint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut gpui::WindowContext,
    ) -> Self::PrepaintState {
        println!("{bounds:?}");
    }

    fn paint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        bounds: gpui::Bounds<gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut gpui::WindowContext,
    ) {
	let state = cx.global::<GifhgState>();
	let ctx = cx.read_model(&state.canvas, move |model, context| {
	   model.layers.iter().cloned().collect::<Vec<_>>()
	});
    }
}

impl IntoElement for Canvas {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for Canvas {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        &mut self.style
    }
}
