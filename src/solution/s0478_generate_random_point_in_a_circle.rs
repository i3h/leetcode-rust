use rand::Rng;

pub struct Solution {
    radius: f64,
    center: (f64, f64),
}

impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            center: (x_center, y_center),
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        // let r = rng.gen_range(0.0, self.radius).sqrt();
        let r = (rng.gen_range(0.0, 1.0) as f64).sqrt() * self.radius;
        let theta = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let x = r * theta.cos() + self.center.0;
        let y = r * theta.sin() + self.center.1;

        vec![x, y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_478() {
        // let s = Solution::new(1.0, 0.0, 0.0);
        // println!("{:?}", s.rand_point());
        // println!("{:?}", s.rand_point());
        // println!("{:?}", s.rand_point());
        let s = Solution::new(0.01, -73839.1, -3289891.3);
        println!("{:?}", s.rand_point());
        println!("{:?}", s.rand_point());
        println!("{:?}", s.rand_point());
        // assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}
