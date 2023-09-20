pub mod bank;
pub mod error;
pub mod query;
pub mod transfer;

use std::sync::Arc;
use log::*;
pub use bank::*;
use error::Error;
use query::QueryResInner;
pub use query::{QueryReq, QueryRes};
use transfer::TransferResInner;
pub use transfer::{TransferReq, TransferRes};

use crate::transfer::TransferReqInner;

pub const ONE_TWO_PAY_URL: &str = "https://payout.1-2-pay.com";

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    channel: String,
    partnercode: String,
    api_key: String,
}

impl Client {
    pub fn new(channel: &str, partner_code: &str, api_key: &str) -> Self {
        Client {
            base_url: ONE_TWO_PAY_URL.to_owned(),
            partnercode: partner_code.to_owned(),
            channel: channel.to_owned(),
            api_key: api_key.to_owned(),
        }
    }

    pub async fn transfer(&self, args: TransferReq) -> Result<TransferRes, Error> {
        let ref1len = args.ref1.len();
        if ref1len < 1 || ref1len > 30 {
            return Err(Error::RefLength(args.ref1));
        }
        let client = reqwest::Client::new();
        let body: TransferReqInner = args.into();
        trace!("Body: {:?}", serde_json::to_string(&body));
        let req = client
            .post(format!("{}/payout", self.base_url))
            .header("Authorization", &self.api_key)
            .header("Partnercode", &self.partnercode)
            .header("Channel", &self.channel)
            .json(&body)
            .build()
            .map_err(|e| Error::Reqwest(Arc::new(e)))?;
        let res: TransferResInner = client.execute(req)
            .await
            .map_err(|e| Error::Reqwest(Arc::new(e)))?
            .json()
            .await
            .map_err(|e| Error::Reqwest(Arc::new(e)))?;
        trace!("Response: {:?}", res);
        let res_conv: TransferRes = res.try_into().map_err(Error::ConvertTransfer)?;
        Ok(res_conv)
    }

    pub async fn query(&self, body: QueryReq) -> Result<QueryRes, Error> {
        let client = reqwest::Client::new();
        trace!("Body: {:?}", serde_json::to_string(&body));
        let res: QueryResInner = client
            .post(format!("{}/inquery-trans", self.base_url))
            .header("Authorization", &self.api_key)
            .header("Partnercode", &self.partnercode)
            .header("Channel", &self.channel)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Reqwest(Arc::new(e)))?
            .json()
            .await
            .map_err(|e| Error::Reqwest(Arc::new(e)))?;
        trace!("Response: {:?}", res);
        let res_conv: QueryRes = res.try_into().map_err(Error::ConvertQuery)?;
        Ok(res_conv)
    }
}
