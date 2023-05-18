//! <https://leetcode.com/problems/valid-palindrome/>

pub fn is_palindrome(s: String) -> bool {
    // Since we don't know if this is ascii, we can't split in the middle nor work with bytes
    // And do twice the work...

    let front = s
        .chars()
        .filter(|char| char.is_alphanumeric())
        .flat_map(|char| char.to_lowercase());
    let back = s
        .chars()
        .rev()
        .filter(|char| char.is_alphanumeric())
        .flat_map(|char| char.to_lowercase());

    front.zip(back).all(|(front, back)| front == back)
}

#[test]
fn test() {
    assert!(is_palindrome(String::from("")));
    assert!(is_palindrome(String::from(
        "A man, a plan, a canal: Panama"
    )));

    assert!(!is_palindrome(String::from("Romain")));
}
