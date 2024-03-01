// Required modules with problems
mod templates;
mod intro;
mod sort;
mod dp;
mod graph;
mod segtree;
mod tree;
mod math;
mod string;
mod geometry;

use geometry::point_location_test::*;
use templates::*;

use std::io;

fn main() {
    let n : usize = read_anything();
    for _ in 0..n {
        let points : Vec<isize> = read_collection();
        let result = geometry::point_location_test::solve(
            (points[0], points[1]),
            (points[2], points[3]),
            (points[4], points[5])
        );
        println!("{}", result);
    }

}