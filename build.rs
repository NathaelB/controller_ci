fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/scheduler/controller.proto"], &["proto/scheduler"])
        .expect("Building scheduler protobuf failed");
    Ok(())
}
