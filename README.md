# drawing

A lightweight Rust graphics engine for rendering 2D primitives and 3D wireframe models to images and GIFs.

## Features

- **2D Primitives**: Points, Lines (Bresenham's algorithm), Circles (Midpoint algorithm), Triangles, Rectangles, and Regular Polygons.
- **3D Wireframe**: Cube rendering with 3D rotation and projection.
- **Image & GIF Export**: Render outputs to PNG images or animated GIFs.

## Quick Start

### Build & Run

```bash
cargo run --release
```

This generates demo outputs in the root directory:
- `image.png`: Benchmark composition with primitives.
- `polygons.png`: Regular polygon grid layout.
- `cube_animation.gif`: Animated rotating 3D cube.

### Run Tests

```bash
cargo test
```

## Usage Example

```rust
use drawing::geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(800, 800);

    // Draw primitives
    gs::Line::new(&gs::Point::new(10, 10), &gs::Point::new(790, 790)).draw(&mut image);
    gs::Circle::random(image.width, image.height).draw(&mut image);

    // Save result
    raster::save(&image, "output.png").unwrap();
}
```

## License

[MIT / Apache 2.0](LICENSE)
