//! <https://leetcode.com/problems/reverse-string/>

pub fn reverse_string(s: &mut Vec<char>) {
    // aka slice.reverse()...

    let len = s.len();

    for i in 0..len / 2 {
        s.swap(i, len - 1 - i);
    }
}

#[test]
fn test() {
    for (mut input, output) in [
        (vec!['h', 'e', 'l', 'l', 'o'], vec!['o', 'l', 'l', 'e', 'h']),
        (
            vec!['H', 'a', 'n', 'n', 'a', 'h'],
            vec!['h', 'a', 'n', 'n', 'a', 'H'],
        ),
    ] {
        reverse_string(&mut input);
        assert!(input == output);
    }
}
