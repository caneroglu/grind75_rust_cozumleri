//A phrase is a palindrome if, after converting all uppercase letters into 
//lowercase letters and removing all non-alphanumeric characters, it reads the same 
//forward and backward. Alphanumeric characters include letters and numbers. 
//
// Given a string s, return true if it is a palindrome, or false otherwise. 
//
// 
// Example 1: 
//
// 
//Input: s = "A man, a plan, a canal: Panama"
//Output: true
//Explanation: "amanaplanacanalpanama" is a palindrome.
// 
//
// Example 2: 
//
// 
//Input: s = "race a car"
//Output: false
//Explanation: "raceacar" is not a palindrome.
// 
//
// Example 3: 
//
// 
//Input: s = " "
//Output: true
//Explanation: s is an empty string "" after removing non-alphanumeric 
//characters.
//Since an empty string reads the same forward and backward, it is a palindrome.
//
// 
//
// 
// Constraints: 
//
// 
// 1 <= s.length <= 2 * 10⁵ 
// s consists only of printable ASCII characters. 
// 
//
// Related Topics Two Pointers String 👍 6462 👎 6899


use std::str::Chars;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {

    // * two pointer çözüm
    pub fn is_palindrome(mut s: String) -> bool {
        use std::str::Chars;
        s = s.to_ascii_lowercase();
        s.retain(|c|c.is_alphanumeric());
        let mut left_ptr = 0usize;
        let mut right_ptr = s.len() - 1;
        let s_vec = s.chars().collect::<Vec<char>>();
        while left_ptr < right_ptr {
            if s_vec.len() <= 1{
                return true // % " " yani boş -whitespace- karakterini de TRUE olarak kabul etmemiz lazımmış.
            }
            if s_vec[left_ptr] != s_vec[right_ptr] {
                return false
            } else {
                left_ptr += 1;
                right_ptr -= 1;
            }
        }
        true
    }

    /* * örneğin, 'kınık' cümlesi olsun.
    -> 'k'
    <- 'k' ?= 'k' TRUE

    ! POLIANDROM'da: SON KARAKTER == ILK KARAKTER,
    ! 2.KARAKTER == SONDAN 2.KARAKTER vs..
    ! her zaman OLMALI!

    */
}
//leetcode submit region end(Prohibit modification and deletion)
