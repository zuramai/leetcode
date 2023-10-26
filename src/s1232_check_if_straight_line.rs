struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut window = coordinates.windows(2);
        let mut first = None;
        let get_slope = |v1: &Vec<i32>, v2: &Vec<i32>| {
            let x = v1[0];
            let y = v1[1];
            let x1 = v2[0];
            let y2 = v2[1];
            let mut m = 0;
            if x-x1 != 0 {
                m = (y2-y) / (x-x1)
            }
            m
        };
        while let Some(d) = window.next() {
            let p1 = &d[0];
            let p2 = &d[1];
            let mut m = get_slope(p1, p2);
            
            match first {
                None => first = Some(m),
                Some(f) => if m != f || f != get_slope(&coordinates[0],p2) {
                    return false;
                }
            }
            println!("m: {m}, {:?}, {:?}", p1, p2);
        };
        println!("OK");
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::check_straight_line(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5],vec![5,6],vec![6,7]]), true);
        assert_eq!(Solution::check_straight_line(vec![vec![1,1],vec![2,2],vec![3,4],vec![4,5],vec![5,6],vec![7,7]]), false);
        assert_eq!(Solution::check_straight_line(vec![vec![0,0],vec![0,1],vec![0,-1]]), true);
        assert_eq!(Solution::check_straight_line(vec![vec![0,0],vec![0,5],vec![5,5],vec![5,0]]), false);
    }
}