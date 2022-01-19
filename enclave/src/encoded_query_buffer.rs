use encoded_query_rep::EncodedQueryRep;
use constant::*;
use std::vec::Vec;
// use primitive::{ AdditionalValue };

// #[derive(Clone, Default, Debug)]
// pub struct AdditionalData {
//     pub addi_: Vec<AdditionalValue>,
// }

// impl AdditionalData {
//     pub fn new() -> Self {
//         AdditionalData::default()
//     }
// }

#[derive(Clone, Default, Debug)]
pub struct EncodedQueryBuffer {
    pub queries: Vec<EncodedQueryRep>,
}

impl EncodedQueryBuffer {
    pub fn new() -> Self {
        EncodedQueryBuffer::default()
    }

    // !!このメソッドでは全くerror処理していない
    // queryを個々に組み立ててbufferに保持する
    pub fn build_query_buffer(
        &mut self,
        total_query_data_vec: Vec<u8>,
        query_id_list_vec   : Vec<u64>,
    ) -> i8 {
        for i in 0_usize..(query_id_list_vec.len()) {
            let mut query = EncodedQueryRep::new();
            // let mut addi_data = AdditionalData::new();
            // let mut addi_data = Vec::with_capacity(QUERY_SIZE);
            query.id = query_id_list_vec[i];
            for j in 0_usize..QUERY_SIZE {
                let mut encoded_all = [0_u8; ENCODEDVALUE_SIZE];
                let mut encoded_value = [0_u8; (ENCODEDVALUE_SIZE-ADDITIONAL_DATA_SIZE)];
                let mut encoded_addi = [0_u8; ADDITIONAL_DATA_SIZE];
                encoded_all.copy_from_slice(&total_query_data_vec[(i*QUERY_SIZE+j)*(ENCODEDVALUE_SIZE)..(i*QUERY_SIZE+j+1)*(ENCODEDVALUE_SIZE)]);
                // println!("Encoded_ALL!!!!!Now {:?} will print!", encoded_all);
                encoded_value.copy_from_slice(&encoded_all[0..(ENCODEDVALUE_SIZE-ADDITIONAL_DATA_SIZE)]);
                // println!("Encoded_value!!!!!Now {:?} will print!", encoded_value);
                encoded_addi.copy_from_slice(&encoded_all[(ENCODEDVALUE_SIZE-ADDITIONAL_DATA_SIZE)..]);
                // println!("encoded_addi!!!!!Now {:?} will print!", encoded_addi);
                query.parameters.push(encoded_value);
                query.addi_parameters.push(encoded_addi);
            }
            self.queries.push(query);
            // self.addi_datas.push(addi_data);
        }
        return 0;
    }
}