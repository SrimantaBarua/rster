// (C) 2019 Srimanta Barua <srimanta.barua1@gmail.com>

use rster::{PathBuilder, Point, Rster};
use std::fs::File;
use std::io::Write;


fn write_pgm(path: &str, width: usize, height: usize, data: &[u8]) {
    let mut f = File::create(path).expect("failed to create file");
    write!(f, "P5\n{} {}\n255\n", width, height).unwrap();
    f.write(data).unwrap();
}


fn main() {
    let mut rster = Rster::new(50, 50);
    let path_obj = PathBuilder::new(Point::new(5.0, 5.0))
        // Square
        .line_to(Point::new(5.0, 10.0))
        .line_to(Point::new(10.0, 10.0))
        .line_to(Point::new(10.0, 5.0))
        .line_to(Point::new(5.0, 5.0))
        // Triangle
        .move_to(Point::new(15.0, 10.0))
        .line_to(Point::new(20.0, 5.0))
        .line_to(Point::new(25.0, 10.0))
        .line_to(Point::new(15.0, 10.0))
        .finish();
    rster.draw_path(path_obj.iter());
    let bitmap = rster.accumulate();
    write_pgm("shapes.pgm", 50, 50, &bitmap);
}
