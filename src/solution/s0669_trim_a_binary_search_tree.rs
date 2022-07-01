use std::cell::RefCell;
use std::rc::Rc;

use crate::util::tree;
use crate::util::tree::{to_tree, TreeNode};

pub struct Solution {}

impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(root) => {
                let root = root.borrow();
                if root.val < low {
                    Solution::trim_bst(root.right.clone(), low, high)
                } else if root.val > high {
                    Solution::trim_bst(root.left.clone(), low, high)
                } else {
                    let left = Solution::trim_bst(root.left.clone(), low, high);
                    let right = Solution::trim_bst(root.right.clone(), low, high);
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: root.val,
                        left: left,
                        right: right,
                    })))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_669() {
        assert_eq!(
            Solution::trim_bst(crate::tree![1, 0, 2], 1, 2),
            crate::tree![1, null, 2]
        );
        assert_eq!(
            Solution::trim_bst(crate::tree![3, 0, 4, null, 2, null, null, 1], 1, 3),
            crate::tree![3, 2, null, 1]
        );
    }
}
