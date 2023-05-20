//! <https://leetcode.com/problems/intersection-of-two-arrays-ii/>

pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    nums1.retain(|num1| {
        if let Some(index) = nums2.iter().position(|num2| num1 == num2) {
            nums2.remove(index);
            true
        } else {
            false
        }
    });

    nums1
}

#[test]
fn test() {
    for (nums1, nums2, mut expected) in [
        (vec![1, 2, 2, 1], vec![2, 2], vec![2, 2]),
        (vec![4, 9, 5], vec![9, 4, 9, 8], vec![4, 9]),
    ] {
        let mut output = intersect(nums1, nums2);
        output.sort();
        expected.sort();
        assert!(output == expected);
    }
}
