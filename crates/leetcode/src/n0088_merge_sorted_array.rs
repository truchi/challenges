//! <https://leetcode.com/problems/merge-sorted-array/>

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
    let mut nums = (&nums1[0..m as usize])
        .iter()
        .copied()
        .chain(nums2.iter().copied())
        .collect::<Vec<_>>();

    nums.sort();
    *nums1 = nums;
}
