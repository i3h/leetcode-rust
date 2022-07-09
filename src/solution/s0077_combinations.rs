pub struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtrack(&mut res, &mut vec![], n, k, 1);
        res
    }

    fn backtrack(res: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, n: i32, k: i32, start: i32) {
        // println!("cur: {:?}", cur);
        if cur.len() == k as usize {
            res.push(cur.clone());
            return;
        }

        for i in start..=n {
            cur.push(i);
            Self::backtrack(res, cur, n, k, i + 1);
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_77() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ]
        );
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combine(0, 1), empty);
        assert_eq!(Solution::combine(2, 1), vec![vec![1], vec![2]]);
    }
}
