use std::{
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use codec::decode_from_bytes;
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};
use reference_trie::GenericNoExtensionLayout;
use reference_trie::ReferenceTrieStreamNoExt as ReferenceTrieStream;
use sc_rpc_api::DenyUnsafe;

use sc_client_api::{backend, HeaderBackend};

use sp_core::traits::FetchRuntimeCode;
use sp_core::Blake2Hasher;

use memory_db::{HashKey, MemoryDB};
use sp_runtime::traits::Block as BlockT;
use std::collections::HashMap;
use trie_db::TrieMut;

mod runtime;
mod scale;

use runtime::HostFunction;

use scale::{scale_encode, ScaleMsg};

// Custom runtime-api provided by zondax for testing host-api
const RUNTIME_STORAGE_SET: &str = "ZondaxTest_set_storage";
const RUNTIME_STORAGE_GET: &str = "ZondaxTest_get_storage";
const RUNTIME_KEY_EXISTS: &str = "ZondaxTest_storage_exists";
// ZondaxTest_get_len
const RUNTIME_TEST: &str = "ZondaxTest_get_len";

/// The Zondax API. All methods are unsafe.
// C:  Client.
// B:  BlockT
// BA: Backend
pub struct Zondax<C, B, BA> {
	deny_unsafe: DenyUnsafe,
	client: Arc<C>,
	backend: Arc<BA>,
	// not ideal
	runtime: Arc<Mutex<crate::runtime::Runtime>>,
	_marker: PhantomData<(B, BA)>,
}

// impl<C, B, BA> Zondax<C, B, BA> {
// }

/// Provides rpc methods for interacting with Zondax.
#[rpc(client, server)]
pub trait ZondaxApi {
	/// Returns SCALE encoded value
	#[method(name = "scale_encode")]
	async fn encode(&self, test: ScaleMsg) -> RpcResult<String>;

	/// Returns trie root of the parameters
	#[method(name = "zondax_trieRoot")]
	async fn trie_root(&self, input: HashMap<String, String>) -> RpcResult<String>;

	/// Returns the root values calculated after each time we drop a node
	#[method(name = "zondax_insertAndDelete")]
	async fn insert_and_delete(&self, input: HashMap<String, String>) -> RpcResult<Vec<String>>;

	#[method(name = "zondax_host_api")]
	async fn host_api(&self, method: String, args: Vec<u8>) -> RpcResult<Vec<u8>>;

	#[method(name = "zondax_host_api_functions")]
	async fn host_api_functions(&self) -> RpcResult<Vec<HostFunction>>;
}

#[async_trait]
impl<C, B, BA> ZondaxApiServer for Zondax<C, B, BA>
where
	C: Send + Sync + 'static + HeaderBackend<B>,
	B: BlockT,
	BA: 'static + backend::Backend<B>,
{
	async fn encode(&self, test: ScaleMsg) -> RpcResult<String> {
		// Called to avoid no_used warnings.
		// this is not necessary as calling encode does not pose any security risk for the node under testing
		// and the rpc endpoint is not meant to be active in production nodes.
		_ = self.deny_unsafe.check_if_safe();

		Ok(scale_encode(test))
	}

	async fn trie_root(&self, input: HashMap<String, String>) -> RpcResult<String> {
		let t = trie_root::trie_root_no_extension::<Blake2Hasher, ReferenceTrieStream, _, _, _>(
			input, None,
		);

		Ok(t.to_string())
	}

	async fn insert_and_delete(&self, input: HashMap<String, String>) -> RpcResult<Vec<String>> {
		let mut memdb = MemoryDB::<_, HashKey<_>, _>::default();
		let mut root = Default::default();
		let mut result: Vec<String> = vec![];

		// let mut memtrie = RefTrieDBMutNoExt::new(&mut memdb, &mut root);
		pub type RefPolkadotTrieDBMutNoExt<'a> =
			trie_db::TrieDBMutBuilder<'a, GenericNoExtensionLayout<Blake2Hasher>>;
		let memtriebuilder = RefPolkadotTrieDBMutNoExt::new(&mut memdb, &mut root);
		let mut memtrie = memtriebuilder.build();

		let mut keys: Vec<_> = input
			.clone()
			.into_keys()
			.map(|k| hex::decode(k).expect("Decoding failed"))
			.collect();
		let values: Vec<_> = input
			.clone()
			.into_values()
			.map(|v| hex::decode(v).expect("Decoding failed"))
			.collect();

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

	async fn host_api(&self, method: String, args: Vec<u8>) -> RpcResult<Vec<u8>> {
		log::info!("zondax_host_api handler");

		_ = self.deny_unsafe.check_if_safe();

		let code =
			Self::get_runtime_code(&*self.client, &*self.backend).map_err(error_into_rpc_err)?;

		log::info!("calling runtime method: {}", method);

		let mut runtime = self.runtime.lock().expect("Can not hold mutex");
		runtime.call(&method, &args, &code).map_err(error_into_rpc_err)
	}

	async fn host_api_functions(&self) -> RpcResult<Vec<HostFunction>> {
		log::info!("zondax_host_api_functions handler");
		_ = self.deny_unsafe.check_if_safe();

		let code =
			Self::get_runtime_code(&*self.client, &*self.backend).map_err(error_into_rpc_err)?;

		let runtime = self.runtime.lock().expect("Can not hold mutex");

		runtime.exported_functions(&code).map_err(error_into_rpc_err)
	}
}

impl<C, B, BA> Zondax<C, B, BA>
where
	C: Send + Sync + 'static + HeaderBackend<B>,
	B: BlockT,
	BA: 'static + backend::Backend<B>,
{
	/// Creates a new instance of the Babe Rpc handler.
	pub fn new(
		deny_unsafe: DenyUnsafe,
		client: Arc<C>,
		backend: Arc<BA>,
	) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
		let mut runtime = crate::runtime::Runtime::new().with_keystore();
		let runtime = Arc::new(Mutex::new(runtime));

		Ok(Self { deny_unsafe, client, backend, _marker: Default::default(), runtime })
	}

	fn get_runtime_code(client: &C, backend: &BA) -> Result<Vec<u8>, String> {
		// state for best block in the chain
		let hash = client.info().best_hash;
		log::info!("state best_hash: {}", hash.to_string());
		let state = backend.state_at(hash).map_err(|e| e.to_string())?;

		// get runtime code
		let state_runtime_code = sp_state_machine::backend::BackendRuntimeCode::new(&state);
		let runtime_code = state_runtime_code.runtime_code().map_err(|e| e.to_string())?;

		runtime_code
			.fetch_runtime_code()
			.map(|cow| cow.into_owned())
			.ok_or("Could not fetch runtime code!".to_string())
	}
}

fn error_into_rpc_err(err: impl std::fmt::Display) -> JsonRpseeError {
	JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
		ErrorCode::InternalError.code(),
		"Error while calling host-api rpc call",
		Some(err.to_string()),
	)))
}
