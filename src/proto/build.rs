use std::env;
use std::io::Result;
fn main() -> Result<()> {
    env::set_var("OUT_DIR", "src/proto");
    prost_build::compile_protos(&["src/proto/word_data.proto"], &["src/"])?;
    Ok(())
}
