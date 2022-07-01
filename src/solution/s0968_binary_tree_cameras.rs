use std::cell::RefCell;
use std::rc::Rc;

use crate::util::tree;
use crate::util::tree::{to_tree, TreeNode};

pub struct Solution {}

// Camera placed
const CAMERA: i32 = 1;
// No camara placed, but be covered.
const NO_CAMERA: i32 = 0;
// Not covered
const UNKNOWN: i32 = -1;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cameras = 0;
        if Solution::min_camera_cover_helper(root, &mut cameras) == UNKNOWN {
            cameras += 1;
        }
        cameras
    }

    fn min_camera_cover_helper(root: Option<Rc<RefCell<TreeNode>>>, cameras: &mut i32) -> i32 {
        match root {
            None => NO_CAMERA,
            Some(root) => {
                let root = root.borrow();
                let left_camera = Solution::min_camera_cover_helper(root.left.clone(), cameras);
                let right_camera = Solution::min_camera_cover_helper(root.right.clone(), cameras);

                if left_camera == UNKNOWN || right_camera == UNKNOWN {
                    *cameras += 1;
                    CAMERA
                } else if left_camera == CAMERA || right_camera == CAMERA {
                    NO_CAMERA
                } else {
                    UNKNOWN
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_968() {
        assert_eq!(
            Solution::min_camera_cover(crate::tree![0, 0, 0, null, null, null, 0]),
            2
        );
        assert_eq!(Solution::min_camera_cover(crate::tree![0]), 1);
        assert_eq!(
            Solution::min_camera_cover(crate::tree![0, 0, null, 0, 0]),
            1
        );
        assert_eq!(
            Solution::min_camera_cover(crate::tree![0, 0, null, 0, null, 0, null, null, 0]),
            2
        );
    }
}
