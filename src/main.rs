use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use prost::Message;

pub mod plugin {
    include!(concat!(env!("OUT_DIR"), "/plugin.rs"));
}

pub fn deserialize_greeter(buf: &[u8]) -> Result<plugin::CodeGenRequest, prost::DecodeError> {
    plugin::CodeGenRequest::decode(&mut Cursor::new(buf))
}

fn main() -> Result<(), prost::DecodeError> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let buffer = stdin.fill_buf().unwrap();

    let request_deserialized_result = match deserialize_greeter(&buffer) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(e) => return Err(e),
    };
    println!("{:#?}", request_deserialized_result);
    Ok(())
}
