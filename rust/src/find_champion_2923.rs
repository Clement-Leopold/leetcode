
use crate::Solution;

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let mut max = -1;
        let mut row = 0;
        let mut  a = 0;
        for v in grid {
            let mut line = 0;
            for i in v {
                line += i;
            }
            if line > max  {
                a = row; 
                max = line;
            }
            row +=1;
        }

        return a as i32;
    }
}