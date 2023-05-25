
use crate::segtree::ds::Segtree;

pub fn solve(arr : &Vec<u64>, queries : &Vec<(usize, usize)>) -> Vec <u64> { 

    let mut ds = Segtree::build(&arr, arr.len(), 0, |x, y| x + y); 
    queries.iter().map( |(u, v)| 
        ds.query(0, arr.len() - 1, u-1, v-1, 0 )
    ).collect()
}