// (C) 2019 Srimanta Barua <srimanta.barua1@gmail.com>

use rster::{PathBuilder, Point, Rster};
use std::fs::File;
use std::io::Write;


fn write_pgm(path: &str, width: usize, height: usize, data: &[u8]) {
    let mut f = File::create(path).expect("failed to create file");
    write!(f, "P5\n{} {}\n255\n", width, height).unwrap();
    f.write(data).unwrap();
}

fn write_reverse_pgm(path: &str, width: usize, height: usize, data: &[u8]) {
    let mut f = File::create(path).expect("failed to create file");
    write!(f, "P5\n{} {}\n255\n", width, height).unwrap();
    let rev_data = data.iter().map(|n| 255 - n).collect::<Vec<u8>>();
    f.write(&rev_data).unwrap();
}

fn main() {
    let mut rster = Rster::new(600, 600);
    let path_obj = PathBuilder::new(Point::new(50.0, 50.0))
        // Square
        .line_to(Point::new(50.0, 100.0))
        .line_to(Point::new(100.0, 100.0))
        .line_to(Point::new(100.0, 50.0))
        .line_to(Point::new(50.0, 50.0))
        // Triangle
        .move_to(Point::new(150.0, 100.0))
        .line_to(Point::new(200.0, 50.0))
        .line_to(Point::new(250.0, 100.0))
        .line_to(Point::new(150.0, 100.0))
        // Test
        // .move_to(Point::new(300.0, 500.0))
        // .line_to(Point::new(309.5, 480.0))
        // .line_to(Point::new(359.49997, 480.00008))
        // .line_to(Point::new(350.0, 500.0))
        // .line_to(Point::new(300.0, 500.0))
        // Curve "C"
        .move_to(Point::new(300.0, 100.0))
        .quad_bez_to(Point::new(400.0, 75.0), Point::new(300.0, 50.0))
        .line_to(Point::new(310.0, 50.0))
        .quad_bez_to(Point::new(410.0, 75.0), Point::new(310.0, 100.0))
        .line_to(Point::new(300.0, 100.0))
        .finish();
    rster.draw_path(path_obj.iter());
    let bitmap = rster.accumulate();
    write_pgm("shapes.pgm", 600, 600, &bitmap);
    write_reverse_pgm("reverse_shapes.pgm", 600, 600, &bitmap);
}
