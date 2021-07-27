pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            match c {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                _ if Some(c) != stack.pop() => return false,
                _ => (),
            }
        }
        return stack.is_empty();
    }
}
