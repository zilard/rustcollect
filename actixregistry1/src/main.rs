
use std::{

    sync::Arc,

};

// Arc - A thread-safe reference-counting pointer. 'Arc' stands for 'Atomically Reference Counted
// The type Arc<T> provides shared ownership of a value of type T, allocated in the heap. 
// Invoking clone on Arc produces a new Arc instance, 
// which points to the same allocation on the heap as the source Arc, 
// while increasing a reference count. 
// When the last Arc pointer to a given allocation is destroyed, 
// the value stored in that allocation (often referred to as “inner value”) is also dropped.



use actix::prelude::*;

use witnet_config::config::Config as NodeConfig;
use witnet_config::{config, config::Config};
use witnet_storage::{backends, storage::Storage};


use std::{
    path::PathBuf,
};

// Cargo.toml:45:witnet_config = { path = "./config" }





/// The total configuration object that contains all other, more
/// specific, configuration objects (connections, storage, etc).
#[derive(PartialStruct, Debug, Clone, PartialEq)]
#[partial_struct(derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq))]:
pub struct Config {

    /// Storage-related configuration
    #[partial_struct(ty = "PartialStorage")]
    #[partial_struct(serde(default))]
    pub storage: Storage,

}



/// Storage-specific configuration
#[derive(PartialStruct, Debug, Clone, PartialEq, Eq)]
#[partial_struct(derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq, Eq))]
pub struct Storage {

    /// Path to the directory that will contain the database. Used
    /// only if backend is RocksDB.
    pub db_path: PathBuf,


}


// std::path::PathBuf -  An owned, mutable path (akin to String).
// use std::path::PathBuf;
// let mut path = PathBuf::new();



impl Default for StorageManager {
    fn default() -> Self {
        StorageManager {
            backend: Arc::new(UtxoDbWrapStorage(backends::nobackend::Backend)),
        }
    }
}

// storage/src/backends/nobackend.rs:12:pub struct Backend;
/// A Backend that is not persisted
///
/// This backend fails to perform any operation defined in
/// [`Storage`](Storage)
pub struct Backend;



// data_structures/src/utxo_pool/utxo_db.rs:86:pub struct UtxoDbWrapStorage<S>(pub S);
/// Wrap a `Storage` implementation that allows to put and delete raw bytes in a way that allows
/// storing `OutputPointer`s. UTXOs are prefixed as `"UTXO-"`, followed by the string representation
/// of the `OutputPointer`. The value stored is a tuple of `(ValueTransferOutput, u32)` serialized
/// using bincode.
///
/// For example: `"UTXO-0222222222222222222222222222222222222222222222222222222222222222:1"`
#[derive(Debug)]
pub struct UtxoDbWrapStorage<S>(pub S);





struct StorageManagerAdapter {
    storage: Addr<StorageManager>,
    config: Option<Config>,
}

impl StorageManagerAdapter {
    pub fn from_config(config: Config) -> Self {
        let storage = SyncArbiter::start(1, StorageManager::default);
        Self {
            storage,
            config: Some(config),
        }
    }
}



/// Start the storage manager from config
pub fn start_from_config(config: Config) {
    let addr = StorageManagerAdapter::from_config(config).start();
    actix::SystemRegistry::set(addr);
}


fn main() {

    // Initialize Storage Manager
    let mut node_config = NodeConfig::default();
    node_config.storage.db_path = config.storage.db_path.clone();
    storage_mngr::start_from_config(node_config);

}

