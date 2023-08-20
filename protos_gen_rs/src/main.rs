use std::io::Result;

fn main() -> Result<()> {
    std::env::set_var("PROTOC", "/usr/local/bin/protoc");
    std::env::set_var("OUT_DIR", "./src/gen_rs");
    // TODO: support multi proto files
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "use serde::{Serialize, Deserialize};");
    config.type_attribute(".", "#[derive(Serialize, Deserialize)]");
    config.compile_protos(&["protos/books/book.proto", "protos/books/page.proto"], &["../"])?;
    // TODO: auto add generate rs to mod 

   Ok(())
}
