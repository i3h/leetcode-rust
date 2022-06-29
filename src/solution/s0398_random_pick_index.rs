use rand::Rng;

struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn pick(&self, target: i32) -> i32 {
        let mut cache: Vec<i32> = Vec::new();

        for (i, n) in self.nums.iter().enumerate() {
            if *n == target {
                cache.push(i as i32);
            }
        }
        // println!("{:?}", cache);

        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, cache.len());

        cache[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_398() {
        let nums = vec![1, 2, 3, 3, 3];
        let s = Solution::new(nums);
        println!("{}", s.pick(3));
    }
}
