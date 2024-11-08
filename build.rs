fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["proto/calculator.proto"], &["proto"])
        .unwrap_or_else(|e| {
            panic!("error compiling .proto: {e}");
        });
}
