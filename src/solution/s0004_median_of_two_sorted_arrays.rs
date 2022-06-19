pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l1 = nums1.len();
        let l2 = nums2.len();
        let l = l1 + l2;
        let i = l / 2 - (1 - l % 2);
        let j = l / 2;

        if l1 == 0 {
            return (nums2[i] + nums2[j]) as f64 / 2.0;
        }
        if l2 == 0 {
            return (nums1[i] + nums1[j]) as f64 / 2.0;
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut m = 0;
        let mut m0 = 0;

        loop {
            if p1 >= l1 {
                m0 = m;
                m = nums2[p2];
                p2 += 1;
            } else if p2 >= l2 {
                m0 = m;
                m = nums1[p1];
                p1 += 1;
            } else {
                if nums1[p1] < nums2[p2] {
                    m0 = m;
                    m = nums1[p1];
                    p1 += 1;
                } else {
                    m0 = m;
                    m = nums2[p2];
                    p2 += 1;
                }
            }
            if p1 + p2 > j {
                break;
            }
        }

        return if i == j {
            m as f64
        } else {
            (m + m0) as f64 / 2.0
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
