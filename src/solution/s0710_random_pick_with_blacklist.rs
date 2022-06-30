use rand::Rng;
use std::collections::HashMap;

struct Solution {
    n: i32,
    blacklist: Vec<i32>,
    map: HashMap<i32, i32>,
}

impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let map = blacklist
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut map, (i, &n)| {
                map.insert(n, i as i32);
                map
            });

        Self { n, blacklist, map }
    }

    fn pick(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut i = rng.gen_range(0, self.n);

        while self.map.contains_key(&i) {
            i = rng.gen_range(0, self.n);
        }

        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_710() {
        let mut s = Solution::new(7, vec![2, 3, 5]);
        for _ in 0..10 {
            println!("{}", s.pick());
        }
    }
}
