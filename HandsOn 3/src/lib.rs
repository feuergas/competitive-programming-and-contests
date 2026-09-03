pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut smallest_endpoint: Vec<i32> = vec![nums[0]];

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_p1() {
        const TEST_PATH_1: &str = "Testset_handson3_p1";
        let t: i32 = 5;

        for index in 0..t {
            let (n, d, mut cities) = read_input1(TEST_PATH_1, index);

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
            let mut best = vec![vec![0; d + 1]; n + 1];

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

            let answers: Vec<i32> = vec![best[n][d]];

            let correct_answers = read_output(TEST_PATH_1, index);
            assert_eq!(answers, correct_answers);
        }
    }

    #[test]
    fn test_p2() {
        const TEST_PATH_2: &str = "Testset_handson3_p2";
        let t: i32 = 11;

        for index in 0..t {
            let mut topics = read_input2(TEST_PATH_2, index);

            topics.sort_by_key(|&(b, d)| (b, -d));

            let difficulty = topics.iter().map(|&(_, d)| d).collect();

            let answers: Vec<i32> = vec![length_of_lis(difficulty)];

            let correct_answers = read_output(TEST_PATH_2, index);
            assert_eq!(answers, correct_answers);
        }
    }

    fn read_input1(folder_path: &str, index: i32) -> (usize, usize, Vec<Vec<i32>>) {
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

    fn read_input2(folder_path: &str, index: i32) -> Vec<(i32, i32)> {
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
}
