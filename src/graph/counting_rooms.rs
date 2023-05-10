type Graph<T> = Vec<Vec<T>>;

pub fn solve(input : impl(Iterator<Item = String>), n : usize) {
    let mut graph : Graph<char> = vec![];
    let mut vtd : Vec< Vec<bool> > = vec![];

    // Fill with values
    (0..n).for_each( |_| { vtd.push( vec![false; n] ); });

    // Make every char acessible by indexing `graph`
    input .for_each(|row| { 
            graph.push( row.chars().collect::<Vec<char>>() ); 
          });


    // TODO

}

pub fn dfs(G : &Graph<char>, vtd : &mut Vec<Vec<bool>>, root : (usize, usize) ) {
    vtd[root.0][root.1] = true;


    // TODO

}

// TODO
/*
pub fn adyacents(root : (usize, usize)) -> Vec<(usize, usize)> {
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base() {
        let n = 5;
        let inp : String = 
            "########
            #..#...#
            ####.#.#
            #..#...#
            ########".to_string() ;

        solve(inp.lines().map(|el| {el.to_string() }), n);

    }

}