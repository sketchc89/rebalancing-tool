use std::f64;
use std::io;
use std::io::Write;
use gtk::*;
use gtk::WidgetExt;

#[test]
fn portfolio_value_is_positive() {
    let value = "-1";
    match parse_portfolio_value(value) {
        Ok(_) => panic!("Negative values should return an error"),
        Err(_) => assert!(true)
    };
}

#[test]
fn portfolio_value_matches_input() {
    let value = "1.23";
    let val: f64 = 1.23;
    let res = match parse_portfolio_value(value) {
        Ok(num) => num,
        Err(why) => panic!("{:?} 1.23 should parse", why)
    };
    assert_eq!(res, val, "Returns incorrect value");
}

#[test]
fn portfolio_value_is_rounded_to_cents() {
    let value = "1.234";
    let val: f64 = 1.23;
    let res = match parse_portfolio_value(value) {
        Ok(num) => num,
        Err(why) => panic!("{:?}", why)
    };
    assert_eq!(res, val, "Result should round to two decimal places");
}

#[test]
fn portfolio_value_fails_to_parse_letters() {
    let value = "abc";
    assert!(parse_portfolio_value(value).is_err());
}

fn parse_portfolio_value
    (value: &str) 
    -> Result<f64, &str> {
    let value: f64 = value.trim().parse().unwrap_or(-1.0);
    if value > 0.0 {
        Ok((value*100.0).round()/100.0)
    } else {
        Err("Input must be a positive number")
    }
}

fn get_portfolio_value (name: &str) -> f64 {
    let value:f64 = loop {
        print!("Input the value of your {} investment account: ", name);
        io::stdout().flush().unwrap();
        let mut val = String::new();
        io::stdin().read_line(&mut val)
            .expect("Failed to read line");
        match parse_portfolio_value(&val) {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };
    };
    return value;
}

fn main() {

    let taxable = get_portfolio_value("taxable");
    let traditional = get_portfolio_value("401k");
    let roth = get_portfolio_value("Roth");
    println!("Your taxable investment accout is worth ${}", taxable);
    println!("Your traditional investment accout is worth ${}", traditional);
    println!("Your roth investment accout is worth ${}", roth);
    if gtk::init().is_err() {
        panic!("Failed to initialize GTK");
    }
    let glade_src = include_str!("builder_basics.glade");
    let builder = gtk::Builder::new_from_string(glade_src);
    let window: gtk::Window = builder.get_object("window1").unwrap();
    let button: gtk::Button = builder.get_object("button1").unwrap();
    let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();
    button.connect_clicked(move |_| {
        dialog.run();
        dialog.hide();
    });
    window.show_all();
    gtk::main();

}
