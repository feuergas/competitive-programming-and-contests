// use handson_3::*;
use std::fs;

fn main() {
    let t: i32 = 5;

    for index in 0..t {
        let (n, d, mut cities) = read_input(FOLDER_PATH, index);

        let prefix_sum = |arr: &Vec<i32>| {
            let mut prefix = Vec::with_capacity(arr.len() + 1);
            prefix.push(0);
            for &x in arr {
                prefix.push(prefix.last().unwrap() + x);
            }
            prefix
        };

        // Convert each city to its prefix sum array
        cities.iter_mut().for_each(|city| {
            *city = prefix_sum(city);
        });

        // store in best[i][j] the most value we can get after having visited the first i cities in j days
        let mut best = vec![vec![0; d+1]; n+1];

        // When i = 0 we have visited no cities, hence the value is always 0
        // When j = 0 we have spent no days in any city, hence the value is always 0
        for i in 1..=n {
            for j in 1..=d {
                for k in 0..=d {
                    if j < k {
                        break;
                    }
                    best[i][j] = best[i][j].max(best[i - 1][j - k] + cities[i - 1][k]);
                }
            }
        }

        let mut answers: Vec<i32> = Vec::new();
        answers.push(best[n][d]);

        let correct_answers = read_output(FOLDER_PATH, index);
        assert_eq!(answers, correct_answers);
        println!("TESTCASE {index} CORRECT");
    }
}

const FOLDER_PATH: &str = "Testset_handson3_p1"; // Set path to input and output folder location

fn read_input(folder_path: &str, index: i32) -> (usize, usize, Vec<Vec<i32>>) {
    let input_name: String = format!("input{index}.txt");
    let input: String = fs::read_to_string(format!("{folder_path}/{input_name}")).unwrap();
    let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let d: usize = it.next().unwrap().parse().unwrap();

    let cities = (0..n)
        .map(|_| {
            (0..d)
                .map(|_| it.next().unwrap().parse().unwrap())
                .collect()
        })
        .collect();

    (n, d, cities)
}

fn read_output(folder_path: &str, index: i32) -> Vec<i32> {
    let output_name: String = format!("output{index}.txt");
    let output: String = fs::read_to_string(format!("{folder_path}/{output_name}")).unwrap();

    output
        .split_whitespace()
        .map(|s: &str| s.parse().unwrap())
        .collect()
}
