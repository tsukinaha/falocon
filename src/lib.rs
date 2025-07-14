mod args;
mod generator;
#[cfg(test)]
mod test;

use args::Args;
use clap::Parser;
pub use generator::*;

pub fn run() {

    let args = Args::parse();

    args.init_tracing();

    let generator = OpenAPIGenerator::from_json(
        &std::fs::read_to_string(&args.json_path).expect("Failed to read OpenAPI file"),
    );

    tracing::info!("Generating code...");

    let methods = generator.gen_methods();

    let structs = generator.gen_types();

    tracing::info!("Code generation completed");

    let output_path = args.output_path();

    CrateWriter::new(output_path, structs, methods)
        .write()
        .expect("Failed to write crate");

    Formatter::new(output_path).format();

    tracing::warn!(
        "Code generation completed successfully. You should write your Client manually, refer to the src/client.rs"
    );
}
