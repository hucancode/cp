use std::collections::HashMap;
struct Cashier {
    prices: HashMap<i32, i32>,
    discount_index: i32,
    discount_factor: f64,
    index: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        Self {
            prices: products.into_iter().zip(prices.into_iter()).collect(),
            discount_index: n,
            discount_factor: 1.0 - discount as f64 * 0.01,
            index: 0,
        }
    }
    
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let ret = product.iter().zip(amount.iter())
            .map(|(p,a)| self.prices.get(p).unwrap_or(&0) * a)
            .sum::<i32>();
        self.index += 1;
        if self.index % self.discount_index == 0 {
            self.discount_factor * ret as f64
        } else {
            ret as f64
        }
    }
}

/**
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */