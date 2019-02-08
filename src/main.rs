use std::f64;
use std::io;
use std::io::Write;
use std::fmt;

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
    Educational,
    Allocation,
}

struct Account {
    classification: AccountType, 
    assets: Vec<Asset>
}

#[derive(PartialEq)]
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

impl Asset {
    fn new(class: AssetClass, value: f64) -> Asset {
        Asset {
            class,
            value,
        }
    }
}

impl User {
    fn new(fname: &str, lname: &str) -> User {
        User {
            fname: fname.to_string(),
            lname: lname.to_string(),
            accounts: Vec::new(),
            allocation: Account::new(AccountType::Allocation),
            target: Account::new(AccountType::Allocation),
        }
    }
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
        self.current_allocation();
    }
    fn target_allocation(&mut self, allocation: Account) {
        self.target = allocation;
    }

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
                    Ok(allocation) => self.target_allocation(allocation),
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

    fn display_allocation_diff(&mut self) {
        loop {
            println!("In (1) $ or (2) % ?");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice)
                .expect("Failed to read line");
            let choice: u8 = choice.trim().parse().unwrap_or(0);
            let diff = self.allocation.diff(&self.target); 
            match choice {
                1 => { println!("{}", diff.multiply(self.get_total_value()/100.0)); // TODO implement multiply account by value
                    break; },
                2 => { println!("{}", diff); 
                    break; },
                _ => continue,
            }
        }
    }

    fn get_total_value(&self) -> f64 {
        let mut dom = 0.0;
        let mut int = 0.0;
        let mut bnd = 0.0;
        //let mut cds = 0.0;
        let mut rle = 0.0;
        for i in &self.accounts {
            dom += i.get_asset_value(AssetClass::Domestic);
            int += i.get_asset_value(AssetClass::International);
            bnd += i.get_asset_value(AssetClass::Bond);
            //cds += i.get_asset_value(AssetClass::Cd);
            rle += i.get_asset_value(AssetClass::RealEstate);
        }
        return dom + int + bnd + rle; //+ cds 
    }

    fn user_asset_value(&self, class: AssetClass) -> f64 {
        let mut tot = 0.0;
        for i in &self.accounts {
            for j in &i.assets {
                if j.class == class {
                    tot += j.value;
                }
            }
        }
        return tot;
    }

    fn user_asset_share(&self, class: AssetClass) -> f64 {
        return 100.0*self.user_asset_value(class) / self.get_total_value();
    }

    fn current_allocation(&mut self) {
        let dom = Asset::new(AssetClass::Domestic, 
                             self.user_asset_share(AssetClass::Domestic));
        let int = Asset::new(AssetClass::International, 
                             self.user_asset_share(AssetClass::International));
        let bnd = Asset::new(AssetClass::Bond, 
                             self.user_asset_share(AssetClass::Bond));
        //let cds = Asset::new(AssetClass::Cd, 
                             //self.user_asset_share(AssetClass::Cd));
        let rle = Asset::new(AssetClass::RealEstate, 
                             self.user_asset_share(AssetClass::RealEstate));

        let mut cur = Account::new(AccountType::Allocation);
        cur.add_asset(dom);
        cur.add_asset(int);
        cur.add_asset(bnd);
        //cur.add_asset(cds);
        cur.add_asset(rle);
        self.allocation = cur;
    }

    /*fn add_asset_with_priority(&mut self, src_acc: Account, val: f64, priority: Vec<AccountType>) -> Account {
        println!("non-functional");
        src_acc
    }*/

    /*fn readd_asset_to_target(&mut self) {

        let target_account = self.target;
        println!("{}", target_account);

        let rel_priority: Vec<AccountType> = vec![AccountType::Roth, AccountType::Traditional, AccountType::Educational, AccountType::Taxable];
        let bnd_priority: Vec<AccountType> = vec![AccountType::Educational, AccountType::Traditional, AccountType::Roth, AccountType::Taxable];
        //let cds_priority: Vec<AccountType> = vec![AccountType::Educational, AccountType::Traditional, AccountType::Roth, AccountType::Taxable];
        let dom_priority: Vec<AccountType> = vec![AccountType::Roth, AccountType::Taxable, AccountType::Educational, AccountType::Traditional];
        let int_priority: Vec<AccountType> = vec![AccountType::Roth, AccountType::Taxable, AccountType::Educational, AccountType::Traditional];
        let target_rel = target_account.get_asset_value(AssetClass::RealEstate);
        let target_bnd = target_account.get_asset_value(AssetClass::Bond);
        let target_dom = target_account.get_asset_value(AssetClass::Bond);
        let target_int = target_account.get_asset_value(AssetClass::Bond);

        // first add_asset real estate
        let target_account = self.add_asset_with_priority(target_account, target_rel, rel_priority);
        
        // next add_asset bonds
        let target_account = self.add_asset_with_priority(target_account, target_bnd, bnd_priority); 

        // next add_asset domestic
        let target_account = self.add_asset_with_priority(target_account, target_dom, int_priority);

        // next add_asset international
        let target_account = self.add_asset_with_priority(target_account, target_int, dom_priority);
    }*/

}

