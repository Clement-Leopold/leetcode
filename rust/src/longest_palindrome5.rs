
use crate::solution::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s_len = s.len();
        let mut dp = vec![vec![false; s_len]; s_len];
        for i in 0..s_len {
            dp[i][i] = true;
        }
        let mut ans = (0, 0);
        let chars_arr : Vec<char>= s.chars().collect();
        for l in 2..s_len + 1 {
            for left in 0..s_len -2 {
                let right = left + l - 1;
                if right >= s_len {
                    break;
                }
                if chars_arr[left] != chars_arr[right]{
                    continue;
                }
                if l < 3 {
                    dp[left][right] = true;
                } else {
                    dp[left][right] = dp[left + 1][right - 1];
                }
                if dp[left][right] && right - left > ans.1 - ans.0 {
                    ans = (left, right);
                }
            }
        }
        (&s[ans.0..ans.1 + 1]).into()
    }
}
