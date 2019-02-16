enum AssetE {
    Domestic(f64),
    International(f64),
    Bond(f64),
    RealEstate(f64),
}

enum AccountE {
    Taxable(HashMap<AssetE>),
    Traditional(HashMap<AssetE>),
    Roth(Vec<AssetE>),
}

impl AssetE {
    fn subtract (&self, other: &AssetE) -> Option<AssetE> {
        match (self, other) {
            (AssetE::Domestic(a), AssetE::Domestic(b)) => Some(AssetE::Domestic(a - b)),
            (AssetE::International(a), AssetE::International(b)) => Some(AssetE::International(a - b)),
            (AssetE::Bond(a), AssetE::Bond(b)) => Some(AssetE::Bond(a - b)),
            (AssetE::RealEstate(a), AssetE::RealEstate(b)) => Some(AssetE::RealEstate(a - b)),
            (_,_) => None,
        }
    }
    
    fn add (&self, other: &AssetE) -> Option<AssetE> {
        match (self, other) {
            (AssetE::Domestic(a), AssetE::Domestic(b)) => Some(AssetE::Domestic(a + b)),
            (AssetE::International(a), AssetE::International(b)) => Some(AssetE::International(a + b)),
            (AssetE::Bond(a), AssetE::Bond(b)) => Some(AssetE::Bond(a + b)),
            (AssetE::RealEstate(a), AssetE::RealEstate(b)) => Some(AssetE::RealEstate(a + b)),
            (_,_) => None,
        }
    }
}

impl AccountE {
    fn add_asset(&mut self, asset: AssetE) {
        match self {
            AccountE::Taxable(v) => v.push(asset),
            AccountE::Traditional(v) => v.push(asset),
            AccountE::Roth(v) => v.push(asset),
        }
    }
}
