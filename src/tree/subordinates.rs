
type Graph<T> = Vec< Vec<T> >;

pub fn solve(structure : &Vec<usize>) -> Vec<isize> {
    let n = structure.len() + 1;
    let mut subordinates : Vec<isize> = vec![-1 ; n];
    let mut graph : Graph<usize> = vec![ vec![]; n ]; 

    for idx in 0..structure.len()  { 
        graph[idx + 1].push( structure[idx] - 1 );
        graph[structure[idx] - 1 as usize].push( idx+1 );
    }

    dfs(&graph, &mut subordinates, 0);

    subordinates
}

pub fn dfs(tree : &Graph<usize>, subordinates : &mut Vec<isize>, root : usize) -> isize {
    subordinates[root] = 0;

    let mut acc = 1;
    for &ady in tree[root].iter() 
    {
        if subordinates[ady] == -1 
        {
            acc += dfs(tree, subordinates, ady);
        }
    }

    subordinates[root] = acc - 1;
    acc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base() {
        let structure = vec![1,1,2,3];

        assert_eq!(solve(&structure), [4,1,1,0,0]);
    }

}