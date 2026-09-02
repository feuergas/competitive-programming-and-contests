use std::{collections::HashMap, io::Read};

fn get_input(it: &mut std::str::SplitWhitespace<'_>) -> (Vec<u32>, u32) {
    let n: u32 = it.next().unwrap().parse().unwrap();
    let k: u32 = it.next().unwrap().parse().unwrap();

    let arr: Vec<u32> = (0..n)
        .map(|_| it.next().unwrap().parse().unwrap())
        .collect();

    (arr, k)
}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

    // let t: u32 = it.next().unwrap().parse().unwrap();
    let t: u32 = 1;

    for _ in 0..t {
        let (arr, k) = get_input(&mut it);

        let sol: (u32, u32) = Solution::longest_k_good_segment(arr, k);

        println!("{} {}", sol.0, sol.1);
    }
}

struct Solution {}

impl Solution {
    pub fn longest_k_good_segment(arr: Vec<u32>, k: u32) -> (u32, u32) {
        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut max_length: usize = 0;

        let mut best_left: usize = 0;
        let mut best_right: usize = 0;

        let mut count_map: HashMap<u32, usize> = HashMap::new();
        while right < arr.len() {
            while count_map.len() <= k as usize && right < arr.len() {
                *count_map.entry(arr[right]).or_insert(0) += 1;
                right += 1;
            }

            if right == arr.len() && count_map.len() <= k as usize {
                right += 1; // Move right pointer to the end to check the last segment
            }

            if right - left > max_length {
                max_length = right - left;
                best_left = left;
                best_right = right - 1;
            }

            while count_map.len() > k as usize && left < right {
                if let Some(count) = count_map.get_mut(&arr[left]) {
                    *count -= 1;
                    if *count == 0 {
                        count_map.remove(&arr[left]);
                    }
                }
                left += 1;
            }
        }

        best_right -= 1; // Adjust best_right to be inclusive

        (best_left as u32 + 1, best_right as u32 + 1) // Convert to 1-based indexing
    }
}
