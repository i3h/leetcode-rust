pub struct Solution {}

impl Solution {
    fn add_sharp(s: &String) -> String {
        let mut s0 = String::from("#");
        for c in s.chars() {
            s0.push(c);
            s0.push('#');
        }
        s0
    }

    fn brute_force(s: String) -> String {
        let mut s0 = Self::add_sharp(&s);
        println!("{}", s0);
        s
    }

    pub fn longest_palindrome(s: String) -> String {
        Self::brute_force(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        Solution::longest_palindrome("babab".to_owned());
        // assert_eq!(Solution::longest_palindrome("aaaaa".to_owned()), "aaaaa");
        // assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
        // assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
        // assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
        // assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
        // assert_eq!(Solution::longest_palindrome("".to_owned()), "");
    }
}
