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

#[derive(Clone, Copy)]
enum AccountType {
    Traditional,
    Taxable,
    Roth,
    Educational,
    Allocation,
    Target
}

struct Account {
    classification: AccountType, 
    assets: Vec<Asset>
}
/*struct Allocation { 
    assets: Vec<Asset>
}*/
#[derive(PartialEq)]
enum Asset {
    Domestic(f64),
    International(f64),
    Bond(f64),
    //Cd,
    RealEstate(f64),
}

impl Account {
    fn new(classification: AccountType) -> Account {
        Account { 
            classification,
            assets: Vec::new(), 
        }
    }
    fn diff(&mut self, other: &Account) -> Account {
        let class = self.classification.clone();
        let mut diff = Account::new(class);
        for i in &self.assets {
            for j in &other.assets {
                match (i, j) {
                    (Asset::Domestic, Asset::Domestic) => 
                        diff.add_asset(Asset::Domestic(i.value - j.value)),
                    (Asset::International, Asset::International) => 
                        diff.add_asset(Asset::International(i.value - j.value)),
                    (Asset::Bond, Asset::Bond) => 
                        diff.add_asset(Asset::Bond(i.value - j.value)),
                    //(Asset::Cd, Asset::Cd) => 
                        //diff.add_asset(Asset::Cd, i.value - j.value)),
                    (Asset::RealEstate, Asset::RealEstate) => 
                        diff.add_asset(Asset::RealEstate(i.value - j.value)),
                    (_,__) => continue,
                }
            }
        }
        return diff;
    }
    fn multiply_account_value(&self, val: f64) -> Account {
        let mut new_account = Account::new(self.classification);
        for i in &self.assets {
            new_account.add_asset(Asset::new(i.class, i.value * val));
        }
        return new_account;
    }


