#set text(font: "Noto Sans")

= Exercise 1
The function ```rust check_bst``` checks whether the given binary tree is a *Binary Search Tree (BST)*. For a tree to be a *BST* it suffices to check the following conditions for every node:
  1. The maximum value in the left subtree is smaller than the node's value;
  2. The minimum value in the right subtree is greater than node's value.
If either of these does not exist we vacuously assign to it the neutral elements of respectively the ```rust min``` and ```rust max``` functions (i.e. ```rust u32::MAX``` and ```rust u32::MIN```), so that the corresponding condition is satisfied.

For each subtree we compute the tuple ```rust (is_bst, min_value, max_value)``` where ```rust is_bst``` is a boolean representing whether the subtree is itself a *BST*. Given any subtree we can compute recursively the tuple of values for its left and right subtrees and then combine this information to compute the tuple of the subtree itself. Notice that, since any subtree of a *BST* is also a *BST*, the boolean ```rust is_bst``` can be used to propagate information upwards along the tree. Calling ```rust check_bst``` on the tree itself gives us the answer.
= Exercise 2
The function ```rust max_branch_sum``` is called at a node and recursively computes the tuple ```rust (max_branch, max_path)```:
- ```rust max_branch``` gives the maximum path sum from the node to a leaf in the subtree rooted at the node (which we hereby refer to as _branch_);
- ```rust max_path``` gives the maximum path sum between two leaves in the subtree rooted at the node.
The tuple is computed at a node recursively using the following observations:
- The maximum branch starting at a node consists of the larger of the maximum branches of its left and right subtrees, plus the value of the node itself;
- The maximum path sum of a tree can either contain the root or not. If it does not, it is the greater between its left and right subtrees' maximum path sums. If it does, it consists of the maximum branch from the left subtree, the maximum branch from the right subtree, and the value of the root itself.
The maximum path sum is given by calling ```rust max_branch_sum``` at the root of the tree.