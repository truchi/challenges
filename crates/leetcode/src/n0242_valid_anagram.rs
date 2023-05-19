//! <https://leetcode.com/problems/valid-anagram/>

pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_bytes = s.into_bytes();
    let mut t_bytes = t.into_bytes();

    s_bytes.sort_unstable();
    t_bytes.sort_unstable();

    s_bytes == t_bytes
}

#[test]
fn test() {
    assert!(is_anagram(String::from("anagram"), String::from("nagaram")));
    assert!(!is_anagram(String::from("rat"), String::from("car")));
}
