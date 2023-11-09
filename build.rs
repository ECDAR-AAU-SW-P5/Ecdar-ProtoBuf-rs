use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let build = tonic_build::configure();

    #[cfg(not(feature = "server"))]
    let build = build.build_server(false);

    #[cfg(not(feature = "client"))]
    let build = build.build_client(false);

    #[cfg(feature = "serde")]
    let build = build.type_attribute(
        ".",
        "#[derive(serde::Deserialize, serde::Serialize)]\n#[serde(rename_all=\"camelCase\")]",
    ).enum_attribute(
        ".", "#[serde(tag=\"type\", content=\"content\")]");

    build
        .emit_rerun_if_changed(true)
        .compile(&["Ecdar-ProtoBuf/services.proto"], &["Ecdar-ProtoBuf"])?;

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
