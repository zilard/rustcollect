fn main() {

    let db_version: u32 = 1026;
    let db_version_bytes = db_version.to_le_bytes();

    println!("db_version_bytes {:?}", db_version_bytes);

}
