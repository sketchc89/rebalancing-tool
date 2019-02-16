extern crate separator;
use separator::FixedPlaceSeparatable;
use std::f64;
use std::io;
use std::io::Write;
use std::fmt;
//mod enum_learning;

struct User {
    fname: String,
    lname: String,
    accounts: Vec<Account>,
    allocation: Account,
    target: Account,
}

#[derive(Clone)]
enum AccountType {
    Traditional,
    Taxable,
    Roth,
    Allocation,
}

struct Account {
    classification: AccountType, 
    assets: Vec<Asset>
}

#[derive(Clone, PartialEq)]
enum AssetClass {
    Domestic,
    International,
    Bond,
    //Cd,
    RealEstate,
}

struct Asset {
    class: AssetClass,
    value: f64
}

/// Assets are considered equal if they share the same asset class
///
/// # Examples
///
/// ```
/// let asset = Asset::new(AssetClass::Domestic, 100.00);
/// let other = Asset::new(AssetClass::Domestic, 555.55);
/// assert_eq!(asset, other);
/// ```
impl PartialEq for Asset {
    fn eq(&self, other: &Asset) -> bool {
        match (&self.class, &other.class) {
            (AssetClass::Domestic, AssetClass::Domestic) => true,
            (AssetClass::International, AssetClass::International) => true,
            (AssetClass::Bond, AssetClass::Bond) => true,
            (AssetClass::RealEstate, AssetClass::RealEstate) => true,
            (_,_) => false,
        }
    }
}
/// Assets are considered equal to their asset class
///
/// # Examples
///
/// ```
/// let asset = Asset::new(AssetClass::Domestic, 100.00);
/// assert_eq!(asset, AssetClass::Domestic);
/// ```
impl PartialEq<AssetClass> for Asset {
    fn eq(&self, other: &AssetClass) -> bool {
        match (&self.class, other) {
            (AssetClass::Domestic, AssetClass::Domestic) => true,
            (AssetClass::International, AssetClass::International) => true,
            (AssetClass::Bond, AssetClass::Bond) => true,
            (AssetClass::RealEstate, AssetClass::RealEstate) => true,
            (_,_) => false,
        }
    }
}

impl Asset {
    /// Creates an Asset given an AssetClass and the amount of money invested in that Asset
    fn new(class: AssetClass, value: f64) -> Asset {
        Asset {
            class,
            value,
        }
    }

    /// Subtracts another asset of the same class from the asset
    /// 
    /// # Examples
    ///
    /// ```
    /// let asset = Asset::new(AssetClass::Domestic, 50.0);
    /// let other = Asset::new(AssetClass::Domestic, 10.0);
    /// let actual = asset.subtract_asset(other);
    /// assert_eq!(actual, Asset::new(AssetClass::Domestic, 40.0);
    /// ```
    fn subtract_asset(&self, other: &Asset) -> Option<Asset> {
        if self == other {
            Some(Asset::new(self.class.clone(), self.value - other.value))
        }
        else {
            None
        }
    }

    /// Subtracts value from asset
    /// # Examples
    ///
    /// ```
    /// let asset = Asset::new(AssetClass::Domestic, 50.0);
    /// let val = 10.0;
    /// let actual = asset.subtract_value(val);
    /// assert_eq!(actual, Asset::new(AssetClass::Domestic, 40.0);
    /// ```
    fn subtract_value(&mut self, val: f64) {
        self.value -= val;
    }
}

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

impl User {
    /// Creates a new user given a first name and last name
    fn new(fname: &str, lname: &str) -> User {
        User {
            fname: fname.to_string(),
            lname: lname.to_string(),
            accounts: Vec::new(),
            allocation: Account::new(AccountType::Allocation),
            target: Account::new(AccountType::Allocation),
        }
    }

