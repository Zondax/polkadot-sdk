// ZONDAX STUFF
sp_api::decl_runtime_apis! {
	pub trait ZondaxApi {
		/// Returns 'Hello World'.
		fn say_hello_world() -> String;
	}
}