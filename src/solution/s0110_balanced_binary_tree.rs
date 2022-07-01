use std::cell::RefCell;
use std::rc::Rc;

use crate::util::tree;
use crate::util::tree::{to_tree, TreeNode};

pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let root = root.borrow();
                let left_height = Solution::height(root.left.clone());
                let right_height = Solution::height(root.right.clone());
                if left_height - right_height > 1 || left_height - right_height < -1 {
                    return false;
                }
                Solution::is_balanced(root.left.clone())
                    && Solution::is_balanced(root.right.clone())
            }
        }
    }

    fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let root = root.borrow();
                1 + std::cmp::max(
                    Solution::height(root.left.clone()),
                    Solution::height(root.right.clone()),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_110() {
        assert_eq!(Solution::is_balanced(crate::tree![]), true);
        assert_eq!(
            Solution::is_balanced(crate::tree![3, 9, 20, null, null, 15, 7]),
            true
        );
        assert_eq!(
            Solution::is_balanced(crate::tree![1, 2, 2, 3, 3, null, null, 4, 4]),
            false
        );
    }
}
