mod geometrical_shapes;

use geometrical_shapes as gs;
use gif::{Encoder, Frame, Repeat, SetParameter};
use gs::Displayable;
use raster::{Color, Image};
use std::borrow::Cow;
use std::fs::{self, File};

fn main() {
    const WIDTH: i32 = 600;
    const HEIGHT: i32 = 600;
    const FRAMES: usize = 60;

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
        frame.delay = 4;
        frame.buffer = Cow::Owned(indexed_pixels(&image));
        encoder.write_frame(&frame).unwrap();
    }

    fs::copy("frames/cube_000.png", "image.png").unwrap();
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
