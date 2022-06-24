pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        return if digits.len() == 0 {
            vec![]
        } else {
            Solution::letter_combinations_recursive(digits)
        };
    }

    pub fn letter_combinations_recursive(digits: String) -> Vec<String> {
        return if digits.len() == 0 {
            vec![String::from("")]
        } else {
            let mut result = Vec::new();
            let mut digits = digits.chars();
            let digit = digits.next().unwrap();
            let letters = match digit {
                '2' => vec!["a", "b", "c"],
                '3' => vec!["d", "e", "f"],
                '4' => vec!["g", "h", "i"],
                '5' => vec!["j", "k", "l"],
                '6' => vec!["m", "n", "o"],
                '7' => vec!["p", "q", "r", "s"],
                '8' => vec!["t", "u", "v"],
                '9' => vec!["w", "x", "y", "z"],
                _ => vec![],
            };
            for letter in letters.iter() {
                let new_result = Solution::letter_combinations_recursive(digits.clone().collect());
                for r in new_result.iter() {
                    result.push(format!("{}{}", letter, r));
                }
            }
            result
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }
}
