
use crate::templates::*;

pub fn run() {

    let sizes : Vec<usize> = read_collection();
    let mut nums : Vec<usize> = read_collection();

    let mut swaps : Vec<(usize, usize)> = vec![];

    for idx in 0..sizes[1] {
        let tmp_vec : Vec<usize> = read_collection();
        swaps.push( (tmp_vec[0], tmp_vec[1]) );
    }

    let resp = solve(&mut nums, &swaps);

    println!("{:?}", resp);
}

fn solve(nums : &mut Vec<usize>, swaps : &Vec<(usize, usize)> ) -> Vec<isize> {
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

    let mut resp : Vec<isize> = vec![];
    let mut acc : isize = 1;

    println!("dbg: comparisons: {:?}", comparisons);

    comparisons.iter().for_each(|&el| if !el { acc = acc + 1 } );

    println!("dbg: resp: {acc}");

    for &(u, v) in swaps 
    {
        // apply the swap
        let tmp = nums[u-1];
        nums[u-1] = nums[v-1];
        nums[v-1] = tmp;

        // update help accordingly
        let i = nums[u-1]; let j = nums[v-1];

        let tmp = help[i-1] ;
        help[i-1] = help[j-1];
        help[j-1] = tmp;

        // minimum check of possible changes in number of passes due to swap
        for idx in [i-1, i, j-1, j] 
        {
            if let Some(val) = helper(&help, idx, &mut comparisons) 
            { acc = acc + val as isize; }
        }

        resp.push(acc as isize);

        println!("Checking {u}, {v}:");
        println!("dbg: nums: {:?}", nums);
        println!("dbg: helper: {:?}", help);
    }

    return resp;
}
    // idx 0 1 2 3 4
    // val 3 2 1 0 4

    // help:
    // idx 0 1 2 3 4 actual val
    // val 3 2 1 0 4 actual idx

    // swap(1, 3) .. idxs

    // idx 0 1 2 3 4
    // val 3 0 1 2 4
    
    // help:
    // idx 0 1 2 3 4 actual val
    // val 1 2 3 0 4 actual idx


pub fn helper(help : &Vec<usize>, idx : usize, comparisons : &mut Vec<bool>) -> Option<isize> {
    if idx == 0 || idx >= help.len() - 1 { return None; }

    let mut result = 0;
    let expr = help[idx] < help[idx + 1]; 

    if  expr ^ comparisons[idx] 
    {
        if comparisons[idx] { result += 1; }
        else { result -= 1; } 

        comparisons[idx] = expr;
    }

    return Some(result);
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