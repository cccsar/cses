
use std::collections::BTreeSet;

pub fn solve(nums : Vec<isize>) -> isize {
    let help : BTreeSet<isize> = BTreeSet::from_iter(nums.into_iter());
    help.len() as isize

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base() {
        let tmp = vec![2,3,2,2,3];

        assert_eq!(solve(tmp), 2);
    }
}