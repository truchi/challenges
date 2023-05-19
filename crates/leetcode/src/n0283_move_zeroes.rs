//! <https://leetcode.com/problems/move-zeroes/>

pub fn allocates(nums: &mut Vec<i32>) {
    let mut moved = Vec::with_capacity(nums.len());

    moved.extend(nums.iter().copied().filter(|num| *num != 0));
    moved.resize(moved.capacity(), 0);

    *nums = moved;
}

pub fn in_place(nums: &mut Vec<i32>) {
    let mut i = 0;
    let mut moved = 0;

    while i != nums.len() {
        let zeroes = nums[i..].iter().take_while(|num| **num == 0).count();
        let non_zeroes = nums[i + zeroes..]
            .iter()
            .take_while(|num| **num != 0)
            .count();

        nums.copy_within(i + zeroes..i + zeroes + non_zeroes, moved);

        i += zeroes + non_zeroes;
        moved += non_zeroes;
    }

    nums[moved..].fill(0);
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    in_place(nums)
}

#[test]
fn test() {
    for (mut input, output) in [
        (vec![], vec![]),
        (vec![0], vec![0]),
        (vec![0, 1, 0, 3, 12], vec![1, 3, 12, 0, 0]),
        (
            vec![0, 0, 0, 1, 2, 3, 4, 0, 1],
            vec![1, 2, 3, 4, 1, 0, 0, 0, 0],
        ),
    ] {
        println!("===========================");

        dbg!(&input);
        move_zeroes(&mut input);
        dbg!(&input);
        assert!(input == output);
    }
}
