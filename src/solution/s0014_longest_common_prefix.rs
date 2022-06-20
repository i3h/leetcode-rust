pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut k = 0;
        let mut b = b' ';

        if strs.len() == 0 {
            return "".to_string();
        }

        'outer: loop {
            for i in 0..strs.len() {
                //println!("{}", strs[i]);
                if strs[i] == "" {
                    break 'outer;
                }
                if k > strs[i].len() - 1 {
                    break 'outer;
                }
                if i == 0 {
                    b = strs[i].as_bytes()[k];
                } else {
                    if b != strs[i].as_bytes()[k] {
                        break 'outer;
                    }
                }
            }
            k += 1;
        }

        let com = String::from(&strs[0][..k]);
        //println!("k: {}", k);
        //println!("com: {}", com);
        com
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "".to_string(),
                "racecar".to_string(),
                "racar".to_string(),
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ]),
            "fl"
        );
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
    }
}
