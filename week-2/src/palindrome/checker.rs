pub struct PalindromeChecker;
impl PalindromeChecker {
    pub fn is_palindrome(num: i32) -> bool {
        let s = num.to_string();
        let chars: Vec<char> = s.chars().collect();
        let mut right = chars.len() -1;
        let mut left = 0 ;

        while left < right {
            if chars[left] != chars[right] {
                return  false;
            }
            right -= 1;
            left += 1;
        }
        true
    }
}