mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);
    gs::Point::new(500, 500).draw(&mut image);
    gs::Line::new(&gs::Point::new(100, 100), &gs::Point::new(700, 700)).draw(&mut image);
    gs::Circle::new(&gs::Point::new(500, 500), 50).draw(&mut image);
    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
