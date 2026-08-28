impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let length=prices.len();
        let mut min=prices[0];
        let mut profit=0;
        for i in 1..length{
            if min>prices[i]{
                min=prices[i];
            }
            if profit<prices[i]-min{
                profit=prices[i]-min;
            }
        }
        profit
    }
}