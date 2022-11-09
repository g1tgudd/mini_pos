use std::collections::HashMap;
struct Purchase {
    id: i64,
    name: String,
    item: String,
    quantity: i32,
    price: i32,
}

struct Purchases {
    inner: HashMap<i64, Purchase>,
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

fn main() {
    println!("Hello, world!");
}