    /// Adds an account to users set of accounts, then recalculates the users account and asset allocation
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
        self.current_allocation();
    }

    /// Sets a users target allocation to match the given account. 
    /// The account must have a value of 100.00
    fn target_allocation(&mut self, allocation: Account) -> Result<(), String> {
        let total = allocation.get_total_value();
        if total == 100.00 {
            self.target = allocation;
            Ok(())
        } else {
            Err(format!("Allocation should be 100.00, but was {}", total))
        }
    }

    /// Ask user of the program what action they would like to perform for the User
    fn request_action(&mut self) {
        loop {
            println!("What would you like to do?");
            println!("1. Change target allocation\t2. Add account\t3. Display user info\t4. Display off target summary\t5. Quit");
            let mut action = String::new();
            io::stdin().read_line(&mut action)
                .expect("Failed to read line");
            let choice: u8 = action.trim().parse().unwrap_or(0);
            match choice {
                1 => match request_allocation() {
                    Ok(allocation) => match self.target_allocation(allocation) {
                        Ok(()) => println!("Successfully set target allocation"),
                        Err(why) => println!("{:?}", why),
                    }
                    Err(why) => println!("{}", why)
                }
                2 => match setup_new_account() {
                    Ok(account) => self.add_account(account),
                    Err(why) => println!("{}", why),
                }
                3 => println!("{}", self),
                4 => self.display_allocation_diff(),
                5 => break,
                _ => continue,
            }

        }
    }

    /// Displays the difference between the current asset allocation of the user and their target.
    /// Positive indicates the user needs to increase the value of those assets to meet their target
    /// Negative indicates the user needs to reduce the value of those assets to meet their target.
    fn display_allocation_diff(&mut self) {
        loop {
            println!("In (1) $ or (2) % ?");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice)
                .expect("Failed to read line");
            let choice: u8 = choice.trim().parse().unwrap_or(0);
            let mut diff = self.allocation.diff(&self.target); 
            println!("The user's accounts differ from the target allocation by: (+) too high, (-) too low");
            match choice {
                1 => { diff.change_account_classification(AccountType::Taxable);
                    println!("{}", diff.multiply(0.01*self.get_total_value())); 
                    break; },
                2 => { 
                    println!("{}", diff); 
                    break; },
                _ => continue,
            }
        }
    }

    /// Returns the total combined value of all of the user's accounts 
    fn get_total_value(&self) -> f64 {
        let mut total = 0.0;
        for i in &self.accounts {
            total += i.get_total_value();
        }
        return total;
    }

    /// Display the account allocation of the user
    fn display_account_allocation(&self) -> String {
        let mut tax = 0.0;
        let mut trad = 0.0;
        let mut roth = 0.0;
        for i in &self.accounts {
            match i.classification {
                AccountType::Taxable => tax = tax + i.get_total_value(),
                AccountType::Traditional => trad = trad + i.get_total_value(),
                AccountType::Roth => roth += i.get_total_value(),
                _ => continue,
            }
        }
        let mut disp = String::new();
        disp.push_str(&format!("Taxable:        ${:>15}{:>8.2} %\n", tax.separated_string_with_fixed_place(2),  100.0*tax/self.get_total_value()));
        disp.push_str(&format!("Traditional:    ${:>15}{:>8.2} %\n", trad.separated_string_with_fixed_place(2), 100.0*trad/self.get_total_value()));
        disp.push_str(&format!("Roth:           ${:>15}{:>8.2} %\n", roth.separated_string_with_fixed_place(2), 100.0*roth/self.get_total_value()));
        disp
    }

    fn get_asset_value(&self, class: &AssetClass) -> f64 {
        let mut value = 0.0;
        for account in &self.accounts {
            for asset in &account.assets {
                if asset == class {
                    value += asset.value;
                }
            }
        }
        return value;
    }

    fn get_asset_share(&self, class: &AssetClass) -> f64 {
        return 100.0*self.get_asset_value(class) / self.get_total_value();
    }

    fn current_allocation(&mut self) {
        let dom = Asset::new(AssetClass::Domestic, 
                             self.get_asset_share(&AssetClass::Domestic));
        let int = Asset::new(AssetClass::International, 
                             self.get_asset_share(&AssetClass::International));
        let bnd = Asset::new(AssetClass::Bond, 
                             self.get_asset_share(&AssetClass::Bond));
        //let cds = Asset::new(AssetClass::Cd, 
        //self.get_asset_share(AssetClass::Cd));
        let rle = Asset::new(AssetClass::RealEstate, 
                             self.get_asset_share(&AssetClass::RealEstate));

        let mut cur = Account::new(AccountType::Allocation);
        cur.add_asset(dom);
        cur.add_asset(int);
        cur.add_asset(bnd);
        //cur.add_asset(cds);
        cur.add_asset(rle);
        self.allocation = cur;
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Account) -> bool {
        match (&self.classification, &other.classification) {
            (AccountType::Taxable, AccountType::Taxable) => true,
            (AccountType::Traditional, AccountType::Traditional) => true,
            (AccountType::Roth, AccountType::Roth) => true,
            (_,_) => false,
        }
    }
}

