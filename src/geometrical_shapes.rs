use rand::Rng;
use raster::Color;

// --- Traits ---

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut impl Displayable);
    fn color(&self) -> Color;
}

// --- Helper Functions ---

/// Draws a line between two points using Bresenham's Line Algorithm.
fn draw_line_pixels(p1: &Point, p2: &Point, color: Color, image: &mut impl Displayable) {
    let mut x0 = p1.x;
    let mut y0 = p1.y;
    let x1 = p2.x;
    let y1 = p2.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        image.display(x0, y0, color.clone());
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

// --- Primitive Structures & Implementations ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut impl Displayable) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::rgb(255, 69, 0)
    }
}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {

    pub fn random(width: i32, height: i32) -> Self {
        Line {
            p1: Point::random(width, height),
            p2: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut impl Displayable) {
        draw_line_pixels(&self.p1, &self.p2, self.color(), image);
    }

    fn color(&self) -> Color {
        Color::rgb(50, 205, 50)
    }
}

pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle { p1: *p1, p2: *p2, p3: *p3 }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut impl Displayable) {
        let c = self.color();
        draw_line_pixels(&self.p1, &self.p2, c.clone(), image);
        draw_line_pixels(&self.p2, &self.p3, c.clone(), image);
        draw_line_pixels(&self.p3, &self.p1, c, image);
    }

    fn color(&self) -> Color {
        Color::rgb(30, 144, 255)
    }
}

pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);
        
        Rectangle {
            p1: Point::new(min_x, min_y),
            p2: Point::new(max_x, max_y),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut impl Displayable) {
        let c = self.color();
        let tl = Point::new(self.p1.x, self.p1.y);
        let tr = Point::new(self.p2.x, self.p1.y);
        let br = Point::new(self.p2.x, self.p2.y);
        let bl = Point::new(self.p1.x, self.p2.y);

        draw_line_pixels(&tl, &tr, c.clone(), image);
        draw_line_pixels(&tr, &br, c.clone(), image);
        draw_line_pixels(&br, &bl, c.clone(), image);
        draw_line_pixels(&bl, &tl, c, image);
    }

    fn color(&self) -> Color {
        Color::rgb(255, 215, 0)
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let center = Point::random(width, height);
        let max_radius = (width.min(height) / 5).max(10);
        let radius = rng.gen_range(5..max_radius);
        Circle { center, radius }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut impl Displayable) {
        let xc = self.center.x;
        let yc = self.center.y;
        let r = self.radius;
        let c = self.color();

        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;

        let mut plot_circle_points = |x: i32, y: i32| {
            image.display(xc + x, yc + y, c.clone());
            image.display(xc - x, yc + y, c.clone());
            image.display(xc + x, yc - y, c.clone());
            image.display(xc - x, yc - y, c.clone());
            image.display(xc + y, yc + x, c.clone());
            image.display(xc - y, yc + x, c.clone());
            image.display(xc + y, yc - x, c.clone());
            image.display(xc - y, yc - x, c.clone());
        };

        plot_circle_points(x, y);
        while y >= x {
            x += 1;
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
            plot_circle_points(x, y);
        }
    }

    fn color(&self) -> Color {
        Color::rgb(238, 130, 238)
    }
}

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    struct MockImage {
        pub points: Vec<(i32, i32)>,
    }

    impl Displayable for MockImage {
        fn display(&mut self, x: i32, y: i32, _color: Color) {
            self.points.push((x, y));
        }
    }

    #[test]
    fn test_random_bounds() {
        let w = 500;
        let h = 400;
        for _ in 0..100 {
            let p = Point::random(w, h);
            assert!(p.x >= 0 && p.x < w, "Point X out of bounds: {}", p.x);
            assert!(p.y >= 0 && p.y < h, "Point Y out of bounds: {}", p.y);
            
            let l = Line::random(w, h);
            assert!(l.p1.x >= 0 && l.p1.x < w && l.p2.x >= 0 && l.p2.x < w);
            assert!(l.p1.y >= 0 && l.p1.y < h && l.p2.y >= 0 && l.p2.y < h);
        }
    }

    #[test]
    fn test_rectangle_normalization() {
        let p1 = Point::new(300, 400);
        let p2 = Point::new(100, 200);
        let rect = Rectangle::new(&p1, &p2);

        assert_eq!(rect.p1.x, 100);
        assert_eq!(rect.p1.y, 200);
        assert_eq!(rect.p2.x, 300);
        assert_eq!(rect.p2.y, 400);
    }

    #[test]
    fn test_line_edge_case_same_coordinate() {
        let p = Point::new(50, 50);
        let line = Line::new(&p, &p);
        let mut mock = MockImage { points: Vec::new() };

        line.draw(&mut mock);

        assert_eq!(mock.points.len(), 1);
        assert_eq!(mock.points[0], (50, 50));
    }
}