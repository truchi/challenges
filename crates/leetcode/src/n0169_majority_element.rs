//! <https://leetcode.com/problems/majority-element/>

pub fn majority_element(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    nums[nums.len() / 2]
}

#[test]
fn test() {
    assert!(majority_element(vec![3, 2, 3]) == 3);
    assert!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]) == 2);
}
