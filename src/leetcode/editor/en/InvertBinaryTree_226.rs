//Given the root of a binary tree, invert the tree, and return its root. 
//
// 
// Example 1: 
// 
// 
//Input: root = [4,2,7,1,3,6,9]
//Output: [4,7,2,9,6,3,1]
// 
//
// Example 2: 
// 
// 
//Input: root = [2,1,3]
//Output: [2,3,1]
// 
//
// Example 3: 
//
// 
//Input: root = []
//Output: []
// 
//
// 
// Constraints: 
//
// 
// The number of nodes in the tree is in the range [0, 100]. 
// -100 <= Node.val <= 100 
// 
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 11
//990 👎 169


//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn invert_tree( root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(agac_ici) = root.as_ref(){
                let tek_mut = &mut *agac_ici.borrow_mut();
                std::mem::swap(&mut tek_mut.right,&mut tek_mut.left);

                Self::invert_tree(agac_ici.borrow_mut().right.clone());
                Self::invert_tree(agac_ici.borrow_mut().left.clone());
            }
        root
    }
}
//leetcode submit region end(Prohibit modification and deletion)
