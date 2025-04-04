use pike::helpers::build;

// Custom build script for plugin, which stores
// plugin's artefacts in corresponding folder
//
// Call of build::main() function is MANDATORY
// for proper artefact storage and packing

fn main() {
    let params = build::ParamsBuilder::default()
        .custom_assets_named(vec![
            ("Cargo.toml", "not.cargo"),
            ("src", "no/src"),
            ("Cargo.lock", "no/src/Cargo.unlock"),
        ])
        .custom_assets(vec!["Cargo.toml"])
        .build()
        .unwrap();
    build::main(&params);
}
