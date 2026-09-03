use handson_2::*;
use std::fs;

fn main() {
    let t: i32 = 8;

    for index in 0..t {
        let (n, segments, queries) = read_input(FOLDER_PATH, index);

        let mut arr: Vec<i32> = vec![0; n as usize + 1];
        segments.iter().for_each(|&(l, r)| {
            arr[l as usize] += 1;
            arr[r as usize + 1] -= 1;
        });

        let arr: Vec<i32> = arr
            .iter()
            .scan(0, |acc, &val| {
                *acc += val;
                Some(*acc)
            })
            .collect();

        let mut answers: Vec<i32> = Vec::new();
        for (i, j, k) in queries {
        }

        let correct_answers = read_output(FOLDER_PATH, index);
        assert_eq!(answers, correct_answers);
        println!("TESTCASE {index} CORRECT");
    }
}

fn get_input(
    it: &mut std::str::SplitWhitespace<'_>,
) -> (i32, Vec<(i32, i32)>, Vec<(i32, i32, i32)>) {
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let segments: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            let left: i32 = it.next().unwrap().parse().unwrap();
            let right: i32 = it.next().unwrap().parse().unwrap();
            (left, right)
        })
        .collect();

    let queries = (0..m)
        .map(|_| {
            let i: i32 = it.next().unwrap().parse().unwrap();
            let j: i32 = it.next().unwrap().parse().unwrap();
            let k: i32 = it.next().unwrap().parse().unwrap();
            (i, j, k)
        })
        .collect();

    (n as i32, segments, queries)
}

const FOLDER_PATH: &str = "Testset_handson2_p2"; // Set path to input and output folder location

fn read_input(folder_path: &str,index: i32) -> (i32, Vec<(i32, i32)>, Vec<(i32, i32, i32)>) {
    let input_name: String = format!("input{index}.txt");
    let input: String = fs::read_to_string(format!("{folder_path}/{input_name}")).unwrap();
    let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

    get_input(&mut it)
}

fn read_output(folder_path: &str, index: i32) -> Vec<i32> {
    let output_name: String = format!("output{index}.txt");
    let output: String = fs::read_to_string(format!("{folder_path}/{output_name}")).unwrap();

    output
        .split_whitespace()
        .map(|s: &str| s.parse().unwrap())
        .collect()
}
