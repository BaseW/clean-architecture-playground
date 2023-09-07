fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("src/grpc/proto/todo.proto")?;
    tonic_build::compile_protos("proto/todo.proto")?;
    Ok(())
}
