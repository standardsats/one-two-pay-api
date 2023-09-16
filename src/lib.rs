pub mod bank;
pub mod error;
pub mod query;
pub mod transfer;

use std::sync::Arc;

pub use bank::*;
use error::Error;
pub use query::{QueryReq, QueryRes};
use transfer::{TransferConvError, TransferResInner};
pub use transfer::{TransferReq, TransferRes};

use crate::transfer::TransferReqInner;

pub const ONE_TWO_PAY_URL: &str = "https://payout.1-2-pay.com/";

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new() -> Self {
        Client {
            base_url: ONE_TWO_PAY_URL.to_owned(),
        }
    }

    pub async fn transfer(&self, args: TransferReq) -> Result<TransferRes, Error> {
        let client = reqwest::Client::new();
        let body: TransferReqInner = args.into();
        let res: TransferResInner = client
            .post(format!("{}/payout", self.base_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Reqwest(Arc::new(e)))?
            .json()
            .await
            .map_err(|e| Error::Reqwest(Arc::new(e)))?;
        let res_conv: TransferRes = res
            .try_into()
            .map_err(|e: TransferConvError| Error::ConvertTransfer(e))?;
        Ok(res_conv)
    }

    pub async fn query(&self, args: QueryReq) -> Result<QueryRes, Error> {
        todo!();
    }
}
