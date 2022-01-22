/* Constants */

include!(concat!(env!("OUT_DIR"), "/init_constants.rs"));

pub const UNIXEPOCH_U8_SIZE: usize = 10;
pub const GEOHASH_U8_SIZE: usize = 10;
pub const QUERY_U8_SIZE: usize = UNIXEPOCH_U8_SIZE + GEOHASH_U8_SIZE;
// risk_level 1バイト + qeuryId ->risk_level=4byte
pub const QUERY_ID_SIZE_U8: usize = 8;
pub const QUERY_RESULT_U8: usize = 4;
pub const ADDITIONAL_DATA_SIZE: usize = 5;
pub const RESPONSE_DATA_SIZE_U8: usize = QUERY_ID_SIZE_U8 + QUERY_RESULT_U8;
// Q? ここって何を書けば正解なんだ？
// pub const POSSIBILITY_SIZE_F32: f32 
pub const THREASHOLD: usize = 100000;

pub const CONTACT_TIME_THREASHOLD: u64 = 600;

// UNIX EPOCH INTERVAL OF THE GPS DATA
pub const TIME_INTERVAL: u64 = 600;

pub const CENTRAL_KEY: u64 = 777;

// for secure channel encryption
pub const COUNTER_BLOCK: [u8; 16] = [0; 16];
pub const SGXSSL_CTR_BITS: u32 = 128;
pub const QUERY_BYTES: usize = QUERY_SIZE*ENCODEDVALUE_SIZE;