pub struct Solution {}

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1267() {
        assert_eq!(0, Solution::count_servers(vec![vec![1, 0], vec![0, 1]]));
        assert_eq!(3, Solution::count_servers(vec![vec![1, 0], vec![1, 1]]));
        assert_eq!(
            4,
            Solution::count_servers(vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1]
            ])
        );
    }
}
