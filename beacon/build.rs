fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/common.proto")?;
    tonic_build::compile_protos("../proto/tasks.proto")?;
    Ok(tonic_build::compile_protos("../proto/iface.proto")?)
}
