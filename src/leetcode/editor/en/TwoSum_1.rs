//Given an array of integers nums and an integer target, return indices of the 
//two numbers such that they add up to target. 
//
// You may assume that each input would have exactly one solution, and you may 
//not use the same element twice. 
//
// You can return the answer in any order. 
//
// 
// Example 1: 
//
// 
//Input: nums = [2,7,11,15], target = 9
//Output: [0,1]
//Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// 
//
// Example 2: 
//
// 
//Input: nums = [3,2,4], target = 6
//Output: [1,2]
// 
//
// Example 3: 
//
// 
//Input: nums = [3,3], target = 6
//Output: [0,1]
// 
//
// 
// Constraints: 
//
// 
// 2 <= nums.length <= 10⁴ 
// -10⁹ <= nums[i] <= 10⁹ 
// -10⁹ <= target <= 10⁹ 
// Only one valid answer exists. 
// 
//
// 
//Follow-up: Can you come up with an algorithm that is less than 
//O(n²) time complexity?
//
// Related Topics Array Hash Table 👍 45130 👎 1478




//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        // target = a[n] + a[m] diyelim.
        // target - a[m] = a[n] olacak. yani tek iterasyon.

        // arraylar üzerinde şöyle bir cevap akla gelebilir:

       /* let mut ans = vec![];
        nums.iter().enumerate().for_each(|(indx,&deger)|{
            let fark = target - deger;


            if nums.contains(&fark) {
                ans.push(indx as i32);
                ans.push(nums.iter().position(|x|*x==fark).unwrap() as i32)
            }
        });

        ans*/
        // * Bunun çıktısı [2,7,11,15] - 13 için [0,2,2,0] olur. Tam olarak aradığımız şey değil. yukarıdaki iterasyon da '&deger' = 2 ve 7 için iki defa işliyor. Sonuçta dört elemanlı oluyor.

        // % HashMap kullanırsak, her veri satırının *nadir* olduğundan emin olabiliriz.

        let mut cevap_vek = Vec::new();
        let mut cevap_hashmap: HashMap<i32,i32> = HashMap::new();
        nums.iter().enumerate().for_each(|(indx,&deger)|{
            let fark = target - deger; // Buradaki 'fark' aslında 'ans' vektöründe bir eleman olacak. Yani eğer varsa, aradığımız şey.
            match cevap_hashmap.get(&fark) { // HashMap'te 'fark' yani 'ans'ta buna karşılık bir eleman 'key' olarak önceden var mı yok mu bak
                None => {
                    cevap_hashmap.insert(deger,indx as i32);
                    // yoksa, 'key' = ans[n], 'value' = n olacak şekilde, o an itare edildiği ans elemanını ve indeksini ekle.
                },
                Some(aradigimiz_onceki_indx) => {
                    cevap_vek.push(*aradigimiz_onceki_indx);
                    cevap_vek.push(indx as i32);
                }
            }
        });
        cevap_vek
    }
}
//leetcode submit region end(Prohibit modification and deletion)
