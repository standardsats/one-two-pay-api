use crate::error::ApiError;

use super::bank::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct TransferReq {
    /// Bank account. Example: “0652078409"
    pub bankacc: String,
    /// Which bank to withdraw to.
    pub bank: Bank,
    /// Name of person owning the account: Manop Tangngam"
    pub accname: String,
    /// Amount of THB to transfer. Example: 1000.50
    pub amount: f64,
    /// Thailand phone number. Example: 0805933181"
    pub mobileno: String,
    /// Name of entity that makes the transaction. Example: "Jack Developer"
    pub transaction_by: String,
    /// External ID of the transaction. Example: "123456789012345678“
    pub ref1: String,
    /// Additional external data
    pub ref2: Option<String>,
    /// Additional external data
    pub ref3: Option<String>,
    /// Additional external data
    pub ref4: Option<String>,
    /// Unknown
    pub line_token: Option<String>,
    /// Email address
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransferReqInner {
    /// Bank account. Example: “0652078409"
    bankacc: String,
    /// 4 digits of bank code. Example: "004"
    bankcode: String,
    /// Full name of bank. Example: "KASIKORN BANK"
    bankname: String,
    /// Name of person owning the account: Manop Tangngam"
    accname: String,
    /// Amount of THB to transfer. Example: 1000.50
    amount: f64,
    /// Thailand phone number. Example: 0805933181"
    mobileno: String,
    /// Name of entity that makes the transaction. Example: "Jack Developer"
    transaction_by: String,
    /// External ID of the transaction. Example: "123456789012345678“
    ref1: String,
    /// Additional external data
    #[serde(skip_serializing_if = "Option::is_none")]
    ref2: Option<String>,
    /// Additional external data
    #[serde(skip_serializing_if = "Option::is_none")]
    ref3: Option<String>,
    /// Additional external data
    #[serde(skip_serializing_if = "Option::is_none")]
    ref4: Option<String>,
    /// Unknown
    #[serde(rename = "lineToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    line_token: Option<String>,
    /// Email address
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

impl From<TransferReq> for TransferReqInner {
    fn from(value: TransferReq) -> Self {
        TransferReqInner {
            bankacc: value.bankacc,
            bankcode: format!("{:0>3}", value.bank.to_code()),
            bankname: format!("{}", value.bank),
            accname: value.accname,
            amount: value.amount,
            mobileno: value.mobileno,
            transaction_by: value.transaction_by,
            ref1: value.ref1,
            ref2: value.ref2,
            ref3: value.ref3,
            ref4: value.ref4,
            line_token: value.line_token,
            email: value.email,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransferRes {
    pub payout_ref: String,
    pub transaction_id: String,
    pub transaction_date_time: String,
    pub qstring: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TransferResInner {
    Success {
        status: i32,
        message: String,
        payout_ref: String,
        transaction_id: String,
        #[serde(rename = "transactionDate_time")]
        transaction_date_time: String,
        qstring: String,
    },
    Failure {
        status: i32,
        message: String,
    },
}

impl TryFrom<TransferResInner> for TransferRes {
    type Error = ApiError;

    fn try_from(value: TransferResInner) -> Result<Self, Self::Error> {
        match value {
            TransferResInner::Success {
                payout_ref,
                transaction_id,
                transaction_date_time,
                qstring,
                ..
            } => Ok(TransferRes {
                payout_ref,
                transaction_id,
                transaction_date_time,
                qstring,
            }),
            TransferResInner::Failure { status, .. } => Err(ApiError::from_code(status)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_request_example() {
        let example = "{
            \"bankacc\":\"0652078409\",
            \"bankcode\":\"004\",
            \"bankname\":\"KASIKORNBANK PUBLIC COMPANY LIMITED\",
            \"accname\":\"Manop Tangngam\",
            \"amount\":1000.50,
            \"mobileno\":\"0805933181\",
            \"transaction_by\":\"Jack Developer\",
            \"ref1\": \"123456789012345678\"
        }";
        let datum = TransferReq {
            bankacc: "0652078409".to_owned(),
            bank: Bank::Kasikorn,
            accname: "Manop Tangngam".to_owned(),
            amount: 1000.5,
            mobileno: "0805933181".to_owned(),
            transaction_by: "Jack Developer".to_owned(),
            ref1: "123456789012345678".to_owned(),
            ref2: None,
            ref3: None,
            ref4: None,
            line_token: None,
            email: None,
        };
        let datum_inner: TransferReqInner = datum.into();

        let example_json: serde_json::Value = serde_json::from_str(example).expect("json");
        assert_eq!(
            example_json,
            serde_json::to_value(&datum_inner).expect("encoded")
        );
    }
}
