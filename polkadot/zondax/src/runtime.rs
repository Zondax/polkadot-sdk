use sc_executor::{WasmExecutionMethod, WasmExecutor, WasmtimeInstantiationStrategy};
use sc_executor_common::{error::Error as ExError, runtime_blob::RuntimeBlob};
use sp_core::{offchain::testing::TestOffchainExt, offchain::OffchainDbExt, Blake2Hasher};
use sp_io::SubstrateHostFunctions;
use sp_keystore::{testing::MemoryKeystore, KeystoreExt};
use sp_state_machine::TestExternalities;
use std::sync::Arc;

// Taken from polkadot-tests/adapters/substrate/host_api/utils.rs

// Helpers to configure and call into runtime environment
pub struct Runtime {
	blob: RuntimeBlob,
	ext: TestExternalities<Blake2Hasher>,
	method: WasmExecutionMethod,
}

impl Runtime {
	pub fn new(blob: RuntimeBlob) -> Self {
		let method = WasmExecutionMethod::Compiled {
			instantiation_strategy: WasmtimeInstantiationStrategy::RecreateInstance,
		};
		Runtime { blob, ext: TestExternalities::default(), method }
	}

	pub fn _with_keystore(mut self) -> Self {
		let key_store = KeystoreExt(Arc::new(MemoryKeystore::new()));
		self.ext.register_extension(key_store);
		self
	}

	pub fn _with_offchain(mut self) -> Self {
		let (offchain, _) = TestOffchainExt::new();
		self.ext.register_extension(OffchainDbExt::new(offchain));
		self
	}

	pub fn call(&mut self, func: &str, args: &[u8]) -> Result<Vec<u8>, ExError> {
		let mut extext = self.ext.ext();

		let builder = WasmExecutor::<SubstrateHostFunctions>::builder();

		let wasm_exec = builder.with_execution_method(self.method).build();

		Ok(wasm_exec.uncached_call(
			self.blob.clone(),
			&mut extext, // TODO: Is it possible to use node's externalities?
			false,       // allow_missing_host_functions
			func,
			args,
		)?)
	}
	// pub fn call_and_decode<T: Decode>(&mut self, func: &str, args: &[u8]) -> T {
	// 	Decode::decode(&mut self.call(func, args).as_slice())
	// 		.expect("Failed to decode returned SCALE data")
	// }
}
