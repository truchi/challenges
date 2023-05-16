//! https://leetcode.com/problems/roman-to-integer/

use Roman::*;

/// Ave, Ceasar!
#[derive(Copy, Clone)]
pub enum Roman {
    I, // 1
    V, // 5
    X, // 10
    L, // 50
    C, // 100
    D, // 500
    M, // 1000
}

impl From<Roman> for i32 {
    fn from(roman: Roman) -> Self {
        match roman {
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000,
        }
    }
}

impl From<Roman> for char {
    fn from(roman: Roman) -> Self {
        match roman {
            I => 'I',
            V => 'V',
            X => 'X',
            L => 'L',
            C => 'C',
            D => 'D',
            M => 'M',
        }
    }
}

impl std::ops::Sub for Roman {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        i32::from(self) - i32::from(rhs)
    }
}

impl PartialEq<Roman> for char {
    fn eq(&self, roman: &Roman) -> bool {
        char::from(*self) == char::from(*roman)
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let mut int = 0;
    let mut chars = s.chars().peekable();

    while let Some(char) = chars.next() {
        let peek = chars.peek().cloned();

        let (add, next) = match char {
            // I
            char if char == I => match peek {
                Some(peek) if peek == V => (V - I, true),
                Some(peek) if peek == X => (X - I, true),
                _ => (I.into(), false),
            },
            // V
            char if char == V => (V.into(), false),
            // X
            char if char == X => match peek {
                Some(peek) if peek == L => (L - X, true),
                Some(peek) if peek == C => (C - X, true),
                _ => (X.into(), false),
            },
            // L
            char if char == L => (L.into(), false),
            // C
            char if char == C => match peek {
                Some(peek) if peek == D => (D - C, true),
                Some(peek) if peek == M => (M - C, true),
                _ => (C.into(), false),
            },
            // D
            char if char == D => (D.into(), false),
            // M
            char if char == M => (M.into(), false),
            //
            _ => unreachable!(),
        };

        int += add;

        if next {
            chars.next();
        }
    }

    int
}

fn main() {
    assert!(roman_to_int("III".into()) == 3);
    assert!(roman_to_int("LVIII".into()) == 58);
    assert!(roman_to_int("MCMXCIV".into()) == 1994);
}
