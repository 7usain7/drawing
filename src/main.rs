mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    // 8 polygons: triangle(3) to decagon(10), arranged in 2 rows x 4 cols
    let cols = 4;
    let cell_w = 1000 / cols;
    let cell_h = 1000 / 2;
    let radius = 100;

    for sides in 3u8..=10 {
        let idx = (sides - 3) as i32;
        let col = idx % cols;
        let row = idx / cols;
        let cx = cell_w * col + cell_w / 2;
        let cy = cell_h * row + cell_h / 2;
        let center = gs::Point::new(cx, cy);
        let angle = (90 - 180 / sides as i32) as u8;
        gs::Polygon::new(sides, &center, radius, angle).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
