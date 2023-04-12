//Given a string s containing just the characters '(', ')', '{', '}', '[' and ']
//', determine if the input string is valid. 
//
// An input string is valid if: 
//
// 
// Open brackets must be closed by the same type of brackets. 
// Open brackets must be closed in the correct order. 
// Every close bracket has a corresponding open bracket of the same type. 
// 
//
// 
// Example 1: 
//
// 
//Input: s = "()"
//Output: true
// 
//
// Example 2: 
//
// 
//Input: s = "()[]{}"
//Output: true
// 
//
// Example 3: 
//
// 
//Input: s = "(]"
//Output: false
// 
//
// 
// Constraints: 
//
// 
// 1 <= s.length <= 10⁴ 
// s consists of parentheses only '()[]{}'. 
// 
//
// Related Topics String Stack 👍 19368 👎 1111


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {

    /*
    * STACK çözümü:
    1. Girdi olan String vektörü üzerinde gezin.
    2. '(,[,{' biriyle karşılaşırsan, 'stack' adlı vektöre push yap.
    3. Örneğin, "[{}]" girdisine bakalım.
    4. For döngüsü, "[,{,},]" olmak üzere dört kez döngüye girecek.
    5. ilk iki döngüde, "[" ve "{" karşılaştığı için bu karakterleri stack vektörüne ekleyecek.
    6. stack iki döngünün sonrasında = ['[','{']
    7. üçüncü döngüde, '}' ile karşılaşacak,
    8. stack'teki son eklenmiş ifadeye bakacak, '(' değilse veya stack boşsa, girdi hatalı demektir.
    9. çünkü ')' tagı KAPANMAMIŞ demektir.
    10. Girdinin doğru olması için '(' ardından illa ki ')' gelmek ZORUNDA.
    11. stackteki son girdi '(' ise, pop yap ve sil.
    12. 3.döngü sonrasında stack = '['
    14. 4.döngüde ']' ile karşılaşacak.
    15. yukarıdaki algoritmayı yürütüp '[' verisini de stackte sileriz.
    16. Stack.len = 0 ise girdi DOĞRU, 1 veya daha büyükse YANLIŞtır.
    17. Bir nevi BİRBİRİNİ SİLME yapıyoruz.
    18. '[' ile ']' birbirini GÖTÜRÜR.
    */


    pub fn is_valid(s: String) -> bool {
        let mut stack_vec: Vec<char> = Vec::new();
        for ch in s.chars(){
            if ch == '(' || ch == '[' || ch == '{' {
                stack_vec.push(ch);
            } else {
                match ch {
                    // % ')',']','}' ifadelerinden biri geldi fakat stack.len == 0 ise,
                    // * girdi HATAlı. Nitekim, kapatma parantezi geldi fakat açma parantezi yok.
                    _ if stack_vec.len() == 0 => {
                        return  false
                    },
                    ')' => {
                        if  stack_vec.last().unwrap() != &'(' {
                            return false
                        } else {
                            stack_vec.pop();
                        }
                    },
                    ']' => {
                        if  stack_vec.last().unwrap() != &'[' {
                            return false
                        } else {
                            stack_vec.pop();
                            // 3.döngü; stack = {
                        }
                    },

                    '}' => {
                        if  stack_vec.last().unwrap() != &'{' {
                            return false
                        } else {
                            stack_vec.pop();
                        }
                    },
                    _ => {
                        return false
                    }
                }
            }
        }
        // * sonuç olarak,
        // * BU AŞAMADA, stack.len == 0 olmalı. 0 ise girdi D değilse Y.
        if stack_vec.len() >= 1 {
            false
        } else {
            true
        }
    }

}
//leetcode submit region end(Prohibit modification and deletion)
