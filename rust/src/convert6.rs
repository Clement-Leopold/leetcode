use core::num;

use crate::solution::Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
       let mut ans: Vec<String> = vec![String::from(""); num_rows as usize] ;
       if num_rows < 2 { return s.into()}
       let mut index = 0;
       let mut direction = true;
       for c in s.chars() {
         ans[index as usize].push(c);
         index += if direction {1} else {-1};
         direction =  direction == (index > 0 && index < num_rows - 1);
       }
        ans.concat()
    }
}