use std::cell::RefCell;
use std::rc::Rc;

use crate::util::tree;
use crate::util::tree::{to_tree, TreeNode};

pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut height = 0;
        Solution::is_balanced_helper(root, &mut height)
    }

    fn is_balanced_helper(root: Option<Rc<RefCell<TreeNode>>>, height: &mut i32) -> bool {
        match root {
            None => {
                *height = 0;
                true
            }
            Some(root) => {
                let root = root.borrow();
                let mut left_height = 0;
                let mut right_height = 0;

                match Solution::is_balanced_helper(root.left.clone(), &mut left_height)
                    && Solution::is_balanced_helper(root.right.clone(), &mut right_height)
                {
                    false => return false,
                    true => {
                        if left_height - right_height > 1 || left_height - right_height < -1 {
                            return false;
                        }
                        *height = std::cmp::max(left_height, right_height) + 1;
                        return true;
                    }
                }
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
