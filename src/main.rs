use std::{io::Read, collections::HashMap};

struct Solution {}

fn get_input(it: &mut std::str::SplitWhitespace<'_>) -> Vec<(i32, i32)> {
    let n: usize = it.next().unwrap().parse().unwrap();

    let segments: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            let l: i32 = it.next().unwrap().parse().unwrap();
            let r: i32 = it.next().unwrap().parse().unwrap();
            (l, r)
        })
        .collect();

    segments
}

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it: std::str::SplitWhitespace<'_> = input.split_whitespace();

    // let t: i32 = it.next().unwrap().parse().unwrap();
    let t: i32 = 1;

    for _ in 0..t {
        let segments: Vec<(i32, i32)> = get_input(&mut it);

        let sol: Vec<u32> = Solution::nested_segments_segtree(segments);

        for ans in sol {
            println!("{}", ans);
        }
    }
}

pub struct SegmentTree {
    tree: Vec<i32>,
}

impl SegmentTree {
    fn merge(&self, left: i32, right: i32) -> i32 {
        left + right
    }

    pub fn with_len(n: usize) -> Self {
        Self {
            tree: vec![0; 4 * n],
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
            self.tree[node] = arr[start];
        } else {
            let mid: usize = start + (end - start) / 2;
            self.build(arr, 2 * node + 1, start, mid);
            self.build(arr, 2 * node + 2, mid + 1, end);
            self.tree[node] = self.merge(self.tree[2 * node + 1], self.tree[2 * node + 2]);
        }
    }

    pub fn query(&self, l: usize, r: usize) -> i32 {
        self.query_rec(0, 0, self.len() - 1, l, r)
    }

    fn query_rec(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        if r < start || end < l {
            return 0;
        }

        if l <= start && end <= r {
            return self.tree[node];
        }

        let mid: usize = start + (end - start) / 2;
        let left_query: i32 = self.query_rec(2 * node + 1, start, mid, l, r);
        let right_query: i32 = self.query_rec(2 * node + 2, mid + 1, end, l, r);
        self.merge(left_query, right_query)
    }

    pub fn update(&mut self, index: usize, value: i32) {
        self.update_rec(0, 0, self.len() - 1, index, value);
    }

    fn update_rec(&mut self, node: usize, start: usize, end: usize, index: usize, value: i32) {
        if start == end {
            self.tree[node] = value;
        } else {
            let mid: usize = start + (end - start) / 2;
            if index <= mid {
                self.update_rec(2 * node + 1, start, mid, index, value);
            } else {
                self.update_rec(2 * node + 2, mid + 1, end, index, value);
            }
            self.tree[node] = self.merge(self.tree[2 * node + 1], self.tree[2 * node + 2]);
        }
    }
}

impl Solution {
    pub fn nested_segments_segtree(mut segments: Vec<(i32, i32)>) -> Vec<u32> {
        // Compresses the coordinates of the segments to a smaller range, so that we can use a Fenwick tree
        // to count the number of segments that are nested within each other.
        let mut coords: Vec<i32> = segments
            .iter()
            .flat_map(|&(l, r)| vec![l, r])
            .collect::<Vec<i32>>();

        coords.sort();

        let positions: HashMap<i32, usize> = coords
            .iter()
            .enumerate()
            .map(|(i, &coord)| (coord, i))
            .collect();

        for (l, r) in &mut segments {
            *l = positions[l] as i32;
            *r = positions[r] as i32;
        }


        let answers: Vec<u32> = vec![0; segments.len()];



        answers
    }
}
