//! <https://leetcode.com/problems/fizz-buzz/>

pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|i| match i {
            _ if i % 3 == 0 && i % 5 == 0 => String::from("FizzBuzz"),
            _ if i % 3 == 0 => String::from("Fizz"),
            _ if i % 5 == 0 => String::from("Buzz"),
            _ => i.to_string(),
        })
        .collect()
}

#[test]
fn test() {
    assert!(
        fizz_buzz(15)
            == [
                String::from("1"),
                String::from("2"),
                String::from("Fizz"),
                String::from("4"),
                String::from("Buzz"),
                String::from("Fizz"),
                String::from("7"),
                String::from("8"),
                String::from("Fizz"),
                String::from("Buzz"),
                String::from("11"),
                String::from("Fizz"),
                String::from("13"),
                String::from("14"),
                String::from("FizzBuzz"),
            ]
    );
}
