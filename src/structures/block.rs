use chrono::offset::Utc;
use sha3::{Sha3_512, Digest};

use crate::block::set_hash;

pub struct Block {
    timestamp: i64,
    previous_block_hash: Vec<u8>,
    my_block_hash: Vec<u8>,
    all_data: Vec<u8>
}

impl Block{

    fn set_hash(b: &Block){

        let mut hash_vector: Vec<u8>;
        hash_vector.extend(b.timestamp.to_be_bytes());
        hash_vector.extend(b.previous_block_hash);
        hash_vector.extend(b.all_data);

        let mut hasher = Sha3_512::new();
        
        hasher.update(hash_vector);

        let result: Vec<u8> = hasher.finalize().to_vec();

        b.my_block_hash.extend(result);

    }


    fn new_block(data: Vec<u8>, previous_block_hash: Vec<u8>) -> Block{

        let time: chrono::DateTime<chrono::Utc> = chrono::offset::Utc::now();
        let timestamp: i64 = time.timestamp();

       let my_block = Block{
           timestamp: timestamp,
           previous_block_hash: previous_block_hash,
           my_block_hash: vec![],
           all_data: data
       };

      set_hash(my_block);

       return my_block;

    }


    fn new_origin() -> Block {
        let my_hash: Vec<u8> = vec![];

         let b = new_block("Genesis".as_bytes(), my_hash);

         return b;
    }

}
