use std::io::{self, Write};

pub fn get_string (descriptor: &str) -> String {
    print!("Input the value of your {}: ", descriptor);
    io::stdout().flush().unwrap();
    let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");
    return val.trim().to_string();
}

pub fn parse_value (value: &str) -> Result<f64, &str> {
    let value: f64 = value.trim().parse().unwrap_or(-1.0);
    if value > 0.0 {
        Ok((value*100.0).round()/100.0)
    } else {
        Err("Input must be a positive number")
    }
}

#[test]
fn portfolio_value_is_positive() {
    let value = "-1";
    match parse_value(value) {
        Ok(_) => panic!("Negative values should return an error"),
        Err(_) => assert!(true)
    };
}

#[test]
fn portfolio_value_matches_input() {
    let value = "1.23";
    let val: f64 = 1.23;
    let res = match parse_value(value) {
        Ok(num) => num,
        Err(why) => panic!("{:?} 1.23 should parse", why)
    };
    assert_eq!(res, val, "Returns incorrect value");
}

#[test]
fn portfolio_value_is_rounded_to_cents() {
    let value = "1.234";
    let val: f64 = 1.23;
    let res = match parse_value(value) {
        Ok(num) => num,
        Err(why) => panic!("{:?}", why)
    };
    assert_eq!(res, val, "Result should round to two decimal places");
}

#[test] fn portfolio_value_fails_to_parse_letters() {
    let value = "abc";
    assert!(parse_value(value).is_err());
}
