fn main() {
    let nums: Vec<i32> = vec![23, 2, 4, 6, 6];
    let k: i32 = 7;

    let sol: bool = Solution::check_subarray_sum(nums, k);

    println!("Solution: {}", sol);
}

use std::collections::HashMap;

struct Solution {}

fn has_duplicates(nums: Vec<i32>) -> bool {
    let mut seen = HashMap::new();

    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&n) {
            if j + 1 < i {
                return true;
            }
        } else {
            seen.insert(n, i);
        }
    }

    false
}

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut prefix_sums: Vec<i32> = vec![0];
        let mut current_sum: i32 = 0;
        for num in &nums {
            let n: i32 = num.rem_euclid(k);

            current_sum = (current_sum + n).rem_euclid(k);
            prefix_sums.push(current_sum);
        }

        has_duplicates(prefix_sums)
    }
}
