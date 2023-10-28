struct Solution;
impl Solution {
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let row = obstacle_grid.len();
        let col = obstacle_grid[0].len();

        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        let mut i_val = 1;
        let mut j_val = 1;
        for i in 0..row {
            for j in 0..col {
                let val = &mut obstacle_grid[i][j];
                let mut newVal = 1;
                if val == &1 {
                    newVal =  0;
                    if i == 0 {
                        i_val = 0;
                    }
                    if j == 0 {
                        j_val = 0
                    }
                }
                else if i != 0 && j != 0 {
                    newVal = -1
                }
                else if i == 0 && i_val == 0 {
                    newVal = 0
                }
                else if j == 0 && j_val == 0 {
                    newVal = 0
                }
                else {
                    newVal = 1
                }
                *val = newVal
            }
        }

        println!("Grid: {obstacle_grid:?}");
        for i in 1..row {
            for j in 1..col {
                if obstacle_grid[i][j] == 0 {
                    continue;
                }
                obstacle_grid[i][j] = obstacle_grid[i-1][j] + obstacle_grid[i][j-1];        
            }
        }
        println!("obstacle_grid2: {obstacle_grid:?}");
        obstacle_grid[row-1][col-1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]]), 2);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0,1],vec![0,0]]), 1);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1,0]]), 0);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0,0],vec![1,1],vec![0,0]]), 0);
    }
}