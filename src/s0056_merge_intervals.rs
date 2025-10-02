struct Solution;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        // println!("sorted {:?}", intervals);

        let mut new_vec = Vec::new();
        let mut left = 0;
        let mut right = std::cmp::min(1, intervals.len() - 1);

        while right < intervals.len() {
            
            let l = &intervals[left];
            let r = &intervals[right];
            
            // println!("l {} r {}", left, right);

            if l[1] >= r[0] {
                // overlap
                intervals[left][1] = std::cmp::max(l[1], r[1]);
            } else {
                new_vec.push(vec![l[0], l[1]]);
                left = right;
            }

            right += 1;
        }
        // println!("left {}", left);
        new_vec.push(vec![intervals[left][0], intervals[left][1]]);

        new_vec
    }
}