impl PartialEq<AccountType> for Account {
    fn eq(&self, other: &AccountType) -> bool {
        match (&self.classification, other) {
            (AccountType::Taxable, AccountType::Taxable) => true,
            (AccountType::Traditional, AccountType::Traditional) => true,
            (AccountType::Roth, AccountType::Roth) => true,
            (_,_) => false,
        }
    }
}

impl Account {
    fn new(classification: AccountType) -> Account {
        Account { 
            classification,
            assets: vec![Asset::new(AssetClass::Domestic, 0.0),
            Asset::new(AssetClass::International, 0.0),
            Asset::new(AssetClass::Bond, 0.0),
            Asset::new(AssetClass::RealEstate, 0.0)]
        }
    }

    fn change_account_classification(&mut self, classification: AccountType) {
        self.classification = classification;
    }


    fn diff(&mut self, other: & Account) -> Account {
        let classification = self.classification.clone();
        let mut diff = Account::new(classification);
        for i in &self.assets {
            for j in &other.assets {
                if i == j {
                    diff.add_asset(Asset::new(i.class.clone(), i.value - j.value));
                }
            }
        }
        return diff;
    }

    fn multiply(&self, scalar: f64) -> Account {
        let classification = self.classification.clone();
        let mut mult = Account::new(classification);
        for i in &self.assets {
            match &i.class {
                AssetClass::Domestic => mult.add_asset(Asset::new(AssetClass::Domestic, i.value * scalar)),
                AssetClass::International => mult.add_asset(Asset::new(AssetClass::International, i.value * scalar)),
                AssetClass::Bond => mult.add_asset(Asset::new(AssetClass::Bond, i.value * scalar)),
                AssetClass::RealEstate => mult.add_asset(Asset::new(AssetClass::RealEstate, i.value * scalar)),
            }
        }
        return mult;
    }

    fn get_asset_value(&self, class: AssetClass) -> f64 {
        let mut value = 0.0;
        for asset in &self.assets {
            if asset == &class {
                value += asset.value;
            }
        }
        return value;
    }

    fn add_asset(&mut self, new_asset: Asset) {
        for asset in &mut self.assets {
            if &new_asset == asset {
                asset.value += new_asset.value;
                return;
            }
        }
        self.assets.push(new_asset)
    }

    fn remove_asset(&mut self, unwanted_asset: &Asset) -> Result<(), String> {
        for asset in &mut self.assets {
            if asset == unwanted_asset {
                if asset.value >= unwanted_asset.value {
                    asset.value -= unwanted_asset.value;
                    return Ok(());
                } else {
                    return Err(format!("Account only contains ${} of {}", asset.value, asset.class)); 
                }
            }
        }
        return Err(format!("Account does not contain assets of type {}", unwanted_asset.class));
    }

