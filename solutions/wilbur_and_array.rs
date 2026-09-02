use std::io::Read;

struct Solution {}

fn get_input(it: &mut std::str::SplitWhitespace<'_>) -> Vec<i32> {
    let n: usize = it.next().unwrap().parse().unwrap();

    let arr: Vec<i32> = (0..n)
        .map(|_| it.next().unwrap().parse().unwrap())
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
        let arr: Vec<i32> = get_input(&mut it);

        let sol: u64 = Solution::wilbur_and_array(arr);

        println!("{}", sol);
    }
}

impl Solution {
    pub fn wilbur_and_array(arr: Vec<i32>) -> u64 {
        arr.windows(2).fold(0, |acc: u64, w: &[i32]| {
            acc + (w[1] - w[0]).unsigned_abs() as u64
        }) + arr[0].unsigned_abs() as u64
    }
}
