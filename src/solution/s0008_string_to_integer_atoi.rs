pub struct Solution {}

impl Solution {
    pub fn overflow(is_negative: bool) -> i32 {
        let upper_bound: i64 = (2 as i64).pow(31) - 1;
        let lower_bound: i64 = -(2 as i64).pow(31);

        return if is_negative {
            lower_bound as i32
        } else {
            upper_bound as i32
        };
    }

    pub fn my_atoi(s: String) -> i32 {
        let mut result: i64 = 0;
        let mut num = String::from("");
        let mut is_negative = false;
        let mut sign_counter = 0;
        let upper_bound: i64 = (2 as i64).pow(31) - 1;
        let lower_bound: i64 = -(2 as i64).pow(31);

        for c in s.chars() {
            //println!("c: {}", c);
            match c {
                ' ' => {
                    if sign_counter > 0 {
                        break;
                    }
                    if num.len() > 0 {
                        break;
                    }
                }
                '-' => {
                    if num.len() > 0 {
                        break;
                    }
                    sign_counter += 1;
                    if sign_counter > 1 {
                        break;
                    }
                    is_negative = true;
                }
                '+' => {
                    if num.len() > 0 {
                        break;
                    }
                    sign_counter += 1;
                    if sign_counter > 1 {
                        break;
                    }
                }
                '0'..='9' => {
                    num.push(c);
                }
                _ => {
                    if num.len() == 0 {
                        return 0;
                    } else {
                        break;
                    }
                }
            }
        }

        num = String::from(num.trim_start_matches('0'));
        let len = num.len();
        if len > 10 {
            return Solution::overflow(is_negative);
        }
        for (i, c) in num.chars().enumerate() {
            //result += ((c as u8 - b'0') as i64) * (10 as i64).pow((len - i - 1) as u32);
            //let add = ((c as u8 - b'0') as i64).checked_mul((10 as i64).pow((len - i - 1) as u32));

            if let Some(add) =
                ((c as u8 - b'0') as i64).checked_mul((10 as i64).pow((len - i - 1) as u32))
            {
                if let Some(sum) = result.checked_add(add) {
                    result = sum;
                } else {
                    return Solution::overflow(is_negative);
                }
            } else {
                return Solution::overflow(is_negative);
            }

            //println!("c = {}, i = {}, result = {}", c as u8 - b'0', i, result);
        }

        if is_negative {
            result = -result;
        }
        if result < lower_bound {
            return lower_bound as i32;
        }
        if result > upper_bound {
            return upper_bound as i32;
        }

        //println!("num: {}", num);
        //println!("result: {}", result);
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("aa".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
        assert_eq!(Solution::my_atoi("3.14159".to_string()), 3);
        assert_eq!(Solution::my_atoi("+1".to_string()), 1);
        assert_eq!(Solution::my_atoi("+-12".to_string()), 0);
        assert_eq!(Solution::my_atoi("00000-42a1234".to_string()), 0);
        assert_eq!(Solution::my_atoi("   +0 123".to_string()), 0);
        assert_eq!(Solution::my_atoi("  +  413".to_string()), 0);
        assert_eq!(
            Solution::my_atoi("  0000000000012345678".to_string()),
            12345678
        );
        assert_eq!(
            Solution::my_atoi("9223372036854775808".to_string()),
            2147483647
        );
        assert_eq!(
            Solution::my_atoi("18446744073709551617".to_string()),
            2147483647
        );
    }
}
