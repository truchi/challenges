//! <https://leetcode.com/problems/contains-duplicate/>

// time: O(nlogn), space: O(1)
pub fn sort(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    nums.sort_unstable();
    nums.windows(2).any(|window| window[0] == window[1])
}

// time: O(n), space: O(n)
pub fn set(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    nums.len() != nums.into_iter().collect::<HashSet<_>>().len()
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let sort = sort(nums.clone());
    let set = set(nums);

    debug_assert!(sort == set);
    sort
}

#[test]
fn test() {
    assert!(!contains_duplicate(vec![]));
    assert!(!contains_duplicate(vec![1]));
    assert!(contains_duplicate(vec![1, 2, 3, 1]));
    assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
}
