use std::collections::HashMap;
struct Purchase {
    id: i64,
    name: String,
    item: String,
    quantity: i32,
    price: i32,
}

struct Purchases {
    inner: HashMap<String, Purchase>,
}
impl Purchases {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, purchase: Purchase) {
        self.inner.insert(purchase.name.clone(), purchase);
    }
}

struct History {
    id: i64,
    name: String,
    total: i32, //Total price dari name yang sama
    discount: f64, //input manual, desimal dari 0 sampe 1 
    payment: i32,//input manual
    change: i32,//Payment - total, ngk boleh negatif.
}

struct Histories {
    inner: HashMap<i64,History>,
}
impl Histories {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
