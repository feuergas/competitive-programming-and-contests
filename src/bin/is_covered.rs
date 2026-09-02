struct Solution {}

fn main() {
    let ranges = vec![vec![3, 4], vec![1, 2], vec![5, 6]];
    let left = 2;
    let right = 5;

    let sol = Solution::is_covered(ranges, left, right);

    println!("Solution: {}", sol);
}

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut sorted_ranges = ranges;
        sorted_ranges.sort();

        let mut merged_ranges: Vec<Vec<i32>> = vec![];

        for current_range in sorted_ranges {
            if merged_ranges.is_empty() {
                merged_ranges.push(current_range);
                continue;
            }

            let mut last_range = merged_ranges.pop().unwrap();

            if last_range[1] + 1 >= current_range[0] {
                last_range[1] = std::cmp::max(last_range[1], current_range[1]);
                merged_ranges.push(last_range);
            } else {
                merged_ranges.push(last_range);
                merged_ranges.push(current_range);
            }
        }

        for range in merged_ranges {
            if range[0] <= left && right <= range[1] {
                return true;
            }
        }

        false
    }
}
