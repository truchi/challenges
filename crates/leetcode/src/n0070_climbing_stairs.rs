//! <https://leetcode.com/problems/climbing-stairs/>

pub fn climb_stairs(n: i32) -> i32 {
    let (mut a, mut b) = (0, 1);

    for _ in 0..n {
        (a, b) = (b, a + b);
    }

    b
}

#[test]
fn test() {
    assert!(climb_stairs(1) == 1);
    assert!(climb_stairs(2) == 2);
    assert!(climb_stairs(3) == 3);
    assert!(climb_stairs(4) == 5);
    assert!(climb_stairs(5) == 8);
    assert!(climb_stairs(6) == 13);
    assert!(climb_stairs(7) == 21);
}
