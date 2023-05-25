// TODO learn why it doesn't work when compiling in rust

type Graph<T> = Vec<Vec<T>>;

pub fn solve(input : impl(Iterator<Item = String>), n : usize, m : usize) -> isize {
    let mut graph : Graph<char> = vec![];
    let mut vtd : Vec< Vec<bool> > = vec![];


    // Make every char acessible by indexing `graph`
    for row in input 
    {
        graph.push( row.chars().collect::<Vec<char>>() ); 
    }

    // Fill with values
    for i in 0..n 
    {
        vtd.push( vec![false;m] );
    }

    let mut count = 0;
    for (i, row)  in graph.iter().enumerate() 
    {
        for (j, _) in row.iter().enumerate() 
        {
            if graph[i][j] != '#' && !vtd[i][j] 
            {
                dfs(&graph, &mut vtd, (i as isize, j as isize) );
                count += 1;
            }
        }
    }

    count

}

pub fn dfs(G : &Graph<char>, vtd : &mut Vec<Vec<bool>>, root : (isize, isize) ) {

    vtd[root.0 as usize][root.1 as usize] = true;

    for ady in adyacents(G, root)
    {
        if !vtd[ady.0 as usize][ady.1 as usize] 
        {
            dfs(G, vtd, ady);
        }
    }
}


pub fn adyacents(G: &Graph<char>, root : (isize, isize)) -> Box< dyn Iterator<Item = (isize, isize)> + '_> {
    Box::new (
        vec![ (root.0 + 1, root.1)
        , (root.0 - 1, root.1)
        , (root.0, root.1 + 1)
        , (root.0, root.1 - 1) 
        ] .into_iter()
          .filter( 
                |(x,y)| 
                    *x >= 0 && *y >= 0 && *x < (G.len() as isize) && *y < (G[0].len() as isize) 
                    && G[*x as usize ][*y as usize] != '#'
        ) 
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base() {
        let n = 5;
        let m = 8;
        let inp : String = 
            "\
########
#..#...#
####.#.#
#..#...#
########".to_string() ;

        let input = inp.lines().map(|el| {el.to_string() });


        assert_eq!( solve(input, n, m), 3) ;

    }

}