    fn move_asset_class_to(&mut self, other: &mut Account, class: &AssetClass) -> Result<(), String> {
        let asset = Asset::new(class.clone(), self.get_asset_value(class.clone()));
        let res = match self.remove_asset(&asset) {
            Ok(()) => Ok(()),
            Err(why) => Err(format!("Failed to move assets of type {}: {:?}", class.clone(), why)),
        };
        if res.is_ok() {
            other.add_asset(asset);
        }
        res
    }

    // Add amount to account. If less than account limit then add asset. If more then return amount
    // remaining
    fn add_to_limit(&mut self, asset: Asset, limit: f64) -> Option<Asset> {
        if self.fits(&asset, limit) {
            self.add_asset(asset);
            None
        } else {
            let space_filler = Asset::new(asset.class.clone(), limit - self.get_total_value());
            let leftover = match asset.subtract_asset(&space_filler) {
                Some(asset) => asset,
                None => panic!("Adding to limit failed because asset types did not match"),
            };
            self.add_asset(space_filler);
            Some(leftover)
        }
    }


    fn fits(&self, asset: &Asset, limit: f64) -> bool {
        asset.value <= limit - self.get_total_value()
    }


    fn move_asset(&mut self, other: &mut Account, asset: Asset) -> Result<(), String>{
        let res = match self.remove_asset(&asset) {
            Ok(()) => Ok(()),
            Err(why) => Err(format!("Failed to move asset: {}", why)),
        };
        if res.is_ok() {
            other.add_asset(asset);
        }
        res
    }

    fn swap_asset(&mut self, src: AssetClass, dst: AssetClass, amount: f64) -> Result<(), String> {
        let old_asset = Asset::new(src, amount);
        let res = match self.remove_asset(&old_asset) {
            Ok(()) => Ok(()),
            Err(why) => Err(format!("Failed to remove asset {}: {:?}", old_asset, why)),
        };
        let new_asset = Asset::new(dst, amount);
        if res.is_ok() {
            self.add_asset(new_asset);
        }
        res
    }

    fn get_total_value(&self) -> f64 {
        let mut x = 0.0;
        for i in &self.assets {
            x = x + i.value;
        }
        return x;
    }
}


impl fmt::Display for AssetClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AssetClass::Domestic => "U.S.A.".fmt(f),
            AssetClass::International => "International".fmt(f),
            AssetClass::Bond => "Bonds".fmt(f),
            //AssetClass::Cd => "CDs".fmt(f),
            AssetClass::RealEstate => "Real Estate".fmt(f),
        }
    }
}

            impl fmt::Display for Asset {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}", &format!("Asset Class: {:<15}{:>12}", self.class, self.value.separated_string_with_fixed_place(2)))
                }
            }
impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AccountType::Traditional => "IRA / 401(k)".fmt(f),
            AccountType::Roth => "Roth IRA / Roth 401(k)".fmt(f),
            AccountType::Taxable => "Brokerage Account".fmt(f),
            AccountType::Allocation => "Allocation".fmt(f),
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut units = '$';
        if let AccountType::Allocation = self.classification {
            units = '%';
        }

        let mut disp = "Account Classification: ".to_string();
        disp.push_str(&format!("{}\n", self.classification));
        for i in &self.assets {
            disp.push_str(&format!("{} {}\n", i, units));
        }
        disp.fmt(f)
    }
}

impl fmt::Display for User {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        let mut disp = "\nName: ".to_string();
        disp.push_str(&format!("{} {}\n", self.fname, self.lname));
        for i in &self.accounts {
            disp.push_str(&format!("{}\n", i));
        }
        disp.push_str(&format!("Target {}\n", self.target));
        disp.push_str(&format!("Current {}\n", self.allocation));
        disp.push_str(&format!("{}", self.display_account_allocation()));
        disp.fmt(f)
    }
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

