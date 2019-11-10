// (C) 2019 Srimanta Barua <srimanta.barua1@gmail.com>

use rster::{PathBuilder, Point, Rster};
use std::fs::File;
use std::io::Write;
use std::time;

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

/*
*/
fn main() {
    let mut rster = Rster::new(120, 200);
    let path_obj = PathBuilder::new(Point::new(58.27865, 193.625))
        // Outer
        .quad_bez_to(Point::new(29.320314, 193.625), Point::new(14.720488, 169.125))
        .quad_bez_to(Point::new(0.0, 144.5), Point::new(0.0, 96.875))
        .quad_bez_to(Point::new(0.0, 49.125), Point::new(14.720488, 24.5))
        .quad_bez_to(Point::new(29.320314, 0.0), Point::new(58.27865, 0.0))
        .quad_bez_to(Point::new(87.23698, 0.0), Point::new(101.957466, 24.5))
        .quad_bez_to(Point::new(109.19705, 36.5), Point::new(112.9375, 54.4375))
        .quad_bez_to(Point::new(116.67795, 72.375), Point::new(116.67795, 96.875))
        .quad_bez_to(Point::new(116.67795, 121.375), Point::new(112.9375, 139.25))
        .quad_bez_to(Point::new(109.19705, 157.125), Point::new(101.957466, 169.125))
        .quad_bez_to(Point::new(87.11632, 193.625), Point::new(58.27865, 193.625))
        // Inner 1
        .move_to(Point::new(58.27865, 173.625))
        .quad_bez_to(Point::new(75.53298, 173.625), Point::new(83.858505, 154.625))
        .quad_bez_to(Point::new(92.18403, 135.75), Point::new(92.18403, 96.875))
        .quad_bez_to(Point::new(92.18403, 57.875), Point::new(83.858505, 39.125))
        .quad_bez_to(Point::new(75.53298, 20.0), Point::new(58.27865, 20.0))
        .quad_bez_to(Point::new(41.265625, 20.0), Point::new(32.9401, 39.0))
        .quad_bez_to(Point::new(28.717016, 48.375), Point::new(26.60547, 62.5625))
        .quad_bez_to(Point::new(24.493925, 76.75), Point::new(24.493925, 96.875))
        .quad_bez_to(Point::new(24.493925, 136.0), Point::new(32.9401, 154.625))
        .quad_bez_to(Point::new(41.386284, 173.625), Point::new(58.27865, 173.625))
        // Inner 2
        .move_to(Point::new(58.519966, 140.5))
        .quad_bez_to(Point::new(54.65885, 140.5), Point::new(52.547306, 135.125))
        .quad_bez_to(Point::new(50.43576, 129.75), Point::new(48.987846, 122.5))
        .quad_bez_to(Point::new(47.78125, 116.75), Point::new(46.996964, 108.625))
        .quad_bez_to(Point::new(46.212677, 100.5), Point::new(46.212677, 96.25))
        .quad_bez_to(Point::new(46.212677, 93.25), Point::new(46.695316, 85.5))
        .quad_bez_to(Point::new(47.177956, 77.75), Point::new(48.62587, 70.875))
        .quad_bez_to(Point::new(52.125, 53.0), Point::new(58.037323, 53.0))
        .quad_bez_to(Point::new(61.415794, 53.0), Point::new(63.76866, 57.75))
        .quad_bez_to(Point::new(66.12153, 62.5), Point::new(67.81076, 71.25))
        .quad_bez_to(Point::new(69.01736, 77.5), Point::new(69.74132, 85.25))
        .quad_bez_to(Point::new(70.46528, 93.0), Point::new(70.46528, 97.25))
        .quad_bez_to(Point::new(70.46528, 99.75), Point::new(70.04297, 107.125))
        .quad_bez_to(Point::new(69.62066, 114.5), Point::new(68.41406, 122.25))
        .quad_bez_to(Point::new(65.39757, 140.5), Point::new(58.519966, 140.5))
        .finish();
    println!(
        "draw: {:?}",
        do_with_time(|| { rster.draw_path(path_obj.iter()) })
    );
    println!(
        "accumulate: {:?}",
        do_with_time(|| {
            let _ = rster.accumulate();
        })
    );
    let bitmap = rster.accumulate();
    write_pgm("zero.pgm", 120, 200, &bitmap);
    write_reverse_pgm("reverse_zero.pgm", 120, 200, &bitmap);
}

fn do_with_time<F>(mut f: F) -> time::Duration
where
    F: FnMut(),
{
    let start = time::SystemTime::now();
    f();
    start.elapsed().unwrap()
}
