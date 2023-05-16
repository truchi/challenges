//! <https://leetcode.com/problems/plus-one/>

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let carry = 1;

    for digit in digits.iter_mut().rev() {
        let d = *digit + carry;

        if d == 10 {
            *digit = 0;
        } else {
            *digit = d;
            return digits;
        }
    }

    digits.insert(0, carry);
    digits
}

#[test]
fn test() {
    assert!(plus_one(vec![0]) == [1]);
    assert!(plus_one(vec![9]) == [1, 0]);
    assert!(plus_one(vec![1, 2, 3]) == [1, 2, 4]);
    assert!(plus_one(vec![1, 8, 9]) == [1, 9, 0]);
    assert!(plus_one(vec![9, 9, 9]) == [1, 0, 0, 0]);
}
