use crate::*;

#[test]

fn test_emby() {

    let data = include_str!("../tests/openapi.json");

    let generator = OpenAPIGenerator::from_json(data);

    let methods = generator.gen_methods();

    let structs = generator.gen_types();

    CrateWriter::new("./client", structs, methods)
        .write()
        .expect("Failed to write crate");
}
