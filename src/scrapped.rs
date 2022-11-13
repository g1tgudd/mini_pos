fn parse_purchase (purchase: &str) -> Result<Purchase, ParseError> {
    let fields:Vec<&str> = purchase.split(',').collect();

    let id = match fields.get(0) {
        Some(id) => i64::from_str_radix(id, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };
    let name = match fields.get(1).filter(|name| **name !="") {
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField("name".to_owned())),
    };
    let item = match fields.get(2).filter(|name| **name !="") {
        Some(item) => item.to_string(),
        None => return Err(ParseError::MissingField("item".to_owned())),
    };
    let quantity = match fields.get(3) {
        Some(quantity) => i32::from_str_radix(quantity, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };
    let price = match fields.get(4) {
        Some(price) => i32::from_str_radix(price, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };
    return Ok(Purchase {id, name, item, quantity, price})
}