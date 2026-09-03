use handson_3::length_of_lis;
use std::fs;

fn main() {
    let t: i32 = 11;

    for index in 0..t {
        let mut topics = read_input(FOLDER_PATH, index);

        topics.sort_by_key(|&(b, d)| (b, -d));

        let difficulty = topics.iter().map(|&(_, d)| d).collect();

        let answers: Vec<i32> = vec![length_of_lis(difficulty)];

        let correct_answers = read_output(FOLDER_PATH, index);
        assert_eq!(answers, correct_answers);
        println!("TESTCASE {index} CORRECT");
    }
}

const FOLDER_PATH: &str = "Testset_handson3_p2"; // Set path to input and output folder location

fn read_input(folder_path: &str, index: i32) -> Vec<(i32, i32)> {
    let input_name: String = format!("input{index}.txt");
    let input: String = fs::read_to_string(format!("{folder_path}/{input_name}")).unwrap();
    let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    (0..n)
        .map(|_| {
            let b = it.next().unwrap().parse().unwrap();
            let d = it.next().unwrap().parse().unwrap();
            (b, d)
        })
        .collect()
}

fn read_output(folder_path: &str, index: i32) -> Vec<i32> {
    let output_name: String = format!("output{index}.txt");
    let output: String = fs::read_to_string(format!("{folder_path}/{output_name}")).unwrap();

    output
        .split_whitespace()
        .map(|s: &str| s.parse().unwrap())
        .collect()
}
