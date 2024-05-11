use super::layers::Layer;

pub struct CanvasContext {
    pub layers: Vec<Layer>,
}

impl CanvasContext {
    pub fn new() -> Self {
	Self {
	    layers: vec![],
	}
    }
}
