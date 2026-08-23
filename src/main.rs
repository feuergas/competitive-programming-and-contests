// `min!` will calculate the minimum of any number of arguments.
macro_rules! min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `min!` on the tail `$y`
        std::cmp::min($x, min!($($y),+))
    )
}

// `max!` will calculate the maximum of any number of arguments.
macro_rules! max {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `max!` on the tail `$y`
        std::cmp::max($x, max!($($y),+))
    )
}

#[derive(Debug)]

pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    pub fn from(arr: &[Option<u32>]) -> Self {
        assert!(!arr.is_empty(), "Tree cannot be empty");
        assert!(arr[0].is_some(), "First node cannot be null");

        let mut tree: Self = Self::with_root(arr[0].unwrap());

        for (i, key) in arr.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if let Some(key) = *key {
                tree.add_node((i - 1) / 2, key, i % 2 == 1);
            }
        }

        tree
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id: usize = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child: &mut Option<usize> = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node: &Node = &self.nodes[id];

            let sum_left: u32 = self.rec_sum(node.id_left);
            let sum_right: u32 = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    pub fn is_bst(&self) -> bool {
        self.check_bst(Some(0)).0
    }

    fn check_bst(&self, node_id: Option<usize>) -> (bool, u32, u32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node: &Node = &self.nodes[id];

            let (left_bst, min_left, max_left) = self.check_bst(node.id_left);
            let (right_bst, min_right, max_right) = self.check_bst(node.id_right);

            let is_balanced: bool = max_left < node.key && node.key < min_right;

            return (
                is_balanced && left_bst && right_bst,
                min!(node.key, min_left, min_right),
                max!(node.key, max_left, max_right),
            );
        }

        (true, u32::MAX, u32::MIN)
    }

    pub fn max_path_sum(&self) -> u32 {
        self.max_branch_sum(Some(0)).1
    }

    fn max_branch_sum(&self, node_id: Option<usize>) -> (u32, u32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node: &Node = &self.nodes[id];

            let (max_branch_left, max_path_left) = self.max_branch_sum(node.id_left);
            let (max_branch_right, max_path_right) = self.max_branch_sum(node.id_right);

            let max_path_center: u32 = max_branch_left + max_branch_right + node.key;

            return (
                max!(max_branch_left, max_branch_right) + node.key,
                max!(max_path_left, max_path_right, max_path_center),
            );
        }

        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree: Tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
    }

    #[test]
    fn test_bst() {
        let mut tree: Tree = Tree::with_root(10);

        assert_eq!(tree.is_bst(), true);

        tree.add_node(0, 5, true);
        tree.add_node(0, 22, false);

        assert_eq!(tree.is_bst(), true);

        let tree: Tree = Tree::from(&[Some(2), Some(1), Some(3)]);

        assert_eq!(tree.is_bst(), true);

        let tree: Tree = Tree::from(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);

        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_max_path_sum() {
        let tree = Tree::from(&[Some(1), Some(2), Some(3)]);

        assert_eq!(tree.max_path_sum(), 6);

        let tree = Tree::from(&[Some(0), Some(5), Some(20), None, None, Some(15), Some(7)]);

        assert_eq!(tree.max_path_sum(), 42);
    }
}

fn main() {}
