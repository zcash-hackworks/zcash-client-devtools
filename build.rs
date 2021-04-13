
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/compact_formats.proto", "proto/service.proto"], &["proto"])?;
    Ok(())
}
