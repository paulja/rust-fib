use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // for reflection
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_prost_build::configure()
        .build_server(true)
        // for reflection
        .file_descriptor_set_path(out_dir.join("fib_descriptor.bin"))
        .compile_protos(&["proto/fib.proto"], &["proto"])?;
    Ok(())
}
