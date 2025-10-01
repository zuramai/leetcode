struct Solution;

use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn max_candies(mut status: Vec<i32>, candies: Vec<i32>, keys: Vec<Vec<i32>>, contained_boxes: Vec<Vec<i32>>, initial_boxes: Vec<i32>) -> i32 {
        let n = status.len();
        let mut total_candies = 0;

        let mut possessed: HashSet<usize> = initial_boxes.iter().map(|&i| i as usize).collect();
        let mut q: VecDeque<usize> = VecDeque::new();

        let mut collected = vec![false; n];

        for &box_idx in initial_boxes.iter() {
            let i = box_idx as usize;
            if status[i] == 1 && !collected[i] {
                q.push_back(i);
                collected[i] = true
            }
        }

        while let Some(i) = q.pop_front() {
            total_candies += candies[i];

            for &key_box_idx in keys[i].iter() {
                let k = key_box_idx as usize;

                // set the box as unlocked
                if status[k] == 0 {
                    status[k] = 1;

                    if possessed.contains(&k) && !collected[k] {
                        q.push_back(k);
                        collected[k] = true
                    }
                }
            }

            for &contained_box_idx in contained_boxes[i].iter() {
                let j = contained_box_idx as usize;

                if possessed.insert(j) {
                    if status[j] == 1 && !collected[j] {
                        q.push_back(j);
                        collected[j] = true;
                    }
                }
            }
        }
        total_candies
    }
}   