use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // for i in 0..nums.len() {
        //     for j in (i + 1)..nums.len() {
        //         if nums[i] + nums[j] == target {
        //             return vec![i as i32, j as i32];
        //         }
        //     }
        // }

        // nums.iter().enumerate().flat_map(
        //     |(i, &num_i)| {
        //         nums.iter().skip(i + 1).enumerate()
        //             .filter(
        //                 move |&(j, num_j)|
        //                     num_i + num_j == target
        //             )
        //             .map(
        //                 move |(j, _)|
        //                     vec![i as i32, (j + i + 1) as i32]
        //             )
        //     }
        // ).next().unwrap_or_default ()

        // let mut map = HashMap::new();
        //
        // for (i, &num) in nums.iter().enumerate() {
        //     let complement = target - num;
        //     if let Some(&j) = map.get(&complement) {
        //         return vec![j as i32, i as i32];
        //     }
        //     // 将当前元素存入 map，供后续元素查询
        //     map.insert(num, i);
        // }
        //
        // vec![]

        // nums.iter()
        //     .enumerate()
        //     // 使用 scan 维护哈希表状态，同时遍历元素
        //     .scan(HashMap::new(), |map, (i, &num)| {
        //         let complement = target - num;
        //         // 检查补数是否存在
        //         if let Some(&j) = map.get(&complement) {
        //             return Some(vec![j as i32, i as i32]);
        //         }
        //         // 存入当前元素，继续迭代
        //         map.insert(num, i);
        //         None
        //     })
        //     // 取第一个匹配的结果（题目保证唯一解）
        //     .next()
        //     .unwrap_or_default()

        // 创建 (值, 原始索引) 对并排序
        let mut indexed_nums: Vec<(i32, usize)> = nums.iter()
            .enumerate()
            .map(|(i, &val)| (val, i))
            .collect();
        indexed_nums.sort_unstable_by_key(|&(val, _)| val);

        let mut left = 0;
        let mut right = indexed_nums.len() - 1;

        // 双指针查找
        while left < right {
            let sum = indexed_nums[left].0 + indexed_nums[right].0;
            match sum.cmp(&target) {
                std::cmp::Ordering::Equal => {
                    // 返回原始索引（注意顺序）
                    let i = indexed_nums[left].1 as i32;
                    let j = indexed_nums[right].1 as i32;
                    return vec![i.min(j), i.max(j)];
                }
                std::cmp::Ordering::Less => left += 1, // 和太小，左指针右移
                std::cmp::Ordering::Greater => right -= 1, // 和太大，右指针左移
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}