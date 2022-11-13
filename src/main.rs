use std::collections::HashMap;
use thiserror::Error;
use std::path::PathBuf;
use std::io::{Read, Write};

struct History {
    id: i64,
    name: String,
    price: i32, //Total price dari name yang sama
    discount: i32, //input manual, desimal dari 0 sampe 1 
    payment: i32,//input manual
    change: i32,//Payment - total, ngk boleh negatif.
}

struct Histories {
    histories: HashMap<i64,History>,
}
impl Histories {
    fn new() -> Self { Self { histories: HashMap::new() } }

    fn add(&mut self, history: History) {
        self.histories.insert(history.id, history);
    }

    fn next_id(&self) -> i64 {
        let mut ids: Vec<_> = self.histories.keys().collect();
        ids.sort();
        match ids.pop() {
            Some(id) => id + 1,
            None => 1
        }
    }

    fn into_vec(mut self) -> Vec<History> {
        let mut reps: Vec<_> = self.histories.drain().map(|kv| kv.1).collect();
        reps.sort_by_key(|rep| rep.id);
        reps
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("id must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError),
    #[error("Empty record")]
    EmptyRecord,
    #[error("Missing field: {0}")]
    MissingField(String),
}

fn parse_history(history: &str) -> Result<History, ParseError> {
    let strings: Vec<&str> = history.split(',').collect();

    let id = match strings.first() {
        Some(id) => id.parse::<i64>()?,
        None => return Err(ParseError::EmptyRecord),
    };

    let name = match strings.get(1) {
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField("name".to_owned())),
    };

    let price = match strings.get(2) {
        Some(price) => price.parse::<i32>()?,
        None => return Err(ParseError::MissingField("price".to_owned())),
    };

    let discount = match strings.get(3) {
        Some(discount) => discount.trim().parse::<i32>()?,
        None => return Err(ParseError::MissingField("discount".to_owned())),
    };

    let payment = match strings.get(4) {
        Some(payment) => payment.parse::<i32>()?,
        None => return Err(ParseError::MissingField("payment".to_owned())),
    };

    let change = match strings.get(5) {
        Some(change) => change.parse::<i32>()?,
        None => return Err(ParseError::MissingField("change".to_owned())),
    };

    Ok(History { id, name, price, discount, payment, change })
}

fn parse_histories(histories: String, verbose: bool) -> Histories{
    let mut hists = Histories::new();

    for(i, history) in histories.split('\n').enumerate() {
        if history.is_empty() {
            continue;
        }
        match parse_history(history) {
            Ok(hist) => hists.add(hist),
            Err(err) => {
                if verbose {
                    println!("Error on {}: {} - {}\n", i+1, err, history);
                }
            }
        }
    }
    hists
}

fn load_histories(verbose: bool) -> std::io::Result<Histories> {
    let mut file = std::fs::File::open(PathBuf::from("bin/History.csv"))?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_histories(buffer, verbose))
}

fn save_histories(histories: Histories) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new().write(true).truncate(true).open(PathBuf::from("bin/History.csv"))?;

    file.write_all( b"id,name,price,discount,payment,change\n")?;

    file.flush()?;

    for history in histories.into_vec().into_iter() {
        file.write_all(format!("{},{},{},{},{}.{}\n", history.id, history.name, history.price, history.discount, history.payment, history.change).as_bytes())?;
    }

    file.flush()?;

    Ok(())
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
