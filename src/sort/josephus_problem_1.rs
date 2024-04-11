use std::collections::VecDeque; 
use crate::read_anything;

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

#[cfg(test)]
pub mod test {
    use super::josephus;

    #[test]
    pub fn basic() {
        assert_eq!(josephus(7), [2,4,6,1,5,3,7]);
    }
}