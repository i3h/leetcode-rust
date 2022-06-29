use rand::Rng;

pub struct Solution {}

fn rand7() -> i32 {
    let mut r = 0;
    while r < 1 || r > 7 {
        let mut rng = rand::thread_rng();
        r = rng.gen_range(1, 8);
    }
    r
}

impl Solution {
    pub fn rand10() -> i32 {
        let mut r = 40;
        while r >= 40 {
            r = (rand7() - 1) * 7 + rand7() - 1;
        }
        r % 10 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_470() {
        println!("{}", Solution::rand10());
        println!("{}", Solution::rand10());
        println!("{}", Solution::rand10());
        println!("{}", Solution::rand10());
        println!("{}", Solution::rand10());
        println!("{}", Solution::rand10());
    }
}
