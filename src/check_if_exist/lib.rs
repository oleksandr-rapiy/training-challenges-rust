pub struct Solution;

// LeetCode link: https://leetcode.com/problems/check-if-n-and-its-double-exist/
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        for i in 0..arr.len() {
            for j in (0..arr.len()).rev() {
                if i != j && arr[i] == (2 * arr[j]) {
                    return true;
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use rstest::rstest;

    #[rstest]
    fn check_if_exist_should_return_true() {
        let result = Solution::check_if_exist(vec![10, 2, 5, 3]);

        assert_eq!(result, true);
    }

    #[rstest]
    fn check_if_exist_should_return_false() {
        let result = Solution::check_if_exist(vec![-2, 0, 10, -19, 4, 6, -8]);

        assert_eq!(result, false);
    }
}