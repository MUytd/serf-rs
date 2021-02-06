use crate::{Client, RPCResult, protocol::{Coordinate, Member}};


impl Member {
    pub async fn coordinate(&self, client: &Client) -> RPCResult<Option<Coordinate>> {
        let res = client.get_coordinate(&self.name).await?;
        Ok(res.coord)
    }
}