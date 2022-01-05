use std::path::PathBuf;

fn main() {
    let output = PathBuf::from("src");
    tonic_build::configure()
        .out_dir(output)
        .compile(
            &[
                "idl/service/kv.proto",
                "idl/service/event.proto",
                "idl/service/keepalive.proto",
            ],
            &["idl"],
        )
        .unwrap()
}
