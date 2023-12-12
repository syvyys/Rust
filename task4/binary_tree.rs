use std::fmt::{Debug};

pub struct TreeNode<T> {
    pub value: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>
}

pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

impl<T: PartialOrd + Copy + Debug> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        if let BinaryTree::NonEmpty(node) = self {
            if value < node.value {
                node.left.add(value);
            } else if value > node.value {
                node.right.add(value);
            }
        } else {
            *self = BinaryTree::<T>::create_leaf(value);
        }
    }

    pub fn print(&self) {
        if let BinaryTree::NonEmpty(node) = self {
            println!("{:?}", node.value);
            node.left.print();
            node.right.print();
        }
    }

    fn create_leaf(value: T) -> BinaryTree<T> {
        let node = TreeNode {
            value,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty
        };
        return BinaryTree::NonEmpty(Box::new(node));
    }
}

impl<T: Copy + PartialOrd + Debug> Clone for BinaryTree<T> {
    fn clone(&self) -> Self {
        let mut res = BinaryTree::Empty;
        let mut stack = vec![self];
        while let Some(tree) = stack.pop() {
            if let BinaryTree::NonEmpty(node) = tree {
                res.add(node.value);
                stack.push(&node.right);
                stack.push(&node.left);
            }
        }
        return res;
    }
}
