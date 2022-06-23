pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if let Some(last) = stack.pop() {
                        if last != '(' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                ']' => {
                    // if stack.pop().unwrap() != '[' {
                    //     return false;
                    // }
                    if let Some(last) = stack.pop() {
                        if last != '[' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                '}' => {
                    if let Some(last) = stack.pop() {
                        if last != '{' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        return if stack.is_empty() { true } else { false };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("[".to_string()), false);
        assert_eq!(Solution::is_valid("]".to_string()), false);
    }
}
