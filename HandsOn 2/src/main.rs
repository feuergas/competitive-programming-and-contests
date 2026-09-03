// use handson_2::*;
use std::fs;

fn main() {
    let t: i32 = 8;

    for index in 0..t {
        let (n, segments, queries) = read_input(FOLDER_PATH, index);

        let mut arr: Vec<i32> = vec![0; n + 1];
        segments.iter().for_each(|&(l, r)| {
            arr[l] += 1;
            arr[r + 1] -= 1;
        });

        let arr: Vec<i32> = arr
            .iter()
            .scan(0, |acc, &val| {
                *acc += val;
                Some(*acc)
            })
            .collect();

        let mut positions: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for (idx, &val) in arr.iter().enumerate() {
            positions[val as usize].push(idx);
        }

        let mut answers: Vec<usize> = Vec::new();
        for (i, j, k) in queries {
            let binary_search = |target: usize| -> usize {
                let mut left: usize = 0;
                let mut right: usize = positions[k].len();

                while left < right {
                    let mid = left + (right - left) / 2;

                    if positions[k][mid] < target {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }

                left
            };

            if positions[k].is_empty() {
                answers.push(0);
                continue;
            }

            let idx = binary_search(i);
            if idx == positions[k].len() {
                answers.push(0);
                continue;
            }

            let pos = positions[k][idx];
            answers.push(if pos <= j { 1 } else { 0 });
        }

        let correct_answers = read_output(FOLDER_PATH, index);
        assert_eq!(answers, correct_answers);
        println!("TESTCASE {index} CORRECT");
    }
}

const FOLDER_PATH: &str = "Testset_handson2_p2"; // Set path to input and output folder location

type Queries = Vec<(usize, usize, usize)>;

fn read_input(folder_path: &str, index: i32) -> (usize, Vec<(usize, usize)>, Queries) {
    let input_name: String = format!("input{index}.txt");
    let input: String = fs::read_to_string(format!("{folder_path}/{input_name}")).unwrap();
    let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let segments = (0..n)
        .map(|_| {
            let left = it.next().unwrap().parse().unwrap();
            let right = it.next().unwrap().parse().unwrap();
            (left, right)
        })
        .collect();

    let queries = (0..m)
        .map(|_| {
            let i = it.next().unwrap().parse().unwrap();
            let j = it.next().unwrap().parse().unwrap();
            let k = it.next().unwrap().parse().unwrap();
            (i, j, k)
        })
        .collect();

    (n, segments, queries)
}

fn read_output(folder_path: &str, index: i32) -> Vec<usize> {
    let output_name: String = format!("output{index}.txt");
    let output: String = fs::read_to_string(format!("{folder_path}/{output_name}")).unwrap();

    output
        .split_whitespace()
        .map(|s: &str| s.parse().unwrap())
        .collect()
}
