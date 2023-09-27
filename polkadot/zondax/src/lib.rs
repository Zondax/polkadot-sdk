use std::sync::Arc;
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::{error::CallError, ErrorObject},
};
use sc_rpc_api::DenyUnsafe;
use codec::{Encode, Decode};

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
	pub fn new(
		deny_unsafe: DenyUnsafe,
	) -> Self {
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
	#[method(name = "zondax_encode")]
	async fn encode(&self, test: u32) -> RpcResult<String>;
}

#[async_trait]
impl ZondaxApiServer for Zondax {
	async fn say_hello_world(&self) -> RpcResult<String> {
		Ok("Hello Zondax".to_string())
	}

	async fn encode(&self, test: u32) -> RpcResult<String> {
		let result = test.encode();

		Ok(hex::encode(result))
	}
}