    fn get_asset_value(&self, class: Asset) -> f64 {
        let mut x = 0.0;
        for i in &self.assets {
            if i.class == class {
                x = x + i.value;
            }
        }
        return x;
    }
    fn add_asset(&mut self, asset: Asset) {
        for account_asset in &mut self.assets {
            match (account_asset, asset) {
                (Asset::Domestic(_), Asset::Domestic(val)) => { account_asset.add_value(val); return; }
                (Asset::International(_), Asset::International(val)) => account_asset.add_value(val),
                (Asset::Bond(_), Asset::Bond(val)) =>
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
    fn is_empty(&self) -> bool {
        return self.assets.is_empty()
    }
    fn get_asset_share(&self, class: Asset) -> f64 {
        return 100.0*self.get_asset_value(class) / self.get_total_value();
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

    fn target_values(&self) -> Account {
        let mut target_account = Account::new(AccountType::Target);
        let user_total = self.user_total();
        for i in &self.target.assets {
            let target_asset = i.clone();
            target_account.add_asset(Asset::new(target_asset.class, target_asset.value*user_total/100.0));
        }
        return target_account;
    }

    fn request_action(&mut self) {
        loop {
            println!("What would you like to do?");
            println!("1. Change target allocation\t2. Add account\t3. Display user info\t4. Display targets\t5. Quit");
            let mut action = String::new();
            io::stdin().read_line(&mut action)
                .expect("Failed to read line");
            let choice: u8 = action.trim().parse().unwrap_or(0);
            match choice {
                1 => self.target_allocation(request_allocation()),
                2 => self.add_account(setup_new_account()),
                3 => println!("{}", self),
                4 => self.display_allocation_diff(),
                5 => break,
                _ => continue,
            }

        }
    }

    fn display_allocation_diff(mut self) {
        loop {
            println!("In (1) $ or (2) % ?");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice)
                .expect("Failed to read line");
            let choice: u8 = choice.trim().parse().unwrap_or(0);
            let diff = self.allocation.diff(&self.target); 
            match choice {
                1 => { println!("{}", diff); 
                    break; },
                2 => { println!("{}", self.target_values());
                    break; },
                _ => continue,
            }
        }
    }

    fn user_total(&self) -> f64 {
        let mut dom = 0.0;
        let mut int = 0.0;
        let mut bnd = 0.0;
        //let mut cds = 0.0;
        let mut rle = 0.0;
        for i in &self.accounts {
            dom += i.get_asset_value(Asset::Domestic);
            int += i.get_asset_value(Asset::International);
            bnd += i.get_asset_value(Asset::Bond);
            //cds += i.get_asset_value(Asset::Cd);
            rle += i.get_asset_value(Asset::RealEstate);
        }
        return dom + int + bnd + rle; //+ cds 
    }

    fn user_asset_value(&self, class: Asset) -> f64 {
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

    fn user_asset_share(&self, class: Asset) -> f64 {
        return 100.0*self.user_asset_value(class) / self.user_total();
    }

    fn current_allocation(&mut self) {
        let dom = Asset::Domestic(self.user_asset_share(Asset::Domestic));
        let int = Asset::International(self.user_asset_share(Asset::International));
        let bnd = Asset::Bond(self.user_asset_share(Asset::Bond));
        //let cds = Asset::Cd, 
                             //self.user_asset_share(Asset::Cd));
        let rle = Asset::RealEstate(self.user_asset_share(Asset::RealEstate));

        let mut cur = Account::new(AccountType::Allocation);
        cur.add_asset(dom);
        cur.add_asset(int);
        cur.add_asset(bnd);
        //cur.add_asset(cds);
        cur.add_asset(rle);
        self.allocation = cur;
    }

    fn allocate_with_priority(&mut self, src_acc: Account, val: f64, priority: Vec<AccountType>) -> Account {
        println!("non-functional");
        src_acc
    }

    fn reallocate_to_target(&mut self) {

        let target_account = self.target_values();
        println!("{}", target_account);

        let rel_priority: Vec<AccountType> = vec![AccountType::Roth, AccountType::Traditional, AccountType::Educational, AccountType::Taxable];
        let bnd_priority: Vec<AccountType> = vec![AccountType::Educational, AccountType::Traditional, AccountType::Roth, AccountType::Taxable];
        //let cds_priority: Vec<AccountType> = vec![AccountType::Educational, AccountType::Traditional, AccountType::Roth, AccountType::Taxable];
        let dom_priority: Vec<AccountType> = vec![AccountType::Roth, AccountType::Taxable, AccountType::Educational, AccountType::Traditional];
        let int_priority: Vec<AccountType> = vec![AccountType::Roth, AccountType::Taxable, AccountType::Educational, AccountType::Traditional];
        let target_rel = target_account.get_asset_value(Asset::RealEstate);
        let target_bnd = target_account.get_asset_value(Asset::Bond);
        let target_dom = target_account.get_asset_value(Asset::Bond);
        let target_int = target_account.get_asset_value(Asset::Bond);

        // first allocate real estate
        let target_account = self.allocate_with_priority(target_account, target_rel, rel_priority);
        
        // next allocate bonds
        let target_account = self.allocate_with_priority(target_account, target_bnd, bnd_priority); 

        // next allocate domestic
        let target_account = self.allocate_with_priority(target_account, target_dom, int_priority);

        // next allocate international
        let target_account = self.allocate_with_priority(target_account, target_int, dom_priority);
    }

}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Asset::Domestic => "U.S.A.".fmt(f),
            Asset::International => "International".fmt(f),
            Asset::Bond => "Bonds".fmt(f),
            //Asset::Cd => "CDs".fmt(f),
            Asset::RealEstate => "Real Estate".fmt(f),
        }
    }
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &format!("Asset Class: {:<15}{:>9.2}", self.class, self.value))
    }
}
/*impl fmt::Display for Allocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp = "Allocation\n".to_string();
        for i in &self.assets {
            disp.push_str(&format!("{}%\n", i));
        }
        disp.fmt(f)
    }
}*/

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AccountType::Traditional => "IRA / 401(k)".fmt(f),
            AccountType::Roth => "Roth IRA / Roth 401(k)".fmt(f),
            AccountType::Taxable => "Brokerage Account".fmt(f),
            AccountType::Educational => "529 / Educational".fmt(f),
            AccountType::Allocation => "Allocation".fmt(f),
            AccountType::Target => "Target".fmt(f),
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp = "Account Classification: ".to_string();
        disp.push_str(&format!("{}\n", self.classification));
        for i in &self.assets {
            disp.push_str(&format!("{} $\n", i));
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
        class: Asset::Domestic,
        value: 50.00,
    };
    let intl = Asset {
        class: Asset::International,
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
        class: Asset::Domestic,
        value: 600.00,
    };
    let intl = Asset {
        class: Asset::International,
        value: 400.00,
    };
    account.add_asset(domestic);
    account.add_asset(intl);
    assert_eq!(0.6, account.get_asset_share(Asset::Domestic));
    assert_eq!(0.4, account.get_asset_share(Asset::International));
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
        class: Asset::Domestic,
        value: 50.00,
    };
    account.add_asset(domestic);
    assert!(!account.is_empty());
}

#[test]
fn desired_asset_allocation_sums_to_100() {
    let mut target_allocation = Account::new(AccountType::Allocation);
    let dom = Asset {
        class: Asset::Domestic,
        value: 35.0,
    };
    let intl = Asset {
        class: Asset::International,
        value: 25.0,
    };
    let bond = Asset {
        class: Asset::Bond,
        value: 40.0,
    };
    target_allocation.add_asset(dom);
    target_allocation.add_asset(intl);
    target_allocation.add_asset(bond);
    match target_allocation.is_valid_allocation() {
        Ordering::Equal => assert!(true),
        Ordering::Less => panic!("Allocation sums to 100 but returned less"),
        Ordering::Greater => panic!("Allocation sums to 100 but returned greater"),
    }
}

