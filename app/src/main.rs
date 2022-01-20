// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

extern crate sgx_types;
extern crate sgx_urts;
extern crate serde;
extern crate serde_json;
extern crate succinct_trie;
extern crate bincode;
extern crate hex;
extern crate glob;
extern crate regex;

use std::env;
use std::collections::HashSet;
use std::iter::FromIterator;
use sgx_types::*;
mod enc_util;
// ecallsはnamedで呼び出す
mod ecalls;
use ecalls::{ 
    upload_encoded_query_data, 
    init_enclave,
    private_encode_contact_trace, get_encoded_result
};
mod central_data;
use central_data::*;
mod util;
use util::*;
pub const QUERY_ID_SIZE_U8: usize = 8;
pub const QUERY_RESULT_U8: usize = 4;
pub const RESPONSE_DATA_SIZE_U8: usize = QUERY_ID_SIZE_U8 + QUERY_RESULT_U8;

/*
    args[0] = threashold of each chunk block size
    args[1] = query data file dir (clientfile format => client-(theta_geo)-(theta_time)-(client_id)-(.+).csv
    args[2] = number of clients
    args[3] = central data file path"
*/
fn _get_options() -> Vec<String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        println!(" ERROR bin/app needs 3 arguments!");
        println!("    args[0] = threashold of each chunk block size");
        println!("    args[1] = query data file dir (clientfile format => client-(theta_geo)-(theta_time)-(client_id)-(.+).csv");
        println!("    args[2] = number of clients");
        println!("    args[3] = central data file path");
        std::process::exit(-1);
    }
    args
}

