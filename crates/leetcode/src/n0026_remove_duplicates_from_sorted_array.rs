//! <https://leetcode.com/problems/remove-duplicates-from-sorted-array/>

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    use std::collections::BTreeSet;

    let set = BTreeSet::from_iter(nums.iter().copied());
    let len = set.len();
    *nums = set.into_iter().collect();
    len as i32
}

#[test]
fn test() {
    for (mut sorted, deduped) in [
        //
        (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], vec![0, 1, 2, 3, 4]),
        (
            vec![-9, -9, -1, -1, 0, 0, 1, 1, 8, 8, 10],
            vec![-9, -1, 0, 1, 8, 10],
        ),
    ] {
        let k = remove_duplicates(&mut sorted) as usize;
        assert!(k == deduped.len());
        assert!(&sorted[..k] == deduped);
    }
}
