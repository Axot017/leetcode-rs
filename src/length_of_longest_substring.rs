use std::collections::HashMap;

#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut chars: HashMap<char, usize> = HashMap::new();
        let mut last = 0;
        let mut top_result = 0;
        let mut result = 0;

        for (i, c) in s.chars().enumerate() {
            let previous = chars.get(&c);

            if let Some(index) = previous {
                if top_result < result {
                    top_result = result;
                }
                if index >= &last {
                    result = i - index;
                    last = *index;
                } else {
                    result += 1;
                }
            } else {
                result += 1;
            }
            chars.insert(c, i);
        }

        return if result > top_result {
            result as i32
        } else {
            top_result as i32
        };
    }
}
