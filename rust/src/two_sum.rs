// 两数之和
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = hash.get(&(target - num)) {
                return vec![i as i32, j as i32];
            }
            hash.insert(num, i);
        }
        vec![]
    }
}

pub struct Solution {}