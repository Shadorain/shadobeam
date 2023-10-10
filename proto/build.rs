fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./common.proto")?;
    tonic_build::compile_protos("./tasks.proto")?;
    Ok(tonic_build::compile_protos("./iface.proto")?)
}
