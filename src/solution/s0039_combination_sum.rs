pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut candidates = candidates;
        candidates.sort();
        Solution::backtrack(&mut res, &mut vec![], &mut candidates, target, 0);
        res
    }

    fn backtrack(
        res: &mut Vec<Vec<i32>>,
        cur: &mut Vec<i32>,
        candidates: &mut Vec<i32>,
        target: i32,
        start: i32,
    ) {
        // println!("cur: {:?}", cur);
        let sum: i32 = cur.iter().sum();
        if sum == target {
            res.push(cur.clone());
            return;
        }
        if sum > target {
            return;
        }

        for i in start..candidates.len() as i32 {
            cur.push(candidates[i as usize]);
            Solution::backtrack(res, cur, candidates, target, i);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39() {
        assert_eq!(
            Solution::combination_sum(vec![1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }
}
