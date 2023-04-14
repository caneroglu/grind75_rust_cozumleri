//You are given an array prices where prices[i] is the price of a given stock 
//on the iᵗʰ day. 
//
// You want to maximize your profit by choosing a single day to buy one stock 
//and choosing a different day in the future to sell that stock. 
//
// Return the maximum profit you can achieve from this transaction. If you 
//cannot achieve any profit, return 0. 
//
// 
// Example 1: 
//
// 
//Input: prices = [7,1,5,3,6,4]
//Output: 5
//Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 
//6-1 = 5.
//Note that buying on day 2 and selling on day 1 is not allowed because you 
//must buy before you sell.
// 
//
// Example 2: 
//
// 
//Input: prices = [7,6,4,3,1]
//Output: 0
//Explanation: In this case, no transactions are done and the max profit = 0.
// 
//
// 
// Constraints: 
//
// 
// 1 <= prices.length <= 10⁵ 
// 0 <= prices[i] <= 10⁴ 
// 
//
// Related Topics Array Dynamic Programming 👍 24689 👎 770


use std::cmp::{max, min};

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // kar = gun[n+1] - gun[n]
        // * O(n^2) ile Brute Force yöntemiyle çözülebilir.
        // * Fakat,
        // ! Kadane's Algoritm ile O(n) çözümü vardır.

        // ----------------------------
        // % Kadane's Algorithm
        /*
            Bir *array* içerisindeki alt-arraylar arasındaki maksimum alt-array'ın toplam değerini bulmamıza imkan verir.

            Mesela, i32 türünden Array gelsin [0,5,18] gibi. Burada 5+18=23 gibi maksimum alt-array'ın değerini bulabiliriz.
        */

        /*
        iki değişkenle başlaman lazım: 'current sum' ve 'max sum'.
        current sum, o an alt-dizi içerisindeki toplamı belirtirken, 'max sum' dizi için maksimum alt-array toplamını belirtir.
        her adımda, EĞER current > max ise, max'ı GUNCELLERIZ.

        1. iki değişkeni tanımla, Array'ın birinci elemanı olarak ata.
        2. İkinci elemandan itibaren iterasyona gir, sonuna kadar git.
        3. Her gördüğün elemanda, 'current sum' değişkenini hesapla,
        4. Curr < 0 ise 0 döndür.
        5. Cur > Max ise Max'ı güncelle.
        */
        // ----------------------------

        let mut fiyat = prices[0]; // current-sum veya local-sum de.
        let mut maks_kazanc = 0; // max-sum de.

        (1..prices.len()).for_each(|n|{ // 1. elemanı atamıştık zaten.
            let kar = prices[n] - fiyat;

            maks_kazanc = std::cmp::max(maks_kazanc,kar);
            fiyat = std::cmp::min(fiyat,prices[n]);
        });
        maks_kazanc

        /*
        ör: [7,1,5,3,6,4]
        beklenen: 5, 2.gün al, 5.gün sat , 6-1 = 5.

        -> prices[1] = 1,
        <- {
            kar = 1 - 7 = -6

            maks_k = 0;
            fiyat = 1
        }
        -> prices[2] = 5,
        <- {
            kar = 5 - 1, = 4

            maks_k = 4;
            fiyat = 1
        }
        ..
        */
    }
}
//leetcode submit region end(Prohibit modification and deletion)
