use std::vec::Vec;
// use std::collections::HashSet;
use std::collections::HashMap;
use primitive::*;
use encoded_query_buffer::EncodedQueryBuffer;
use query_result::QueryResult;

//ここでEncodedResultBufferの型を定義してる, 構造のメンバがdaraだけでそのdataの型はHashSetでキーとしてQueryIdが入ってる、値はないらしい、この値に付加情報を持したらよいのではないか？そしてここで計算してから
#[derive(Clone, Default, Debug)]
// pub struct EncodedResultBuffer {
//     pub data: HashSet<QueryId>,
// }
pub struct EncodedResultBuffer {
    // dataではserverと接触とされたclientのidがキーになっていて、値は接触した際のserverの付加情報となっている
    pub data: HashMap::<QueryId, Vec<AdditionalValue>>,
    pub client_data: HashMap::<QueryId, Vec<AdditionalValue>>
    // pub data: HashSet<QueryId>,
    // pub addi_data: Vec<AdditionalValue>,
}

impl EncodedResultBuffer {
    pub fn new() -> Self {
        EncodedResultBuffer::default()
        // pub data: HashSet<QueryId>,
    }

    fn cal_pos_age(server_data: u8, client_data: u8) -> f32 {
        // 年齢による人にうつされる確率
        let mut age_get = HashMap::<u8, f32>::new();
        age_get.insert(0, 0.5);
        age_get.insert(1, 0.4);
        age_get.insert(2, 0.3);
        age_get.insert(3, 0.5);
        age_get.insert(4, 0.6);
        // 年齢による人に移す確率
        let mut age_to = HashMap::<u8, f32>::new();
        age_to.insert(0, 0.8);
        age_to.insert(1, 0.7);
        age_to.insert(2, 0.9);
        age_to.insert(3, 0.6);
        age_to.insert(4, 0.4);

        let p = age_get[&client_data] * age_to[&server_data];
        p
    }

    fn cal_pos_infected(server_data: u8, client_data: u8) -> f32 {
        // 感染歴による人にうつされる確率
        let mut infected_get = HashMap::<u8, f32>::new();
        infected_get.insert(0, 0.8);
        infected_get.insert(1, 0.3);
        // 感染歴による人に移す確率
        let mut infected_to = HashMap::<u8, f32>::new();
        infected_to.insert(0, 0.5);
        infected_to.insert(1, 0.5);

        let p = infected_get[&client_data] * infected_to[&server_data];
        p
    }

    fn cal_pos_vaccine(server_data: u8, client_data: u8) -> f32 {
        // ワクチン接種回数による人にうつされる確率
        let mut vaccine_get = HashMap::<u8, f32>::new();
        vaccine_get.insert(0, 0.6);
        vaccine_get.insert(1, 0.5);
        vaccine_get.insert(2, 0.3);
        vaccine_get.insert(3, 0.2);
        // ワクチン接種回数による人に移す確率
        let mut vaccine_to = HashMap::<u8, f32>::new();
        vaccine_to.insert(0, 0.1);
        vaccine_to.insert(1, 0.3);
        vaccine_to.insert(2, 0.4);
        vaccine_to.insert(3, 0.3);

        let p = vaccine_get[&client_data] * vaccine_to[&server_data];
        p
    }

    fn cal_pos_mask(server_data: u8, client_data: u8) -> f32 {
        // clientがserverにマスクの有無でうつされる確率
        let mut p: f32 = 1.0;
        if server_data == 0 && client_data == 0 {
            p = 0.4;
        } else if server_data == 1 && client_data == 0 {
            p = 0.25;
        } else if server_data == 0 && client_data == 1 {
            p = 0.3;
        } else if server_data == 1 && client_data == 1 {
            p = 0.05;
        } else {
            p = 1.0;
        }
        p
    }

    // 全ての付加情報を使って確率を計算した
    fn calculate_possibility(server_data: AdditionalValue, client_data: AdditionalValue) -> f32 {
        let p = EncodedResultBuffer::cal_pos_age(server_data[0], client_data[0]) * EncodedResultBuffer::cal_pos_infected(server_data[1], client_data[1]) * EncodedResultBuffer::cal_pos_vaccine(server_data[2], client_data[2]) * EncodedResultBuffer::cal_pos_mask(server_data[3], client_data[3]);
        p
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
        // println!("self {:?} will print!", self);
        // ここでのqueriesは構造体のメンバ変数,クエリにあったidを一つ一つみていって、
        // それをresult.idに格納して同時にとresult.risk_levelを設定した
        for query in query_buffer.queries.iter() {
            let mut result = QueryResult::new();
            result.query_id = query.id;
            // println!("Now {:?} will print!", query_buffer);
            // println!("query {:?} will print!", query);
            // println!("self {:?} will print!", self);
            // ここはself.data={1}がquiery.idを含むかどうか
            if self.data.contains_key(&query.id) {
                // println!("query_id {:?} will print!", query.id);
                // println!("len {:?} will print!", self.data[&query.id].len());
                // println!("result_data {:?} will print!", self.data[&query.id]);
                // println!("query_id {:?} will print!", self.client_data[&query.id]);
                // println!("======================================");
                if self.data[&query.id].len() == 1 {
                    result.risk_level = EncodedResultBuffer::calculate_possibility(self.data[&query.id][0], self.client_data[&query.id][0]);
                } else {
                    let len = self.data[&query.id].len();
                    // let mut v= vec![5, 6, 8, 4, 2, 7];
                    // let minValue= *v.iter().min().unwrap();
                    let mut v_pos = vec![::std::f32::NAN];
                    for i in 0..len {
                        v_pos.push(EncodedResultBuffer::calculate_possibility(self.data[&query.id][i], self.client_data[&query.id][i]));
                    }
                    // println!("v_pos{:?} will print!", v_pos);
                    result.risk_level = v_pos.iter().fold(0.0/0.0, |m, v| v.max(m));
                    // result.risk_level = *v_pos.iter().max().unwrap(); //ここのリスクレベルで確率を入力しちゃえばいいんじゃね!?
                }
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