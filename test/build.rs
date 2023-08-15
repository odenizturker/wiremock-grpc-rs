fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = ["./proto/hello.proto"];

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(&proto_files, &["proto"])?;

    Ok(())
}