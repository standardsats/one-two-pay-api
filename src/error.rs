use std::fmt::Display;
use thiserror::Error;
use std::sync::Arc;

use crate::transfer::TransferConvError;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("API error: {0}")]
    Api(ApiError),
    #[error("Network error: {0}")]
    Reqwest(Arc<reqwest::Error>),
    #[error("Payout method conversion: {0}")]
    ConvertTransfer(TransferConvError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApiError(i32);

impl ApiError {
    pub fn to_code(self) -> i32 {
        self.0
    }

    pub fn from_code(code: i32) -> Self {
        ApiError(code)
    }

    pub fn is_success(&self) -> bool {
        self.0 == 1000
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            1000 => write!(f, "Success"),
            -2000 => write!(f, "Amount over 100,000 THB waiting to transfer, manual transfer"),
            1899 => write!(f, "We cannot process this transaction at the moment (1899)"),
            1999 => write!(f, "We cannot process this transaction at the moment (1999)"),
            5009 => write!(f, "Incorrect 'Account To' number. Please try again"),
            5016 => write!(f, "Please enter only Arabic numerals"),
            6000 => write!(f, "Amount exceeds transfer limit for today. Please re-enter amount again."),
            9001 => write!(f, "This service is temporarily unavailable and will be back soon"),
            9003 => write!(f, "You are about to make a similar transfer-same amount, same recipient. Please check, you transaction details before proceeding further."),
            -1001 => write!(f, "Invalid json request"),
            -1002 => write!(f, "Invalid Authorization"),
            -1003 => write!(f, "Duplicate Transaction"),
            -1004 => write!(f, "Invalid payout config"),
            -1009 => write!(f, "Balance is not enough"),
            9091 => write!(f, "Request has no response from the bank. Please try again later."),
            c => write!(f, "Unknown error with code {c}"), 
        }
    }
}
