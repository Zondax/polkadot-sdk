use sc_executor::{WasmExecutionMethod, WasmExecutor, WasmtimeInstantiationStrategy};
use sc_executor_common::{error::Error as ExError, runtime_blob::RuntimeBlob};
use sp_core::{
	offchain::testing::TestOffchainExt, offchain::OffchainDbExt, traits::ReadRuntimeVersion,
	Blake2Hasher,
};
use sp_io::SubstrateHostFunctions;
use sp_keystore::{testing::MemoryKeystore, KeystoreExt};
use sp_state_machine::TestExternalities;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HostFunction {
	pub name: String,
	pub params: Vec<String>,
	pub results: Vec<String>,
}

// Taken from polkadot-tests/adapters/substrate/host_api/utils.rs

// Helpers to configure and call into runtime environment
pub struct Runtime {
	ext: TestExternalities<Blake2Hasher>,
	method: WasmExecutionMethod,
}

impl Runtime {
	pub fn new() -> Self {
		let method = WasmExecutionMethod::Compiled {
			instantiation_strategy: WasmtimeInstantiationStrategy::RecreateInstance,
		};
		Runtime { ext: TestExternalities::default(), method }
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

	pub fn call(&mut self, func: &str, args: &[u8], code: &[u8]) -> Result<Vec<u8>, ExError> {
		let mut extext = self.ext.ext();

		let builder = WasmExecutor::<SubstrateHostFunctions>::builder();

		let wasm_exec = builder.with_execution_method(self.method).build();

		let blob = RuntimeBlob::uncompress_if_needed(code)?;

		wasm_exec.uncached_call(
			blob,
			&mut extext, // TODO: Is it possible to use node's externalities?
			false,       // allow_missing_host_functions
			func,
			args,
		)
	}

	pub fn _read_version(&mut self, code: &[u8]) -> Result<Vec<u8>, String> {
		let mut ext = self.ext.ext();
		let builder = WasmExecutor::<SubstrateHostFunctions>::builder();

		let wasm_exec = builder.with_execution_method(self.method).build();
		wasm_exec.read_runtime_version(code, &mut ext)
	}

	/// Returns a list with the host functions names
	pub fn host_functions(&self, code: &[u8]) -> Result<Vec<HostFunction>, String> {
		use wasmtime::*;

		let engine = Engine::default();
		let module = Module::new(&engine, code).map_err(|e| e.to_string())?;

		// Extract and print the imports
		let imports = module.imports();
		let mut host_functions = Vec::new();

		for import in imports {
			if import.module() == "env" && import.ty().func().is_some() {
				let func_ty = import.ty();
				let func_ty = func_ty.func().clone().unwrap();

				let params = func_ty.params().map(|t| t.to_string()).collect::<Vec<_>>(); // getting the parameters
				let results = func_ty.results().map(|t| t.to_string()).collect::<Vec<_>>(); // getting the return types
				let host_function =
					HostFunction { name: import.name().to_owned(), params, results };

				host_functions.push(host_function);
			}
		}

		Ok(host_functions)
	}

	pub fn exported_functions(&self, code: &[u8]) -> Result<Vec<HostFunction>, String> {
		use wasmtime::*;

		log::info!("exported_functions handler");

		let engine = Engine::default();
		log::info!("engine done");
		let module = Module::new(&engine, code).map_err(|e| {
			let e = e.to_string();
			log::error!("{}", e);
			e
		})?;
		log::info!("module done!");

		// Extract and print the exports
		let exports = module.exports();
		log::info!("exports ok");
		let mut host_functions = Vec::new();

		for export in exports {
			log::info!("exports loop");
			if let Some(func) = export.ty().func() {
				// Check if the export is a function
				let func_ty = func.clone();

				let params = func_ty.params().map(|t| t.to_string()).collect::<Vec<_>>(); // getting the parameters
				let results = func_ty.results().map(|t| t.to_string()).collect::<Vec<_>>(); // getting the return types
				let host_function =
					HostFunction { name: export.name().to_owned(), params, results };

				log::info!("pushing function: {}", export.name());
				host_functions.push(host_function);
			}
		}

		Ok(host_functions)
	}

	// pub fn call_and_decode<T: Decode>(&mut self, func: &str, args: &[u8]) -> T {
	// 	Decode::decode(&mut self.call(func, args).as_slice())
	// 		.expect("Failed to decode returned SCALE data")
	// }
}
