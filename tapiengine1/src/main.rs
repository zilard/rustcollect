
// Cargo.toml  bincode = "1.2.1"


//use serde::{Serialize, Deserialize};
use serde::{Serialize};

use std::collections::HashMap;

use bincode;


pub type Epoch = u32;

// An epoch is a fixed timeframe to provide all miners
// validators a common frame of reference for scheduled events on a blockchain.

/// Struct that count the positives votes of a WIP
//#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Serialize, Default, Debug)]
pub struct BitVotesCounter {
    pub votes: u32,
    pub period: Epoch,
    pub wip: String,
    pub init: Epoch,
    pub end: Epoch,
    pub bit: usize,
}


//#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Serialize, Default, Debug)]
pub struct BitTapiCounter {
    info: [Option<BitVotesCounter>; 32],
    last_epoch: Epoch,
    current_length: usize,
}


/*
    let bit = 0;
    let wip = BitVotesCounter {
         votes: 0,
         period: 100,
         wip: "test0".to_string(),
         init: 10_000,
         end: 20_000,
         bit,
    };
    tapi.bit_tapi_counter.insert(wip);
*/



impl BitTapiCounter {

    pub fn insert(&mut self, bit_info: BitVotesCounter) {
        let k = bit_info.bit;
        if k >= self.info.len() {
            log::error!(
                "Tapi Engine: This bit position ({}) is invalid. {} has not been included",
                k,
                bit_info.wip
            );
        } else {
            self.info[k] = Some(bit_info);

            if k >= self.current_length {
                self.current_length = k + 1;
            }
        }
    }

}


// migrated_bytes: [2, 0, 0, 0, 0, 1, 2, 3, 4, 5, 1, 0, 0, 0, 0, 100, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 116, 101, 115, 116, 48, 16, 39, 0, 0, 32, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]



/// TAPI Engine
//#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Serialize, Default, Debug)]
pub struct TapiEngine {
    /// bit votes counter by bits
    pub bit_tapi_counter: BitTapiCounter,
    /// wip activation
    pub wip_activation: HashMap<String, Epoch>,
}



fn migrate_chain_state_v0_to_v2(old_chain_state_bytes: &[u8]) -> Vec<u8> {
    let db_version: u32 = 2;
    let db_version_bytes = db_version.to_le_bytes();

    // Extra fields in ChainState v2:
    let mut tapi = TapiEngine::default();


    let bit = 0;
    let wip = BitVotesCounter {
         votes: 0,
         period: 100,
         wip: "test0".to_string(),
         init: 10_000,
         end: 20_000,
         bit,
    };
    tapi.bit_tapi_counter.insert(wip);



    let tapi_bytes = bincode::serialize(&tapi).unwrap();

    [&db_version_bytes, old_chain_state_bytes, &tapi_bytes].concat()
}



fn main() {

    let chain_state_bytes: &[u8] = &[0, 1, 2, 3, 4, 5];

    let migrated_bytes = migrate_chain_state_v0_to_v2(chain_state_bytes);

    println!("migrated_bytes: {:?}", migrated_bytes);


    // migrated_bytes: [2, 0, 0, 0, 0, 1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

}




