pub fn main() {
    solve();
}

use std::fmt;
use std::io;
use std::str::FromStr;

/// Meant to be used as c++ cin
pub fn read_anything<F : FromStr>() -> F 
    where <F as FromStr>::Err : fmt::Debug 
{
    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .unwrap();

    generic_parse(&inp)
}

/// Given a string, convert it to the required type using typesystem
fn generic_parse<F : FromStr>(word : &str) -> F 
    where <F as FromStr>::Err : fmt::Debug 
{
    word
        .trim()  
        .parse() 
        .unwrap()
}

use std::collections::VecDeque; 

pub fn solve() {
    let n : usize = read_anything();
    let output = josephus(n);
    
    for el in output {
        print!("{el} ");
    }
    println!("");
}

pub fn josephus(n : usize) -> Vec<usize> {
    let mut help : VecDeque<usize> = VecDeque::from( (1..n+1).collect::<Vec<usize>>() );
    let mut result = vec![];

    while !help.is_empty() 
    {
        let to_back = help.pop_front().unwrap();

        help.push_back(to_back);

        let to_add = help.pop_front().unwrap();

        result.push(to_add);
    }

    return result;
}