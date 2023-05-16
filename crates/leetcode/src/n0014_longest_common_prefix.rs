//! <https://leetcode.com/problems/longest-common-prefix/>

pub fn prefix<'s>(a: &'s str, b: &'s str) -> &'s str {
    let len = a.len().min(b.len());

    let a_bytes = &a.as_bytes()[..len];
    let b_bytes = &b.as_bytes()[..len];

    let index = a_bytes
        .iter()
        .zip(b_bytes)
        .map(|(a, b)| a == b)
        .take_while(|equals| *equals)
        .count();

    &a[..index]
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut strings = strs.iter();

    let mut longest = if let Some(longest) = strings.next() {
        longest.as_str()
    } else {
        return String::new();
    };

    for string in strings {
        longest = prefix(longest, &string);

        if longest.is_empty() {
            return String::new();
        }
    }

    longest.into()
}

#[test]
fn test() {
    fn strings(strs: &[&str]) -> Vec<String> {
        strs.into_iter().map(|str| str.to_string()).collect()
    }

    assert!(dbg!(prefix("flower", "flow")) == "flow");
    assert!(dbg!(prefix("flower", "flight")) == "fl");

    assert!(longest_common_prefix(strings(&["flower", "flow", "flight"])) == "fl");
    assert!(longest_common_prefix(strings(&["dog", "racecar", "car"])) == "");
}
