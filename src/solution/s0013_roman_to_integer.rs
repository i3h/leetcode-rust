use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // let map = HashMap::from([
        //            ('I', 1),
        //     ('V', 5),
        //            ('X', 10),
        //     ('L', 50),
        //            ('C', 100),
        //     ('D', 500),
        //     ('M', 1000),
        // ]);
        let vec: Vec<char> = s.chars().collect();
        let mut val = 0;

        let len = vec.len();
        for mut i in 0..len {
            //println!("c: {}", vec[i]);
            match vec[i] {
                'V' => val += 5,
                'L' => val += 50,
                'D' => val += 500,
                'M' => val += 1000,
                'I' => {
                    if i < len - 1 {
                        match vec[i + 1] {
                            'V' => val -= 1,
                            'X' => val -= 1,
                            _ => val += 1,
                        }
                    } else {
                        val += 1;
                    }
                }
                'X' => {
                    if i < len - 1 {
                        match vec[i + 1] {
                            'L' => val -= 10,
                            'C' => val -= 10,
                            _ => val += 10,
                        }
                    } else {
                        val += 10;
                    }
                }
                'C' => {
                    if i < len - 1 {
                        match vec[i + 1] {
                            'D' => val -= 100,
                            'M' => val -= 100,
                            _ => val += 100,
                        }
                    } else {
                        val += 100;
                    }
                }
                _ => {}
            }
        }

        //println!("val: {}", val);
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        //Solution::roman_to_int("III".to_string());
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
    }
}
