use std::env::args;
use std::fs;
use prost::Message;
use srep::game_recording::GameRecording;

fn main() {
	let path =  args().nth(1).expect("No Path passed");
	// let input_dir = fs::read_dir(&path).unwrap().into_iter().collect();


	// Example binary stream (Protobuf-encoded data)
	let bin_data = fs::read(&path).unwrap();

	// Decode the binary stream into the struct
	let message = GameRecording::decode(bin_data.as_ref()).expect("Failed to decode Protobuf message");

	dbg!(&message);
}