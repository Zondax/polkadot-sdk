use codec::Encode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ScaleMsg {
	U32(u32),
	I64(i32),
	F64(f64),
	Str(String),
	Vec(Vec<u8>),
	Tuple((u16, String)),
}

pub fn scale_encode(input: ScaleMsg) -> String {
	let result = match input {
		ScaleMsg::U32(v) => v.encode(),
		ScaleMsg::I64(v) => v.encode(),
		ScaleMsg::F64(v) => v.encode(),
		ScaleMsg::Str(v) => v.encode(),
		ScaleMsg::Vec(v) => v.encode(),
		ScaleMsg::Tuple(v) => v.encode(),
	};

	hex::encode(result)
}
