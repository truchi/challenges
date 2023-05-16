//! https://leetcode.com/problems/palindrome-number/

fn main() {
    assert!(is_palindrome(0));
    assert!(is_palindrome(11));
    assert!(is_palindrome(232));

    assert!(!is_palindrome(123));
    assert!(!is_palindrome(45678));
}

pub fn is_palindrome(x: i32) -> bool {
    let string = x.to_string();
    let len = string.len();

    let front = string.chars().take(len / 2);
    let back = string.chars().rev().take(len / 2);

    front.zip(back).all(|(front, back)| front == back)
}
