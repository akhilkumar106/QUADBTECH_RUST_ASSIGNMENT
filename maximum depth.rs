// 8. Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::io;

// Define a simple binary tree structure
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn construct_tree() -> Option<Box<TreeNode>> {
    println!("Enter the value of the root node:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let val: i32 = input.trim().parse().expect("Invalid value");

    if val == -1 {
        return None;
    }

    let mut root = Box::new(TreeNode::new(val));

    println!("Enter the left subtree of node {}: (-1 for none)", val);
    root.left = construct_tree();

    println!("Enter the right subtree of node {}: (-1 for none)", val);
    root.right = construct_tree();

    Some(root)
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    println!("Construct the binary tree:");
    let root = construct_tree();

    let depth = max_depth(&root);
    println!("The maximum depth of the binary tree is: {}", depth);
}

