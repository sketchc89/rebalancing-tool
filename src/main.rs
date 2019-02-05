use std::f64;
use std::io;
use std::io::Write;
use std::cmp::Ordering;
use std::fmt;

struct User {
    fname: String,
    lname: String,
    accounts: Vec<Account>,
    allocation: Allocation,
    target: Allocation,
}

#[derive(Clone)]
enum AccountType {
    Traditional,
    Taxable,
    Roth,
    Educational
}

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
    //Cd,
    RealEstate,
}

struct Asset {
    class: AssetClass,
    value: f64
}

trait HoldsAssets {
    fn add_asset(&mut self, asset: Asset);
    fn get_total_value(&self) -> f64;
    fn is_empty(&self) -> bool;
    fn get_asset_share(&self, class: AssetClass) -> f64;
}

impl Asset {
    fn new(class: AssetClass, value: f64) -> Asset {
        Asset {
            class,
            value,
        }
    }
}

impl Account {
    fn new(classification: AccountType) -> Account {
        Account { 
            classification,
            assets: Vec::new(), 
        }
    }
    fn diff(&mut self, other: & Account) -> Account{
        let class = self.classification.clone();
        let mut diff = Account::new(class);
        for i in &self.assets {
            for j in &other.assets {
                match (&i.class, &j.class) {
                    (AssetClass::Domestic, AssetClass::Domestic) => 
                        diff.add_asset(Asset::new(AssetClass::Domestic, i.value - j.value)),
                    (AssetClass::International, AssetClass::International) => 
                        diff.add_asset(Asset::new(AssetClass::International, i.value - j.value)),
                    (AssetClass::Bond, AssetClass::Bond) => 
                        diff.add_asset(Asset::new(AssetClass::Bond, i.value - j.value)),
                    //(AssetClass::Cd, AssetClass::Cd) => 
                        //diff.add_asset(Asset::new(AssetClass::Cd, i.value - j.value)),
                    (AssetClass::RealEstate, AssetClass::RealEstate) => 
                        diff.add_asset(Asset::new(AssetClass::RealEstate, i.value - j.value)),
                    (_,__) => continue,
                }
            }
        }
        return diff;
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
}

impl HoldsAssets for Account {
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
    fn is_empty(&self) -> bool {
        return self.assets.is_empty()
    }
    fn get_asset_share(&self, class: AssetClass) -> f64 {
        return 100.0*self.get_asset_value(class) / self.get_total_value();
    }
}

impl Allocation {
    fn new() -> Allocation {
        Allocation { assets: Vec::new() }
    }
    fn diff(&self, other: &Allocation) -> Allocation {
        let mut diff = Allocation::new();
        for i in &self.assets {
            for j in &other.assets {
                match (&i.class, &j.class) {
                    (AssetClass::Domestic, AssetClass::Domestic) => 
                        diff.allocate(Asset::new(AssetClass::Domestic, i.value - j.value)),
                    (AssetClass::International, AssetClass::International) => 
                        diff.allocate(Asset::new(AssetClass::International, i.value - j.value)),
                    (AssetClass::Bond, AssetClass::Bond) => 
                        diff.allocate(Asset::new(AssetClass::Bond, i.value - j.value)),
                    //(AssetClass::Cd, AssetClass::Cd) => 
                        //diff.allocate(Asset::new(AssetClass::Cd, i.value - j.value)),
                    (AssetClass::RealEstate, AssetClass::RealEstate) => 
                        diff.allocate(Asset::new(AssetClass::RealEstate, i.value - j.value)),
                    (_,__) => continue,
                }
            }
        }
        return diff;
    }
    fn account_from_allocation(&self, total: f64) -> Account {
        let mut account = Account::new(AccountType::Taxable);
        for i in &self.assets {
            match &i.class {
                AssetClass::Domestic => account.add_asset(Asset::new(AssetClass::Domestic, i.value*total/100.0)),
                AssetClass::International => account.add_asset(Asset::new(AssetClass::International, i.value*total/100.0)),
                AssetClass::Bond => account.add_asset(Asset::new(AssetClass::Bond, i.value*total/100.0)),
                //AssetClass::Cd => account.add_asset(Asset::new(AssetClass::Cd, i.value*total/100.0)),
                AssetClass::RealEstate => account.add_asset(Asset::new(AssetClass::RealEstate, i.value*total/100.0)),
            }
        }
        return account;
    }

