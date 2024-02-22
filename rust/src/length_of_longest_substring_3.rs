use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash = HashMap::new();
        let mut left = 0;
        let mut ans = 0;
        for (right, c) in s.chars().enumerate() {
            if let Some(&k) = hash.get(&c) {
                left = max(k + 1, right);
            }
            hash.insert(c, right);
            ans = max(ans, right - left + 1);
        }
        ans as i32
    }
}

pub struct Solution {}