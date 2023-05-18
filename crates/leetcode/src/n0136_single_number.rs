//! <https://leetcode.com/problems/single-number/>

// "You must implement a solution with a linear runtime complexity
// and use only constant extra space."
pub fn single_number(nums: Vec<i32>) -> i32 {
    // Wait, what?
    nums.into_iter().reduce(|a, b| a ^ b).unwrap()
}

#[test]
fn test() {
    assert!(single_number(vec![2, 2, 1]) == 1);
    assert!(single_number(vec![4, 1, 2, 1, 2]) == 4);
    assert!(single_number(vec![1]) == 1);

    assert!(single_number(vec![1, 2, 3, 4, 1, 2, 3]) == 4);
    assert!(single_number(vec![1, 2, 3, 4, 3, 2, 1]) == 4);
}
