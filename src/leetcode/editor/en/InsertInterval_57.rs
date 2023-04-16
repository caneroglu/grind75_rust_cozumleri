//You are given an array of non-overlapping intervals intervals where intervals[
//i] = [starti, endi] represent the start and the end of the iᵗʰ interval and 
//intervals is sorted in ascending order by starti. You are also given an interval 
//newInterval = [start, end] that represents the start and end of another interval. 
//
// Insert newInterval into intervals such that intervals is still sorted in 
//ascending order by starti and intervals still does not have any overlapping 
//intervals (merge overlapping intervals if necessary). 
//
// Return intervals after the insertion. 
//
// 
// Example 1: 
//
// 
//Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
//Output: [[1,5],[6,9]]
// 
//
// Example 2: 
//
// 
//Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
//Output: [[1,2],[3,10],[12,16]]
//Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
// 
//
// 
// Constraints: 
//
// 
// 0 <= intervals.length <= 10⁴ 
// intervals[i].length == 2 
// 0 <= starti <= endi <= 10⁵ 
// intervals is sorted by starti in ascending order. 
// newInterval.length == 2 
// 0 <= start <= end <= 10⁵ 
// 
//
// Related Topics Array 👍 8165 👎 566


use std::cmp::{max, min};

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::*;

        let mut cevap = Vec::new();
        let mut yeni_aralik = new_interval;

        // dört durum var; yeni gelen aralık hepsinden küçük, hepsinden büyük, ikisinin arasında veya biriyle çakışıyor.
        // hepsinden büyük olma durumu için dizide 2.indise, gelende ise 1.indise bakmamız yeterli.
            // Mesela, '[[1,3],[6,9]]' yeni aralık, '[10,12]' gelsin. Hepsinden büyük olur.

        intervals.into_iter().enumerate().for_each(|(indis,vekt)|{
            // Birinci diziden büyük, ikincisinden bilmiyoruz. O sebeple birinci diziyi cevaba ekle.
            if vekt[1] < yeni_aralik[0] {
                cevap.push(vekt.clone())
                // Birinci diziden küçükse, 'yeni_aralik'i cevaba ekle.
            } else if vekt[0] > yeni_aralik[1] {
                cevap.push(yeni_aralik.clone());
                yeni_aralik = vekt.clone();
                // çakışıyorsa 'MERGE' yap.
            } else if vekt[1] >= yeni_aralik[0] || vekt[0] <= yeni_aralik[1]{
                /* Araliklarda MERGE işlemi basittir;
                [1,5] ve [2,7] merge yapmak istersek,
                1. indisteki verilerin MIN,
                2. indisteki verilerin MAX,
                al.
                = [1,7]
                */
                yeni_aralik[0] = min(yeni_aralik[0],vekt[0]);
                yeni_aralik[1] = max(yeni_aralik[1],vekt[1]);

            }

        });
        cevap.push(yeni_aralik);
        cevap
    }
}
//leetcode submit region end(Prohibit modification and deletion)
