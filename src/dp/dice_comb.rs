
pub fn solve(n : isize) -> isize {
    let mut memo : Vec<isize> = vec![-1;(n+1) as usize]; 

    foo(n, 1, &mut memo)
}

pub fn foo(n : isize, idx : usize, memo : &mut Vec<isize>) -> isize {
    if n == 0 { return 1; }

    if n < 0 || idx == 7 { return 0; }

    if memo[n as usize] != -1 {
        return memo[n as usize] 
    }

    memo[n as usize] = (foo(n - idx as isize, idx + 1, memo) + foo(n, idx + 1, memo) ) % 1e9 as isize + 7 ;

    memo[n as usize] 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(solve(3), 4);
    }
}