impl Account {
    fn new(classification: AccountType) -> Account {
        Account { 
            classification,
            assets: Vec::new(), 
        }
    }
    // TODO fix diff function so that funds in one account and not another are displayed
    fn diff(&mut self, other: & Account) -> Account {
        let classification = self.classification.clone();
        let mut diff = Account::new(classification);
        for i in &self.assets {
            for j in &other.assets {
                match (&i.class, &j.class) {
                    (AssetClass::Domestic, AssetClass::Domestic) => 
                        diff.add_asset(Asset::new(AssetClass::Domestic, i.value - j.value)),
                    (AssetClass::International, AssetClass::International) => 
                        diff.add_asset(Asset::new(AssetClass::International, i.value - j.value)),
                    (AssetClass::Bond, AssetClass::Bond) => 
                        diff.add_asset(Asset::new(AssetClass::Bond, i.value - j.value)),
                    (AssetClass::RealEstate, AssetClass::RealEstate) => 
                        diff.add_asset(Asset::new(AssetClass::RealEstate, i.value - j.value)),
                    (_,__) => continue,
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
        let mut x = 0.0;
        for i in &self.assets {
            if i.class == class {
                x = x + i.value;
            }
        }
        return x;
    }

    /*fn get_asset_share(&self, class: AssetClass) -> f64 {
        let mut x = 0.0;
        for i in &self.assets {
            if i.class == class {
                x = x + i.value;
            }
        }
        return 100.0*x / self.get_total_value();
    }*/

    fn add_asset(&mut self, asset: Asset) {
        for i in &mut self.assets {
            if i.class == asset.class {
                i.value = i.value + asset.value;
                return;
            }
        }
        self.assets.push(asset)
    }

    fn get_total_value(&self) -> f64 {
        let mut x = 0.0;
        for i in &self.assets {
            x = x + i.value;
        }
        return x;
    }
    /*fn is_empty(&self) -> bool {
        return self.assets.is_empty()
    }*/
    /*fn get_asset_share(&self, class: AssetClass) -> f64 {
        let mut x = 0.0;
        for i in &self.assets {
            if i.class == class {
                x = x + i.value;
            }
        }
        return 100.0*x / self.get_total_value();
    }*/

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
        write!(f, "{}", &format!("Asset Class: {:<15}{:>9.2}", self.class, self.value))
    }
}
impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AccountType::Traditional => "IRA / 401(k)".fmt(f),
            AccountType::Roth => "Roth IRA / Roth 401(k)".fmt(f),
            AccountType::Taxable => "Brokerage Account".fmt(f),
            AccountType::Educational => "529 / Educational".fmt(f),
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
        println!("1. Taxable\t2. Traditional/401(k)\t3. Roth/Roth 401(k)\t4. Educational\t5. Cancel");
        let mut account_type = String::new();
        io::stdin().read_line(&mut account_type)
            .expect("Failed to read line");
        let choice: u8 = account_type.trim().parse().unwrap_or(0);
        println!("\n");
        match choice {
            1 => break AccountType::Taxable,
            2 => break AccountType::Traditional,
            3 => break AccountType::Roth,
            4 => break AccountType::Educational,
            5 => return Err("Cancelled account creation".to_string()),
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

    // TODO move calculating difference to own function

}
