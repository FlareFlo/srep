#[derive(prost::Message)]
pub struct GameRecording {
	#[prost(message, repeated, tag = "1")]
	pub background_recording: Vec<BackgroundRecording>,

	#[prost(message, repeated, tag = "2")]
	pub user_recordings: Vec<UserRecording>,
}

#[derive(prost::Message)]
pub struct BackgroundRecording {
	/// Name of json file related to this replay
	#[prost(string, tag = "1")]
	pub timeline: String,

	/// Steam game-id
	#[prost(uint32, tag = "2")]
	pub game_id: u32,

	/// Unix timestamp
	#[prost(uint32, tag = "3")]
	pub _uk1: u32,

	#[prost(uint32, tag = "4")]
	pub _uk2: u32,

	#[prost(message, repeated, tag = "5")]
	pub subfields_5: Vec<Subfield5>,

	// Most likely some form of in-game events
	#[prost(message, repeated, tag = "7")]
	pub subfields_7: Vec<Subfield7>,
}

#[derive(prost::Message)]
pub struct UserRecording {
	/// Steam game-id
	#[prost(uint32, tag = "1")]
	pub game_id: u32,

	#[prost(message, tag = "2")]
	pub subfields: Option<UserRecordingMetadata>,
}

#[derive(prost::Message)]
pub struct Subfield7 {
	/// Looks like some Enum descriptor, tends to be 2 or 3
	#[prost(uint32, tag = "3")]
	pub _uk3: u32,

	#[prost(uint32, tag = "5")]
	pub _uk5: u32,

	#[prost(uint32, tag = "6")]
	pub _uk6: u32,

	#[prost(uint32, tag = "7")]
	pub _uk7: u32,
}

#[derive(prost::Message)]
pub struct Subfield5 {
	/// Folder path in /video/_uk1
	/// Format: bg_$GAMEID_$YEAR$MONTH$DAY_$HOUR$MINUTE$SECOND
	/// Example: bg_1621690_20240910_215420_0
	/// Example: bg_1621690_20240910_233148_0
	/// Example: bg_1621690_20240910_233357
	#[prost(string, tag = "1")]
	pub path: String,

	#[prost(uint32, tag = "2")]
	pub _uk2: u32,

	#[prost(uint32, tag = "3")]
	pub _uk3: u32,

	/// Looks like some enum descriptor
	#[prost(uint32, tag = "4")]
	pub _uk4: u32,

	/// Size of folder in bytes
	#[prost(uint32, optional, tag = "9")]
	pub _uk9: Option<u32>,

	#[prost(uint32, tag = "10")]
	pub _uk10: u32,
}

#[derive(prost::Message)]
pub struct UserRecordingMetadata {
	/// Game ID
	#[prost(uint32, tag = "1")]
	pub _uk1: u32,

	/// Unix timestamp (start time)
	#[prost(uint32, tag = "2")]
	pub _uk2: u32,

	/// Looks like some Enum descriptor, tends to be 2 or 3
	#[prost(uint32, tag = "3")]
	pub _uk3: u32,

	/// Path to timeline file /timelines/$PATH
	/// $PATH is formatted as: timeline_$GAMEID$YEAR$MONTH$DAY_$HOUR$MINUTE$SECOND
	#[prost(string, tag = "4")]
	pub _uk4: String,

	#[prost(uint32, tag = "5")]
	pub _uk5: u32,

	#[prost(uint32, tag = "6")]
	pub _uk6: u32,

	#[prost(uint32, tag = "7")]
	pub _uk7: u32,

	#[prost(bytes, tag = "8")]
	pub _uk8: Vec<u8>,

	#[prost(bytes, tag = "9")]
	pub _uk9: Vec<u8>,

	#[prost(uint32, tag = "10")]
	pub _uk10: u32,
}