fn main() {
    let nums: Vec<i32> = vec![1, 5, 11, 5];

    let sol: bool = Solution::can_partition(nums);

    println!("Soluzione: {}", sol);
}

struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut target: usize = nums.iter().sum::<i32>() as usize; // Calculate the total sum of the array
        if !target.is_multiple_of(2) {
            return false; // If the total sum is odd, we cannot partition it into two equal subsets
        }
        target /= 2; // We need to find a subset with this sum

        // Create a DP table where dp[i][j] indicates whether a sum of j can be achieved with the first i numbers
        let mut dp: Vec<Vec<bool>> = vec![vec![false; target + 1]; nums.len() + 1];
        dp[0][0] = true; // Base case: a sum of 0 is always possible (empty subset)
        for (i, &num) in (1..=nums.len()).zip(&nums) {
            dp[i][0] = true; // Base case: a sum of 0 is always possible (empty subset)
            for j in 1..=target {
                dp[i][j] = dp[i - 1][j]; // If we don't take the current number
                if j >= num as usize {
                    dp[i][j] |= dp[i - 1][j - num as usize]; // If we take the current number
                }
            }
        }

        dp[nums.len()][target]
    }
}
