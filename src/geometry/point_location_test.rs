
pub struct Point(f64, f64);

impl Point {

    /// Usefull for computing the cross products of vectors ending in the given points
    pub fn cross_prod(self, other : Point) -> f64 {
        self.0 * other.1 - self.1 * other.0
    }

    pub fn to_the_right(self, other : Point) -> Option<bool> {
        let cp = self.cross_prod(other) ;
        if cp == 0.0 {
            None
        } else {
            Some(cp > 0.0)
        }
    }
}

pub fn solve(a : (isize, isize), b : (isize, isize), c : (isize, isize)) -> String {

    let a = Point(a.0 as f64, a.1 as f64);
    let b = Point(b.0 as f64, b.1 as f64);
    let c = Point(c.0 as f64, c.1 as f64);

    let b = Point(f64::abs( b.0 - a.0 ),f64::abs( b.1 - a.1 ));
    let c = Point(f64::abs( c.0 - a.0 ),f64::abs( c.1 - a.1 ));

    match  c.to_the_right(b) 
    {
        Some(el) => 
            if el {
                "RIGHT".to_string()
            }
            else {
                "LEFT".to_string()
            }
        None => "TOUCH".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_a() {
        let a = (1, 1);
        let b = (5, 3);
        let c = (2, 3);
        assert_eq!( solve(a,b,c), "LEFT".to_string())
    }

    #[test]
    fn base_b() {
        let a = (1, 1);
        let b = (5, 3);
        let c = (4, 1);

        assert_eq!(solve(a,b,c), "RIGHT".to_string());
    }

    #[test]
    fn base_c() {
        let a = (1, 1);
        let b = (5, 3);
        let c = (3, 2);

        assert_eq!(solve(a,b,c), "TOUCH");
    }

}