use crate::Solution;
use std::collections::VecDeque;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut curry = x;
        let mut deq = VecDeque::new();
        while curry > 0 {
            deq.push_front(curry % 10);
            curry /= 10;
        }
        while deq.len() > 0 {
            if let Some(front) = deq.pop_front() {
                if let Some(back) = deq.pop_back() {
                    if front == back {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    return true;
                }
            }
        }
        return true;
    }
}
