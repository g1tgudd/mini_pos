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

    fn get_all(&self) -> Vec<&Purchase> {
        let mut purchases = vec![];
        for purchase in self.inner.values() {
            purchases.push(purchase)
        }
        return purchases
    }
}

//Sub Menu --> Input purchase, add ke csv, add ke struct purchases.
fn add_purchase(purchases: &mut Purchases) {
    // let id: i64 = match get_price(){
    //     Some(id) => id + 1,
    //     None => return
    // };
    
    let id:i64 = purchases.next_id();

    println!("===== Input Name: ");
    let name: String = match get_input(){
        Some(name) => name,
        None => return
    };

    println!("===== Input Item: ");
    let item: String = match get_input(){
        Some(item) => item,
        None => return
    };

    println!("===== Input Quantity: ");
    let quantity: i32 = match get_price(){
        Some(quantity) => quantity,
        None => return
    };

    println!("===== Input Price: ");
    let price: i32 = match get_price(){
        Some(price) => price,
        None => return
    };

    //function untuk write ke Purchase.csv
    fn write_purchase(id_input:i64, name_input:&str, item_input:&str, quantity_input: i32, price_input: i32)-> Result<(), Box<dyn Error>>{
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("src/bin/Purchase.csv")
            .unwrap();

        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);
        
        writer.serialize(Purchase {
            id: id_input, //error not in scope next_id, make temp value.
            name: name_input.to_owned(),
            item: item_input.to_owned(),
            quantity: quantity_input,
            price: price_input,
        })?;

        writer.flush()?;
        Ok(())
    }
    if let Err(e) = write_purchase(id,name.trim(),item.trim(),quantity,price){
        eprintln!("{}",e);
    }

    let purchase: Purchase = Purchase { id, name, item, quantity, price };
    purchases.add(purchase); //add ke struct purchases. belum ke add ke csv.
    println!("Purchase Added.");
    
}


//function buat Parse Purchase.csv PARAM : .... 
fn read_purchase(purchases: &mut Purchases) -> Result<(&mut Purchases), Box<dyn Error>> {
    
    let mut reader = csv::Reader::from_path("src/bin/Purchase.csv")?;

    for result in reader.deserialize() {
        let record: Purchase = result?;
        // println!("DEBUG {:?}", &record);
        purchases.add(record);
    }
    for purchase in purchases.get_all() {
        println!("{:?}", purchase);
    }
    return Ok(purchases)
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
    fn get_all(&self) -> Vec<&History> {
        let mut histories = vec![];
        for history in self.inner.values() {
            histories.push(history)
        }
        return histories
    }
}

//NGK DIPAKE
fn view_histories(histories: &Histories) {
    for history in histories.get_all() {
        println!("{:?}", history);
    }
}

fn process_transaction(purchases: &Purchases, histories: &mut Histories) {
    
    println!("===== Input Customer Name: ");
    let name: String = match get_input(){
        Some(name) => name,
        None => return
    };
    
    let purchases_search = purchases.search(&name);
    let mut total = 0;
    let mut change = 0;

    for purchase in purchases_search.iter() {
        // println!("DEBUG DATA HASIL SEARCH {:?}",purchase);
        total = total + purchase.price;
    }

    println!("Total Charge for {:?} : {:?}",name, total);
    

    let id:i64 = histories.next_id();

    println!("===== Input Discount (From Total): ");
    let discount: i32 = match get_price(){
        Some(discount) => discount,
        None => return
    };

    println!("===== Input Payment : ");
    let payment: i32 = match get_price(){
        Some(payment) => payment,
        None => return
    };

    change = payment - (total-discount);
    println!("Your Change : {:?}", change);

    //function untuk write ke Purchase.csv
    fn write_history(id_input:i64, name_input:&str, total_input:i32, discount_input:i32, payment_input:i32, change_input:i32)-> Result<(), Box<dyn Error>>{
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("src/bin/History.csv")
            .unwrap();

        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);
        
        writer.serialize(History {
            id: id_input, //error not in scope next_id, make temp value.
            name: name_input.to_owned(),
            total: total_input,
            discount: discount_input,
            payment: payment_input,
            change: change_input,
        })?;

        writer.flush()?;
        Ok(())
    }


    if let Err(e) = write_history( 
            id, name.trim(), total, discount, payment, change){
                eprintln!("{}",e);
            };

    let history:History = History {
        id, name, total, discount, payment, change
    };
    histories.add(history);
    println!("Transaction added to History.");

}

//histories: &mut Histories (param read_history)
fn read_history(histories: &mut Histories) -> Result<(&mut Histories), Box<dyn Error>> {
    
    let mut reader = csv::Reader::from_path("src/bin/History.csv")?;

    for result in reader.deserialize() {
        let record: History = result?;
        // println!("DEBUG {:?}", &record);
        histories.add(record);
    }
    for history in histories.get_all() {
        println!("{:?}", history);
    }
    return Ok(histories)
}

fn main_menu() {
    fn show() {
        println!("");
        println!("=== Mini Pos ===");
        println!("By Henry Hamilton Prasetya & Richie Junior Soewito");
        println!("1. Input Purchase");
        println!("2. Process Order");
        println!("3. View History");
        println!("4. View All Purchases");
        println!("");
        println!("Enter Selection");
    }
    
    

    let mut purchases = Purchases::new();//scan purchase .csv
    let mut histories = Histories::new();//scan history.csv

    println!("Parsing Purchases...");
    if let Err(e) = read_purchase(&mut purchases){
        eprintln!("{}",e);
        }
    println!("Parsing Histories...");
    if let Err(e) = read_history(&mut histories){
        eprintln!("{}",e);
        }
    println!("Parsing Complete!");

    loop{
        show();
        let input = match get_input(){
            Some(input) => input,
            None => return
        };

        match input.as_str() {
            "1" => add_purchase(&mut purchases),
            "2" => process_transaction(&mut purchases, &mut histories),
            "3" => if let Err(e) = read_history(&mut histories){
                eprintln!("{}",e);
            },
            "4" => if let Err(e) = read_purchase(&mut purchases){
                eprintln!("{}",e);
            },
            _ => break,
        };
    }

}
fn main() {
    main_menu();
}