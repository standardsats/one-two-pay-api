pub mod bank;
pub mod error;
pub mod transfer;

pub use bank::*;
use error::Error;
pub use transfer::{TransferReq, TransferRes};

pub const ONE_TWO_PAY_URL: &str = "https://payout.1-2-pay.com/";

pub struct Client {}

pub struct QueryReq {}

pub struct QueryRes {}

impl Client {
    pub async fn transfer(client: &Client, args: TransferReq) -> Result<TransferRes, Error> {
        todo!();
    }

    pub async fn query(client: &Client, args: QueryReq) -> Result<QueryRes, Error> {
        todo!();
    }
}
