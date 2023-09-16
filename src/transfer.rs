use crate::error::ApiError;

use super::bank::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

#[derive(Debug, Clone, PartialEq)]
pub struct TransferRes {
    pub payout_ref: String,
    pub transaction_id: String,
    pub transaction_date_time: NaiveDateTime,
    pub qrstring: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TransferResInner {
    status: i32,
    message: String,
    payout_ref: Option<String>,
    transaction_id: Option<String>,
    #[serde(rename = "transactionDate_time")]
    transaction_date_time: Option<String>,
    qrstring: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum TransferConvError {
    #[error("API returned failed code: {0}")]
    Api(ApiError),
    #[error("Cannot parse timestamp: {0}. Error: {1}")]
    TimestampParse(String, String),
    #[error("The field that must be in response in success: {0}")]
    SuccessNones(String),
}

impl TryFrom<TransferResInner> for TransferRes {
    type Error = TransferConvError;

    fn try_from(value: TransferResInner) -> Result<Self, Self::Error> {
        let code = ApiError::from_code(value.status);

        if code.is_success() {
            let date_time_str =
                value
                    .transaction_date_time
                    .ok_or(TransferConvError::SuccessNones(
                        "transactionDate_time".to_owned(),
                    ))?;
            Ok(TransferRes {
                payout_ref: value
                    .payout_ref
                    .ok_or(TransferConvError::SuccessNones("payout_ref".to_owned()))?,
                transaction_id: value
                    .transaction_id
                    .ok_or(TransferConvError::SuccessNones("transaction_id".to_owned()))?,
                transaction_date_time: NaiveDateTime::parse_from_str(
                    &date_time_str,
                    "%Y-%m-%dT%H:%M:%S%.f%z",
                )
                .map_err(|e| TransferConvError::TimestampParse(date_time_str, e.to_string()))?,
                qrstring: value
                    .qrstring
                    .ok_or(TransferConvError::SuccessNones("qrstring".to_owned()))?,
            })
        } else {
            Err(TransferConvError::Api(code))
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

    #[test]
    fn transfer_response_success() {
        let example = "{
            \"status\": 1000,
            \"message\": \"Success\",
            \"payout_ref\": \"2022030288DtbRwK0IKr536t4\",
            \"transaction_id\": \"2022030288DtbRwK0IKr536t4\",
            \"transactionDate_time\": \"2022-03-02T20:30:04+07:00\",
            \"qrstring\": \"00460006022030288DtbRwK0IKr536t45102TH91042337\"
            }";
        let datum = TransferRes {
            payout_ref: "2022030288DtbRwK0IKr536t4".to_owned(),
            transaction_id: "2022030288DtbRwK0IKr536t4".to_owned(),
            transaction_date_time: NaiveDateTime::from_timestamp_millis(1646253004000)
                .expect("timestamp"),
            qrstring: "00460006022030288DtbRwK0IKr536t45102TH91042337".to_owned(),
        };

        let example_inner: TransferResInner = serde_json::from_str(&example).expect("parsed");
        let example_conv: Result<TransferRes, TransferConvError> = example_inner.try_into();
        assert_eq!(example_conv, Ok(datum));
    }

    #[test]
    fn transfer_response_failure() {
        let example = "{
            \"status\": 9091,
            \"message\": \"Request has no response from the bank. Please try again later.\"
            }";
        let datum = TransferResInner {
            status: 9091,
            message: "Request has no response from the bank. Please try again later.".to_owned(),
            payout_ref: None, 
            transaction_id: None, 
            transaction_date_time: None, 
            qrstring: None,
        };

        let example_inner: TransferResInner = serde_json::from_str(&example).expect("parsed");
        let example_conv: Result<TransferRes, TransferConvError> = example_inner.clone().try_into();
        assert_eq!(example_inner, datum);
        assert_eq!(
            example_conv,
            Err(TransferConvError::Api(ApiError::from_code(9091)))
        );
    }
}
