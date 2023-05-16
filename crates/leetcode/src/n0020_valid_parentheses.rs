//! <https://leetcode.com/problems/valid-parentheses/>

use Type::*;

#[derive(PartialEq)]
pub enum Type {
    Round,
    Square,
    Curly,
}

impl Type {
    pub fn open(&self) -> u8 {
        match self {
            Round => b'(',
            Square => b'[',
            Curly => b'{',
        }
    }

    pub fn close(&self) -> u8 {
        match self {
            Round => b')',
            Square => b']',
            Curly => b'}',
        }
    }
}

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::<Type>::with_capacity(512);

    for &byte in s.as_bytes() {
        match byte {
            byte if byte == Round.open() => stack.push(Round),
            byte if byte == Square.open() => stack.push(Square),
            byte if byte == Curly.open() => stack.push(Curly),

            byte if byte == Round.close() => {
                if stack.pop() != Some(Round) {
                    return false;
                }
            }
            byte if byte == Square.close() => {
                if stack.pop() != Some(Square) {
                    return false;
                }
            }
            byte if byte == Curly.close() => {
                if stack.pop() != Some(Curly) {
                    return false;
                }
            }

            _ => unreachable!(),
        }
    }

    stack.is_empty()
}

#[test]
fn test() {
    assert!(is_valid(String::from("")));
    assert!(is_valid(String::from("()[]{}")));
    assert!(is_valid(String::from("([{}])")));

    assert!(!is_valid(String::from("(]")));
    assert!(!is_valid(String::from("([{")));
}
