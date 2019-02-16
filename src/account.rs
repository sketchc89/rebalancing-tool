use std::io;
use std::fmt;
use crate::asset::{Asset, AssetClass};
use crate::utils::parse_value;

#[derive(Clone)]
pub enum AccountType {
    Traditional,
    Taxable,
    Roth,
    Allocation,
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

pub struct Account {
    pub classification: AccountType, 
    pub assets: Vec<Asset>
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
    pub fn new(classification: AccountType) -> Account {
        Account { 
            classification,
            assets: vec![Asset::new(AssetClass::Domestic, 0.0),
            Asset::new(AssetClass::International, 0.0),
            Asset::new(AssetClass::Bond, 0.0),
            Asset::new(AssetClass::RealEstate, 0.0)]
        }
    }

    pub fn change_account_classification(&mut self, classification: AccountType) {
        self.classification = classification;
    }


    pub fn diff(&mut self, other: & Account) -> Account {
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

    pub fn multiply(&self, scalar: f64) -> Account {
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

    pub fn add_asset(&mut self, new_asset: Asset) {
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

    pub fn get_total_value(&self) -> f64 {
        let mut x = 0.0;
        for i in &self.assets {
            x = x + i.value;
        }
        return x;
    }
}

pub fn setup_new_account() -> Result<Account, String> {
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
        let value: f64 = match parse_value(&value) {
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

pub fn request_allocation()-> Result<Account, String> {
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
