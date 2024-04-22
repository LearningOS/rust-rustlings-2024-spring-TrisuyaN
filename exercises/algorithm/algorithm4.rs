/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DON
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
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
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if let Some(ref mut rootn) = self.root {
            insert_curn(rootn.as_mut(), value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        
        if let Some(ref rn) = self.root {
            search_curn(rn, value)
        } else{
            false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    // fn insert(&mut self, value: T) {
    //     if value >= self.value {
    //         if let None = self.right {
    //             self.right = Some(Box::new(TreeNode::new(value)));
    //         } else {
    //             self.insert(self.right, value);
    //         }
    //     } else {

    //     }
    // }
}

fn insert_curn<T: std::cmp::Ord>(node: &mut TreeNode<T>, value: T) {
    if value > node.value {
        if let Some(ref mut rn) = node.right {
            insert_curn(rn.as_mut(), value);
        } else {
            node.right = Some(Box::new(TreeNode::new(value)));
        }
    } else if value < node.value {
        if let Some(ref mut ln) = node.left {
            insert_curn(ln.as_mut(), value);
        } else {
            node.left = Some(Box::new(TreeNode::new(value)));
        }
    }
}

fn search_curn<T: std::cmp::Ord>(node: &TreeNode<T>, value: T) -> bool {
    if node.value == value {
        return true;
    } else if node.value > value {
        if let Some(ln) = node.left.as_ref() {
            return search_curn(ln, value);
        };
    } else if node.value < value {
        if let Some(lr) = node.right.as_ref() {
            return search_curn(lr, value);
        };
    }
    return false;
}

// fn search_curn<T: std::cmp::Ord>(node: &TreeNode<T>, value: T) -> bool {
//     match node.value.cmp(&value) {
//         Ordering::Equal => true,
//         Ordering::Less => node.right.as_ref().map_or(false, |n| search_curn(n, value)),
//         Ordering::Greater => node.left.as_ref().map_or(false, |n| search_curn(n, value)),
//     }
// }


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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


