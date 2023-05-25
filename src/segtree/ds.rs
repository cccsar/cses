
#[derive(Debug)]
pub struct Segtree<T,  F> 
    where
T : Copy + std::fmt::Debug, 
F : FnMut(T, T) -> T
{
    st : Vec<T> , 
    bin_op : F, // Binary operation with identity
    ident : T,  // Identity for the operation
}

impl<T,F> Segtree<T, F> 
    where
T : Copy + std::fmt::Debug, 
F : FnMut(T, T) -> T
{

    /// Intended to be used as a default constructor only specifying size
    /// ```
    /// use cses::segtree::ds::Segtree;
    /// let collection_size = 10;
    /// 
    /// /* create MAX segtree */
    /// let mut ds = Segtree::new(collection_size, (-1) as isize, |x, y| std::cmp::max(x, y));
    /// 
    /// /* Then use `update` to progresively create st */ 
    /// 
    /// for idx in 0..collection_size {
    ///     ds.update(0, collection_size-1, idx, (idx + 1) as isize, 0);
    /// }
    /// 
    /// assert_eq!(ds.query(0, collection_size-1, 0, collection_size-1, 0), 10);
    /// ```
    pub fn new(n : usize, ident : T, bin_op : F) -> Self 
    {
        Segtree {
            st : vec![ident ; 4 * n],
            ident ,
            bin_op, 
        }
    }

    /// Intended to be called like:
    /// ```
    /// use cses::segtree::ds::Segtree;
    /// let input = vec![5; 5];
    /// let ds = Segtree::build(&input, input.len(), 0, std::cmp::max ); // OBSERVE
    /// ```
    pub fn build(collection : &[T], n : usize, ident : T, bin_op : F)  -> Self
    {
        let mut output = Segtree {
            st : vec![ident ; 4*n],
            ident,
            bin_op ,
        };
        output.create(collection, 0, n - 1, 0);

        output
    }

    /// Intended to be called like `self.create(input, 0, input.len() - 1, 0 );`
    fn create(&mut self, collection : &[T], p : usize, q : usize, curr : usize) -> T { 

        if p == q {
            self.st[curr] = collection[p];
            return self.st[curr];
        }

        let mid = (p + q) / 2;

        let lst = self.create(collection, p, mid, 2*curr + 1);
        let rst = self.create(collection, mid + 1, q, 2*(curr + 1) );

        self.st[curr] = (self.bin_op)(lst, rst);

        return self.st[curr];
    }

    /// Intended to be called like:
    /// ```
    /// use cses::segtree::ds::Segtree;
    /// let input = vec![5; 5];
    /// let mut ds = Segtree::build(&input, input.len() , 0, |x, y| x + y); 
    /// assert_eq!(ds.query(0, input.len()-1, 0, 0, 0), 5 ); // OBSERVE
    /// ```
    pub fn query(&mut self, lb : usize, ub : usize, i : usize, j : usize, curr : usize) -> T {
        
        if j < i  {
            return self.ident;
        }

        if lb == i && ub == j {
            return self.st[curr];
        }


        let mid = (lb + ub) / 2;

        let ls = (2*curr + 1) % self.st.len(); 
        let rs = (2*(curr + 1) )% self.st.len();

        let lst = self.query(lb, mid, i, std::cmp::min(j, mid),ls );
        let rst = self.query(mid + 1, ub, std::cmp::max(i, mid + 1), j, rs);

        return (self.bin_op)(lst, rst);
    }

    /// Intended to be used like:
    /// ```
    /// use cses::segtree::ds::Segtree;
    /// let input = vec![5; 5];
    /// let mut ds = Segtree::build(&input, input.len() , 0, |x, y| x + y);
    /// assert_eq!( ds.update(0, input.len() - 1, 0, 4, 0 ), 24 ); // OBSERVE
    /// ```
    pub fn update(&mut self, lb : usize, ub : usize, idx : usize, el : T, curr : usize) -> T {
        if lb == ub { 
            self.st[curr] = el;
            return self.st[curr];
        }

        let mid = (lb + ub) / 2;

        let further : T;
        let result : T;

        if mid >= idx {
            further = self.update(lb, mid , idx, el, (2*curr + 1) % self.st.len() );
            result = (self.bin_op)(further, self.st[2*(curr + 1)]);
        }
        else {
            further = self.update(mid+1, ub, idx, el, (2*(curr + 1))% self.st.len() ) ;
            result = (self.bin_op)(self.st[2*curr + 1], further);
        }

        self.st[curr] = result;
        return self.st[curr]


    }

    pub fn dbg(self : &Self) {
        println!("{:?}", self.st);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn get_base_input() -> Vec<isize> {
        vec![1,2,3,4,5]
    }

    #[test]
    fn test_sum_st_build() {
        let input = get_base_input();

        let ds = Segtree::build(&input, input.len() , 0, |x, y| x + y);
    }

    #[test]
    fn test_sum_st_query_basic() {
        let input  = get_base_input();
        let mut ds = Segtree::build(&input, input.len(), 0, |x, y| x + y);

        assert_eq!(ds.query(0, input.len() - 1 , 0, input.len() - 1, 0), 15);
    }

    #[test]
    fn test_sum_st_query_edge_case_individual() {
        let input = get_base_input();
        let mut ds = Segtree::build(&input, input.len(), 0, |x, y| x + y);

        for idx in 0..input.len() {
            assert_eq!(ds.query(0, input.len() - 1, idx, idx , 0), input[idx]);
        }

    }

    #[test]
    fn test_sum_st_update() {
        let input = get_base_input();
        let mut ds = Segtree::build(&input, input.len(), 0, |x, y| x + y);

        assert_eq!(ds.update(0, input.len() - 1, 4, 6, 0), 16);
    }

}