use canvas::{context::CanvasContext, layers::{Buffer, Layer, LayerKind}};
use gifhg::{Gifhg, GifhgState};
use gpui::{actions, div, prelude::*, px, size, App, AppContext, Bounds, Global, Menu, MenuItem, ModelContext, Size, WindowOptions};
use rfd::FileDialog;

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
	    cx.activate(true);
	    cx.on_action(import_image);
	    cx.set_menus(vec![Menu {
		name: "set_menus",
		items: vec![MenuItem::action("Quit", ImportImage)],
	    }]);
	    cx.set_global(GifhgState::new(canvas_cx));
	    cx.open_window(options, |cx| {
		cx.new_view(|cx| {
		    Gifhg::new()
		})
	    });
	});
}

actions!(set_menus, [ImportImage]);

fn import_image(_: &ImportImage, cx: &mut AppContext) {
    let file = FileDialog::new()
        .add_filter("image", &["jpg", "jpeg", "png", "webp"])
        .pick_file().unwrap();
    let state = cx.global::<GifhgState>().canvas.clone();
    cx.update_model(&state, |model, cx| {
	model.layers.push(Layer {
	    kind: LayerKind::Buffer(Buffer::from_img_path(file)),
	});
    });
}
