mod geometrical_shapes;

use geometrical_shapes as gs;
use gif::{Encoder, Frame, Repeat, SetParameter};
use gs::{Displayable, Drawable};
use raster::{Color, Image};
use std::borrow::Cow;
use std::fs::{self, File};

fn main() {
    draw_original_image();
    draw_polygons_image();
    draw_rotating_cube_gif();
}

fn draw_original_image() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 300), &gs::Point::new(50, 60));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

fn draw_polygons_image() {
    let mut image = Image::blank(1000, 1000);

    let cols = 4;
    let cell_w = image.width / cols;
    let cell_h = image.height / 2;
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

    raster::save(&image, "polygons.png").unwrap();
}

fn draw_rotating_cube_gif() {
    const WIDTH: i32 = 600;
    const HEIGHT: i32 = 600;
    const FRAMES: usize = 120;

    fs::create_dir_all("frames").unwrap();

    let cube = gs::Cube::new(&gs::Point::new(220, 220), &gs::Point::new(380, 380), 160);
    let cube_color = Color::rgb(40, 220, 255);
    let palette = &[0, 0, 0, 40, 220, 255];
    let mut gif_file = File::create("cube_animation.gif").unwrap();
    let mut encoder = Encoder::new(&mut gif_file, WIDTH as u16, HEIGHT as u16, palette).unwrap();
    encoder.set(Repeat::Infinite).unwrap();

    for frame_number in 0..FRAMES {
        let mut image = Image::blank(WIDTH, HEIGHT);
        let angle = frame_number as f64 * 360.0 / FRAMES as f64;

        cube.draw_rotated(&mut image, angle, cube_color.clone());

        let frame_path = format!("frames/cube_{:03}.png", frame_number);
        raster::save(&image, &frame_path).unwrap();

        let mut frame = Frame::default();
        frame.width = WIDTH as u16;
        frame.height = HEIGHT as u16;
        frame.delay = 2;
        frame.buffer = Cow::Owned(indexed_pixels(&image));
        encoder.write_frame(&frame).unwrap();
    }
}

fn indexed_pixels(image: &Image) -> Vec<u8> {
    image
        .bytes
        .chunks(4)
        .map(|pixel| {
            if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
                0
            } else {
                1
            }
        })
        .collect()
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
