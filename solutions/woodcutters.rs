use std::io::Read;

struct Solution {}

fn get_input(it: &mut std::str::SplitWhitespace<'_>) -> Vec<(i32, i32)> {
    let n: usize = it.next().unwrap().parse().unwrap();

    let arr: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            let x: i32 = it.next().unwrap().parse().unwrap();
            let h: i32 = it.next().unwrap().parse().unwrap();
            (x, h)
        })
        .collect();

    arr
}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

    // let t: u64 = it.next().unwrap().parse().unwrap();
    let t: u64 = 1;

    for _ in 0..t {
        let arr: Vec<(i32, i32)> = get_input(&mut it);

        let sol: i32 = Solution::woodcutters(arr);

        println!("{}", sol);
    }
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    None,
}

impl Solution {
    pub fn woodcutters(arr: Vec<(i32, i32)>) -> i32 {
        if arr.len() <= 1 {
            return arr.len() as i32;
        }
        let mut answer: i32 = 0;

        // Start by letting the first tree fall left
        let mut prev_direction: Direction = Direction::Left;
        answer += 1;

        for i in 1..arr.len() - 1 {
            // Try to fall left
            if arr[i].0 - arr[i].1
                > arr[i - 1].0
                    + if prev_direction == Direction::Right {
                        arr[i - 1].1
                    } else {
                        0
                    }
            {
                prev_direction = Direction::Left;
                answer += 1;
                continue;
            }
            // Try to fall right
            if arr[i].0 + arr[i].1 < arr[i + 1].0 {
                prev_direction = Direction::Right;
                answer += 1;
                continue;
            }
            // Didn't fall
            prev_direction = Direction::None
        }

        // Let last tree fall right
        answer += 1;

        answer
    }
}
