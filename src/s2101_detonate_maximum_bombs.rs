struct Solution;

impl Solution {
    pub fn is_inside_bomb(bomb_pos: &Vec<i32>, point_pos: &Vec<i32>) -> bool {
        ((point_pos[0] - bomb_pos[0]) as i128).pow(2) + ((point_pos[1] - bomb_pos[1]) as i128).pow(2) <= (bomb_pos[2] as i128).pow(2)
    }

    pub fn check(bombs: &Vec<Vec<i32>>, mut to_visit_indexes: Vec<i32>, current_index: i32) -> i32 {
        let current_xyr = &bombs[current_index as usize];
        for to_visit_index in to_visit_indexes.iter() {
            let to_visit_xyr = &bombs[*to_visit_index as usize];
            let is_inside = Solution::is_inside_bomb(current_xyr, to_visit_xyr);
            println!("INSIDE {current_xyr:?} == {to_visit_xyr:?}, {is_inside}");
            let _index = to_visit_index.clone();
            if is_inside {
                to_visit_indexes.retain(|f| *f != _index);
                if to_visit_indexes.len() > 0 {
                    let first = to_visit_indexes[0];
                    return Solution::check(bombs, to_visit_indexes, first) + 1;
                }
                return 1;
            }
        }
        0
    }

    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        
        for (index, xyr) in bombs.iter().enumerate() {
            let mut x = xyr[0];
            let mut y = xyr[1];
            let mut r = xyr[2];

            let mut neighbour_indexes: Vec<i32> = (0..(bombs.len() as i32)).collect();
            // println!("neighbour_indexes! {:?}",neighbour_indexes);
            // Except the current
            neighbour_indexes.retain(|&x| x != index as i32);
            let count = Solution::check(&bombs, neighbour_indexes, index as i32) + 1;
            if count > max {
                max = count;
            }
        }
        println!("count: {max}"); 
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::maximum_detonation(vec![vec![1,1,100000],vec![100000,100000,1]]), 1);
        assert_eq!(Solution::maximum_detonation(vec![vec![1,1,5],vec![10,10,5]]), 1);
        assert_eq!(Solution::maximum_detonation(vec![vec![1,2,3],vec![2,3,1],vec![3,4,2],vec![4,5,3],vec![5,6,4]]), 5);
        assert_eq!(Solution::maximum_detonation(vec![vec![2,1,3],vec![6,1,4]]), 2);
    }
}