/*#[test]
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

fn request_allocation()-> Result<Account, String> {
    let mut allocation = Account::new(AccountType::Allocation);
    loop {
        println!("Select the number of the asset class to add_asset");
        println!("1. Domestic\t2. International\t3. Bonds\t4. Real Estate\t5. Cancel");

        let mut class = String::new();
        io::stdin().read_line(&mut class)
            .expect("Failed to read line");
        println!("Percent (0-100) to add_asset to this asset");
        let class: u8 = class.trim().parse().unwrap_or(0);
        if class > 5 || class < 1 { 
            continue; 
        } else if class == 5 {
            return Err("Cancelled target allocation".to_string());
        }

        println!("Already added assets {}%", allocation.get_total_value());
        let mut value = String::new();
        io::stdin().read_line(&mut value)
            .expect("Failed to read line");
        let value: f64 = value.trim().parse().unwrap_or(0.0);
        println!("\n");
        match class {
            1 => allocation.add_asset(Asset::new(AssetClass::Domestic, value)),
            2 => allocation.add_asset(Asset::new(AssetClass::International, value)),
            3 => allocation.add_asset(Asset::new(AssetClass::Bond, value)),
            4 => allocation.add_asset(Asset::new(AssetClass::RealEstate, value)),
            _ => continue,
        }
        let total = allocation.get_total_value();
        if total < 100.0 {
            continue;
        } else if total > 100.0 {
            allocation = Account::new(AccountType::Allocation); 
            continue;
        } else {
            break;
        }

    }
    return Ok(allocation);
}

fn setup_new_account() -> Result<Account, String> {
    let account_type = loop {
        println!("What type of account would you like to setup?");
        println!("1. Taxable\t2. Traditional/401(k)\t3. Roth/Roth 401(k)\t4. Cancel");
        let mut account_type = String::new();
        io::stdin().read_line(&mut account_type)
            .expect("Failed to read line");
        let choice: u8 = account_type.trim().parse().unwrap_or(0);
        println!("\n");
        match choice {
            1 => break AccountType::Taxable,
            2 => break AccountType::Traditional,
            3 => break AccountType::Roth,
            4 => return Err("Cancelled account creation".to_string()),
            _ => continue,
        }
    };
    setup_account(account_type)
}

fn setup_account(account_type: AccountType) -> Result<Account, String> {
    let mut account = Account::new(account_type);
    loop {
        println!("What type of asset to account?");
        println!("1. Domestic\t2. International\t3. Bonds\t4. Real Estate\t5. Finish\t6. Cancel");
        let mut asset_class = String::new();
        io::stdin().read_line(&mut asset_class)
            .expect("Failed to read line");
        let choice: u8 = asset_class.trim().parse().unwrap_or(0);

        if choice == 5 { 
            break; 
        } else if choice == 6 {
            return Err("Cancelled account creation".to_string());
        } else if choice > 6 || choice < 1 {
            continue;
        }

        let mut value = String::new();
        println!("How much money would you like to put towards this asset class?");
        io::stdin().read_line(&mut value)
            .expect("Failed to read line");
        let value: f64 = match parse_portfolio_value(&value) {
            Ok(val) => val,
            Err(why) => {println!("{:?}", why); 
                0.0},
        };
        println!("\n");
        match choice {
            1 => account.add_asset(Asset::new(AssetClass::Domestic, value)),
            2 => account.add_asset(Asset::new(AssetClass::International, value)),
            3 => account.add_asset(Asset::new(AssetClass::Bond, value)),
            4 => account.add_asset(Asset::new(AssetClass::RealEstate, value)),
            _ => continue,
        }
    }
    return Ok(account);
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

/*fn get_portfolio_value (name: &str) -> f64 {
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
  }*/

fn get_string (descriptor: &str) -> String {
    print!("Input the value of your {}: ", descriptor);
    io::stdout().flush().unwrap();
    let mut val = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read line");
    return val.trim().to_string();
}

fn main() {

    //println!("{}", &format!("{:a^20}", AssetClass::Domestic));

    let first = get_string("first name");
    let last = get_string("last name");
    let mut user = User::new(&first, &last);
    user.request_action();
}
