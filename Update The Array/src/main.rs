use std::io::Read;

fn get_input(it: &mut std::str::SplitWhitespace<'_>) -> (i32, Vec<(i32, i32, i32)>, Vec<i32>) {
    let n: i32 = it.next().unwrap().parse().unwrap();
    let u: i32 = it.next().unwrap().parse().unwrap();

    let updates: Vec<(i32, i32, i32)> = (0..u)
        .map(|_| {
            let l: i32 = it.next().unwrap().parse().unwrap();
            let r: i32 = it.next().unwrap().parse().unwrap();
            let v: i32 = it.next().unwrap().parse().unwrap();
            (l, r, v)
        })
        .collect();

    let q: i32 = it.next().unwrap().parse().unwrap();

    let queries: Vec<i32> = (0..q)
        .map(|_| {
            let idx: i32 = it.next().unwrap().parse().unwrap();
            idx
        })
        .collect();

    (n, updates, queries)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: i32 = it.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let (n, updates, queries) = get_input(&mut it);

        let sol: Vec<i32> = Solution::update_the_array(n, updates, queries);

        for val in sol.iter() {
            println!("{}", val);
        }
    }
}

struct Solution {}

impl Solution {
    pub fn update_the_array(n: i32, updates: Vec<(i32, i32, i32)>, queries: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![0; n as usize];

        for (l, r, v) in updates {
            arr[l as usize] += v;
            if r < n - 1 {
                arr[(r + 1) as usize] -= v;
            }
        }

        for i in 1..n as usize {
            arr[i] += arr[i - 1];
        }

        queries
            .into_iter()
            .map(|idx: i32| arr[idx as usize])
            .collect()
    }
}