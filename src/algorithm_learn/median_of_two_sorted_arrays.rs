pub struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums_vec: Vec<i32>= nums1.iter().chain(nums2.iter()).copied().collect();
        nums_vec.sort();
        if (nums_vec.len() == 0) {
            0.0
        } else if (nums_vec.len() == 1) {
            nums_vec[0] as f64
        } else {
            let i = (nums_vec.len() as f64 / 2 as f64).round() as usize;
            if (nums_vec.len() % 2 == 0) {
                (nums_vec[i-1] as f64 + nums_vec[i] as f64) / 2f64
            } else {
                nums_vec[i] as f64
            }
        }
    }

    pub fn find_median_sorted_arrays_v1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            // 采用较短的数组做二分查找
            return Solution::find_median_sorted_arrays_v1(nums2, nums1);
        }

        let m = nums1.len();
        let n = nums2.len();
        let mut low = 0;
        let mut high = m;
        let left = (m + n).div_ceil(2);

        loop {
            if low > high {
                break;
            }

            let mid1 = (low + high) / 2;
            let mid2 = left - mid1;

            let r1 = if mid1 < m { nums1[mid1] } else { i32::MAX };
            let r2 = if mid2 < n { nums2[mid2] } else { i32::MAX };
            let l1 = if mid1 > 0 { nums1[mid1-1] } else { i32::MIN };
            let l2 = if mid2 > 0 { nums2[mid2-1] } else { i32::MIN };

            if l1 <= r2 && l2 <= r1 {
                return  if (m + n).is_multiple_of(2) {
                    ((l1.max(l2) + r1.min(r2)) as f64) / 2.0
                } else {
                    l1.max(l2) as f64
                }
            } else if l1 > r2 {
                high = mid1 - 1;
            } else {
                low = mid1 + 1;
            }
        }

        0.0
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::find_median_sorted_arrays_v1(vec![0,3,2,5,1], vec![-2,7,3,8]), 3.0);
    }
}