fn private_set_intersection() {
    let args = _get_options();
    /* parameters */
    let threashould: usize = args[0].parse().unwrap();
    let q_dirname = &args[1];
    let client_num: u32 = args[2].parse().unwrap();
    let c_filename = &args[3];

    let mut clocker = Clocker::new();

    // central dataというのはサーバに保存してあるデータのことかな？ clokerで経過時間を測っている、後ろのあれはclockの名前
    /* read central data */
    clocker.set_and_start("Read Central Data");
    let mut central_data = util::read_trajectory_hash_from_csv(c_filename);
    // println!("central_data!!!!!Now {:?} will print!", central_data); //ここまではちゃんと12区切りでいい感じに格納されてた
    clocker.stop("Read Central Data");
    let central_data_size = central_data.len();

    /* preprocess central data */
    clocker.set_and_start("Distribute central data");
    #[cfg(feature = "hashtable")]
    // R: CentralHashSetにバイナリデータが入っているという想定で考える。と、Rにvec<vec<u8>>が入っている？内側のvec<u8>がchunkで、サーばで与えられたthreshholdごとのデータ、それが複数ある
    // 多分ここで事故ったのかな？？ここで8ずつになった気がする
    let mut R: CentralHashSet = CentralHashSet::from_encoded_data(central_data, threashould);
    // println!("R_data!!!!!Now {:?} will print!", R.data);
    #[cfg(feature = "fsa")]
    let mut R: CentralTrie = CentralTrie::from_encoded_data(central_data, threashould);
    clocker.stop("Distribute central data");

    /* initialize enclave */
    println!("init_enclave...");
    clocker.set_and_start("ECALL init_enclave");
    let enclave = match init_enclave() {
        Ok(r) => {
            println!(" Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!(" Init Enclave Failed {}!", x.as_str());
            return;
        },
    };
    clocker.stop("ECALL init_enclave");

    /* read query data */
    clocker.set_and_start("Read Query Data");
    let query_data = util::read_trajectory_hash_from_csv_for_clients(q_dirname, client_num);
    let client_size = query_data.len();
    let query_id_list: Vec<u64> = Vec::from_iter((0u64..client_size as u64).into_iter());
    clocker.stop("Read Query Data");

    /* encrypt and upload query data */
    let total_data_vec: Vec<u8> = enc_util::encrypt_to_flat_vec_u8(&query_data, &query_id_list);
    clocker.set_and_start("ECALL upload_query_data");
    let mut retval = sgx_status_t::SGX_SUCCESS;
    let result = unsafe {
        upload_encoded_query_data(
            enclave.geteid(),
            &mut retval,
            total_data_vec.as_ptr() as * const u8,
            total_data_vec.len(),
            client_size,
            query_id_list.as_ptr() as * const u64
        )
    };
    match result {
        sgx_status_t::SGX_SUCCESS => {
            // println!("[UNTRUSTED] upload_query_data Succes!");
        },
        _ => {
            println!("[UNTRUSTED] upload_query_data Failed {}!", result.as_str());
            return;
        }
    }
    clocker.stop("ECALL upload_query_data");

    /* main logic contact tracing */
    // chunkはデータがデカすぎるときにSGXで動かないかもだから、分ける
    // chunkを毎回読み込んで、private_encode_contact_traceを読み込んでいる、バイナリのフォーマットをしていしなくてはいけない。
    let mut chunk_index: usize = 0;
    let last = R.len() - 1;
    clocker.set_and_start("ECALL private_contact_trace");
    while last >= chunk_index {
        // chunkはバイナリデータで、ここでメモリを渡している
        let chunk: &Vec<u8> = R.prepare_sgx_data(chunk_index);
        let result = unsafe {
            private_encode_contact_trace(
                enclave.geteid(),
                &mut retval,
                chunk.as_ptr() as * const u8,
                // ここで長さを渡している
                chunk.len()
            )
        };
        match result {
            sgx_status_t::SGX_SUCCESS => {
                // print!("\r[UNTRUSTED] private_contact_trace Succes! {} th iteration", chunk_index);
            },
            _ => {
                println!("[UNTRUSTED] private_contact_trace Failed {}!", result.as_str());
                return;
            }
        }
        chunk_index += 1;
    }
    // println!("");
    
    clocker.stop("ECALL private_contact_trace");

    /* response reconstruction */

    clocker.set_and_start("ECALL get_result");
    let response_size = client_size * RESPONSE_DATA_SIZE_U8;
    let mut response: Vec<u8> = vec![0; response_size];
    let result = unsafe {
        get_encoded_result(
            enclave.geteid(),
            &mut retval,
            response.as_mut_ptr(),
            response_size
        )
    };
    match result {
        sgx_status_t::SGX_SUCCESS => {
            // println!("[UNTRUSTED] get_result Succes!");
        },
        _ => {
            println!("[UNTRUSTED] get_result Failed {}!", result.as_str());
            return;
        }
    }
    clocker.stop("ECALL get_result");
    
    let mut positive_queries = vec![];
    for i in 0..client_size {
        /* decryption for each clients using their keys 前のコードだと,decryptionしたら、もともとその値が入ってる。*/
        // ここでquery_idをデコードしてる、query_idは暗号化してない。リクエスト自体は匿名化されてない。
        let query_id = query_id_from_u8(&response[i*RESPONSE_DATA_SIZE_U8..i*RESPONSE_DATA_SIZE_U8+QUERY_ID_SIZE_U8]);
        // クエリのIDから共有鍵を作る、encryptionでも
        let mut shared_key: [u8; 16] = [0; 16];
        shared_key[..8].copy_from_slice(&query_id.to_be_bytes());
        let counter_block: [u8; 16] = COUNTER_BLOCK; // 暗号化に必要なもの
        let ctr_inc_bits: u32 = SGXSSL_CTR_BITS;
        let src_len: usize = QUERY_RESULT_U8;
        let mut result: Vec<u8> = vec![0; src_len];
        let ret = unsafe {
            util::sgx_aes_ctr_decrypt(
                &shared_key,
                // このresponseには暗号化されている状態のf32が入ってる
                response[i*RESPONSE_DATA_SIZE_U8+QUERY_ID_SIZE_U8..i*RESPONSE_DATA_SIZE_U8+RESPONSE_DATA_SIZE_U8].as_ptr() as *const u8,
                src_len as u32,
                &counter_block as * const u8,
                ctr_inc_bits,
                // 復号した結果がここのresultに入る
                result.as_mut_ptr()
            )
        };
        if ret < 0 { println!("Error in CTR decryption."); std::process::exit(-1); }
        // ここの復号化されたresultの中には、エンコードされたf32の確率が入っているので、デコードの必要がある
        // println!("Result1 {:?} will print!", result);
        let result = query_result_from_u8(result.as_slice());
        // をパクって書く. 初期の頃は1の時だけ必要だったから、こんな感じなっている、今は全部出力して欲しい
        // println!("============!!!!!Now {:?} will print!", query_id);
        // println!("============!!!!!Result2 {:?} will print!", result);
        if result > 0.0 {
            positive_queries.push((query_id, result));
        }
    }
    println!("positive result queryIds: {:?}", positive_queries); // ここも全部ところで確率ができるようなレスポンスの変数を作る。

    /* finish */
    enclave.destroy();
    // println!("[UNTRUSTED] All process is successful!!");
    clocker.show_all();
    let now: String = get_timestamp();


    #[cfg(feature = "hashtable")]
    let data_st = "hashtable";
    #[cfg(feature = "fsa")]
    let data_st = "fsa";

    write_to_file(
        format!("result/{}-{}-{}-{}-{}.txt",
            data_st.to_string(), threashould, client_size, central_data_size, now
        ),
        data_st.to_string(),
        c_filename.to_string(),
        central_data_size,
        q_dirname.to_string(),
        client_size,
        1440,
        threashould,
        clocker
    );
}

// ここのnon_pctってなんだ？実験用にやったやつ的なのか？->sgx使わないやり方での実験で使った、とりあえず、あまり関係はない部分。
// 実験でデータ出すときにワンチャン使うかも
fn non_private_set_intersection() {
    let args = _get_options();
    /* parameters */
    let threashould: usize = args[0].parse().unwrap();
    let q_dirname = &args[1];
    let client_num: u32 = args[2].parse().unwrap();
    let c_filename = &args[3];

    let mut clocker = Clocker::new();

    /* read central data */
    clocker.set_and_start("Read Central Data");
    let central_data = util::read_trajectory_hash_from_csv(c_filename);
    clocker.stop("Read Central Data");
    let central_data_size = central_data.len();

    /* preprocess central data */
    clocker.set_and_start("Distribute central data");
    #[cfg(feature = "hashtable")]
    let mut R: NonPrivateHashSet = NonPrivateHashSet::from_encoded_data(central_data);
    #[cfg(feature = "fsa")]
    let mut R: NonPrivateFST = NonPrivateFST::from_encoded_data(central_data);
    clocker.stop("Distribute central data");

    R.calc_memory();

    /* read query data */
    clocker.set_and_start("Read Query Data");
    let query_data = util::read_trajectory_hash_from_csv_for_clients(q_dirname, client_num);
    let client_size = query_data.len();
    let query_id_list: Vec<u64> = Vec::from_iter((0u64..client_size as u64).into_iter());
    clocker.stop("Read Query Data");

    let mut query_set: HashSet<EncodedValue> = HashSet::with_capacity(client_size*1440);
    for detail in query_data.iter() {
        for hash in detail.iter() {
            query_set.insert(hash.clone());
        }
    }

    /* main logic contact tracing */
    clocker.set_and_start("Contact trace");
    let mut reuslt: Vec<EncodedValue> = Vec::default();
    for data in query_set.iter() {
        if R.set.contains(data) {
            reuslt.push(data.clone());
        }
    }
    clocker.stop("Contact trace");

    let mut positive_queries: HashSet<u64> = HashSet::default();
    query_data.iter().zip(query_id_list).for_each( |(query, query_id)| {
        let query_id = query_id;
        let contact = query.iter().any(|hash| {
            R.set.contains(hash.as_slice())
        });
        if contact {
            positive_queries.insert(query_id);
        }
    });
    // ここで感染者のidを出力している,これと同じ感じで確率も出力できるようになりそう？
    println!("positive result queryIds: {:?}", positive_queries);
    
    clocker.show_all();
    let now: String = get_timestamp();

    #[cfg(feature = "hashtable")]
    let data_st = "hashtable";
    #[cfg(feature = "fsa")]
    let data_st = "fsa";

    write_to_file(
        format!("result/{}-{}-{}-{}-{}.txt",
            data_st.to_string(), threashould, client_size, central_data_size, now
        ),
        data_st.to_string(),
        c_filename.to_string(),
        central_data_size,
        q_dirname.to_string(),
        client_size,
        1440,
        threashould,
        clocker
    );
}

fn size_compare() {
    let args = _get_options();
    /* parameters */
    let threashould: usize = args[0].parse().unwrap();
    let q_dirname = &args[1];
    let client_num: u32 = args[2].parse().unwrap();
    let c_filename = &args[3];

    let mut clocker = Clocker::new();

    /* read central data */
    clocker.set_and_start("Read Central Data");
    let mut central_data = util::read_trajectory_hash_from_csv(c_filename);
    clocker.stop("Read Central Data");
    let central_data_size = central_data.len();

    /* preprocess central data */
    clocker.set_and_start("Distribute central data");
    // let mut R_hash_set: CentralHashSet = CentralHashSet::from_encoded_data(central_data, threashould);
    let mut R_trie: CentralTrie = CentralTrie::from_encoded_data(central_data, threashould);
    clocker.stop("Distribute central data");
}

fn main() {
    private_set_intersection()
    // non_private_set_intersection()
    // size_compare()
}