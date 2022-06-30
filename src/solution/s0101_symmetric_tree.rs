use std::cell::RefCell;
use std::rc::Rc;

use crate::util::tree;
use crate::util::tree::{to_tree, TreeNode};

pub struct Solution {}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let root = root.borrow();
                Solution::is_symmetric_helper(root.left.clone(), root.right.clone())
            }
        }
    }

    fn is_symmetric_helper(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();
                left.val == right.val
                    && Solution::is_symmetric_helper(left.left.clone(), right.right.clone())
                    && Solution::is_symmetric_helper(left.right.clone(), right.left.clone())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_101() {
        assert_eq!(
            Solution::is_symmetric(crate::tree![1, 2, 2, 3, 4, 4, 3]),
            true
        );
        assert_eq!(
            Solution::is_symmetric(crate::tree![1, 2, 2, null, 3, null, 3]),
            false
        );
        assert_eq!(Solution::is_symmetric(crate::tree![]), true);
    }
}
