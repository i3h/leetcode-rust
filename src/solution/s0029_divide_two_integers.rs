pub struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut dividend: i64 = dividend as i64;
        let mut divisor: i64 = divisor as i64;
        let min = i32::MIN as i64;
        let max = i32::MAX as i64;

        let is_negative_dividend = if dividend < 0 {
            dividend = -dividend;
            true
        } else {
            false
        };
        let is_negative_divisor = if divisor < 0 {
            divisor = -divisor;
            true
        } else {
            false
        };
        let is_negative_result = if is_negative_dividend == is_negative_divisor {
            false
        } else {
            true
        };

        let mut quotient: i64 = 0;

        while dividend >= divisor {
            let mut temp = divisor;
            let mut multiple = 1;
            while dividend >= temp << 1 {
                temp <<= 1;
                multiple <<= 1;
            }
            dividend -= temp;
            quotient += multiple;
        }

        match is_negative_result {
            true => {
                if quotient > max {
                    return min as i32;
                } else {
                    return -quotient as i32;
                }
            }
            false => {
                if quotient > max {
                    return max as i32;
                } else {
                    return quotient as i32;
                }
            }
        }

        // quotient as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_29() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
    }
}
