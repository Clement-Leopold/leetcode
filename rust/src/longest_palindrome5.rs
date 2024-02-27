use crate::solution::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let length = s.len();
        let mut dp = vec![vec![false; length]];
        let palin: Vec<char> = s.chars().collect();

        let long = 2;
        for k in 2..length + 1 {
            for left in 0..length {
                let right = length + left - 1;
                if right >= length {
                    break;
                }
            }
        }
    }
}
