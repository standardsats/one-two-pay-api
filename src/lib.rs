pub struct Client {

}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {

}

pub struct TransferReq {

}

pub struct TransferRes {

}

pub struct QueryReq {

}

pub struct QueryRes {

}

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
