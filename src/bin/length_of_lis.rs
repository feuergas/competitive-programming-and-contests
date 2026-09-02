struct Solution {}

fn main() {
    let nums: Vec<i32> = vec![10, 9, 2, 5, 3, 7, 101, 18];

    let sol: i32 = Solution::length_of_lis(nums);

    println!("Soluzione: {}", sol);
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut smallest_endpoint: Vec<i32> = Vec::new();

        smallest_endpoint.push(nums[0]);

        for num in nums.into_iter().skip(1) {
            let binary_search = |target: i32| -> usize {
                let mut left: usize = 0;
                let mut right: usize = smallest_endpoint.len();

                while left < right {
                    let mid = left + (right - left) / 2;

                    if smallest_endpoint[mid] < target {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }

                left
            };

            let pos: usize = binary_search(num);

            if pos == smallest_endpoint.len() {
                smallest_endpoint.push(num);
            } else {
                smallest_endpoint[pos] = num;
            }
        }

        smallest_endpoint.len() as i32
    }
}
