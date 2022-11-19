fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/client_daemon.proto")?;
    println!("cargo:rerun-if-changed=migrations");
    Ok(())
}
