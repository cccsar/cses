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

/// Read several (readable) items (of the same type) into a collection
pub fn read_collection<F : FromStr>() -> Vec<F>
    where <F as FromStr>:: Err : fmt::Debug 
{
    let mut inp = String::new();

    io::stdin()
        .read_line(&mut inp)
        .unwrap();

    inp.split_whitespace()
       .map(|word| generic_parse(word) ).collect()
}

pub fn solve(n : isize) -> Vec<isize> {
    let mut output : Vec<isize> = vec![n];
    let mut tmp = n;
    while tmp != 1
    {
        if tmp%2 == 0 { tmp = tmp/2; } 
        else { tmp = 3*tmp + 1; }

        output.push( tmp );
    }

    return output;
}

fn main() {
    let n : isize = read_anything();
    let resp = solve(n);
    resp.iter().for_each( |el| { print!("{} ", el); });
    println!();
}