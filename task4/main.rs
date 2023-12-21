pub mod binary_tree;

use std::fmt::Debug;
use std::str::FromStr;
use crate::binary_tree::{BinaryTree};

// ----------- ITERATOR ------------

pub struct BinaryTreeIterator<T> {
    pub stack: Vec<BinaryTree<T>>
}

impl<T> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = BinaryTreeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        let stack = vec![self];
        return BinaryTreeIterator { stack };
    }
}

impl<T> Iterator for BinaryTreeIterator<T>
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        while let Some(tree) = self.stack.pop() {
            if let BinaryTree::NonEmpty(node) = tree {
                self.stack.push(node.right);
                self.stack.push(node.left);
                return Some(node.value);
            }
        }
        return None;
    }
}

impl<U: PartialOrd + Copy + Debug> FromIterator<U> for BinaryTreeIterator<U> {
    fn from_iter<T: IntoIterator<Item=U>>(iter: T) -> Self {
        let mut tree = BinaryTree::Empty;
        for i in iter {
            tree.add(i);
        }
        return tree.into_iter();
    }
}

// ----------- REF ITERATOR ------------

pub struct RefBinaryTreeIterator<'a, T: 'a> {
    stack: Vec<&'a BinaryTree<T>>
}

impl<'a, T> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = RefBinaryTreeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let stack = vec![self];
        return RefBinaryTreeIterator { stack };
    }
}

impl<'a, T> Iterator for RefBinaryTreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        while let Some(tree) = self.stack.pop() {
            if let BinaryTree::NonEmpty(node) = tree {
                self.stack.push(&node.right);
                self.stack.push(&node.left);
                return Some(&node.value);
            }
        }
        return None;
    }
}

// ----------- MUT REF ITERATOR ------------

pub struct MutRefBinaryTreeIterator<'a, T: 'a> {
    stack: Vec<&'a mut BinaryTree<T>>
}

impl<'a, T> IntoIterator for &'a mut BinaryTree<T> {
    type Item = &'a mut T;
    type IntoIter = MutRefBinaryTreeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let stack = vec![self];
        return MutRefBinaryTreeIterator { stack };
    }
}

impl<'a, T> Iterator for MutRefBinaryTreeIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        while let Some(tree) = self.stack.pop() {
            if let BinaryTree::NonEmpty(node) = tree {
                self.stack.push(&mut node.right);
                self.stack.push(&mut node.left);
                return Some(&mut node.value);
            }
        }
        return None;
    }
}

// ----------- TESTS ------------

fn tree_to_vec<T: Copy>(tree: &BinaryTree<T>) -> Vec<T> {
    let mut vec: Vec<T> = vec![];
    for elem in tree {
        vec.push(*elem);
    }
    return vec;
}

#[test]
fn test0() {
    let tree = BinaryTree::<i32>::from_str("1 2 3 10").unwrap();
    assert_eq!(tree_to_vec(&tree), vec![1, 2, 3, 10]);

    let tree = BinaryTree::<i32>::from_str("1 1 1 3 10").unwrap();
    assert_eq!(tree_to_vec(&tree), vec![1, 1, 1, 3, 10]);

    match BinaryTree::<i32>::from_str("1 1 A 1 3 10") {
        Ok(_) => {
            assert!(false);
        },
        Err(error) => {
            assert_eq!(error, "Error");
        }
    }
}

#[test]
fn test1() {
    let mut tree = BinaryTree::Empty;
    tree.add(0);
    tree.add(-10);
    tree.add(-12);
    tree.add(-9);
    tree.add(1);

    let vec = vec![0, -10, -12, -9, 1];
    for (i, val) in (&tree).into_iter().enumerate() {
        assert_eq!(vec[i], *val);
    }
    for (i, val) in (&mut tree).into_iter().enumerate() {
        assert_eq!(vec[i], *val);
    }
    for (i, val) in tree.into_iter().enumerate() {
        assert_eq!(vec[i], val);
    }
}

#[test]
fn test2() {
    let vec = vec![0, -10, -12, -9, 1];
    assert_eq!(BinaryTreeIterator::from_iter(vec.clone()).max(), Some(1));
    assert_eq!(BinaryTreeIterator::from_iter(vec.clone()).min(), Some(-12));
    assert_eq!(BinaryTreeIterator::from_iter(vec.clone()).sum::<i32>(), -30);
    assert_eq!(BinaryTreeIterator::from_iter(vec.clone()).product::<i32>(), 0);
    assert_eq!(BinaryTreeIterator::from_iter(vec.clone()).count(), 5);

    let vec = vec![1, 1, 1, 2, 2, 8, -5];
    assert_eq!(BinaryTreeIterator::from_iter(vec.clone()).max(), Some(8));
    assert_eq!(BinaryTreeIterator::from_iter(vec.clone()).min(), Some(-5));
    assert_eq!(BinaryTreeIterator::from_iter(vec.clone()).sum::<i32>(), 10);
    assert_eq!(BinaryTreeIterator::from_iter(vec.clone()).product::<i32>(), -160);
    assert_eq!(BinaryTreeIterator::from_iter(vec.clone()).count(), 7);
}

#[test]
fn test3() {
    let mut tree = BinaryTree::Empty;
    tree.add(0);
    tree.add(-10);
    tree.add(-12);
    tree.add(-9);
    tree.add(1);

    let mut vec: Vec<i32> = vec![];
    (&tree).into_iter().for_each(|item| {
        vec.push(item + 12);
    });
    assert_eq!(vec, vec![12, 2, 0, 3, 13]);
}

#[test]
fn test4() {
    assert_eq!(BinaryTreeIterator::from_iter(
        vec![0, -10, -12, -9, 1]
    ).any(|x| x > 0), true);

    assert_eq!(BinaryTreeIterator::from_iter(
        vec![-8, -10, -12, -9, -1]
    ).any(|x| x > 0), false);

    assert_eq!(BinaryTreeIterator::from_iter(
        vec![0, 0, 50, 60, 70]
    ).any(|x| x * x > 100000000), false);

    assert_eq!(BinaryTreeIterator::from_iter(
        vec![0, -10, -12, -9, 1]
    ).all(|x| x > 0), false);

    assert_eq!(BinaryTreeIterator::from_iter(
        vec![-8, -10, -12, -9, -1]
    ).all(|x| x * x * -1 <= 0), true);
}

#[test]
fn test5() {
    let mut tree = BinaryTree::Empty;
    tree.add(0);
    tree.add(-10);
    tree.add(-12);
    tree.add(-9);
    tree.add(1);

    assert_eq!((&tree).into_iter().map(|x| x + 100).collect::<Vec<i32>>(),
               vec![100, 90, 88, 91, 101]);
    assert_eq!((&tree).into_iter().map(|x| x * x).collect::<Vec<i32>>(),
               vec![0, 100, 144, 81, 1]);
    assert_eq!(tree.into_iter().filter(|x| *x >= 0).collect::<Vec<i32>>(),
               vec![0, 1]);
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
    let tree = BinaryTree::<i32>::from_str("1 2 3 10").unwrap();
    println!("{}", tree);
}
