use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, mut _image: Image) {}
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&self, x: i32, y: i32, color: Color);
}

pub struct Point {}

pub struct Line {}

pub struct Triangle {}

pub struct Rectangle {}

pub struct Circle {}
