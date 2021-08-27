fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/protos/")
        .compile(&["src/protos/api.proto"], &["src/protos"])?;

    return Ok(());
}
