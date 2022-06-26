use crate::util::linked_list::print_list;

pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;

        if nums.len() == 0 {
            return count;
        }

        let mut head = 0;
        let mut tail = nums.len() - 1;

        while head <= tail {
            //println!("head: {}, tail: {}", head, tail);
            if nums[head] == val {
                nums.swap(head, tail);
                if tail == 0 {
                    break;
                }
                tail -= 1;
            } else {
                count += 1;
                head += 1;
            }
        }

        // println!("{:?}", nums);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        assert_eq!(Solution::remove_element(&mut vec![], 2), 0);
        assert_eq!(
            Solution::remove_element(&mut vec![1, 2, 2, 2, 2, 2, 2], 2),
            1
        );
        assert_eq!(
            Solution::remove_element(&mut vec![2, 2, 2, 2, 2, 2, 2], 2),
            0
        );
        assert_eq!(Solution::remove_element(&mut vec![1], 1), 0);
    }
}
