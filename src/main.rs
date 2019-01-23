use std::f64;
use std::io;
use std::io::Write;
//use gtk::*;
//use gtk::WidgetExt;

struct Account {
    classification: AccountType,
    assets: Vec<Asset>
}

struct Allocation {
    assets: Vec<Asset>
}
#[derive(PartialEq)]
enum AssetClass {
    Domestic,
    International,
    Bond,
    Cd,
    RealEstate,
}

struct Asset {
    class: AssetClass,
    value: f64
}

impl Account {
    fn new(a_type: AccountType) -> Account {
        Account { 
            classification: a_type,
            assets: Vec::new(), 
        }
    }
    fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);
    }
    fn get_value(&self) -> f64 {
        let mut x = 0.0;
        for i in &self.assets {
            x = x + i.value;
        }
        return x;
    }
    fn is_empty(&self) -> bool {
        return self.assets.is_empty()
    }
    fn get_asset_share(&self, class: AssetClass) -> f64 {
        let mut x = 0.0;
        for i in &self.assets {
            if i.class == class {
                x = x + i.value;
            }
        }
        return x / self.get_value();
    }

}

impl Allocation {
    fn new() -> Allocation {
        Allocation { assets: Vec::new() }
    }
    fn allocate(&mut self, asset: Asset) {
        &mut self.assets.push(asset);
    }
    fn get_allocated_amount(&mut self) -> f64 {
        let mut x = 0.0;
        for i in &self.assets {
            x = x + i.value;
        }
        return x;
    }
    fn is_valid_allocation(&mut self) -> Result<&Allocation, String> {
        let x = self.get_allocated_amount();
        if x != 100.0 {
            let reason = format!("Allocation must sum to {}",  x);
            return Err(reason);
        }
        return Ok(self);
    }
        
}

enum AccountType {
    Traditional,
    Taxable,
    Roth,
    Educational
}

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

#[test]
fn get_total_account_value() {
    let mut account  = Account::new(AccountType::Taxable);
    let domestic = Asset {
        class: AssetClass::Domestic,
        value: 50.00,
    };
    let intl = Asset {
        class: AssetClass::International,
        value: 50.00,
    };
    account.add_asset(domestic);
    account.add_asset(intl);
    assert_eq!(100.00, account.get_value());
}

#[test]
fn get_asset_allocation_by_asset_type() {
    let mut account  = Account::new(AccountType::Taxable);
    let domestic = Asset {
        class: AssetClass::Domestic,
        value: 600.00,
    };
    let intl = Asset {
        class: AssetClass::International,
        value: 400.00,
    };
    account.add_asset(domestic);
    account.add_asset(intl);
    assert_eq!(0.6, account.get_asset_share(AssetClass::Domestic));
    println!("goodbye cruel world:");
    assert_eq!(0.4, account.get_asset_share(AssetClass::International));
}

#[test]
fn checks_account_is_empty() {
    let account  = Account::new(AccountType::Taxable);
    assert!(account.is_empty());
}

#[test]
fn check_account_is_not_empty() {
    let mut account  = Account::new(AccountType::Taxable);
    let domestic = Asset {
        class: AssetClass::Domestic,
        value: 50.00,
    };
    account.add_asset(domestic);
    assert!(!account.is_empty());
}

#[test]
fn desired_asset_allocation_sums_to_100() {
    let mut target_allocation = Allocation::new();
    let mut allocated = 0.0;
    let dom = Asset {
        class: AssetClass::Domestic,
        value: 35.0,
    };
    let intl = Asset {
        class: AssetClass::International,
        value: 25.0,
    };
    let bond = Asset {
        class: AssetClass::Bond,
        value: 40.0,
    };
    target_allocation.allocate(dom);
    target_allocation.allocate(intl);
    target_allocation.allocate(bond);
    match target_allocation.is_valid_allocation() {
        Ok(_) => assert!(true),
        Err(why) => panic!("{:?}", why),
    }
}

#[test]
fn allocation_complains_if_assets_dont_sum_to_100() {
    let mut target_allocation = Allocation::new();
    let dom = Asset {
        class: AssetClass::Domestic,
        value: 1.0,
    };
    let intl = Asset {
        class: AssetClass::International,
        value: 2.0,
    };
    let bond = Asset {
        class: AssetClass::Bond,
        value: 3.0,
    };
    target_allocation.allocate(dom);
    target_allocation.allocate(intl);
    target_allocation.allocate(bond);

    match target_allocation.is_valid_allocation() {
        Ok(_) => panic!("Asset allocation not equal to 100.0 should be invalid"),
        Err(why) => assert!(true, "{}", why),
    }
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

    let taxable_dom = get_portfolio_value("taxable domestic");
    let taxable_intl = get_portfolio_value("taxable international");
    let traditional = get_portfolio_value("401k");
    let roth = get_portfolio_value("Roth");
    let taxable_domestic = Asset {
        class: AssetClass::Domestic,
        value: taxable_dom,
    };
    let taxable_international = Asset {
        class: AssetClass::International,
        value: taxable_intl,
    };
    let mut taxable_portfolio = Account::new(AccountType::Taxable);
    taxable_portfolio.add_asset(taxable_domestic);
    taxable_portfolio.add_asset(taxable_international);

    if !taxable_portfolio.is_empty() {
        println!("Your taxable investment accout is worth ${}", 
                 taxable_portfolio.get_value());
        println!("Your taxable investment account is {}% domestic", 
                 100.0*taxable_portfolio.get_asset_share(AssetClass::Domestic));
        println!("Your taxable investment account is {}% international", 
                 100.0*taxable_portfolio.get_asset_share(AssetClass::International));
    }
    println!("Your traditional investment accout is worth ${}", traditional);
    println!("Your roth investment accout is worth ${}", roth);
    /*if gtk::init().is_err() {
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
    gtk::main();*/

}
