// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
fn calculate_price_of_apples(quantity: u32) -> u32 {
    const BASE_PRICE: u32 = 2;
    const DISCOUNTED_PRICE: u32 = 1;
    const DISCOUNT_QUANTITY: u32 = 40;

    let price = if quantity > DISCOUNT_QUANTITY {
        DISCOUNTED_PRICE
    } else {
        BASE_PRICE
    };

    let cost = price * quantity;
    println!("{price}, {quantity}, {cost}");

    cost
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
