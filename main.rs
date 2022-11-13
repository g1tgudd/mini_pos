use std::collections::HashMap;
use std::io;
use thiserror::Error;
use std::error::Error;
use csv::WriterBuilder;
use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;


//Helper function untuk scan input
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

//Helper function scan input bentuk i32
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

#[derive(Error, Debug)]
enum ParseError {
    #[error("id must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError),
    
    #[error("empty record")]
    EmptyRecord,
    
    #[error("missing field: {0}")]
    MissingField(String),
}

#[derive(Debug, Deserialize, Serialize)]
struct Purchase {
    id: i64,
    name: String,
    item: String,
    quantity: i32,
    price: i32,
}
#[derive(Debug, Deserialize, Serialize)]
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

    fn search(&self, name:&str) -> Vec<&Purchase> {
        self.inner
            .values()
            .filter(|rec| rec.name.to_lowercase().contains(&name.to_lowercase()))
            .collect()
    }

    fn remove (&mut self, id:i64) -> Option<Purchase> {
        self.inner.remove(&id)
    }
}

//function buat Parse Purchase.csv
fn read_purchase(purchases: &mut Purchases) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("src/bin/Purchase.csv")?;

    for result in reader.deserialize() {
        let record: Purchase = result?;
        purchases.add(record);
        println!("{:?}",record);
    }
    Ok(())
}

//function untuk write ke Purchase.csv
fn write_purchase(name_input:&str, item_input:&str, quantity_input: i32, price_input: i32)-> Result<(), Box<dyn Error>>{
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("Purchase.csv")
        .unwrap();

    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    
    writer.serialize(Purchase {
        id: next_id, //error not in scope next_id
        name: name_input.to_owned(),
        item: item_input.to_owned(),
        quantity: quantity_input,
        price: price_input,
    })?;

    writer.flush()?;
    Ok(())
}
#[derive(Debug, Deserialize, Serialize)]
struct History {
    id: i64,
    name: String,
    total: i32, //Total price dari name yang sama
    discount: i32, //input manual
    payment: i32,//input manual
    change: i32,//Payment - total, ngk boleh negatif.
}

#[derive(Debug, Deserialize, Serialize)]
struct Histories {
    inner: HashMap<i64,History>,
}

impl Histories {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn next_id(&self) -> i64 {
        let mut ids: Vec<_> = self.inner.keys().collect();
        ids.sort();
        match ids.pop() {
            Some(id) => id + 1,
            None => 1
        }
    }
    fn add(&mut self, history: History) {
        self.inner.insert(history.id, history);
    }
}

fn read_history(histories: &mut Histories) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("src/bin/History.csv")?;

    for result in reader.deserialize() {
        let record: History = result?;
        histories.add(record);
        println!("{:?}",record);
    }
    Ok(())
}

fn main_menu() {
    fn show() {
        println!("");
        println!("=== Mini Pos ===");
        println!("1. Input Purchase");
        println!("2. Process Order");
        println!("3. View History");
        println!("4. (DEBUG) View All Purchases");
        println!("");
        println!("Enter Selection");
    }
    
    let mut purchases = Purchases::new();//scan purchase .csv
    let mut histories = Histories::new();//scan history.csv

    loop{
        show();
        let input = match get_input(){
            Some(input) => input,
            None => return
        };

        match input. as_str() {
            // "1" => ,
            // "2" => ,
            "3" => read_history(&mut histories),
            "4" => read_purchase(&mut purchases),
            _ => break,
        };
    }

}
fn main() {
    
}