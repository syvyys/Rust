use std::str::FromStr;
use std::fmt::{Debug, Display, Formatter};

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
            if value <= node.value {
                node.left.add(value);
            } else {
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

impl<T: Ord + Copy + Debug + FromStr> FromStr for BinaryTree<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tree = BinaryTree::Empty;
        for str in s.split_whitespace() {
            let value = T::from_str(str);
            match value {
                Ok(value) =>  {
                    tree.add(value);
                }
                Err(_) => {
                    return Err(String::from("Error"));
                }
            }
        }
        return Ok(tree);
    }
}

impl<T: Ord + Copy + Debug + Display> Display for TreeNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.left.fmt(f).expect("Error");
        self.right.fmt(f).expect("Error");
        return write!(f, "{:?}", self.value);
    }
}

impl<T: Ord + Copy + Debug + Display> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryTree::Empty =>  {
                write!(f, "")
            }
            BinaryTree::NonEmpty(node) => {
                node.left.fmt(f).expect("Error");
                node.right.fmt(f).expect("Error");
                return write!(f, "{:?} ", node.value);
            }
        }
    }
}