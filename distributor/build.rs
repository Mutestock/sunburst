fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/article.proto")?;
    tonic_build::compile_protos("../proto/basic.proto")?;
    Ok(())
}