
use crate::templates::*;

pub fn run() {

    let sizes : Vec<usize> = read_collection();
    let mut nums : Vec<usize> = read_collection();

    let mut swaps : Vec<(usize, usize)> = vec![];

    for idx in 0..sizes[1] {
        let tmp_vec : Vec<usize> = read_collection();
        swaps.push( (tmp_vec[0], tmp_vec[1]) );
    }

    solve(&mut nums, &swaps);
}

fn solve(nums : &mut Vec<usize>, swaps : &Vec<(usize, usize)> ) -> Vec<usize> {
    let N : usize = nums.len();
    
    // for help[idx] idx represent each number in `nums` and `help[idx]` represent
    // its position in nums. This way a new round is detected if for
    // i < j it holds that `help[i] > help[j]`. In other words, the successor for `i`
    // is found before `i` in `nums`
    let mut help : Vec<usize> = vec![0; N]; 


    nums.iter()
        .enumerate()
        .for_each(|(idx, &num)| {
            help[num - 1] = idx;
        });

    println!("dbg: help: {:?}", help);

    // the main use for `comparisons` is easily aplying changes on `nums` represented
    // by `swaps` array. 
    let mut comparisons : Vec<bool> = vec![true ; N - 1];

    for idx in 1..N {
        comparisons[idx - 1] = help[idx - 1] < help[idx];
    }

    let mut resp : usize = 1;

    println!("dbg: comparisons: {:?}", comparisons);

    comparisons.iter().for_each(|&el| if !el { resp = resp + 1 } );

    println!("dbg: resp: {resp}");

    for &(u, v) in swaps {
        let tmp = nums[u-1];
        nums[u-1] = nums[v-1];
        nums[v-1] = tmp;

        help[ nums[u-1] ] = u-1;
        help[ nums[v-1] ] = v-1;


        // TODO a lot of logics goes here
    }


    return vec![];
}

#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn basic() {
        let mut nums = vec![4,2,1,5,3];

        let swaps = vec![(2,3), (1,5), (2,3)];

        assert_eq!(solve(&mut nums, &swaps), vec![]);
    }
}