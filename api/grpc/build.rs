use std::{env, fs};

const PROTO_DIR: &str = "src/proto";

fn main() {
    let proto_dir = env::current_dir()
        .expect("Failed to get current directory")
        .join(PROTO_DIR)
        .canonicalize()
        .expect("Failed to canonicalize path");

    let protos: Vec<_> = fs::read_dir(&proto_dir)
        .expect("Failed to read directory")
        .filter_map(|f| {
            let f = f.ok()?;
            let proto_path = f.path();
            if proto_path
                .extension()
                .map(|e| e == "proto")
                .unwrap_or(false)
            {
                return Some(proto_dir.join(proto_path));
            }
            None
        })
        .collect();

    for f in &protos {
        println!("cargo:rerun-if-changed={}", f.display());
    }

    tonic_build::configure()
        .compile(&protos, &[proto_dir])
        .expect("Failed to compile protos");
}
