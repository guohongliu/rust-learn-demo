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
            // 1. num1 = [1,2] num2 = [3,4] m = 2 n = 2 left = 2
            //    round1: mid1 = (0 + 2) / 2 = 1 mid2 = 2 - 1 = 1 r1=1 r2=4 l1=1 l2=3
            //    round2: mid1 = 2 mid2 = 0 r1=i32::MAX r2=3 l1=2 l2=i32::MIN => 2.5
            // 2. nums1 = []，nums2 = [1,2,3]
            // 3. nums1 = [5]，nums2 = [1,2,3,4,6,7,8]
            // 4. nums1 = [i32::MIN, 2]，nums2 = [3, i32::MAX]
            // 5. nums1 = [1]，nums2 = []
            let r1 = if mid1 < m { nums1[mid1] } else { i32::MAX }; // nums1 右半部分的第一个元素（分割点右侧紧邻的值）
            let r2 = if mid2 < n { nums2[mid2] } else { i32::MAX }; // nums2 右半部分的第一个元素（分割点右侧紧邻的值）
            let l1 = if mid1 > 0 { nums1[mid1-1] } else { i32::MIN }; // nums1 左半部分的最后一个元素（分割点左侧紧邻的值）
            let l2 = if mid2 > 0 { nums2[mid2-1] } else { i32::MIN }; // nums2 左半部分的最后一个元素（分割点左侧紧邻的值）
            // num1 左半部分的最后一个元素（分割点左侧紧邻的值）< num2 右半部分的第一个元素（分割点右侧紧邻的值）
            // nums2 左半部分的最后一个元素（分割点左侧紧邻的值） < nums1 右半部分的第一个元素（分割点右侧紧邻的值）
            if l1 <= r2 && l2 <= r1 {
                return  if (m + n).is_multiple_of(2) {
                    // (2 + 2) / 2 = 2
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
        assert_eq!(Solution::find_median_sorted_arrays_v1(vec![1,2], vec![3,4]), 2.5);
    }
}