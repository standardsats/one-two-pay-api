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
            "bbl" => Some(Bank::Bangkok),
            "kbank" => Some(Bank::Kasikorn),
            "ktb" => Some(Bank::KrungThai),
            "ttb" => Some(Bank::TmbThanachart),
            "scb" => Some(Bank::SiamCommercial),
            "bay" => Some(Bank::Ayudhya),
            "kkp" => Some(Bank::KiatNakinPhatra),
            "cimbt" => Some(Bank::CimbThai),
            "tisco" => Some(Bank::Tisco),
            "uobt" => Some(Bank::UnitedOverseas),
            "tcd" => Some(Bank::CreditRetail),
            "lhfg" => Some(Bank::LandAndHouses),
            "icbct" => Some(Bank::China),
            "sme" => Some(Bank::EnterpriseDevelopment),
            "baac" => Some(Bank::Agricultural),
            "exim" => Some(Bank::ExportImport),
            "gsb" => Some(Bank::GovernmentSavings),
            "ghb" => Some(Bank::GovernmentHousing),
            "isbt" => Some(Bank::Islamic),
            _ => None,
        }
    }

    pub fn to_code(self) -> u32 {
        match self {
            Bank::Bangkok => 2,
            Bank::Kasikorn => 4,
            Bank::KrungThai => 6,
            Bank::TmbThanachart => 11,
            Bank::SiamCommercial => 14,
            Bank::Ayudhya => 25,
            Bank::KiatNakinPhatra => 69,
            Bank::CimbThai => 22,
            Bank::Tisco => 67,
            Bank::UnitedOverseas => 24,
            Bank::CreditRetail => 71,
            Bank::LandAndHouses => 73,
            Bank::China => 70,
            Bank::EnterpriseDevelopment => 98,
            Bank::Agricultural => 34,
            Bank::ExportImport => 35,
            Bank::GovernmentSavings => 30,
            Bank::GovernmentHousing => 33,
            Bank::Islamic => 66,
        }
    }

    pub fn from_code(code: u32) -> Option<Self> {
        match code {
            2 => Some(Bank::Bangkok),
            4 => Some(Bank::Kasikorn),
            6 => Some(Bank::KrungThai),
            11 => Some(Bank::TmbThanachart),
            14 => Some(Bank::SiamCommercial),
            25 => Some(Bank::Ayudhya),
            69 => Some(Bank::KiatNakinPhatra),
            22 => Some(Bank::CimbThai),
            67 => Some(Bank::Tisco),
            24 => Some(Bank::UnitedOverseas),
            71 => Some(Bank::CreditRetail),
            73 => Some(Bank::LandAndHouses),
            70 => Some(Bank::China),
            98 => Some(Bank::EnterpriseDevelopment),
            34 => Some(Bank::Agricultural),
            35 => Some(Bank::ExportImport),
            30 => Some(Bank::GovernmentSavings),
            33 => Some(Bank::GovernmentHousing),
            66 => Some(Bank::Islamic),
            _ => None,
        }
    }
}
