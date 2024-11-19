fn main() {
    std::fs::create_dir_all("src/protos").unwrap();
    protobuf_codegen::Codegen::new()
        .protoc()
        .include("protos")
        .input("protos/rendezvous.proto")
        .input("protos/message.proto")
        .out_dir("src/protos")
        .run_from_script();
}
