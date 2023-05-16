//! <https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/>

pub fn str_str(haystack: String, needle: String) -> i32 {
    haystack
        .find(&needle)
        .map(|index| index as i32)
        .unwrap_or(-1)
}

#[test]
fn test() {
    assert!(str_str(String::from("sadbutsad"), String::from("sad")) == 0);
    assert!(str_str(String::from("abcd"), String::from("efgh")) == -1);
}
