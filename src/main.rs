use std::cmp::max;

struct Solution {}

fn main() {
    let text1: String = String::from("abcde");
    let text2: String = String::from("ace");

    let sol: i32 = Solution::longest_common_subsequence(text1, text2);

    println!("Soluzione: {}", sol);
}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        for i in 1..=text1.len() {
            for j in 1..=text2.len() {
                if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }

        dp[text1.len()][text2.len()]
    }
}
