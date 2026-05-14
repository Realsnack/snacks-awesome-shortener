extern crate prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(
        &[
            "../proto/common/v1/shorts.proto",
            "../proto/messaging/v1/events.proto",
            "../proto/messaging/v1/commands.proto",
        ],
        &["../proto"],
    )?;

    Ok(())
}
