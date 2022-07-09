pub struct Solution {}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
            if i > start && candidates[i as usize] == candidates[i as usize - 1] {
                continue;
            }
            cur.push(candidates[i as usize]);
            Solution::backtrack(res, cur, candidates, target, i + 1);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_40() {
        assert_eq!(
            Solution::combination_sum2(vec![1, 1, 1, 1, 1, 1, 1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
