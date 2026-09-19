fn main() {
    emit_cli_version();
    println!("cargo:rerun-if-changed=../../../termsurf/proto/termsurf.proto");

    prost_build::Config::new()
        .compile_protos(
            &["../../../termsurf/proto/termsurf.proto"],
            &["../../../termsurf/proto/"],
        )
        .unwrap();
}

fn emit_cli_version() {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    println!("cargo:rustc-env=ASTROHACKER_CLI_VERSION={version}");
}
