pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut sum = 0;
        let mut diff = i32::MAX;
        let mut nums = nums;
        nums.sort();

        for k in 0..nums.len() {
            let mut i = k + 1;
            let mut j = nums.len() - 1;
            while i < j {
                sum = nums[k] + nums[i] + nums[j];
                //println!("i: {}, j: {}, k: {}, sum: {}", i, j, k, sum);
                if sum == target {
                    return sum;
                }
                if (sum - target).abs() < diff.abs() {
                    diff = sum - target;
                }
                if sum < target {
                    while i < j && nums[i] == nums[i + 1] {
                        i += 1;
                    }
                    i += 1;
                } else {
                    while i < j && nums[j] == nums[j - 1] {
                        j -= 1;
                    }
                    j -= 1;
                }
            }
        }

        //println!("final: {}", diff + target);
        diff + target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, -1, -1, 3], -1), -1);
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3], 1), 6);
        assert_eq!(
            Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
            82
        );
    }
}
