use std::collections::HashMap;

pub fn two_sum_v1(nums: &[i32], target: i32) -> Vec<usize> {
    let hm = nums.iter().zip(0..).collect::<HashMap<_, _>>();
    for (i, &n) in nums.iter().enumerate() {
        let sub = target - n;
        if let Some(&sub_index) = hm.get(&sub) {
            if i == sub_index {
                continue;
            }
            return vec![i, sub_index];
        }
    }
    vec![]
}

pub fn two_sum_v2(nums: &[i32], target: i32) -> Vec<usize> {
    let mut hm = HashMap::with_capacity(nums.len());
    for (i, &n) in nums.iter().enumerate() {
        let sub = target - n;
        if let Some(&sub_index) = hm.get(&sub) {
            return vec![i, sub_index];
        }
        hm.insert(n, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum_v1() {
        let nums = vec![2, 11, 7, 15];
        let res = two_sum_v1(&nums, 9);
        assert_eq!(nums[res[0]] + nums[res[1]], 9);
    }
    #[test]
    fn test_two_sum_v2() {
        let nums = vec![2, 11, 7, 15];
        let res = two_sum_v2(&nums, 9);
        assert_eq!(nums[res[0]] + nums[res[1]], 9);
    }
}
