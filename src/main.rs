use gifhg::Gifhg;
use gpui::{div, prelude::*, px, size, App, AppContext, Bounds, Size, WindowOptions};

pub mod gifhg;
pub mod canvas;

fn main() {
    App::new()
        .run(|cx: &mut AppContext| {
	    let options = WindowOptions {
		bounds: Some(Bounds::centered(None, size(px(640.), px(480.)), cx)),
		..Default::default()
	    };
	    cx.open_window(options, |cx| {
		cx.activate(true);
		cx.new_view(|cx| {
		    Gifhg::new()
		})
	    });
	});
}
