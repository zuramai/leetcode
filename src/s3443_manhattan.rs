pub struct Solution;
impl Solution {
    pub fn max_distance(s: String, mut k: i32) -> i32 {
        let mut max_distance = 0;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
    
        for (i,c) in s.chars().enumerate() {
            match c {
                'W' => x -= 1,
                'S' => y += 1,
                'E' => x += 1,
                'N' => y -= 1,
                _ => ()
            };
            let mut distance = x.abs() + y.abs();
            distance = std::cmp::min(distance + k * 2, i as i32 + 1);
            max_distance = std::cmp::max(distance,max_distance);
             
        }
        max_distance
    }
}