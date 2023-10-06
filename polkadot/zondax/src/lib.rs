use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
};
use sc_rpc_api::DenyUnsafe;
use sp_core::Blake2Hasher;
use reference_trie::ReferenceTrieStreamNoExt as ReferenceTrieStream;
use std::collections::HashMap;

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

	/// Returns trie rootof the parameters
	#[method(name = "zondax_trieRoot")]
	async fn trie_root(&self, input: HashMap<String, String>) -> RpcResult<String>;
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
}
