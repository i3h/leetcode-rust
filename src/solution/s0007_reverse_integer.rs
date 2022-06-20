pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let max = (2 as i64).pow(31) - 1;
        let mut x: i64 = x as i64;
        let mut y: i64 = 0;
        let is_negative = x < 0;
        if is_negative {
            x = -x;
        }

        while x > 0 {
            y = y * 10 + x % 10;
            x = x / 10;
            //println!("x = {}, y = {}", x, y);
            if y >= max {
                return 0;
            }
        }
        if is_negative {
            y = -y
        }

        y as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-123000), -321);
        let base: i64 = 2;
        assert_eq!(Solution::reverse((base.pow(31) - 1) as i32), 0);
        assert_eq!(Solution::reverse((-base.pow(31)) as i32), 0);
    }
}
