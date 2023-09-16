use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bank {
    Bangkok,
    Kasikorn,
    KrungThai,
    TmbThanachart,
    SiamCommercial,
    Ayudhya,
    KiatNakinPhatra,
    CimbThai,
    Tisco,
    UnitedOverseas,
    CreditRetail,
    LandAndHouses,
    China,
    EnterpriseDevelopment,
    Agricultural,
    ExportImport,
    GovernmentSavings,
    GovernmentHousing,
    Islamic,
}

impl Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bank::Bangkok => write!(f, "BANGKOK BANK PUBLIC COMPANY LTD."),
            Bank::Kasikorn => write!(f, "KASIKORNBANK PUBLIC COMPANY LIMITED"),
            Bank::KrungThai => write!(f, "KRUNG THAI BANK PUBLIC COMPANY LTD."),
            Bank::TmbThanachart => write!(f, "TMBTHANACHART BANK PUBLIC COMPANY LIMITED"),
            Bank::SiamCommercial => write!(f, "SIAM COMMERCIAL BANK PUBLIC COMPANY LTD."),
            Bank::Ayudhya => write!(f, "BANK OF AYUDHYA PUBLIC COMPANY LTD."),
            Bank::KiatNakinPhatra => write!(f, "KIATNAKIN PHATRA BANK PUBLIC COMPANY LIMITED"),
            Bank::CimbThai => write!(f, "CIMB THAI BANK PUBLIC COMPANY LIMITED"),
            Bank::Tisco => write!(f, "TISCO BANK PUBLIC COMPANY LIMITED"),
            Bank::UnitedOverseas => write!(f, "UNITED OVERSEAS BANK (THAI) PUBLIC COMPANY LIMITED"),
            Bank::CreditRetail => write!(f, "THE THAI CREDIT RETAIL BANK PUBLIC COMPANY LIMITED"),
            Bank::LandAndHouses => write!(f, "LAND AND HOUSES BANK PUBLIC COMPANY LMITED"),
            Bank::China => write!(
                f,
                "INDUSTRIAL AND COMMERCIAL BANK OF CHINA (THAI) PUBLIC COMPANY LIMITED"
            ),
            Bank::EnterpriseDevelopment => write!(
                f,
                "SMALL AND MEDIUM ENTERPRISE DEVELOPMENT BANK OF THAILAND"
            ),
            Bank::Agricultural => write!(f, "BANK FOR AGRICULTURE AND AGRICULTURAL COOPERATIVES"),
            Bank::ExportImport => write!(f, "EXPORT-IMPORT BANK OF THAILAND"),
            Bank::GovernmentSavings => write!(f, "GOVERNMENT SAVINGS BANK"),
            Bank::GovernmentHousing => write!(f, "THE GOVERNMENT HOUSING BANK"),
            Bank::Islamic => write!(f, "ISLAMIC BANK OF THAILAND"),
        }
    }
}

impl Bank {
    pub fn to_acronym(self) -> &'static str {
        match self {
            Bank::Bangkok => "BBL",
            Bank::Kasikorn => "KBANK",
            Bank::KrungThai => "KTB",
            Bank::TmbThanachart => "TTB",
            Bank::SiamCommercial => "SCB",
            Bank::Ayudhya => "BAY",
            Bank::KiatNakinPhatra => "KKP",
            Bank::CimbThai => "CIMBT",
            Bank::Tisco => "TISCO",
            Bank::UnitedOverseas => "UOBT",
            Bank::CreditRetail => "TCD",
            Bank::LandAndHouses => "LHFG",
            Bank::China => "ICBCT",
            Bank::EnterpriseDevelopment => "SME",
            Bank::Agricultural => "BAAC",
            Bank::ExportImport => "EXIM",
            Bank::GovernmentSavings => "GSB",
            Bank::GovernmentHousing => "GHB",
            Bank::Islamic => "ISBT",
        }
    }

    pub fn from_acronym(acronym: &str) -> Option<Self> {
        match acronym.to_lowercase().as_str() {
            "BBL" => Some(Bank::Bangkok),
            "KBANK" => Some(Bank::Kasikorn),
            "KTB" => Some(Bank::KrungThai),
            "TTB" => Some(Bank::TmbThanachart),
            "SCB" => Some(Bank::SiamCommercial),
            "BAY" => Some(Bank::Ayudhya),
            "KKP" => Some(Bank::KiatNakinPhatra),
            "CIMBT" => Some(Bank::CimbThai),
            "TISCO" => Some(Bank::Tisco),
            "UOBT" => Some(Bank::UnitedOverseas),
            "TCD" => Some(Bank::CreditRetail),
            "LHFG" => Some(Bank::LandAndHouses),
            "ICBCT" => Some(Bank::China),
            "SME" => Some(Bank::EnterpriseDevelopment),
            "BAAC" => Some(Bank::Agricultural),
            "EXIM" => Some(Bank::ExportImport),
            "GSB" => Some(Bank::GovernmentSavings),
            "GHB" => Some(Bank::GovernmentHousing),
            "ISBT" => Some(Bank::Islamic),
            _ => None,
        }
    }

    pub fn to_code(self) -> u32 {
        match self {
            Bank::Bangkok => 002,
            Bank::Kasikorn => 004,
            Bank::KrungThai => 006,
            Bank::TmbThanachart => 011,
            Bank::SiamCommercial => 014,
            Bank::Ayudhya => 025,
            Bank::KiatNakinPhatra => 069,
            Bank::CimbThai => 022,
            Bank::Tisco => 067,
            Bank::UnitedOverseas => 024,
            Bank::CreditRetail => 071,
            Bank::LandAndHouses => 073,
            Bank::China => 070,
            Bank::EnterpriseDevelopment => 098,
            Bank::Agricultural => 034,
            Bank::ExportImport => 035,
            Bank::GovernmentSavings => 030,
            Bank::GovernmentHousing => 033,
            Bank::Islamic => 066,
        }
    }

    pub fn from_code(code: u32) -> Option<Self> {
        match code {
            002 => Some(Bank::Bangkok),
            004 => Some(Bank::Kasikorn),
            006 => Some(Bank::KrungThai),
            011 => Some(Bank::TmbThanachart),
            014 => Some(Bank::SiamCommercial),
            025 => Some(Bank::Ayudhya),
            069 => Some(Bank::KiatNakinPhatra),
            022 => Some(Bank::CimbThai),
            067 => Some(Bank::Tisco),
            024 => Some(Bank::UnitedOverseas),
            071 => Some(Bank::CreditRetail),
            073 => Some(Bank::LandAndHouses),
            070 => Some(Bank::China),
            098 => Some(Bank::EnterpriseDevelopment),
            034 => Some(Bank::Agricultural),
            035 => Some(Bank::ExportImport),
            030 => Some(Bank::GovernmentSavings),
            033 => Some(Bank::GovernmentHousing),
            066 => Some(Bank::Islamic),
            _ => None,
        }
    }
}
