pub mod bank;
pub mod error;
pub mod query;
pub mod transfer;

pub use bank::*;
use error::Error;
pub use query::{QueryReq, QueryRes};
pub use transfer::{TransferReq, TransferRes};

pub const ONE_TWO_PAY_URL: &str = "https://payout.1-2-pay.com/";

pub struct Client {}

impl Client {
    pub async fn transfer(client: &Client, args: TransferReq) -> Result<TransferRes, Error> {
        todo!();
    }

    pub async fn query(client: &Client, args: QueryReq) -> Result<QueryRes, Error> {
        todo!();
    }
}
