extern crate separator;
use separator::FixedPlaceSeparatable;
use std::io;
use std::fmt;
use super::asset::{Asset,AssetClass};
use super::account::{self, AccountType, Account};
pub struct User {
    fname: String,
    lname: String,
    pub accounts: Vec<Account>,
    allocation: Account,
    target: Account,
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

impl User {
    /// Creates a new user given a first name and last name
    pub fn new(fname: &str, lname: &str) -> User {
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
    pub fn request_action(&mut self) {
        loop {
            println!("What would you like to do?");
            println!("1. Change target allocation\t2. Add account\t3. Display user info\t4. Display off target summary\t5. Quit");
            let mut action = String::new();
            io::stdin().read_line(&mut action)
                .expect("Failed to read line");
            let choice: u8 = action.trim().parse().unwrap_or(0);
            match choice {
                1 => match account::request_allocation() {
                    Ok(allocation) => match self.target_allocation(allocation) {
                        Ok(()) => println!("Successfully set target allocation"),
                        Err(why) => println!("{:?}", why),
                    }
                    Err(why) => println!("{}", why)
                }
                2 => match account::setup_new_account() {
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
