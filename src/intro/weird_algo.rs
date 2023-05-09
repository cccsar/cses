
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

pub fn solve_v1(n : isize, mut output : &mut Vec<isize>) -> () {
    output.push( n );

    if n > 1 {
        if n%2 == 0 {
            solve_v1(n/2, output);
        }
        else {
            solve_v1(n*3 + 1, output);
        }
    }
}

#[cfg(test)]
mod test {
    
    use super::*;

    #[test]
    fn base() {
        let resp = [3,10,5,16,8,4,2,1];

        assert_eq!(solve(3), resp);
    }

    #[test]
    fn base_v1() {
        let resp = [3,10,5,16,8,4,2,1];
        let mut tmp : Vec<isize> = vec![];

        solve_v1(3, &mut tmp);

        assert_eq!(tmp, resp);
    }

}