use canvas::context::CanvasContext;
use gifhg::{Gifhg, GifhgState};
use gpui::{div, prelude::*, px, size, App, AppContext, Bounds, Global, ModelContext, Size, WindowOptions};

pub mod gifhg;
pub mod canvas;

fn main() {
    App::new()
        .run(|cx: &mut AppContext| {
	    let options = WindowOptions {
		bounds: Some(Bounds::centered(None, size(px(640.), px(480.)), cx)),
		..Default::default()
	    };
	    let canvas_cx = cx.new_model(|cx: &mut ModelContext<CanvasContext>| {
		CanvasContext::new()
	    });
	    cx.set_global(GifhgState::new(canvas_cx));
	    cx.open_window(options, |cx| {
		cx.activate(true);
		cx.new_view(|cx| {
		    Gifhg::new()
		})
	    });
	});
}
