struct Solution {}

fn main() {
    let nums = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    let sol = Solution::trap(nums);

    println!("Soluzione: {}", sol);
}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut peaks: Vec<usize> = vec![];
        let mut right_peaks: Vec<usize> = vec![];
        let mut max_height: i32 = 0;
        let mut total_water: i32 = 0;

        for i in 0..height.len() {
            let left_peak = i == 0 || height[i - 1] <= height[i];
            let right_peak = i == height.len() - 1 || height[i + 1] <= height[i];
            if left_peak && right_peak && height[i] >= max_height {
                peaks.push(i);
                max_height = height[i];
            }
        }

        let highest_peak = peaks[peaks.len() - 1];
        max_height = 0;
        for i in (highest_peak..height.len()).rev() {
            let left_peak = i == 0 || height[i - 1] <= height[i];
            let right_peak = i == height.len() - 1 || height[i + 1] <= height[i];
            if left_peak && right_peak && height[i] >= max_height {
                right_peaks.push(i);
                max_height = height[i];
            }
        }
        right_peaks.pop(); // remove highest peak to avoid duplicates

        peaks.extend(right_peaks.into_iter().rev());

        for i in 1..peaks.len() {
            let l = peaks[i - 1];
            let r = peaks[i];

            let mut j: usize;
            if height[l] < height[r] {
                // left peak is lower than right peak
                j = l + 1;
                while height[j] < height[l] {
                    total_water += height[l] - height[j];
                    j += 1;
                }
            } else {
                // left peak is at least as tall as right peak
                j = r - 1;
                while height[j] < height[r] {
                    total_water += height[r] - height[j];
                    j -= 1;
                }
            }
        }

        total_water
    }
}
