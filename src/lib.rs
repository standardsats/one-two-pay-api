pub mod bank;
pub mod error;

pub use bank::*;
use error::Error;

pub const ONE_TWO_PAY_URL: &str = "https://payout.1-2-pay.com/";

pub struct Client {}

pub struct TransferReq {}

pub struct TransferRes {}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
