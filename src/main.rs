use std::num::ParseFloatError;
use std::error::Error;
use std::fmt;
//use std::io;

#[derive(Debug)]
struct NegRangeError {
    details: String
}

impl NegRangeError {
    fn new(msg: &str) -> NegRangeError {
        NegRangeError{details: msg.to_string()}
    }
}

impl fmt::Display for NegRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for NegRangeError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseFloatError> for NegRangeError {
    fn from(err: ParseFloatError) -> Self {
        NegRangeError::new(err.description())
    }
}

#[test]
fn portfolio_value_is_positive() {
    let value = String::from("-1");
    match parse_portfolio_value(value) {
        Ok(_) => panic!("Negative values should return an error"),
        Err(_) => assert!(true)
    };
}

#[test]
fn portfolio_value_matches_input() {
    let value = String::from("1.23");
    let val: f64 = 1.23;
    let res = match parse_portfolio_value(value) {
        Ok(num) => num,
        Err(why) => panic!("{:?} 1.23 should parse", why)
    };
    assert_eq!(res, val, "Returns incorrect value");
}

#[test]
fn portfolio_value_is_rounded_to_cents() {
    let value = String::from("1.234");
    let val: f64 = 1.23;
    let res = match parse_portfolio_value(value) {
        Ok(num) => num,
        Err(why) => panic!("{:?}", why)
    };
    assert_eq!(res, val, "Result should round to two decimal places");
}

fn parse_portfolio_value
    (value: String) 
    -> Result<f64, NegRangeError> {
    let value: f64 = value.trim().parse()
        .expect("Input must be a positive number");
    if value < 0.0 {
        Err(NegRangeError::new("Portfolio value must be positive"))
    } else {
        Ok((value*100.0).round()/100.0)
    }
}

fn main() {
    println!("Hello, world!");

/*    let mut taxable_value = String::new();

    io::stdin().read_line(&mut taxable_value)
        .expect("Failed to read line");
    println!("Your taxable investment accout is worth ${}", taxable_value);*/
}
