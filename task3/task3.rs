use std::str::FromStr;
use std::fmt::{Debug, Display, Formatter};

struct TreeNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

impl<T: Ord + Copy + Debug> TreeNode<T> {
    fn add(&mut self, value: T) {
        if self.value == value {
            return;
        }

        let next_node = self.next(value);
        match next_node {
            BinaryTree::Empty => {
                *next_node = BinaryTree::<T>::create_leaf(value);
            }
            BinaryTree::NonEmpty(node) => {
                node.add(value);
            }
        }
    }

    fn print(&self) {
        println!("{:?}", self.value);
        self.left.print();
        self.right.print();
    }

    fn next(&mut self, value: T) -> &mut BinaryTree<T> {
        if value < self.value {
            return &mut self.left;
        }
        return &mut self.right;
    }
}

impl<T: Ord + Copy + Debug> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::<T>::create_leaf(value);
            }
            BinaryTree::NonEmpty(tree) => {
                tree.add(value);
            }
        }
    }

    fn print(&self) {
        match self {
            BinaryTree::Empty => return,
            BinaryTree::NonEmpty(tree) => tree.print()
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

fn main() {
    // create and fill tree
    let mut tree = BinaryTree::Empty;
    tree.add(10);
    tree.add(50);
    tree.add(0);
    tree.add(25);

    // print tree
    tree.print();
    println!("{}", tree);

    // create tree from str
    let res = BinaryTree::<i32>::from_str("1 2 3 10");
    match res {
        Ok(tree) => println!("{}", tree),
        Err(error) => println!("{:?}", error)
    }
}
