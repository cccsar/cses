mod intro;
mod sort;
mod dp;
mod templates;

use templates::*;



fn main() {
    let n : isize = read_anything();
    let inp : Vec<isize> = read_collection(); 

    println!("{}", sort::distinct::solve(inp));
}
