//! <https://leetcode.com/problems/first-unique-character-in-a-string/>

pub fn first_uniq_char(s: String) -> i32 {
    let mut occurences = std::collections::HashMap::with_capacity(s.len());

    for (index, byte) in s.bytes().enumerate() {
        occurences.entry(byte).or_insert((index, 0)).1 += 1;
    }

    occurences
        .values()
        .filter(|(_, occurences)| *occurences == 1)
        .map(|(index, _)| *index as i32)
        .min()
        .unwrap_or(-1)
}

#[test]
fn test() {
    assert!(first_uniq_char(String::from("leetcode")) == 0);
    assert!(first_uniq_char(String::from("loveleetcode")) == 2);
    assert!(first_uniq_char(String::from("aabb")) == -1);
}
