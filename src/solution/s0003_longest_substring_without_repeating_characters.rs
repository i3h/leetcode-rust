use std::cmp;
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();
        let mut p = 0;
        let mut max = 0;

        for (i, c) in s.as_bytes().iter().enumerate() {
            // println!("{} {}", i, *c as char);
            while set.contains(c) {
                // println!(
                //     "p: {}, i: {}, contain {}, remove {}",
                //     p,
                //     i,
                //     *c as char,
                //     s.as_bytes()[p] as char
                // );
                set.remove(&s.as_bytes()[p]);
                p += 1;
                //println!("new p: {}", p);
            }
            set.insert(c);
            max = cmp::max(max, i - p + 1);
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("tmmzuxt".to_string()),
            5
        );

        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);

        assert_eq!(Solution::length_of_longest_substring("abc".to_string()), 3);

        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );

        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);

        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
