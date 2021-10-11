// https://leetcode.com/problems/two-sum
use std::collections::HashMap;
use std::convert::TryFrom;

pub fn two_sum_v1(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let map = nums.iter().zip(0..).collect::<HashMap<_, _>>();
    for (i, &n) in nums.iter().enumerate() {
        let i = i32::try_from(i).unwrap();
        let look = target - n;
        if let Some(&j) = map.get(&look) {
            if i != j {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn two_sum_v2(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut map = HashMap::with_capacity(nums.len());
    for (i, &n) in nums.iter().enumerate() {
        let i = i32::try_from(i).unwrap();
        let look = target - n;
        if let Some(&j) = map.get(&look) {
            return Some((j, i));
        }
        map.insert(n, i);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum_v1() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum_v1(&nums, 9), Some((0, 1)));
        let nums = vec![3, 2, 4];
        assert_eq!(two_sum_v1(&nums, 6), Some((1, 2)));
        let nums = vec![3, 3];
        assert_eq!(two_sum_v1(&nums, 6), Some((0, 1)));
        assert_eq!(two_sum_v1(&nums, 5), None);
    }
    #[test]
    fn test_two_sum_v2() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum_v2(&nums, 9), Some((0, 1)));
        let nums = vec![3, 2, 4];
        assert_eq!(two_sum_v2(&nums, 6), Some((1, 2)));
        let nums = vec![3, 3];
        assert_eq!(two_sum_v2(&nums, 6), Some((0, 1)));
        assert_eq!(two_sum_v2(&nums, 5), None);
    }
}
