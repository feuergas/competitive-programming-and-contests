use std::io::Read;

struct Solution {}

fn get_input(it: &mut std::str::SplitWhitespace<'_>) -> (Vec<u64>, Vec<(u64, u64)>) {
    let n: u64 = it.next().unwrap().parse().unwrap();
    let t: u64 = it.next().unwrap().parse().unwrap();

    let arr: Vec<u64> = (0..n)
        .map(|_| it.next().unwrap().parse().unwrap())
        .collect();

    let queries: Vec<(u64, u64)> = (0..t)
        .map(|_| {
            let l: u64 = it.next().unwrap().parse().unwrap();
            let r: u64 = it.next().unwrap().parse().unwrap();
            (l - 1, r - 1) // Convert to 0-based indexing
        })
        .collect();

    (arr, queries)
}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

    // let t: u64 = it.next().unwrap().parse().unwrap();
    let t: u64 = 1;

    for _ in 0..t {
        let (arr, queries) = get_input(&mut it);

        let sol: Vec<u64> = Solution::powerful_array(arr, queries);

        for ans in sol {
            println!("{}", ans);
        }
    }
}

impl Solution {
    pub fn powerful_array(arr: Vec<u64>, queries: Vec<(u64, u64)>) -> Vec<u64> {
        const RANGE: usize = 1_000_001;

        let mut sorted_queries: Vec<(u64, u64, usize)> = queries
            .iter()
            .enumerate()
            .map(|(i, &q)| (q.0, q.1, i))
            .collect();

        let sqrt_n: u64 = (arr.len() as f64).sqrt() as u64 + 1;
        sorted_queries.sort_by_key(|&(l, r, _)| (l / sqrt_n, r));

        let mut answers: Vec<u64> = vec![0; queries.len()];
        let mut counters: Vec<u64> = vec![0; RANGE];

        let mut cur_l: u64 = 0;
        let mut cur_r: u64 = 0;
        let mut cur_ans: u64 = arr[0];
        counters[arr[0] as usize] = 1;

        for &(l, r, index) in &sorted_queries {
            let mut add = |i: u64| {
                let x: u64 = arr[i as usize];
                counters[x as usize] += 1;
                cur_ans += (2 * counters[x as usize] - 1) * x;
            };

            while cur_l > l {
                cur_l -= 1;
                add(cur_l);
            }

            while cur_r < r {
                cur_r += 1;
                add(cur_r);
            }

            let mut remove = |i: u64| {
                let x: u64 = arr[i as usize];
                cur_ans -= (2 * counters[x as usize] - 1) * x;
                counters[x as usize] -= 1;
            };

            while cur_l < l {
                remove(cur_l);
                cur_l += 1;
            }

            while cur_r > r {
                remove(cur_r);
                cur_r -= 1;
            }

            answers[index] = cur_ans;
        }

        answers
    }
}
