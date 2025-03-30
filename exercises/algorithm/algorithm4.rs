/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug, Clone)]
struct TreeNode<T>
where
    T: Ord + Clone,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord + Clone,
{
    root: Option<Box<TreeNode<T>>>,
}
// impl<T: Copy> Copy for TreeNode<T> {}
// impl<T: Copy> Copy for Option<Box<TreeNode<T>>> {}

impl<T> TreeNode<T>
where
    T: Ord + Clone,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO

        match self.root {
            None => self.root = Some(Box::new(TreeNode::new(value))),
            Some(ref mut node) => TreeNode::insert(node, value),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        if let Some(node) = &self.root {
            if node.value == value {
                true
            } else if node.value < value {
                BinarySearchTree {
                    root: node.right.clone(),
                }
                .search(value)
            } else {
                BinarySearchTree {
                    root: node.left.clone(),
                }
                .search(value)
            }
        } else {
            false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord + Clone,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        let new_node = Some(Box::new(TreeNode::new(value.clone())));

        if value == self.value {
            return ();
        } else if value < self.value {
            match &mut self.left {
                None => self.left = new_node,
                Some(ref mut node) => node.insert(value),
            }
        } else {
            match &mut self.right {
                None => self.right = new_node,
                Some(ref mut node) => node.insert(value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
