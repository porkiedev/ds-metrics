
fn main() {
    let proto_files = &["proto/api.proto"];

    // Rerun the build script if any of the proto files change
    for proto in proto_files {
        println!("cargo:rerun-if-changed={}", proto);
    }

    tonic_build::configure()
        .compile_protos(proto_files, &["proto"])
        .unwrap();
}