    fn get_allocated_amount(&mut self) -> f64 {
        let mut x = 0.0;
        for i in &self.assets {
            x = x + i.value;
        }
        return x;
    }
    fn is_valid_allocation(&mut self) -> Ordering {
        let i = self.get_allocated_amount().round() as i64;
        match i.cmp(&100) {
            Ordering::Equal => return Ordering::Equal,
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
        }
    }
    fn allocate(&mut self, asset: Asset) { self.add_asset(asset); }
}

impl User {
    fn new(fname: &str, lname: &str) -> User {
        User {
            fname: fname.to_string(),
            lname: lname.to_string(),
            accounts: Vec::new(),
            allocation: Allocation::new(),
            target: Allocation::new(),
        }
    }
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
        self.current_allocation();
    }
    fn target_allocation(&mut self, allocation: Allocation) {
        self.target = allocation;
    }

    fn request_action(&mut self) {
        loop {
            println!("What would you like to do?");
            println!("1. Change target allocation\t2. Add account\t3. Quit");
            let mut action = String::new();
            io::stdin().read_line(&mut action)
                .expect("Failed to read line");
            let choice: u8 = action.trim().parse().unwrap_or(0);
            if choice == 3 { break; }
            match choice {
                1 => self.target_allocation(request_allocation()),
                2 => self.add_account(setup_new_account()),
                3 => break,
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
        return 100.0*self.user_asset_value(class) / self.user_total();
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

        let mut cur = Allocation::new();
        cur.allocate(dom);
        cur.allocate(int);
        cur.allocate(bnd);
        //cur.allocate(cds);
        cur.allocate(rle);
        self.allocation = cur;
    }

    fn allocate_with_priority(&mut self, src_acc: Account, val: f64, priority: Vec<AccountType>) -> Account {
        println!("non-functional");
        src_acc
    }
    fn reallocate_to_target(&mut self) {

        let target_account = self.target.account_from_allocation(self.user_total());
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

impl HoldsAssets for Allocation {
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
        return 100.0*x / self.get_total_value();
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
        write!(f, "{}", &format!("Asset Class: {:<15}{:>9.2}", self.class, self.value))
    }
}
impl fmt::Display for Allocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp = "Allocation\n".to_string();
        for i in &self.assets {
            disp.push_str(&format!("{}%\n", i));
        }
        disp.fmt(f)
    }
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AccountType::Traditional => "IRA / 401(k)".fmt(f),
            AccountType::Roth => "Roth IRA / Roth 401(k)".fmt(f),
            AccountType::Taxable => "Brokerage Account".fmt(f),
            AccountType::Educational => "529 / Educational".fmt(f),
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
    let mut target_allocation = Allocation::new();
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
        Ordering::Equal => assert!(true),
        Ordering::Less => panic!("Allocation sums to 100 but returned less"),
        Ordering::Greater => panic!("Allocation sums to 100 but returned greater"),
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
        Ordering::Equal => panic!("Allocation should be less than 100 but returned equal"),
        Ordering::Less => assert!(true),
        Ordering::Greater => panic!("Allocation should be less than 100 but returned greater"),
    }
}

#[test]
fn allocation_says_whether_over_100() {
    let mut target_allocation = Allocation::new();
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
    target_allocation.allocate(dom);
    target_allocation.allocate(intl);
    target_allocation.allocate(bond);

    match target_allocation.is_valid_allocation() {
        Ordering::Equal => panic!("Allocation is greater than 100 but returned equal"),
        Ordering::Greater => assert!(true),
        Ordering::Less => panic!("Allocation is less than 100 but returned less"),
    }
}

