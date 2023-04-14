//You are given the heads of two sorted linked lists list1 and list2. 
//
// Merge the two lists in a one sorted list. The list should be made by 
//splicing together the nodes of the first two lists. 
//
// Return the head of the merged linked list. 
//
// 
// Example 1: 
// 
// 
//Input: list1 = [1,2,4], list2 = [1,3,4]
//Output: [1,1,2,3,4,4]
// 
//
// Example 2: 
//
// 
//Input: list1 = [], list2 = []
//Output: []
// 
//
// Example 3: 
//
// 
//Input: list1 = [], list2 = [0]
//Output: [0]
// 
//
// 
// Constraints: 
//
// 
// The number of nodes in both lists is in the range [0, 50]. 
// -100 <= Node.val <= 100 
// Both list1 and list2 are sorted in non-decreasing order. 
// 
//
// Related Topics Linked List Recursion 👍 17869 👎 1651


//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut birinci_list = &mut list1;

        while list2.is_some() {

            if birinci_list.is_none() || list2.as_ref()?.val < birinci_list.as_ref()?.val{
                std::mem::swap(birinci_list,&mut list2)
                // birinci_list = None ise veya birinci_list değeri 2.listten büyükse, Merge için, list2 konumu ile birinci listi RAM'de değiştir.
            }

            birinci_list = &mut birinci_list.as_mut()?.next;

        }

        list1
    }

    //let mut siradaki_node = &mut list1;
    //println!("{}",siradaki_node.as_ref()?.val);

    // siradaki_node = &mut Option<Box..>
    // okumamız için '&mut' olarak almamız şart.
    // atarkende yine '&mut'lu atama yapmalıyız.
    // aşağıdaki '&mut' yukarıdaki değişken tanımını götürüyor.
    // '.as_mut()' ise 'siradaki_node'u mutable referansıyla alıyor, çünkü solda yine KENDİSİNİ değiştiriyoruz.
    // '?' ile Option'u açıyoruz.
    // Açtıktan sonra 'next'teki Option'lu değeri okuyabiliyoruz.
    // * Bu şekilde LinkedList üzerinde ilerleyebiliriz.
    // siradaki_node = &mut siradaki_node.as_mut()?.next;
    // % ----

}
//leetcode submit region end(Prohibit modification and deletion)
