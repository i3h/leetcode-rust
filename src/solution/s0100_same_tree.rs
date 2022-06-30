use std::cell::RefCell;
use std::rc::Rc;

use crate::util::tree;
use crate::util::tree::{to_tree, TreeNode};

pub struct Solution {}

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        p == q
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_100() {
        assert_eq!(
            Solution::is_same_tree(
                crate::tree![1, 2, 3, 4, null, 5],
                crate::tree![1, 2, 3, 4, null, 5]
            ),
            true
        )
    }
}
