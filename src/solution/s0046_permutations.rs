pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut final_result = vec![];
        match nums.len() {
            1 => vec![nums],
            _ => {
                for i in 0..nums.len() {
                    let mut nums_clone = nums.clone();
                    let num = nums_clone.remove(i);
                    let mut result = Solution::permute(nums_clone);
                    for j in 0..result.len() {
                        result[j].push(num);
                        final_result.push(result[j].clone());
                    }
                }
                // println!("final_result: {:?}", final_result);
                final_result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_46() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![3, 2, 1],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![1, 2, 3],
            ]
        )
    }
}