#[test]
fn allocation_complains_if_assets_dont_sum_to_100() {
    let mut target_allocation = Account::new(AccountType::Allocation);
    let dom = Asset {
        class: Asset::Domestic,
        value: 1.0,
    };
    let intl = Asset {
        class: Asset::International,
        value: 2.0,
    };
    let bond = Asset {
        class: Asset::Bond,
        value: 3.0,
    };
    target_allocation.add_asset(dom);
    target_allocation.add_asset(intl);
    target_allocation.add_asset(bond);

    match target_allocation.is_valid_allocation() {
        Ordering::Equal => panic!("Allocation should be less than 100 but returned equal"),
        Ordering::Less => assert!(true),
        Ordering::Greater => panic!("Allocation should be less than 100 but returned greater"),
    }
}

#[test]
fn allocation_says_whether_over_100() {
    let mut target_allocation = Account::new(AccountType::Allocation);
    let dom = Asset::Domestic(100.0);
    let intl = Asset::International(200.0);
    let bond = Asset::Bond(300.0);
    target_allocation.add_asset(dom);
    target_allocation.add_asset(intl);
    target_allocation.add_asset(bond);

    match target_allocation.is_valid_allocation() {
        Ordering::Equal => panic!("Allocation is greater than 100 but returned equal"),
        Ordering::Greater => assert!(true),
        Ordering::Less => panic!("Allocation is less than 100 but returned less"),
    }
}

fn request_allocation()-> Account {
    let mut allocation = Account::new(AccountType::Allocation);
    loop {
        println!("Select the number of the asset class to allocate");
        println!("1. Domestic\t2. International\t3. Bonds\t4. Real Estate");
        let mut class = String::new();
        io::stdin().read_line(&mut class)
            .expect("Failed to read line");
        println!("Percent (0-100) to allocate to this asset");
        println!("Already allocated {}%", allocation.get_total_value());
        let mut value = String::new();
        io::stdin().read_line(&mut value)
            .expect("Failed to read line");
        let class: u8 = class.trim().parse().unwrap_or(0);
        let value: f64 = value.trim().parse().unwrap_or(0.0);
        match class {
                1 => allocation.add_asset(Asset::Domestic(value)),
                2 => allocation.add_asset(Asset::International(value)),
                3 => allocation.add_asset(Asset::Bond(value)),
                4 => allocation.add_asset(Asset::RealEstate(value)),
                _ => continue,
        }
        let total_value = allocation.get_total_value();
        if total_value == 100.0 {
            break;
        } else if total_value > 100.0 {
            println!("You can't allocate more than 100%");
            println!("You allocated {}%", total_value);
            allocation = Account::new(AccountType::Allocation); 
        } else if total_value < 100.0 {
            println!("You allocated {}%", total_value);
        }


    }
    return allocation;
}

fn setup_new_account() -> Account {
    let account_type = loop {
        println!("What type of account would you like to setup?");
        println!("1. Taxable\t2. Traditional/401(k)\t3. Roth/Roth 401(k)\t4. Educational");
        let mut account_type = String::new();
        io::stdin().read_line(&mut account_type)
            .expect("Failed to read line");
        let choice: u8 = account_type.trim().parse().unwrap_or(0);
        match choice {
            1 => break AccountType::Taxable,
            2 => break AccountType::Traditional,
            3 => break AccountType::Roth,
            4 => break AccountType::Educational,
            _ => continue,
        }
    };
    return setup_account(account_type);
}

fn setup_account(account_type: AccountType) -> Account {
    let mut account = Account::new(account_type);
    loop {
        println!("What type of asset to account?");
        println!("1. Domestic\t2. International\t3. Bonds\t4. Real Estate\t5. Finish");
        let mut asset_class = String::new();
        io::stdin().read_line(&mut asset_class)
            .expect("Failed to read line");
        let choice: u8 = asset_class.trim().parse().unwrap_or(0);
        if choice == 5 { break; }
        let mut value = String::new();
        println!("How much money would you like to put towards this asset class?");
        io::stdin().read_line(&mut value)
            .expect("Failed to read line");
        let value: f64 = match parse_portfolio_value(&value) {
            Ok(val) => val,
            Err(_) => 0.0 
        };
        match choice {
            1 => account.add_asset(Asset::Domestic(value)),
            2 => account.add_asset(Asset::International(value)),
            3 => account.add_asset(Asset::Bond(value)),
            4 => account.add_asset(Asset::RealEstate(value)),
            5 => break,
            _ => continue,
        }
    }
    return account;
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
    let last = get_string("first name");
    let mut user = User::new(&first, &last);
    user.request_action();

    // TODO move calculating difference to own function

}