fn request_allocation()-> Allocation {
    let mut allocation = Allocation::new();
    loop {
        println!("Select the number of the asset class to allocate");
        println!("1. Domestic\t2. International\t3. Bonds\t4. Real Estate");
        let mut class = String::new();
        io::stdin().read_line(&mut class)
            .expect("Failed to read line");
        println!("Percent (0-100) to allocate to this asset");
        println!("Already allocated {}%", allocation.get_allocated_amount());
        let mut value = String::new();
        io::stdin().read_line(&mut value)
            .expect("Failed to read line");
        let class: u8 = class.trim().parse().unwrap_or(0);
        let value: f64 = value.trim().parse().unwrap_or(0.0);
        match class {
                1 => allocation.allocate(Asset::new(AssetClass::Domestic, value)),
                2 => allocation.allocate(Asset::new(AssetClass::International, value)),
                3 => allocation.allocate(Asset::new(AssetClass::Bond, value)),
                4 => allocation.allocate(Asset::new(AssetClass::RealEstate, value)),
                _ => continue,
        }
        match allocation.is_valid_allocation() {
            Ordering::Equal => break,
            Ordering::Less => continue,
            Ordering::Greater => { 
                println!("You can't allocate more than 100%");
                println!("You allocated {}%", allocation.get_allocated_amount());
                allocation = Allocation::new(); 
            },
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
        println!("1. Domestic\t2. International\t3. Bonds\t4. Real Estate\t5. Quit");
        let mut asset_class = String::new();
        io::stdin().read_line(&mut asset_class)
            .expect("Failed to read line");
        let choice: u8 = asset_class.trim().parse().unwrap_or(0);
        if choice == 5 { break; }
        let mut value = String::new();
        println!("How much money would you like to put towards this asset class?");
        io::stdin().read_line(&mut value)
            .expect("Failed to read line");
        let value: f64 = value.trim().parse().unwrap_or(0.0);
        match choice {
            1 => account.add_asset(Asset::new(AssetClass::Domestic, value)),
            2 => account.add_asset(Asset::new(AssetClass::International, value)),
            3 => account.add_asset(Asset::new(AssetClass::Bond, value)),
            4 => account.add_asset(Asset::new(AssetClass::RealEstate, value)),
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
    let allocation = request_allocation();
    println!("{}", allocation);
    user.add_account(setup_new_account());
    let taxable_dom = get_portfolio_value("taxable domestic");
    let taxable_intl = get_portfolio_value("taxable international");
    let traditional_dom = get_portfolio_value("401k domestic");
    let traditional_intl = get_portfolio_value("401k international");
    let roth_dom = get_portfolio_value("Roth domestic");
    let roth_intl = get_portfolio_value("Roth international");
    let edu_dom = get_portfolio_value("529 domestic");
    let edu_intl = get_portfolio_value("529 international");
    let taxable_domestic = Asset::new(AssetClass::Domestic, taxable_dom);
    let taxable_international = Asset::new(AssetClass::International, taxable_intl);
    let traditional_domestic = Asset::new(AssetClass::Domestic, traditional_dom);
    let traditional_international = Asset::new(AssetClass::International, traditional_intl);
    let roth_domestic = Asset::new(AssetClass::Domestic, roth_dom);
    let roth_international = Asset::new(AssetClass::International, roth_intl);
    let edu_domestic = Asset::new(AssetClass::Domestic, edu_dom);
    let edu_international = Asset::new(AssetClass::International, edu_intl);
    let mut taxable_account = Account::new(AccountType::Taxable);
    taxable_account.add_asset(taxable_domestic);
    taxable_account.add_asset(taxable_international);
    let mut traditional_account = Account::new(AccountType::Traditional);
    traditional_account.add_asset(traditional_domestic);
    traditional_account.add_asset(traditional_international);
    let mut roth_account = Account::new(AccountType::Roth);
    roth_account.add_asset(roth_domestic);
    roth_account.add_asset(roth_international);
    let mut edu_account = Account::new(AccountType::Educational);
    edu_account.add_asset(edu_domestic);
    edu_account.add_asset(edu_international);
    user.add_account(taxable_account);
    user.add_account(traditional_account);
    user.add_account(roth_account);
    user.target_allocation(allocation);

    println!("{}", user);
    
    let diff = user.allocation.diff(&user.target);
    let acc_tot = user.user_total();
    let diff_acc = diff.account_from_allocation(acc_tot);
    println!("Difference current and target:\n{}", diff);
    println!("Total assets: ${}\n", acc_tot);
    println!("Difference current and target in $:\n{}", diff_acc);
    user.reallocate_to_target();

}
