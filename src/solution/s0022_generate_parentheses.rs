pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        return if n == 0 {
            vec![String::from("")]
        } else {
            let mut result = Vec::new();
            for i in 0..n {
                let left = Solution::generate_parenthesis(i);
                let right = Solution::generate_parenthesis(n - i - 1);
                for l in left.iter() {
                    for r in right.iter() {
                        result.push(format!("({}){}", l, r));
                    }
                }
            }
            result.sort();
            result
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
