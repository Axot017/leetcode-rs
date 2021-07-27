pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        return x.to_string() == x.to_string().chars().rev().collect::<String>();
    }
}
