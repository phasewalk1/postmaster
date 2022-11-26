use std::path::Path;

static PROTO: &'static str = "../proto";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir("src/pb")
        .compile(&[Path::new(PROTO).join("msg.proto")], &[Path::new(PROTO)])?;
    Ok(())
}
