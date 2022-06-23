use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        return Solution::k_sum(nums, target, 4);
    }

    pub fn k_sum(nums: Vec<i32>, target: i32, k: i32) -> Vec<Vec<i32>> {
        return if k == 2 {
            Solution::two_sum(nums, target)
        } else {
            let mut final_result = Vec::new();
            let mut map = HashMap::new();
            for i in 0..nums.len() {
                if i < nums.len() - 1 && nums[i] == nums[i + 1] {
                    continue;
                }
                let mut nums = nums.clone();
                let n = nums[i];
                let target = target - n;
                nums.remove(i);
                let mut result = Solution::k_sum(nums, target, k - 1);
                for r in result.iter_mut() {
                    r.push(n);
                    r.sort();
                    map.insert(r.clone(), ());
                    //final_result.push(r.clone());
                }
                //println!("result: {:?}", result);
            }
            for (k, v) in map.iter() {
                final_result.push(k.clone());
            }
            final_result.sort();
            // println!(
            //     "nums: {:?}, target: {}, result: {:?}",
            //     nums, target, final_result
            // );
            final_result
        };
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut map = HashMap::with_capacity(nums.len());
        let mut rmap = HashMap::new();

        if target.abs() > (10 as i32).pow(9) {
            return vec![];
        }

        for (i, num) in nums.iter().enumerate() {
            //println!("i: {}, num: {}", i, num);
            if *num > (10 as i32).pow(9) {
                return vec![];
            }
            map.insert(num, i);
        }

        for i in 0..nums.len() {
            if let Some(j) = map.get(&(target - nums[i])) {
                if j != &i {
                    let mut vec = vec![nums[i], nums[*j]];
                    vec.sort();
                    rmap.insert(vec, ());
                    //result.push(vec);
                }
            }
        }

        for (k, v) in rmap.iter() {
            result.push(k.clone());
        }

        //println!("nums: {:?}, target: {}, result: {:?}", nums, target, result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_18() {
        /*
                [1000000000,1000000000,1000000000,1000000000]
        -294967296
                 */
        Solution::four_sum(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 20,
                20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 40, 40, 40,
                40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 50, 50, 50, 50,
                50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 60, 60, 60, 60, 60,
                60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 70, 70, 70, 70, 70, 70,
                70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 80, 80, 80, 80, 80, 80, 80,
                80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 90, 90, 90, 90, 90, 90, 90, 90,
                90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90,
            ],
            200,
        );
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );
    }
}
