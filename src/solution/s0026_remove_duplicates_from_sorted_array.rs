pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => 1,
            _ => {
                let mut i = 0;
                let mut j = 1;
                while j < nums.len() {
                    if nums[i] != nums[j] {
                        i += 1;
                        nums.swap(i, j);
                    }
                    j += 1;
                }
                i as i32 + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26() {
        assert_eq!(Solution::remove_duplicates(&mut vec![]), 0);
        let mut vec1 = vec![1, 1, 1, 1, 3];
        assert_eq!(Solution::remove_duplicates(&mut vec1), 2);
        assert_eq!(vec1, vec![1, 3, 1, 1, 1]);
        let mut vec2 = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut vec2), 2);
        assert_eq!(vec2, vec![1, 2, 1]);
    }
}
