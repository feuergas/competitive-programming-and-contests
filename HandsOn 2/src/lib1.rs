use std::marker::PhantomData;

pub trait SegmentOperation<T> {
    fn neutral() -> T;
    fn merge(left: T, right: T) -> T;
    fn update(old_value: T, new_value: T) -> T;
    fn lazy_update(old_value: T, new_value: T) -> T {
        Self::update(old_value, new_value)
    }
}

#[derive(Clone, Copy, Default)]
struct SegmentNode<T, Op> {
    value: T,
    lazy_value: Option<T>,
    operation: PhantomData<Op>,
}

impl<T: Default + Ord + Copy, Op: SegmentOperation<T> + Default> SegmentNode<T, Op> {
    fn new() -> Self {
        SegmentNode {
            value: Op::neutral(),
            ..Default::default()
        }
    }

    fn update(&mut self, value: T) {
        self.value = Op::update(self.value, value);

        self.lazy_value = Some(match self.lazy_value.take() {
            Some(x) => Op::lazy_update(x, value),
            None => value,
        });
    }
}

pub struct SegmentTree<T, Op> {
    tree: Vec<SegmentNode<T, Op>>,
    operation: PhantomData<Op>,
}

impl<T: Default + Ord + Clone + Copy, Op: SegmentOperation<T> + Default + Clone>
    SegmentTree<T, Op>
{
    pub fn with_len(n: usize) -> Self {
        Self {
            tree: vec![SegmentNode::<T, Op>::new(); 4 * n],
            operation: PhantomData,
        }
    }

    pub fn from(arr: &Vec<T>) -> Self {
        let mut seg_tree: Self = SegmentTree::with_len(arr.len());
        seg_tree.build(arr, 0, 0, arr.len() - 1);
        seg_tree
    }

    pub fn len(&self) -> usize {
        self.tree.len() / 4
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn build(&mut self, arr: &Vec<T>, node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node].value = arr[start];
            return;
        }

        let mid: usize = start + (end - start) / 2;
        self.build(arr, 2 * node + 1, start, mid);
        self.build(arr, 2 * node + 2, mid + 1, end);
        self.tree[node].value =
            Op::merge(self.tree[2 * node + 1].value, self.tree[2 * node + 2].value);
    }

    pub fn query(&mut self, l: usize, r: usize) -> T {
        self.query_rec(0, 0, self.len() - 1, l, r)
    }

    fn query_rec(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) -> T {
        if r < start || end < l {
            return Op::neutral();
        }

        if l <= start && end <= r {
            return self.tree[node].value;
        }

        self.push_propagation(node);

        let mid: usize = start + (end - start) / 2;
        let left_query: T = self.query_rec(2 * node + 1, start, mid, l, r);
        let right_query: T = self.query_rec(2 * node + 2, mid + 1, end, l, r);
        Op::merge(left_query, right_query)
    }

    pub fn update(&mut self, left: usize, right: usize, value: T) {
        self.update_rec(0, 0, self.len() - 1, left, right, value);
    }

    fn update_rec(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize, value: T) {
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
            Op::merge(self.tree[2 * node + 1].value, self.tree[2 * node + 2].value);
    }

    fn push_propagation(&mut self, node: usize) {
        if let Some(lazy_value) = self.tree[node].lazy_value {
            self.tree[2 * node + 1].update(lazy_value);
            self.tree[2 * node + 2].update(lazy_value);

            self.tree[node].lazy_value = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_p1() {
        let t: i32 = 10;

        for index in 0..t {
            let (arr, queries) = read_input(index);

            #[derive(Default, Clone)]
            struct FirstOp;

            impl SegmentOperation<i32> for FirstOp {
                fn neutral() -> i32 {
                    i32::MIN
                }

                fn merge(left: i32, right: i32) -> i32 {
                    left.max(right)
                }

                fn update(old_value: i32, new_value: i32) -> i32 {
                    old_value.min(new_value)
                }
            }

            let mut tree: SegmentTree<i32, FirstOp> = SegmentTree::from(&arr);

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
        }
    }

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

    type Query = Vec<(i8, usize, usize, i32)>;
    const FOLDER_PATH: &str = "Testset_handson2_p1"; // Set path to input and output folder location

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
}
