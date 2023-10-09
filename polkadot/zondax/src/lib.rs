use std::{marker::PhantomData, sync::Arc};

use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};
use reference_trie::GenericNoExtensionLayout;
use reference_trie::{RefTrieDBMutNoExt, ReferenceTrieStreamNoExt as ReferenceTrieStream};
use sc_rpc_api::DenyUnsafe;

use sc_client_api::{backend, HeaderBackend};

use sc_executor_common::runtime_blob::RuntimeBlob;

use sp_core::traits::FetchRuntimeCode;
use sp_core::Blake2Hasher;

use memory_db::{HashKey, MemoryDB};
use sp_runtime::traits::Block as BlockT;
use std::collections::HashMap;
use trie_db::DBValue;
use trie_db::TrieMut;

mod runtime;
mod scale;

use scale::{scale_encode, ScaleMsg};

/// The Zondax API. All methods are unsafe.
// C:  Client.
// B:  BlockT
// BA: Backend
pub struct Zondax<C, B, BA> {
	deny_unsafe: DenyUnsafe,
	client: Arc<C>,
	backend: Arc<BA>,
	_marker: PhantomData<(B, BA)>,
}

impl<C, B, BA> Zondax<C, B, BA> {
	/// Creates a new instance of the Babe Rpc handler.
	pub fn new(deny_unsafe: DenyUnsafe, client: Arc<C>, backend: Arc<BA>) -> Self {
		Self { deny_unsafe, client, backend, _marker: Default::default() }
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

	#[method(name = "zondax_host_api")]
	async fn host_api(&self, method: String, args: Vec<u8>) -> RpcResult<Vec<u8>>;
}

#[async_trait]
impl<C, B, BA> ZondaxApiServer for Zondax<C, B, BA>
where
	C: Send + Sync + 'static + HeaderBackend<B>,
	B: BlockT,
	BA: 'static + backend::Backend<B>,
{
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
		let mut memtriebuilder = RefPolkadotTrieDBMutNoExt::new(&mut memdb, &mut root);
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
		_ = self.deny_unsafe.check_if_safe();

		// state for best block in the chain
		let hash = self.client.info().best_hash;
		let state = self.backend.state_at(hash).map_err(error_into_rpc_err)?;

		// get runtime code
		let state_runtime_code = sp_state_machine::backend::BackendRuntimeCode::new(&state);
		// Runtime code so our execution method should be WasmExecutionMethod::Interpreted
		// that is my take here.
		let runtime_code =
			// state_runtime_code.runtime_code().map_err(sp_blockchain::Error::RuntimeCode)?;
			state_runtime_code.runtime_code().map_err(error_into_rpc_err)?;

		let code = runtime_code
			.fetch_runtime_code()
			.ok_or(error_into_rpc_err("Could not fetch runtime code!"))?;

		// create or runtime wasm executor
		let mut runtime = crate::runtime::Runtime::new(
			RuntimeBlob::uncompress_if_needed(&code).map_err(error_into_rpc_err)?,
		);

		runtime.call(&method, &args).map_err(error_into_rpc_err)
	}
}

fn error_into_rpc_err(err: impl std::fmt::Display) -> JsonRpseeError {
	JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
		ErrorCode::InternalError.code(),
		"Error while calling host-api rpc call",
		Some(err.to_string()),
	)))
}
