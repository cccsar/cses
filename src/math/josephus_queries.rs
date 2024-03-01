/*
n: numbers remaining
i: iteration number (up to log2(n))
curr: amount of elements traversed on this iteration
start: either 1, 0, representing either we're starting from the first  or 2nd element of the `curr` to traverse


Initial values
n:=*given*; start:=0; curr:=0; 

Update after every it
if start = 0 ^ n & 1 -> { curr:=(n+1)>>1; start:=1; }
elif start = 1 ^ n & 1 -> { curr:=n>>1; start:= 0; } 
elif start = 0 ^ !(n & 1) -> { curr:=n>>1; start:=0; }
elif start = 1 ^ !(n & 1) -> { curr:=n>>1; start:=1; }
n:=n-curr;

next_state(curr, start, n) -> (int, int , int) {
    prev := curr;
    if start = 0 ^ n & 1 -> { curr:=(n+1)>>1; start:=1; }
    elif start = 1 ^ n & 1 -> { curr:=n>>1; start:= 0; } 
    elif start = 0 ^ !(n & 1) -> { curr:=n>>1; start:=0; }
    elif start = 1 ^ !(n & 1) -> { curr:=n>>1; start:=1; }

    return (curr, start, n-prev)
}

find_curr(n, k, curr, start) {
    acc := 0;
    while (acc + curr < k) 
    {
        acc += prev_curr;
        (curr, start, n) := next_state(curr, start, n);
    }
    / Up to here I used curr to reach k
    TODO now I need to use `start` and `curr` to reach its image 
}


(n=15): 
i: 1 -- step: 2^1
    start:0, n: 15, curr:8 ( range [1..8] )
    1 3 5 7 9 11 13 15 
i: 2 -- step:2^2
    start:1, n: 7, curr: 3 (range [8 + 1..8 + 3] == [9..11] )
    4 8 12
i: 3 -- step:2^3
    start:0, n: 4, curr: 2 (range [11+1..11+2] == [12..13] )
    2 10
i: 4 -- step: .. 2^i
    start:0, n: 2, curr: 1 (range [13+1..13+1] == [14..14] )
    6
i: 5
    start:0, n: 1, curr: 1 (range [14+1..14+1] == [15..15] )
    14

Observe on every i we sort of apply a bitmask that selects elements that:
* haven't been selected before
* won't be selected later

after the previous point, observe that on the p-th (where p is up to log2(n) ) the mask is comprised of n ones 

Observe on every i, the size of the mask (curr) either decreases or stays the same

given a n, and a k, can I find image of k through josephus(..) using something like binsearch?:
    If I find a way to figure out in which range I am, i can relate it to i and then deduce the state

Observe one can keep track of the startpoints that have been selected. 
    e.g.: 
        it always start at 1 jumping 2 by 2
        then either 2 or 4 jumping 4 by 4
        and so on
    knowing the start point of the previous (or the full history) can I deduce the new startpoint
    (I certainly have the jump size: i)
        

(29)
1 3 5 7 9 11 13 15 17 19 21 23 25 27 29 

*/

pub fn solve() {

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base() {

    }

}
