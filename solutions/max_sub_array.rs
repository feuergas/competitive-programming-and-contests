struct Solution {}
// REMOVE ME

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

    let sol = Solution::max_sub_array(nums);

    println!("Soluzione: {}", sol);
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut best: i32 = -1e5 as i32;
        let mut current_best: i32 = -1e5 as i32;

        for n in nums {
            current_best = std::cmp::max(n, current_best + n);
            best = std::cmp::max(best, current_best);
        }

        best
    }
}
