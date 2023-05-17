//! <https://leetcode.com/problems/pascals-triangle/>

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut pascal = Vec::with_capacity(num_rows as usize);
    pascal.push(vec![1]);

    for _ in 1..num_rows {
        let last = pascal.last().map(|vec| vec.as_slice()).unwrap_or_default();
        let middle = last.windows(2).map(|window| window[0] + window[1]);

        pascal.push([1].into_iter().chain(middle).chain([1]).collect());
    }

    pascal
}

#[test]
fn test() {
    let generated = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
    ];

    for i in 1..=generated.len() {
        assert!(generate(i as i32) == generated[..i]);
    }
}
