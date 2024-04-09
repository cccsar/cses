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

// use geometry::point_location_test::*;
use sort::collecting_numbers_2;
use templates::*;

use std::io;

fn main() {
    collecting_numbers_2::run();
}