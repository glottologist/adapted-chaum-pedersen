extern crate prost_build;
use std::io::Result;
fn main() -> Result<()> {
    tonic_build::compile_protos("src/proto/zkp_auth.proto")?;
    Ok(())
}
