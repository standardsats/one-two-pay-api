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
use serde::Serialize;
use transfer::TransferResInner;
pub use transfer::{TransferReq, TransferRes};

use crate::transfer::TransferReqInner;

pub const ONE_TWO_PAY_URL: &str = "https://payout.1-2-pay.com/";

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    type_header: TypeHeader,
    api_key: String,
}

#[derive(Debug, Clone, Serialize)]
struct TypeHeader {
    #[serde(rename = "Channel")]
    channel: String,
    #[serde(rename = "Partnercode")]
    partnercode: String,
}

impl Client {
    pub fn new(channel: &str, partner_code: &str, api_key: &str) -> Self {
        Client {
            base_url: ONE_TWO_PAY_URL.to_owned(),
            type_header: TypeHeader {
                partnercode: partner_code.to_owned(),
                channel: channel.to_owned(),
            },
            api_key: api_key.to_owned(),
        }
    }

    pub async fn transfer(&self, args: TransferReq) -> Result<TransferRes, Error> {
        let client = reqwest::Client::new();
        let body: TransferReqInner = args.into();
        trace!("Body: {:?}", serde_json::to_string(&body));
        let res: TransferResInner = client
            .post(format!("{}/payout", self.base_url))
            .header(
                "type",
                serde_json::to_string(&self.type_header)
                    .map_err(|e| Error::TypeHeaderEncoding(Arc::new(e)))?,
            )
            .header("Authorization", &self.api_key)
            .json(&body)
            .send()
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
            .header(
                "type",
                serde_json::to_string(&self.type_header)
                    .map_err(|e| Error::TypeHeaderEncoding(Arc::new(e)))?,
            )
            .header("Authorization", &self.api_key)
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
