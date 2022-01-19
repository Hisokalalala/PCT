use primitive::QueryId;
use constant::*;

/* 
Type QueryResult 
    バイトへのシリアライズを担当するよ
*/
#[derive(Clone, Default, Debug)]
pub struct QueryResult {
    pub query_id: QueryId,
    pub risk_level: f32,
}

impl QueryResult {
    pub fn new() -> Self {
        return QueryResult {
            query_id: 1,
            risk_level: 0.0,
        }
    }

    // Q?ここはPOSSIBILITYのサイズも足したらなんかいい感じになりそう。
    pub fn to_be_bytes(&self) -> [u8; RESPONSE_DATA_SIZE_U8] {
        let mut res = [0; RESPONSE_DATA_SIZE_U8];
        res[..QUERY_ID_SIZE_U8].clone_from_slice(&self.query_id.to_be_bytes());
        res[QUERY_ID_SIZE_U8..].clone_from_slice(&self.risk_level.to_be_bytes());
        res
    }
}