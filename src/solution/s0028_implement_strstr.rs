pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let mut i = 0;
        let mut j = 0;
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        while i < haystack.len() && j < needle.len() {
            if haystack[i] == needle[j] {
                j += 1;
            } else {
                i -= j;
                j = 0;
            }
            i += 1;
        }
        // println!("i: {}, j: {}", i, j);

        match j == needle.len() {
            true => (i - j) as i32,
            false => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_28() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2)
    }
}
