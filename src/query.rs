use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{error::ApiError, Bank};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QueryReq {
    /// External ID of withdraw request
    ref1: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryRes {
    status: ApiError,
    accname: String,
    bankacc: String,
    bank: Bank,
    amount: f64,
    ref1: String,
    ref2: Option<String>,
    ref3: Option<String>,
    ref4: Option<String>,
    created_date: NaiveDateTime,
    transfer_date: Option<NaiveDateTime>,
    transfer_transaction_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct QueryResInner {
    status: String,
    message: String,
    accname: String,
    bankacc: String,
    bankcode: String,
    amount: String,
    ref1: String,
    ref2: String,
    ref3: String,
    ref4: String,
    created_date: String,
    transfer_date: Option<String>,
    #[serde(rename = "transfer_transactionId")]
    transfer_transaction_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum QueryResError {
    #[error("Status is not integer: {0}")]
    StatusIsNotInt(String),
    #[error("Bank code is not integer: {0}")]
    BankcodeIsNotInt(String),
    #[error("We don't know bank with code: {0}")]
    UnknownBank(u32),
    #[error("Amount THB is not in float format: {0}")]
    AmountIsNotFloat(String),
    #[error("Failed to parse timestamp: {0}. Error: {1}")]
    TimestampParse(String, String),
}

impl TryFrom<QueryResInner> for QueryRes {
    type Error = QueryResError;

    fn try_from(value: QueryResInner) -> Result<Self, Self::Error> {
        let code = value
            .status
            .parse::<i32>()
            .map_err(|_| QueryResError::StatusIsNotInt(value.status))?;
        let bank_code = value
            .bankcode
            .parse::<u32>()
            .map_err(|_| QueryResError::BankcodeIsNotInt(value.bankcode))?;
        Ok(QueryRes {
            status: ApiError::from_code(code),
            accname: value.accname,
            bankacc: value.bankacc,
            bank: Bank::from_code(bank_code).ok_or(QueryResError::UnknownBank(bank_code))?,
            amount: value
                .amount
                .trim()
                .replace(',', "")
                .parse()
                .map_err(|_| QueryResError::AmountIsNotFloat(value.amount))?,
            ref1: value.ref1,
            ref2: if value.ref2.is_empty() {
                None
            } else {
                Some(value.ref2)
            },
            ref3: if value.ref3.is_empty() {
                None
            } else {
                Some(value.ref3)
            },
            ref4: if value.ref4.is_empty() {
                None
            } else {
                Some(value.ref4)
            },
            created_date: NaiveDateTime::parse_from_str(
                &value.created_date,
                "%Y-%m-%d %H:%M:%S%.f",
            )
            .map_err(|e| QueryResError::TimestampParse(value.created_date, e.to_string()))?,
            transfer_date: value
                .transfer_date
                .map(|t| {
                    NaiveDateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S%.f")
                        .map_err(|e| QueryResError::TimestampParse(t, e.to_string()))
                })
                .transpose()?,
            transfer_transaction_id: value.transfer_transaction_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_request_example() {
        let example = "{
            \"status\": \"1000\",
            \"message\": \"Success\",
            \"accname\": \"MANOP DEVELOPER\",
            \"bankacc\": \"6652078409\",
            \"bankcode\": \"004\",
            \"amount\": \"1.00\",
            \"ref1\": \"202205170841\",
            \"ref2\": \"KASiKORN BANK\",
            \"ref3\": \"\",
            \"ref4\": \"\",
            \"created_date\": \"2022-05-17 08:41:48.320\",
            \"transfer_date\": \"2022-05-17 08:41:50.447\",
            \"transfer_transactionId\": \"2022051790WiXyi9Lwu0iuHgT\"
            }";
        let datum = QueryRes {
            status: ApiError::from_code(1000),
            bankacc: "6652078409".to_owned(),
            bank: Bank::Kasikorn,
            accname: "MANOP DEVELOPER".to_owned(),
            amount: 1.0,
            ref1: "202205170841".to_owned(),
            ref2: Some("KASiKORN BANK".to_owned()),
            ref3: None,
            ref4: None,
            created_date: NaiveDateTime::from_timestamp_millis(1652776908320).expect("timestamp"),
            transfer_date: Some(
                NaiveDateTime::from_timestamp_millis(1652776910447).expect("timestamp"),
            ),
            transfer_transaction_id: Some("2022051790WiXyi9Lwu0iuHgT".to_owned()),
        };
        let example_inner: QueryResInner = serde_json::from_str(&example).expect("parsed");
        let example_pretty: QueryRes = example_inner.try_into().expect("converted");
        assert_eq!(example_pretty, datum);
    }
}
