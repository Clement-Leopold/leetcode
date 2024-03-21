mod two_sum;
mod longest_palindrome5;
use solution::Solution;
mod solution;
mod convert6;

fn main() {
    let ans = Solution::convert("PAYPALISHIRING".to_string(), 3);
    print!("{:?}", ans);
}
