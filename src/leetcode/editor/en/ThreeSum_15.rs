//Given an integer array nums, return all the triplets [nums[i], nums[j], nums[
//k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0. 
//
// Notice that the solution set must not contain duplicate triplets. 
//
// 
// Example 1: 
//
// 
//Input: nums = [-1,0,1,2,-1,-4]
//Output: [[-1,-1,2],[-1,0,1]]
//Explanation: 
//nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
//nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
//nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
//The distinct triplets are [-1,0,1] and [-1,-1,2].
//Notice that the order of the output and the order of the triplets does not 
//matter.
// 
//
// Example 2: 
//
// 
//Input: nums = [0,1,1]
//Output: []
//Explanation: The only possible triplet does not sum up to 0.
// 
//
// Example 3: 
//
// 
//Input: nums = [0,0,0]
//Output: [[0,0,0]]
//Explanation: The only possible triplet sums up to 0.
// 
//
// 
// Constraints: 
//
// 
// 3 <= nums.length <= 3000 
// -10⁵ <= nums[i] <= 10⁵ 
// 
//
// Related Topics Array Two Pointers Sorting 👍 25187 👎 2271


//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    // 'TWO SUM' sorusunun biraz daha özel hali.
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        nums.dedup();

        let vc = nums.iter().enumerate().map(|(indis,&eleman)|{
            let mut left_ptr = indis + 1;
            let mut right_ptr = nums.len() - 1;
            while left_ptr < right_ptr {
                let toplam = eleman + nums[left_ptr] + nums[right_ptr];
                if toplam > 0 {
                    right_ptr -= 1;
                } else if toplam < 0 {
                    left_ptr += 1;
                } else {
                    left_ptr += 1;
                    return vec![eleman,nums[left_ptr],nums[right_ptr]]
                }
            }
            vec![]
        }).collect::<Vec<Vec<i32>>>();

        vc
    }
}
//leetcode submit region end(Prohibit modification and deletion)
