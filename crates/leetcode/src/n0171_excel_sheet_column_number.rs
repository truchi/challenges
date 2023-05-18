//! <https://leetcode.com/problems/excel-sheet-column-number/>

pub fn title_to_number(column_title: String) -> i32 {
    const BASE: i32 = 26;

    column_title
        .as_bytes()
        .iter()
        .rev()
        .copied()
        .map(|u8| u8 - b'A' + 1)
        .enumerate()
        .map(|(pow, u8)| (u8 as i32) * BASE.pow(pow as u32))
        .sum()
}

#[test]
fn test() {
    assert!(title_to_number(String::from("A")) == 1);
    assert!(title_to_number(String::from("AB")) == 28);
    assert!(title_to_number(String::from("ZY")) == 701);
}
