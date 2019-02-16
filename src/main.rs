mod utils;
mod user;
mod account;
mod asset;
//use asset::{Asset, AssetClass};
//use account::{Account, AccountType};
//use user::User;

//mod enum_learning;




// Naive way - don't account for current assets
// calculate new asset amounts
// 1. Total user value
// 2. Multiply by target asset allocation
// 3. Real Estate: Place in Roth. If Roth full then place in traditional. If traditional full then
//    place in taxable.
// 4. Bond: Place in traditional. If traditional full then place in roth. If roth full then place
//    in taxable.
// 5. Add together domestic and international. Fill accounts back to previous totals with half
//    domestic and half international.






/*#[test]
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
  let mut target_allocation = Account::new();
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
  target_allocation.add_asset(dom);
  target_allocation.add_asset(intl);
  target_allocation.add_asset(bond);
  match target_allocation.is_valid_allocation() {
  Ordering::Equal => assert!(true),
  Ordering::Less => panic!("Account sums to 100 but returned less"),
  Ordering::Greater => panic!("Account sums to 100 but returned greater"),
  }
  }*/

/*#[test]
  fn allocation_complains_if_assets_dont_sum_to_100() {
  let mut target_allocation = Account::new();
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
  target_allocation.add_asset(dom);
  target_allocation.add_asset(intl);
  target_allocation.add_asset(bond);

  match target_allocation.is_valid_allocation() {
  Ordering::Equal => panic!("Account should be less than 100 but returned equal"),
  Ordering::Less => assert!(true),
  Ordering::Greater => panic!("Account should be less than 100 but returned greater"),
  }
  }*/

/*#[test]
  fn allocation_says_whether_over_100() {
  let mut target_allocation = Account::new();
  let dom = Asset {
  class: AssetClass::Domestic,
  value: 100.0,
  };
  let intl = Asset {
  class: AssetClass::International,
  value: 200.0,
  };
  let bond = Asset {
  class: AssetClass::Bond,
  value: 300.0,
  };
  target_allocation.add_asset(dom);
  target_allocation.add_asset(intl);
  target_allocation.add_asset(bond);

  match target_allocation.is_valid_allocation() {
  Ordering::Equal => panic!("Account is greater than 100 but returned equal"),
  Ordering::Greater => assert!(true),
  Ordering::Less => panic!("Account is less than 100 but returned less"),
  }
  }*/



/*fn get_portfolio_value (name: &str) -> f64 {
  let value:f64 = loop {
  print!("Input the value of your {} investment account: ", name);
  io::stdout().flush().unwrap();
  let mut val = String::new();
  io::stdin().read_line(&mut val)
  .expect("Failed to read line");
  match parse_value(&val) {
  Ok(num) => break num,
  Err(_) => {
  println!("Invalid input.");
  continue;
  }
  };
  };
  return value;
  }*/

fn main() {

    //println!("{}", &format!("{:a^20}", AssetClass::Domestic));

    let first = utils::get_string("first name");
    let last = utils::get_string("last name");
    let mut user = user::User::new(&first, &last);
    user.request_action();
}
