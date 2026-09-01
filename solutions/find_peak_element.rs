struct Solution {}
// REMOVE ME

fn main() {
    let nums = vec![1, 2];

    let sol = Solution::find_peak_element(nums);

    println!("Soluzione: {}", sol);
}

enum State {
    Peak,
    Left,
}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;

            if Self::check_peak(&nums, &mid, State::Peak) {
                left = mid;
                right = mid;
            } else if Self::check_peak(&nums, &mid, State::Left) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        left.try_into().unwrap()
    }

    pub fn check_peak(nums: &[i32], index: &usize, flag: State) -> bool {
        let left_peak: bool = *index == 0 || nums[*index - 1] < nums[*index];
        let right_peak: bool = *index == nums.len() - 1 || nums[*index + 1] < nums[*index];
        match flag {
            State::Peak => left_peak && right_peak,
            State::Left => left_peak,
        }
    }
}
