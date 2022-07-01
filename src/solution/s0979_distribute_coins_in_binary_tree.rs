use std::cell::RefCell;
use std::rc::Rc;

use crate::util::tree;
use crate::util::tree::{to_tree, TreeNode};

pub struct Solution {}

impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut moves = 0;
        Solution::distribute_coins_helper(root, &mut moves);
        moves
    }

    fn distribute_coins_helper(root: Option<Rc<RefCell<TreeNode>>>, moves: &mut i32) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let root = root.borrow();
                let left_moves = Solution::distribute_coins_helper(root.left.clone(), moves);
                let right_moves = Solution::distribute_coins_helper(root.right.clone(), moves);
                *moves += left_moves.abs() + right_moves.abs();
                root.val + left_moves + right_moves - 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_979() {
        assert_eq!(Solution::distribute_coins(crate::tree![3, 0, 0]), 2);
        assert_eq!(Solution::distribute_coins(crate::tree![0, 3, 0]), 3);
    }
}
