use std::vec::Vec;
use std::collections::HashMap;
// use std::collections::HashSet;
use std::mem;
use bincode;
use primitive::*;
use constant::*;
use encoded_result_buffer::EncodedResultBuffer;
use encoded_query_buffer::EncodedQueryBuffer;

#[derive(Clone, Debug)]
pub struct EncodedHashTable {
    // hogeは補助情報のデータ構造
    pub map: HashMap<HashValue, AdditionalValue>,
    // pub map: HashSet<HashValue>,
    // pub addi_data: Vec<AdditionalValue>,
}
impl EncodedHashTable {
    pub fn new() -> Self {
        EncodedHashTable {
            // map: HashSet::<HashValue>::with_capacity(THREASHOLD)
            map: HashMap::<HashValue, AdditionalValue>::with_capacity(THREASHOLD)
            // map: HashSet::<HashValue>::with_capacity(THREASHOLD),
            // addi_data: Vec::with_capacity(THREASHOLD),
        }
    }

    // ここでPSIしてるのかな、もし既にencoded_value_vec.idがresult.dataに入ってたらcontinueして、つまり、一回接触したらずっと接触
    // ここをcontinueじゃなくて、データを付加情報データを格納するものにすればいい
    // もし未接触だったら、接触リスト候補(これはどう作ってるんだろう)にあるかみて、接触したとresult.dataにいれる
    // ここには、queryとserver両方のデータがあるから、ここで確率計算した方がいいかもしれない。いや、違うわwここで、result.dataに、addi_dataを入れて渡すべきなんだわ！んー複数の接触があるときのこと、考えてないな.......
    pub fn intersect(&self, query_buffer: &EncodedQueryBuffer, result: &mut EncodedResultBuffer) {
        // println!("Server!!!!Now {:?} will print!", self.map);
        for encoded_value_vec in query_buffer.queries.iter() {
            // println!("=======================!!!!!Now {:?} will print!", encoded_value_vec);
            // Q?なんでこれ反映されない？
            if result.data.contains_key(&encoded_value_vec.id) {
                // ここを修正すれば一応複数対応どうにかなりそう
                continue; 
            }
            for key in encoded_value_vec.parameters.iter() {
                // println!("key!!!!!Now {:?} will print!", key); // ok
                // println!("map!!!!!Now {:?} will print!", self.map);
                if self.map.contains_key(key) {
                    result.data.insert(encoded_value_vec.id, self.map[key]);
                    continue;
                }
            }
        }
    }

    pub fn build_dictionary_buffer(
        bytes: Vec<u8>,
    ) -> Self {
        // server_data_length = bytes / 12
        // for i in serer_data_length:
        //     hash = [I*8.. I*8+ 8].copy_slice()
        //     hash = [I*8.. I*8+ 8].copy_slice()
        //     age = [I*8+8.. I*8+ 9].copy_slice()
        //     dict.insert(hash, (age, …))
        // (age, …)の部分は, なんか別のデータ構造を定義する
        let dict: HashMap<HashValue, AdditionalValue> = bincode::deserialize(&bytes[..]).unwrap();
        // let deserialized: HashSet<EncodedValue> = bincode::deserialize(&bytes[..]).unwrap();
        // let dict
        // let dict: HashSet<HashValue> = bincode::deserialize(&bytes[..]).unwrap();
        Self { map: dict }
        // let mask: 
        // Self { map: mask}
    }

    pub fn calc_memory(&self) {
        println!("[HashSet] r_i size = {} bytes", (self.map.capacity() * 11 / 10) * (mem::size_of::<EncodedValue>() + mem::size_of::<()>() + mem::size_of::<u64>()));
    }
}