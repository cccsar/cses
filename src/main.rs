mod intro;
mod sort;
mod dp;
mod graph;
mod templates;

use templates::*;
use graph::counting_rooms;



fn main() {
    let n : Vec<usize>  = read_collection();
    let mut inp : Vec<String> = vec![];

    (0..n[0]).for_each(|_| { inp.push( read_anything() );});


    counting_rooms::solve(inp.into_iter(), n[0]);
}
