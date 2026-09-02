fn main() {
    let nums: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

    let sol: i32 = Solution::max_sub_array(nums);

    println!("Soluzione: {}", sol);
}

struct Solution {}

use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_best: i32 = 0;

        nums.iter().fold(i32::MIN, |best, &n| {
            current_best = max(n, current_best + n);
            max(best, current_best)
        })
    }
}
