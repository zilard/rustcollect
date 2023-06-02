
fn main() {

    let chain_state_bytes: &[u8] = &[0, 1, 2, 3, 4, 5];

    let mut four_bytes = [0; 4];

    four_bytes.copy_from_slice(&chain_state_bytes[0..4]);

    let db_version = u8::from_le_bytes(four_bytes);

    println!("four_bytes {:?}", four_bytes);

    println!("db_version {}", db_version);

}




fn migrate_chain_state_v0_to_v2(old_chain_state_bytes: &[u8]) -> Vec<u8> {
    let db_version: u32 = 2;
    let db_version_bytes = db_version.to_le_bytes();

    // Extra fields in ChainState v2:
    let tapi = TapiEngine::default();
    let tapi_bytes = bincode::serialize(&tapi).unwrap();

    [&db_version_bytes, old_chain_state_bytes, &tapi_bytes].concat()
}



