//! <https://leetcode.com/problems/best-time-to-buy-and-sell-stock/>

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut buy = None;
    let mut sell = None;

    for price in prices {
        match (buy, sell) {
            //
            // Never bought nor sold
            //
            (None, None) => {
                // Let's buy!
                buy = Some(price);
            }

            //
            // Sold without buying...
            //
            (None, Some(_)) => unreachable!(),

            //
            // Bought but never with *this* price yet
            //
            (Some(bought), None) => {
                // Interesting price to sell
                if price > bought {
                    sell = Some(price);
                    max_profit = max_profit.max(price - bought);
                }
                // Interesting price to buy
                else {
                    buy = Some(price);
                }
            }

            //
            // Bought and sold with this price already
            //
            (Some(bought), Some(sold)) => {
                debug_assert!(sold > bought);

                // Interesting price to sell
                if price > sold {
                    sell = Some(price);
                    max_profit = max_profit.max(price - bought);
                }
                // Interesting price to buy
                else if price < bought {
                    buy = Some(price);
                    sell = None;
                }
            }
        }
    }

    max_profit
}

#[test]
fn test() {
    assert!(max_profit(vec![7, 1, 5, 3, 6, 4]) == 5);
    assert!(max_profit(vec![7, 6, 4, 3, 1]) == 0);

    assert!(max_profit(vec![10, 15, 20, 1, 2, 3]) == 10);
    assert!(max_profit(vec![10, 9, 15, 20, 1, 2, 3]) == 11);
    assert!(max_profit(vec![10, 15, 9, 20, 1, 2, 3]) == 11);
    assert!(max_profit(vec![10, 15, 9, 2, 3, 1, 20]) == 19);
}
