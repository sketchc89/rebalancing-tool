extern crate separator;
use separator::FixedPlaceSeparatable;
use std::fmt;
#[derive(Clone, PartialEq)]
pub enum AssetClass {
    Domestic,
    International,
    Bond,
    //Cd,
    RealEstate,
}
pub struct Asset {
    pub class: AssetClass,
    pub value: f64
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
    pub fn new(class: AssetClass, value: f64) -> Asset {
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
    pub fn subtract_asset(&self, other: &Asset) -> Option<Asset> {
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
