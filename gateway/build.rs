use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Couldn't get OUT_DIR env"));

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("gateway_descriptor.bin"))
        .compile_protos(&["proto/gateway.proto"], &["proto"])?;
    tonic_build::compile_protos("proto/gateway.proto")?;
    Ok(())
}
