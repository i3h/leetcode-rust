pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // println!("=====================");
        // println!("{:?}", nums);
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != 0 {
                nums.swap(i, j);
                i += 1;
            }
            j += 1;
        }
        // println!("{:?}", nums);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_283() {
        let mut vec = vec![1, 0, 3, 12];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec, vec![1, 3, 12, 0]);
        let mut vec = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec, vec![1, 3, 12, 0, 0]);
    }
}
