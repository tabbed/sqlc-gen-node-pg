use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use prost::Message;

pub mod plugin {
    include!(concat!(env!("OUT_DIR"), "/plugin.rs"));
}

pub fn deserialize_codegen_request(buf: &[u8]) -> Result<plugin::CodeGenRequest, prost::DecodeError> {
    plugin::CodeGenRequest::decode(&mut Cursor::new(buf))
}

pub fn serialize_codegen_response(resp: &plugin::CodeGenResponse) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(resp.encoded_len());

    resp.encode(&mut buf).unwrap();
    buf
}

pub fn create_codegen_response() -> plugin::CodeGenResponse {
    let mut file = plugin::File::default();
    file.name = "hello.txt".to_string();
    file.contents = "Hello World".as_bytes().to_vec();

    let mut resp = plugin::CodeGenResponse::default();
    resp.files.push(file);
    resp
}

fn main() -> Result<(), prost::DecodeError> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let buffer = stdin.fill_buf().unwrap();

    let _ = match deserialize_codegen_request(&buffer) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(_e) => std::process::exit(1),
    };

    let resp = create_codegen_response();
    let out = serialize_codegen_response(&resp);

    let _ = match io::stdout().write_all(&out) {
        Ok(result) => result,
        Err(_e) => std::process::exit(1),
    };

    Ok(())
}
