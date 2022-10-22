// https://leetcode.com/problems/find-all-duplicates-in-an-array
use std::collections::HashMap;

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    let mut res = vec![];
    for n in nums {
        let v = map.entry(n).or_insert(0);
        *v += 1;
        if *v > 1 {
            res.push(n);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_to_find_duplicates_with_empty() {
        assert_eq!(find_duplicates(vec![]), vec![]);
    }

    #[test]
    fn works_to_find_duplicates_with_dups() {
        assert_eq!(find_duplicates(vec![4, 3, 2, 2, 5]), vec![2]);
        assert_eq!(find_duplicates(vec![4, 5, 2, 2, 5, 6]), vec![2, 5]);
        assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
        assert_eq!(find_duplicates(vec![1, 1, 2]), vec![1]);
    }
}
