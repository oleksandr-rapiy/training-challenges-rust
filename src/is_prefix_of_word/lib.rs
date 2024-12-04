pub struct Solution;

// LeetCode link: https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/description/
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let words = sentence.split(' ');

        for (index, word) in words.enumerate() {
            if word.starts_with(search_word.as_str()) {
                return (index + 1) as i32;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("i love eating burger", "burg", 4)]
    fn is_prefix_of_word_return_index_of_prefix(#[case] sentence: &str, #[case] search_word: &str, #[case]expected_index: i32) {
        let result = Solution::is_prefix_of_word(sentence.to_string(), search_word.to_string());

        assert_eq!(result, expected_index);
    }
}

