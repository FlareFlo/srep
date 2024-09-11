use crate::game_recording::Subfield5;

#[derive(prost::Message)]
pub struct Clip {
	#[prost(message, tag = "1")]
	pub some_meta: Option<SomeMeta>,

	#[prost(uint32, tag = "2")]
	pub _uk2: u32,

	/// Unix timestamp
	#[prost(uint32, tag = "3")]
	pub _uk3: u32,

	/// Game ID
	#[prost(uint32, tag = "4")]
	pub _uk4: u32,

	#[prost(uint32, tag = "6")]
	pub _uk6: u32,

	#[prost(uint32, tag = "8")]
	pub _uk8: u32,

	/// Horizontal resolution
	#[prost(uint32, tag = "12")]
	pub _uk12: u32,


	/// Vertical resolution
	#[prost(uint32, tag = "13")]
	pub _uk13: u32,
}

#[derive(prost::Message)]
pub struct SomeMeta {
	#[prost(string, tag = "1")]
	pub timeline: String,

	#[prost(uint32, tag = "2")]
	pub game_id: u32,

	#[prost(uint32, tag = "3")]
	pub _uk32: u32,

	#[prost(uint32, tag = "4")]
	pub _uk4: u32,

	#[prost(message, tag = "5")]
	pub bg_data: Option<Subfield5>,
}