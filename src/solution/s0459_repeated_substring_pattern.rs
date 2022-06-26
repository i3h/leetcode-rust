pub struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        let len = s.len();
        // println!("len: {}", len);

        for i in 1..len {
            if len % i == 0 {
                'outer: for j in 0..i {
                    for k in 0..len / i {
                        if s[j + k * i] != s[j] {
                            break 'outer;
                        }
                    }
                    if j == i - 1 {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_459() {
        assert_eq!(
            Solution::repeated_substring_pattern("ababba".to_string()),
            false
        );
        assert_eq!(
            Solution::repeated_substring_pattern("abab".to_string()),
            true
        );
        assert_eq!(
            Solution::repeated_substring_pattern("aba".to_string()),
            false
        );
        assert_eq!(
            Solution::repeated_substring_pattern("abcabcabcabc".to_string()),
            true
        );
    }
}
