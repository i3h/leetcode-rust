pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let s = x.to_string();
        let len = s.len();
        let counter = (len - len % 2) / 2;
        //println!("len = {}, counter = {}", len, counter);

        for i in 0..counter {
            // println!(
            //     "i: {}, a: {}, b: {}",
            //     i,
            //     s.as_bytes()[i] as char,
            //     s.as_bytes()[len - i - 1] as char
            // );
            if s.as_bytes()[i] != s.as_bytes()[len - i - 1] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        //Solution::is_palindrome(11222211);
        assert_eq!(Solution::is_palindrome(-32), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(9), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(2222), true);
        assert_eq!(Solution::is_palindrome(11222211), true);
    }
}
