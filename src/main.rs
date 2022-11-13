use std::collections::HashMap;
use std::io;
use thiserror::Error;

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
impl Purchases {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, purchase: Purchase) {
        self.inner.insert(purchase.id, purchase);
    }

    fn next_id(&self) -> i64 {
        let mut ids: Vec<_> = self.inner.keys().collect();
        ids.sort();
        match ids.pop() {
            Some(id) => id + 1,
            None => 1
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again")
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_price() -> Option<i32> {
    print!("Price:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<i32, _> = input.parse();
        match parsed_input {
            Ok(price) => return Some(price),
            Err(_) => println!("Please enter the item price")
        }
    }
}

fn get_order (purchases: &mut Purchases) {
    println!("Customer Name: ");
    let name = match get_input() {
        Some(input) => input,
        None => return
    };
    let price: i32  = match get_price() {
        Some(price) => price,
        None => return
    };
    let purchase = Purchase { id, name, item, quantity, price };
    purchases.add(purchase);
    println!("Bill added");
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

fn main_menu() {
    fn show() {
        println!("");
        println!("=== Mini Pos ===");
        println!("1. Input Order");
        println!("2. Process Order");
        println!("3. Remove Order");
        println!("4. View Orders History");
        println!("");
        println!("Enter Selection");
    }
}

fn main() {
    println!("Hello, world!");
}
