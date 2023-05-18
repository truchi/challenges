//! <https://leetcode.com/problems/reverse-bits/>

pub fn reverse_bits(x: u32) -> u32 {
    let mut original = x;
    let mut reversed = 0;

    for _ in 0..32 {
        reversed <<= 1;
        reversed |= original & 1;
        original >>= 1;
    }

    reversed
}

#[test]
fn test() {
    for u in [
        0,
        1,
        100,
        259,
        1000,
        43261596,
        4294967293,
        u32::MAX,
        u32::MIN,
    ] {
        assert!(reverse_bits(u) == u.reverse_bits());
    }
}
