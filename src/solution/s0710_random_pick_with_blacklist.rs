use std::collections::HashMap;

use rand::Rng;

struct Solution {
    n: i32,
    blacklist: Vec<i32>,
    map: HashMap<i32, i32>,
}

impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let mut blacklist = blacklist;
        blacklist.sort();
        let m = blacklist.len() as i32;
        let mut map = HashMap::new();

        // println!("n: {}", n);
        // println!("m: {}", m);
        // println!("blacklist: {:?}", blacklist);

        let mut map_blacklist = HashMap::new();
        for i in 0..m {
            map_blacklist.insert(blacklist[i as usize], i);
        }
        // println!("map_blacklist: {:?}", map_blacklist);

        let mut i = 0;
        let mut j = n - 1;
        if m != 0 {
            while blacklist[i as usize] < n - m {
                while map_blacklist.contains_key(&j) {
                    j -= 1;
                }
                map.insert(blacklist[i as usize], j);
                i += 1;
                if i > m - 1 {
                    break;
                }
                j -= 1;
            }
        }

        // println!("map: {:?}", map);

        Self { n, blacklist, map }
    }

    fn pick(&self) -> i32 {
        let m = self.blacklist.len() as i32;
        let mut rng = rand::thread_rng();
        let mut i = rng.gen_range(0, self.n - m);

        if self.map.contains_key(&i) {
            i = *self.map.get(&i).unwrap();
        }

        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_710() {
        // let mut s = Solution::new(7, vec![2, 3, 5]);
        // for _ in 0..10 {
        //     println!("{}", s.pick());
        // }
        // let mut s = Solution::new(1, vec![]);
        // for _ in 0..10 {
        //     println!("{}", s.pick());
        // }
        // let mut s = Solution::new(3, vec![1]);
        // for _ in 0..10 {
        //     println!("{}", s.pick());
        // }
        let mut s = Solution::new(3, vec![2]);
        for _ in 0..10 {
            println!("{}", s.pick());
        }
    }
}
