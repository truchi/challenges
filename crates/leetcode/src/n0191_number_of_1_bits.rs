//! <https://leetcode.com/problems/number-of-1-bits/>

pub fn hamming_weight(n: u32) -> i32 {
    let mut n = n;
    let mut count = 0;

    for _ in 0..32 {
        count += n & 1;
        n >>= 1;
    }

    count as i32
}

#[test]
fn test() {
    assert!(hamming_weight(0b00000000000000000000000000001011) == 3);
    assert!(hamming_weight(0b00000000000000000000000010000000) == 1);
    assert!(hamming_weight(0b11111111111111111111111111111101) == 31);

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
        assert!(hamming_weight(u) == u.count_ones() as i32);
    }
}
