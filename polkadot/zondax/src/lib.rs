use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
};
use sc_rpc_api::DenyUnsafe;
use reference_trie::{ReferenceTrieStreamNoExt as ReferenceTrieStream, RefTrieDBMutNoExt};
use reference_trie::GenericNoExtensionLayout;

use std::collections::HashMap;
use memory_db::{HashKey, MemoryDB};
use trie_db::TrieMut;
use trie_db::DBValue;

use sp_core::Blake2Hasher;

mod scale;

use scale::{scale_encode, ScaleMsg};

// ZONDAX STUFF
// sp_api::decl_runtime_apis! {
// 	pub trait ZondaxApi {
// 		/// Returns 'Hello World'.
// 		fn say_hello_world() -> String;
// 	}
// }

/// The Zondax API. All methods are unsafe.
pub struct Zondax {
	deny_unsafe: DenyUnsafe,
}

impl Zondax {
	/// Creates a new instance of the Babe Rpc handler.
	pub fn new(deny_unsafe: DenyUnsafe) -> Self {
		Self { deny_unsafe }
	}
}

/// Provides rpc methods for interacting with Zondax.
#[rpc(client, server)]
pub trait ZondaxApi {
	/// Returns 'Hello Zondax'.
	#[method(name = "zondax_helloWorld")]
	async fn say_hello_world(&self) -> RpcResult<String>;

	/// Returns SCALE encoded value
	#[method(name = "scale_encode")]
	async fn encode(&self, test: ScaleMsg) -> RpcResult<String>;

	/// Returns trie root of the parameters
	#[method(name = "zondax_trieRoot")]
	async fn trie_root(&self, input: HashMap<String, String>) -> RpcResult<String>;

	/// Returns the root values calculated after each time we drop a node
	#[method(name = "zondax_insertAndDelete")]
	async fn insert_and_delete(&self, input: HashMap<String, String>) -> RpcResult<Vec<String>>;
}

#[async_trait]
impl ZondaxApiServer for Zondax {
	async fn say_hello_world(&self) -> RpcResult<String> {
		Ok("Hello Zondax".to_string())
	}

	async fn encode(&self, test: ScaleMsg) -> RpcResult<String> {
		// Called to avoid no_used warnings.
		// this is not necessary as calling encode does not pose any security risk for the node under testing
		// and the rpc endpoint is not meant to be active in production nodes.
		_ = self.deny_unsafe.check_if_safe();

		Ok(scale_encode(test))
	}

	async fn trie_root(&self, input: HashMap<String, String>) -> RpcResult<String> {
		let t = trie_root::trie_root_no_extension::<Blake2Hasher, ReferenceTrieStream, _, _, _>(input, None);

		Ok(t.to_string())
	}

	async fn insert_and_delete(&self, input: HashMap<String, String>) -> RpcResult<Vec<String>> {
        let mut memdb = MemoryDB::<_, HashKey<_>, _>::default();
		let mut root = Default::default();
		let mut result: Vec<String> = vec![];

        // let mut memtrie = RefTrieDBMutNoExt::new(&mut memdb, &mut root);
        pub type RefPolkadotTrieDBMutNoExt<'a> =
            trie_db::TrieDBMutBuilder<'a, GenericNoExtensionLayout<Blake2Hasher>>;
        let mut memtriebuilder = RefPolkadotTrieDBMutNoExt::new(&mut memdb, &mut root);
        let mut memtrie = memtriebuilder.build();

		let mut keys: Vec<_> = input.clone().into_keys().map(|k| {
			hex::decode(k).expect("Decoding failed")
		}).collect();
		let values: Vec<_> = input.clone().into_values().map(|v| {
			hex::decode(v).expect("Decoding failed")
		}).collect();

        for i in 0..input.len() {
            let key: &[u8] = &keys[i];
            let val: &[u8] = &values[i];
            memtrie.insert(key, val).unwrap();
            memtrie.commit();
        }

        //now we randomly drop nodes
        while keys.len() > 0 {
            let key_index_to_drop = memtrie.root()[0] as usize % keys.len();
            let key_to_drop = &keys[key_index_to_drop];
            memtrie.remove(key_to_drop).unwrap();
            memtrie.commit();
            result.push(memtrie.root().to_string());
            keys.remove(key_index_to_drop);
        }

		Ok(result)
	}
}
