use std::collections::HashMap;

// LeetCode link: https://leetcode.com/problems/valid-parentheses/
pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    let open_pairs: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}')
    ]);

    let close_pairs: HashMap<char, char> = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{')
    ]);

    for c in s.chars() {
        if open_pairs.contains_key(&c) {
            stack.push(c.clone());
        }

        if close_pairs.contains_key(&c) {
            if stack.len() == 0 {
                return false;
            }

            let top = stack.pop();
            let close = close_pairs.get_key_value(&c).unwrap();
            if top.is_none() || top.unwrap().to_string() != *close.1.to_string() {
                return false;
            }
        }
    }

    return stack.len() == 0;
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("()", true)]
    #[case("([])", true)]
    #[case("]]", false)]
    #[case("((", false)]
    #[case("(]", false)]
    fn it_works(#[case] input: &str, #[case] expected: bool) {
        let result = is_valid(input.to_string());
        assert_eq!(result, expected);
    }
}
