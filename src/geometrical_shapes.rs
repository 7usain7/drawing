use std::mem::swap;

use raster::Color;

pub trait Drawable {
    fn draw(&self, _canvas: &mut impl Displayable) {}
    fn color(&self) -> Color {
        Color::red()
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
    pub fn random(_width: i32, _height: i32) -> Point {
        // TODO - Use actual random function sample range and make sure its not the same point
        Point { x: 0, y: 0 }
    }
}

impl Drawable for Point {
    fn draw(&self, canvas: &mut impl Displayable) {
        canvas.display(self.x, self.y, self.color());
    }
}

pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Line {
        Line { p1: *p1, p2: *p2 }
    }

    pub fn random(width: i32, height: i32) -> Line {
        Line {
            p1: Point::random(width, height),
            p2: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &mut impl Displayable) {
        let mut p0 = self.p1;
        let mut p1 = self.p2;

        let steep = (p0.x - p1.x).abs() < (p0.y - p1.y).abs();
        if steep {
            swap(&mut p0.x, &mut p0.y);
            swap(&mut p1.x, &mut p1.y);
        }

        if p0.x > p1.x {
            swap(&mut p0.x, &mut p1.x);
            swap(&mut p0.y, &mut p1.y)
        }

        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;

        let derror2 = dy.abs() * 2;
        let mut error2 = 0;
        let mut y = p0.y;
        let mut x = p0.x;
        while x <= p1.x {
            if steep {
                canvas.display(y, x, self.color());
            } else {
                canvas.display(x, y, self.color());
            }
            error2 += derror2;
            if error2 > x {
                if p1.y > p0.y {
                    y += 1;
                } else {
                    y -= 1;
                }
                error2 -= dx * 2;
            }
            x += 1;
        }
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Circle {
        Circle {
            center: *center,
            radius: radius,
        }
    }

    pub fn random(_width: i32, _height: i32) -> Circle {
        Circle {
            center: Point::random(_width, _height),
            radius: 0,
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, canvas: &mut impl Displayable) {
        let xc = self.center.x;
        let yc = self.center.y;

        let mut x = 0;
        let mut y = self.radius;

        let mut d = 3 - (2 * self.radius);

        while x < y {
            canvas.display(xc + x, yc + y, self.color());
            canvas.display(xc + x, yc - y, self.color());
            canvas.display(xc - x, yc - y, self.color());
            canvas.display(xc - x, yc + y, self.color());
            canvas.display(xc + y, yc + x, self.color());
            canvas.display(xc + y, yc - x, self.color());
            canvas.display(xc - y, yc - x, self.color());
            canvas.display(xc - y, yc + x, self.color());

            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }
}
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Triangle {
        Triangle {
            p1: *p1,
            p2: *p2,
            p3: *p3,
        }
    }
}

impl Drawable for Triangle {}

pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Rectangle {
        Rectangle { p1: *p1, p2: *p2 }
    }
}

impl Drawable for Rectangle {}

#[cfg(test)]
mod tests {
    use super::*;
    use raster::Image;

    #[test]
    fn point_draw_sets_pixel() {
        let mut image = Image::blank(100, 100);
        let p = Point::new(10, 10);
        let expected = p.color();
        p.draw(&mut image);
        let pixel = image.get_pixel(10, 10).unwrap();
        assert_eq!(pixel.r, expected.r);
        assert_eq!(pixel.g, expected.g);
        assert_eq!(pixel.b, expected.b);
    }

    #[test]
    fn line_draw_same_point() {
        let mut image = Image::blank(100, 100);
        let p = Point::new(50, 50);
        let l = Line::new(&p, &p);
        // should not panic when both endpoints are identical
        l.draw(&mut image);
        let expected = l.color();
        let pixel = image.get_pixel(50, 50).unwrap();
        assert_eq!(pixel.r, expected.r);
        assert_eq!(pixel.g, expected.g);
        assert_eq!(pixel.b, expected.b);
    }

    #[test]
    fn line_draw_horizontal() {
        let mut image = Image::blank(100, 100);
        let p1 = Point::new(10, 10);
        let p2 = Point::new(20, 10);
        let l = Line::new(&p1, &p2);
        l.draw(&mut image);
        let expected = l.color();
        // midpoint should be painted
        let pixel = image.get_pixel(15, 10).unwrap();
        assert_eq!(pixel.r, expected.r);
        assert_eq!(pixel.g, expected.g);
        assert_eq!(pixel.b, expected.b);
    }
}
