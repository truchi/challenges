//! <https://leetcode.com/problems/power-of-three/>

pub fn is_power_of_three(n: i32) -> bool {
    match n {
        0 => false,
        1 => true,
        n if n % 3 == 0 => is_power_of_three(n / 3),
        _ => false,
    }
}

#[test]
fn test() {
    assert!(!is_power_of_three(0));
    assert!(is_power_of_three(3));
    assert!(is_power_of_three(27));
    assert!(!is_power_of_three(-1));
    assert!(!is_power_of_three(-27));
    assert!(!is_power_of_three(28));
}
