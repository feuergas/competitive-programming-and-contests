use std::{
    cmp::{max, min},
    fs,
};

type Query = Vec<(i8, usize, usize, i32)>;

fn get_input(it: &mut std::str::SplitWhitespace<'_>) -> (Vec<i32>, Query) {
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let arr: Vec<i32> = (0..n)
        .map(|_| it.next().unwrap().parse().unwrap())
        .collect();

    let queries: Query = (0..m)
        .map(|_| {
            let q_type: i8 = it.next().unwrap().parse().unwrap();
            let i: usize = it.next().unwrap().parse().unwrap();
            let j: usize = it.next().unwrap().parse().unwrap();
            let mut value: i32 = 0;
            if q_type == 0 {
                value = it.next().unwrap().parse().unwrap();
            }
            (q_type, i - 1, j - 1, value) // Conversion to 0-based
        })
        .collect();

    (arr, queries)
}

const FOLDER_PATH: &str = "handson_2/Testset_handson2_p1";

fn read_input(index: i32) -> (Vec<i32>, Query) {
    let input_name: String = format!("input{index}.txt");
    let input: String = fs::read_to_string(format!("{FOLDER_PATH}/{input_name}")).unwrap();
    let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

    get_input(&mut it)
}

fn read_output(index: i32) -> Vec<i32> {
    let output_name: String = format!("output{index}.txt");
    let output: String = fs::read_to_string(format!("{FOLDER_PATH}/{output_name}")).unwrap();

    output
        .split_whitespace()
        .map(|s: &str| s.parse().unwrap())
        .collect()
}

fn main() {
    let t: i32 = 10;

    for index in 0..t {
        let (arr, queries) = read_input(index);

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

        let correct_answers: Vec<i32> = read_output(index);
        assert_eq!(answers, correct_answers);
        println!("TESTCASE {index} PASSED");
    }
}

#[derive(Clone, Default, Debug)]
struct SegmentNode {
    value: i32,
    lazy_value: i32,
    needs_propagation: bool,
    range: (i32, i32),
}

impl SegmentNode {
    fn new() -> Self {
        SegmentNode {
            lazy_value: i32::MAX,
            range: (-1, -1),
            ..Default::default()
        }
    }

    fn update(&mut self, value: i32) {
        self.value = min(self.value, value);
        self.lazy_value = min(self.lazy_value, value);
        self.needs_propagation = true;
    }
}

#[derive(Debug)]
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
        self.tree[node].range = (start as i32, end as i32);

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
