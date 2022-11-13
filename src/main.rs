use std::collections::HashMap;
use std::io;
use std::fs::{File, OpenOptions}; 
use std::path::PathBuf; 
use std::error::Error;
use csv;

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

    // loop {
    //     show();
    //     let input = match get_input() {
    //         Some(input) => input,
    //         None => return
    //     };
    //     match input.as_str() {
    //         _ => break
    //     }
    // }
}

fn main() {
    println!("Hello, world!");
}
