// ! Based on https://cses.fi/problemset/task/1633

pub fn solve(n : isize) -> isize {
    let mut memo : Vec<isize> = vec![-1;(n+1) as usize]; 

    let output = foo(n, 1, &mut memo);

    println!("{}", memo[n as usize]);

    return output;
}

/// Recursive DP that doesn't quite meet time bounds
pub fn foo(n : isize, idx : usize, memo : &mut Vec<isize>) -> isize {
    if n == 0 { return 1; }

    if n < 0 || idx == 7 { return 0; }

    if memo[n as usize] != -1 { 
        return memo[n as usize]; 
    }

    // Either 
    // consider current dice `idx` by substracting it from `n` and starting over `idx` 
    // or consider the following `idx` for current `n`
    memo[n as usize] = (foo(n - idx as isize, 1, memo) + foo(n, idx + 1, memo) ) % (1e9 as isize + 7) ;

    memo[n as usize] 
}

/// Iterative DP based on the observation that the solution for any n > 6 is the sum of the
/// previous 6 values
pub fn solve_it(n : isize) -> isize {
    let mut memo : Vec<isize> = vec![1; 8 as usize];

    // Set base cases
    for i in 1 .. std::cmp::min(7, n + 1) as usize {
        memo[i] = 1<<(i-1);
    }

    // Every memo[i] equals the accumulation of the last 6 cases
    for i in 7..(n + 1) as usize {
        memo[i % 8] = ( ( (2 * memo[(i + 7) % 8 ]) % (1e9 as isize + 7) ) - memo[(i + 1) % 8] + 1e9 as isize + 7 ) % (1e9 as isize + 7);
    }

    memo[(n % 8) as usize]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rec_base() {
        assert_eq!(solve(3), 4);
    }

    #[test]
    fn rec_observe_past_base_case() {
        solve(10);
    }

    #[ignore]
    #[test]
    fn rec_big() {
        solve(1e5 as isize);
    }

    #[test]
    fn it_base() {
        assert_eq!(solve_it(3), 4);
    }

    #[test]
    fn it_observe_past_base_case() {
        solve_it(10);
    }

    #[test]
    fn it_big() {
        solve_it(1e6 as isize);
    }
}