//! <https://leetcode.com/problems/two-sum/>

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::</* num */ i32, /* index */ usize>::with_capacity(1024);

    for (index, &num) in nums.iter().enumerate() {
        if let Some(&index2) = map.get(&(target - num)) {
            return vec![index as i32, index2 as i32];
        } else {
            map.insert(num, index);
        }
    }

    // "Exactly one solution"
    unreachable!()
}

#[test]
fn test() {
    fn sort(mut vec: Vec<i32>) -> Vec<i32> {
        vec.sort();
        vec
    }

    assert!(sort(two_sum(vec![0, 1, 2, 3, 4], 6)) == [2, 4]);
    assert!(sort(two_sum(vec![0, 1, 2, 3, 4], 7)) == [3, 4]);

    assert!(sort(two_sum(vec![2, 7, 11, 15], 9)) == [0, 1]);
    assert!(sort(two_sum(vec![3, 2, 4], 6)) == [1, 2]);
    assert!(sort(two_sum(vec![3, 3], 6)) == [0, 1]);
}
