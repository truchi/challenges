//! <https://leetcode.com/problems/missing-number/>

pub fn missing_number(nums: Vec<i32>) -> i32 {
    (nums.len() * (nums.len() + 1) / 2) as i32 - nums.into_iter().sum::<i32>()
}

#[test]
fn test() {
    assert!(missing_number(vec![3, 0, 1]) == 2);
    assert!(missing_number(vec![0]) == 1);
    assert!(missing_number(vec![1]) == 0);
    assert!(missing_number(vec![0, 1]) == 2);
    assert!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]) == 8);
}
