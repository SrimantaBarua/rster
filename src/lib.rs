//! Rust library for generating grayscale bitmaps
// (C) 2019 Srimanta Barua <srimanta.barua1@gmail.com>

/// A 2-D point
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

/// A single path operation
#[derive(Clone, Debug)]
pub enum PathOp {
    Move(Point),
    Line(Point),
    QuadBez(Point, Point),
    CubBez(Point, Point, Point),
}

/// Utility for building paths
pub struct PathBuilder {
    ops: Vec<PathOp>,
}

impl PathBuilder {
    pub fn new(start: Point) -> PathBuilder {
        PathBuilder {
            ops: vec![PathOp::Move(start)],
        }
    }

    pub fn finish(self) -> PathObj {
        PathObj {
            ops: self.ops.into_boxed_slice(),
        }
    }

    pub fn move_to(mut self, point: Point) -> PathBuilder {
        self.ops.push(PathOp::Move(point));
        self
    }

    pub fn line_to(mut self, point: Point) -> PathBuilder {
        self.ops.push(PathOp::Line(point));
        self
    }

    pub fn quad_bez_to(mut self, ctrl: Point, end: Point) -> PathBuilder {
        self.ops.push(PathOp::QuadBez(ctrl, end));
        self
    }

    pub fn cub_bez_to(mut self, ctrl0: Point, ctrl1: Point, end: Point) -> PathBuilder {
        self.ops.push(PathOp::CubBez(ctrl0, ctrl1, end));
        self
    }
}

/// Storage for path segments, which can be used to draw a path multiple times
pub struct PathObj {
    ops: Box<[PathOp]>,
}

impl PathObj {
    pub fn iter(&self) -> std::slice::Iter<PathOp> {
        self.ops.iter()
    }
}

/// Handle to rasteriser
pub struct Rster {
    width: usize,
    height: usize,
    buf: Box<[f32]>,
}

impl Rster {
    /// Initialize a new rasteriser with given dimensions
    pub fn new(width: usize, height: usize) -> Rster {
        Rster {
            width: width,
            height: height,
            buf: vec![0.0; width * height + 8].into_boxed_slice(),
        }
    }

    /// Draw a line
    pub fn draw_line(&mut self, p0: Point, p1: Point) {
        assert!(
            p0.x >= 0.0 && p0.y >= 0.0 && p0.x <= self.width as f32 && p0.y <= self.height as f32
        );
        assert!(
            p1.x >= 0.0 && p1.y >= 0.0 && p1.x <= self.width as f32 && p1.y <= self.height as f32
        );
        // If we're on the same y coord, there's no need to draw
        if p0.y == p1.y {
            return;
        }
        // Always draw up (but keep track of direction)
        let (p0, p1, direction) = if p0.y < p1.y {
            (p0, p1, 1.0)
        } else {
            (p1, p0, -1.0)
        };
        let dxdy = (p1.x - p0.x) / (p1.y - p0.y);
        let (y0, y1) = (p0.y.floor() as usize, p1.y.ceil() as usize);
        let mut xhere = p0.x;
        for y in y0..y1 {
            let linestart = self.width * y;
            let dy = ((y + 1) as f32).min(p1.y) - (y as f32).max(p0.y);
            let dydir = dy * direction;
            let xnext = xhere + dy * dxdy;
            let (x0, x1) = if xhere < xnext {
                (xhere, xnext)
            } else {
                (xnext, xhere)
            };
            let (x0floor, x1ceil) = (x0.floor(), x1.ceil());
            let x0i = x0floor as usize;
            if x1ceil <= x0floor + 1.0 {
                // If x0 and x1 are within the same pixel, then area to the right is
                // (1 - (mid(x0, x1) - x0floor)) * dy
                let area = ((x0 + x1) * 0.5) - x0floor;
                self.buf[linestart + x0i] += dydir * (1.0 - area);
                self.buf[linestart + x0i + 1] += dydir * area;
            } else {
                let dydx = 1.0 / dxdy;
                let mut x0right = 1.0 - (x0 - x0floor);
                let x1_floor_i = x1.floor() as usize;
                let mut area_upto_here = 0.5 * x0right * x0right * dydx;
                self.buf[linestart + x0i] += direction * area_upto_here;
                for x in (x0i + 1)..x1_floor_i {
                    x0right += 1.0;
                    let total_area_here = 0.5 * x0right * x0right * dydx;
                    self.buf[linestart + x] += direction * (total_area_here - area_upto_here);
                    area_upto_here = total_area_here;
                }
                x0right = x1 - x0;
                let total_area = 0.5 * x0right * x0right * dydx;
                self.buf[linestart + x1_floor_i] += direction * (total_area - area_upto_here);
            }
            xhere = xnext;
        }
    }

    /// Draw a path
    pub fn draw_path<'a, I>(&mut self, path: I)
    where
        I: Iterator<Item = &'a PathOp>,
    {
        let mut path = path.peekable();
        let mut last_point = if let Some(op) = path.peek() {
            match op {
                PathOp::Move(p) => *p,
                _ => Point::new(0.0, 0.0),
            }
        } else {
            return;
        };
        for op in path {
            match op {
                PathOp::Move(p) => last_point = *p,
                PathOp::Line(p) => {
                    self.draw_line(last_point, *p);
                    last_point = *p;
                }
                _ => (),
            }
        }
    }

    /// Accumulate buffer data and generate bitmap
    pub fn accumulate(&self) -> Box<[u8]> {
        let mut acc = 0.0;
        self.buf[..(self.width * self.height)]
            .iter()
            .map(|f| {
                acc += f;
                let val = acc.abs();
                let val = if val > 1.0 { 1.0 } else { val };
                (val * 255.0) as u8
            })
            .collect::<Vec<u8>>()
            .into_boxed_slice()
    }
}