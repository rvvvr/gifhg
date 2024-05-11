use std::path::PathBuf;

use image::{io::Reader, Bgra, ImageBuffer};

#[derive(Clone)]
pub struct Layer {
    pub kind: LayerKind,
}

#[derive(Clone)]
pub enum LayerKind {
    Buffer(Buffer),
}

#[derive(Clone)]
pub struct Buffer {
    pub img: ImageBuffer<Bgra<u8>, Vec<u8>>,
}

impl Buffer {
    pub fn new(img: ImageBuffer<Bgra<u8>, Vec<u8>>) -> Self {
	Self {
	    img,
	}
    }

    pub fn from_img_path(path: PathBuf) -> Self {
	let img = Reader::open(path).unwrap().decode().unwrap().into_bgra8();
	Self {
	    img
	}
    }
}
