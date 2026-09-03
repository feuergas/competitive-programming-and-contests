use std::cmp::{max, min};

#[derive(Clone, Default)]
struct SegmentNode {
    value: i32,
    lazy_value: i32,
    needs_propagation: bool,
}

impl SegmentNode {
    fn new() -> Self {
        SegmentNode {
            lazy_value: i32::MAX,
            ..Default::default()
        }
    }

    fn update(&mut self, value: i32) {
        self.value = min(self.value, value);
        self.lazy_value = min(self.lazy_value, value);
        self.needs_propagation = true;
    }
}

pub struct SegmentTree {
    tree: Vec<SegmentNode>,
}

impl SegmentTree {
    fn merge(&self, left: i32, right: i32) -> i32 {
        max(left, right)
    }

    pub fn with_len(n: usize) -> Self {
        Self {
            tree: vec![SegmentNode::new(); 4 * n],
        }
    }

    pub fn from(arr: &Vec<i32>) -> Self {
        let mut seg_tree: SegmentTree = SegmentTree::with_len(arr.len());
        seg_tree.build(arr, 0, 0, arr.len() - 1);
        seg_tree
    }

    pub fn len(&self) -> usize {
        self.tree.len() / 4
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn build(&mut self, arr: &Vec<i32>, node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node].value = arr[start];
            return;
        }

        let mid: usize = start + (end - start) / 2;
        self.build(arr, 2 * node + 1, start, mid);
        self.build(arr, 2 * node + 2, mid + 1, end);
        self.tree[node].value =
            self.merge(self.tree[2 * node + 1].value, self.tree[2 * node + 2].value);
    }

    pub fn query(&mut self, l: usize, r: usize) -> i32 {
        self.query_rec(0, 0, self.len() - 1, l, r)
    }

    fn query_rec(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        if r < start || end < l {
            return i32::MIN;
        }

        if l <= start && end <= r {
            return self.tree[node].value;
        }

        self.push_propagation(node);

        let mid: usize = start + (end - start) / 2;
        let left_query: i32 = self.query_rec(2 * node + 1, start, mid, l, r);
        let right_query: i32 = self.query_rec(2 * node + 2, mid + 1, end, l, r);
        self.merge(left_query, right_query)
    }

    pub fn update(&mut self, left: usize, right: usize, value: i32) {
        self.update_rec(0, 0, self.len() - 1, left, right, value);
    }

    fn update_rec(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
        value: i32,
    ) {
        if r < start || end < l {
            return;
        }

        if l <= start && end <= r {
            self.tree[node].update(value);
            return;
        }

        self.push_propagation(node);

        let mid: usize = start + (end - start) / 2;
        self.update_rec(2 * node + 1, start, mid, l, r, value);
        self.update_rec(2 * node + 2, mid + 1, end, l, r, value);

        self.tree[node].value =
            self.merge(self.tree[2 * node + 1].value, self.tree[2 * node + 2].value);
    }

    fn push_propagation(&mut self, node: usize) {
        if !self.tree[node].needs_propagation {
            return;
        }
        let lazy_value: i32 = self.tree[node].lazy_value;

        self.tree[2 * node + 1].update(lazy_value);
        self.tree[2 * node + 2].update(lazy_value);

        self.tree[node].needs_propagation = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_p1() {
        const TEST_PATH_1: &str = "Testset_handson2_p1";
        let t: i32 = 10;

        for index in 0..t {
            let (arr, queries) = read_input1(TEST_PATH_1, index);

            let mut tree: SegmentTree = SegmentTree::from(&arr);

            let mut answers: Vec<i32> = Vec::new();
            for (q_type, i, j, value) in queries {
                match q_type {
                    0 => tree.update(i, j, value),
                    1 => {
                        let answer: i32 = tree.query(i, j);
                        answers.push(answer);
                    }
                    _ => {}
                }
            }

            let correct_answers: Vec<i32> = read_output(TEST_PATH_1, index);
            assert_eq!(answers, correct_answers);
        }
    }

    #[test]
    fn test_p2() {
        const TEST_PATH_2: &str = "Testset_handson2_p2";
        let t: i32 = 8;

        for index in 0..t {
            let (n, segments, queries) = read_input2(TEST_PATH_2, index);

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

            let mut answers: Vec<i32> = Vec::new();
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

            let correct_answers = read_output(TEST_PATH_2, index);
            assert_eq!(answers, correct_answers);
        }
    }

    type Queries1 = Vec<(i8, usize, usize, i32)>;

    fn read_input1(folder_path: &str, index: i32) -> (Vec<i32>, Queries1) {
        let input_name: String = format!("input{index}.txt");
        let input: String = fs::read_to_string(format!("{folder_path}/{input_name}")).unwrap();
        let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

        let n: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        let arr = (0..n)
            .map(|_| it.next().unwrap().parse().unwrap())
            .collect();

        let queries = (0..m)
            .map(|_| {
                let q_type = it.next().unwrap().parse().unwrap();
                let i: usize = it.next().unwrap().parse().unwrap();
                let j: usize = it.next().unwrap().parse().unwrap();
                let mut value = 0;
                if q_type == 0 {
                    value = it.next().unwrap().parse().unwrap();
                }
                (q_type, i - 1, j - 1, value) // Conversion to 0-based
            })
            .collect();

        (arr, queries)
    }

    type Queries2 = Vec<(usize, usize, usize)>;

    fn read_input2(folder_path: &str, index: i32) -> (usize, Vec<(usize, usize)>, Queries2) {
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

    fn read_output(folder_path: &str, index: i32) -> Vec<i32> {
        let output_name: String = format!("output{index}.txt");
        let output: String = fs::read_to_string(format!("{folder_path}/{output_name}")).unwrap();

        output
            .split_whitespace()
            .map(|s: &str| s.parse().unwrap())
            .collect()
    }
}
