use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut frequency = HashMap::new();
        for item in nums {
            *frequency.entry(item).or_insert(0) += 1;
        }
        let max_freq = frequency.values().max().copied().unwrap_or(0);
        frequency.values()
            .filter(|&&freq| freq == max_freq)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        assert_eq!(Solution::max_frequency_elements(vec![1,2,3,1,2,1]), 3);
    }
}