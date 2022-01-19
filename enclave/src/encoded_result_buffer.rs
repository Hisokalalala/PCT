use std::vec::Vec;
use std::collections::HashSet;
use primitive::*;
use encoded_query_buffer::EncodedQueryBuffer;
use query_result::QueryResult;

//ここでEncodedResultBufferの型を定義してる, 構造のメンバがdaraだけでそのdataの型はHashSetでキーとしてQueryIdが入ってる、値はないらしい、この値に付加情報を持したらよいのではないか？そしてここで計算してから
#[derive(Clone, Default, Debug)]
// pub struct EncodedResultBuffer {
//     pub data: HashSet<QueryId>,
// }
pub struct EncodedResultBuffer {
    pub data: HashSet<QueryId>,
}

impl EncodedResultBuffer {
    pub fn new() -> Self {
        EncodedResultBuffer::default()
    }

    // ここで最後の出力をしていて、resultに付加情報のidをキーとしたデータ構造を持たせれば参照できるはず。
    // responce_vecは接触と判定されたidを返すためのレスポンス、selfにはEncodedResultBuffer { data: {1} }が入ってる。
    // reposne format
    // query.id(8byte) + reuslt(0or1 1byte)
    pub fn build_query_response(
        &self,
        query_buffer: &EncodedQueryBuffer,
        response_vec: &mut Vec<u8>,
    ) {
        // ここでのqueriesは構造体のメンバ変数,クエリにあったidを一つ一つみていって、
        // それをresult.idに格納して同時にとresult.risk_levelを設定した
        for query in query_buffer.queries.iter() {
            let mut result = QueryResult::new();
            result.query_id = query.id;
            // println!("Now {:?} will print!", query_buffer);
            // println!("Now {:?} will print!", query);
            // println!("Now {:?} will print!", self);
            // ここはself.data={1}がquiery.idを含むかどうか
            if self.data.contains(&query.id) {
                // println!("{0}, this is {1}.", &query.id, "1");
                result.risk_level = 0.8; //ここのリスクレベルで確率を入力しちゃえばいいんじゃね!?
            } else {
                // println!("{0}, this is {1}.", &query.id, "0");
                result.risk_level = 0.0;
            };
            // println!("22222Now {:?} will print!", result);
            // Q?ここ、バイナリにする理由がわからん 
            response_vec.extend_from_slice(&result.to_be_bytes());
            // println!("Result {:?} will print!", &result); 
                // Result QueryResult { query_id: 0, risk_level: 0 } will print!, Result QueryResult { query_id: 1, risk_level: 1 }
                // will print!
            // println!("Result {:?} will print!", 17_u8.to_be_bytes());
            // println!("Binary {:?} will print!", &result.to_be_bytes()); 
                // Binary [0, 0, 0, 0, 0, 0, 0, 0, 0] will print!, Binary [0, 0, 0, 0, 0, 0, 0, 1, 1] will print!
        }
    }
}