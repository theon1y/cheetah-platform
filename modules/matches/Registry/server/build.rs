use std::io::Error;

fn main() -> Result<(), Error> {
    println!(
        "cargo:rerun-if-changed=../proto/matches.registry.internal.proto"
    );
    println!("cargo:rerun-if-changed=../proto/matches.relay.internal.proto");
    tonic_build::configure().build_client(false).compile(
        &["../../Registry/proto/matches.registry.internal.proto"],
        &["../../Registry/proto/"],
    )?;

    tonic_build::configure().build_server(false).compile(
        &["../../Realtime/proto/matches.relay.internal.proto"],
        &["../../Realtime/proto/"],
    )?;

    Ok(())
}
