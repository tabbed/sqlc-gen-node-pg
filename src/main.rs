use std::io;
use std::io::prelude::*;
use std::io::Cursor;

use prost::Message;

use swc_ecma_ast::*;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_common::{
    sync::Lrc,
    SourceMap,
};

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
    let cm = Lrc::new(SourceMap::default());
	let mut buf = vec![];
	{
		let mut emitter = Emitter {
			cfg: swc_ecma_codegen::Config {
				..Default::default()
			},
			cm: cm.clone(),
			comments: None,
			wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
		};

        let fields = vec![
            TsTypeElement::TsPropertySignature(TsPropertySignature {
 				span: Default::default(),
                readonly: false,
                computed: false,
                optional: true,
                init: None,
                params: vec![],
                key: Box::new(Expr::Ident(Ident { 
 					span: Default::default(),
                    sym: "bar".into(),
                    optional: false,
                })),
                type_ann: Some(
                    TsTypeAnn {
 					    span: Default::default(),
 					    kind: TsStringKeyword,
                    },
                ),
                type_params: None,
            }),
        ];

		emitter
			.emit_module(&Module {
				body: vec![
                    ModuleItem::Stmt(
                        Stmt::Decl(
                            Decl::TsInterface(TsInterfaceDecl {
 						        span: Default::default(),
                                id: Ident { 
 						            span: Default::default(),
                                    sym: "Foo".into(), // HOW!?!?
                                    optional: false,
                                },
 						        type_params: None,
                                declare: false,
                                extends: vec![],
 						        body: TsInterfaceBody {
 						            span: Default::default(),
                                    body: fields,
                                },
                            }),
                        ),
                    ),
                ],
				span: Default::default(),
				shebang: None,
			})
			.unwrap();
	}

    let mut file = plugin::File::default();
    file.name = "hello.ts".to_string();
    file.contents = buf;

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
