use std::num::ParseFloatError;
use std::error::Error;
use std::fmt;
//use std::io;

#[derive(Debug)]
struct neg_range_error {
    details: String
}

impl neg_range_error {
    fn new(msg: &str) -> neg_range_error {
        neg_range_error{details: msg.to_string()}
    }
}

impl fmt::Display for neg_range_error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for neg_range_error {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseFloatError> for neg_range_error {
    fn from(err: ParseFloatError) -> Self {
        neg_range_error::new(err.description())
    }
}

#[test]
fn portfolio_value_is_positive() {
    let value = String::from("-1");
    match parse_portfolio_value(value) {
        Ok(_) => assert!(false),
        Err(neg_range_error) => assert!(true) 
    }
}

#[test]
fn portfolio_value_matches_input() {
    let value = String::from("1.23");
    match parse_portfolio_value(value) {
        Ok(1.23) => assert!(true),
        Ok(_) => assert!(false, "Returns incorrect value"),
        Err(_) => assert!(false, "String should parse")
    }
}

fn parse_portfolio_value
    (value: String) 
    -> Result<f64, neg_range_error> {
    let value: f64 = value.trim().parse()
        .expect("Input must be a positive number");
    if value < 0.0 {
        Err(neg_range_error::new("Portfolio value must be positive"))
    } else {
        Ok(value)
    }
}

fn main() {
    println!("Hello, world!");

/*    let mut taxable_value = String::new();

    io::stdin().read_line(&mut taxable_value)
        .expect("Failed to read line");
    println!("Your taxable investment accout is worth ${}", taxable_value);*/
}
