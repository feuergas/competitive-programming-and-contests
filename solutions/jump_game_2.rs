struct Solution {}

fn main() {
    let nums: Vec<i32> = vec![2, 3, 1, 1, 4];

    let sol: i32 = Solution::jump_game_2(nums);

    println!("Soluzione: {}", sol);
}

impl Solution {
    pub fn jump_game_2(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0; // No jumps needed if there's one or no element
        }
        let mut cur_index: usize = 0;
        let mut jumps: i32 = 0;

        while cur_index < nums.len() - 1 {
            let mut max_reach: usize = 0;
            let mut next_index: usize = cur_index;

            for i in (cur_index + 1)..=cur_index + nums[cur_index] as usize {
                if i == nums.len() - 1 {
                    return jumps + 1; // Reached the end
                }

                if i < nums.len() && i + nums[i] as usize > max_reach {
                    max_reach = i + nums[i] as usize;
                    next_index = i;
                }
            }

            if next_index == cur_index {
                break; // No further progress can be made
            }

            cur_index = next_index;
            jumps += 1;
        }

        -1 // If we exit the loop without reaching the end, return -1
    }
}
