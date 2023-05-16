//! <https://leetcode.com/problems/sqrtx/>

use std::cmp::Ordering::{self, *};

fn pow2(i: i32, target: i32) -> Ordering {
    let i = i as i64;
    let target = target as i64;

    (i * i).cmp(&target)
}

pub fn my_sqrt(x: i32) -> i32 {
    let target = x;
    let mut range = 0..target;

    loop {
        let i = range.start + (range.end - range.start) / 2;

        match pow2(i, target) {
            Less => match pow2(i + 1, target) {
                Less => range.start = i + 2,
                Equal => return i + 1,
                Greater => return i,
            },
            Equal => return i,
            Greater => match pow2(i - 1, target) {
                Less => return i - 1,
                Equal => return i - 1,
                Greater => range.end = i - 1,
            },
        }
    }
}

#[test]
fn test() {
    assert!(my_sqrt(0) == 0);
    assert!(my_sqrt(1) == 1);
    assert!(my_sqrt(2) == 1);
    assert!(my_sqrt(3) == 1);
    assert!(my_sqrt(4) == 2);
    assert!(my_sqrt(5) == 2);
    assert!(my_sqrt(6) == 2);
    assert!(my_sqrt(7) == 2);
    assert!(my_sqrt(8) == 2);
    assert!(my_sqrt(9) == 3);
    assert!(my_sqrt(10) == 3);
}
