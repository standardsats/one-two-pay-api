use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{error::ApiError, Bank};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QueryReq {
    /// External ID of withdraw request
    pub ref1: String,
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
    transfer_date: NaiveDateTime,
    transfer_transaction_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct QueryResInner {
    status: String,
    message: String,
    accname: Option<String>,
    bankacc: Option<String>,
    bankcode: Option<String>,
    amount: Option<String>,
    ref1: Option<String>,
    ref2: Option<String>,
    ref3: Option<String>,
    ref4: Option<String>,
    created_date: Option<String>,
    transfer_date: Option<String>,
    #[serde(rename = "transfer_transactionId")]
    transfer_transaction_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum QueryResError {
    #[error("Response contains not success status: {0}")]
    ApiError(ApiError),
    #[error("Success body missing field: {0}")]
    MissingField(&'static str),
    #[error("Bank code is not integer: {0}")]
    BankcodeIsNotInt(String),
    #[error("We don't know bank with code: {0}")]
    UnknownBank(u32),
    #[error("Amount THB is not in float format: {0}")]
    AmountIsNotFloat(String),
    #[error("Failed to parse timestamp: {0}. Error: {1}")]
    TimestampParse(String, String),
    #[error("Failed to parse status as int: {0}")]
    StatusIsNotInt(String),
}

impl TryFrom<QueryResInner> for QueryRes {
    type Error = QueryResError;

    fn try_from(value: QueryResInner) -> Result<Self, Self::Error> {
        let istatus = value
            .status
            .parse()
            .map_err(|_| QueryResError::StatusIsNotInt(value.status))?;
        let status = ApiError::from_code(istatus);
        if !status.is_success() {
            return Err(QueryResError::ApiError(status));
        }

        let bankcode = value
            .bankcode
            .ok_or(QueryResError::MissingField("bankcode"))?;
        let accname = value
            .accname
            .ok_or(QueryResError::MissingField("accname"))?;
        let bankacc = value
            .bankacc
            .ok_or(QueryResError::MissingField("bankacc"))?;
        let ref1 = value.ref1.ok_or(QueryResError::MissingField("ref1"))?;
        let ref2 = value.ref2.ok_or(QueryResError::MissingField("ref2"))?;
        let ref3 = value.ref3.ok_or(QueryResError::MissingField("ref3"))?;
        let ref4 = value.ref4.ok_or(QueryResError::MissingField("ref4"))?;
        let amount = value.amount.ok_or(QueryResError::MissingField("amount"))?;
        let created_date = value
            .created_date
            .ok_or(QueryResError::MissingField("created_date"))?;
        let transfer_date = value
            .transfer_date
            .ok_or(QueryResError::MissingField("transfer_date"))?;
        let transfer_transaction_id = value
            .transfer_transaction_id
            .ok_or(QueryResError::MissingField("transfer_transaction_id"))?;

        let bank_code = bankcode
            .parse::<u32>()
            .map_err(|_| QueryResError::BankcodeIsNotInt(bankcode))?;
        Ok(QueryRes {
            status,
            accname,
            bankacc,
            bank: Bank::from_code(bank_code).ok_or(QueryResError::UnknownBank(bank_code))?,
            amount: amount
                .trim()
                .replace(',', "")
                .parse()
                .map_err(|_| QueryResError::AmountIsNotFloat(amount))?,
            ref1: ref1,
            ref2: if ref2.is_empty() { None } else { Some(ref2) },
            ref3: if ref3.is_empty() { None } else { Some(ref3) },
            ref4: if ref4.is_empty() { None } else { Some(ref4) },
            created_date: NaiveDateTime::parse_from_str(&created_date, "%Y-%m-%d %H:%M:%S%.f")
                .map_err(|e| QueryResError::TimestampParse(created_date, e.to_string()))?,
            transfer_date: NaiveDateTime::parse_from_str(&transfer_date, "%Y-%m-%d %H:%M:%S%.f")
                .map_err(|e| QueryResError::TimestampParse(transfer_date, e.to_string()))?,
            transfer_transaction_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_response_success() {
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
            transfer_date: NaiveDateTime::from_timestamp_millis(1652776910447).expect("timestamp"),
            transfer_transaction_id: "2022051790WiXyi9Lwu0iuHgT".to_owned(),
        };
        let example_inner: QueryResInner = serde_json::from_str(&example).expect("parsed");
        let example_pretty: QueryRes = example_inner.try_into().expect("converted");
        assert_eq!(example_pretty, datum);
    }

    #[test]
    fn query_response_failure() {
        let example = "{
            \"status\": 5009,
            \"message\": \"Incorrect 'Account To' number. Please try again\",
            \"accname\": \"MANOP DEVELOPER\",
            \"bankacc\": \"66520784099\",
            \"bankcode\": \"004\",
            \"amount\": \"100,001.00\",
            \"ref1\": \"202205170648\",
            \"ref2\": \"KASiKORN BANK\",
            \"ref3\": \"\",
            \"ref4\": \"\",
            \"created_date\": \"2022-05-17 06:47:58.860\"
            }";
        let example_inner: QueryResInner = serde_json::from_str(&example).expect("parsed");
        let example_pretty: Result<QueryRes, QueryResError> = example_inner.try_into();
        assert_eq!(
            example_pretty,
            Err(QueryResError::ApiError(ApiError::from_code(5009)))
        );
    }
}
