mod helpers;

use helpers::{cleanup_dir, exec_pike, PLUGIN_DIR, TESTS_DIR};
use std::{
    fs,
    path::{Path, PathBuf},
    vec,
};

pub const PACK_PLUGIN_NAME: &str = "test-pack-plugin";

#[test]
fn test_cargo_pack() {
    cleanup_dir(&Path::new(TESTS_DIR).join(PACK_PLUGIN_NAME));

    exec_pike(vec!["plugin", "new", PACK_PLUGIN_NAME], TESTS_DIR, &vec![]);

    exec_pike(
        vec!["plugin", "pack"],
        Path::new(TESTS_DIR).join(PACK_PLUGIN_NAME),
        &vec!["--target-dir".to_string(), "tmp_target".to_string()],
    );

    // Hail for archive handling in Rust
    let plugin_path = Path::new(TESTS_DIR)
        .join(PACK_PLUGIN_NAME)
        .join("tmp_target");
    helpers::unpack_archive(
        &plugin_path
            .join("release")
            .join("test_pack_plugin-0.1.0.tar.gz"),
        &plugin_path,
    );

    let base_file_path = plugin_path.join("test_pack_plugin").join("0.1.0");
    assert!(base_file_path.join("libtest_pack_plugin.so").exists());
    assert!(base_file_path.join("manifest.yaml").exists());
    assert!(base_file_path.join("migrations").is_dir());
}

#[test]
fn test_cargo_pack_assets() {
    let pack_plugin_path = Path::new(TESTS_DIR).join(PACK_PLUGIN_NAME);
    cleanup_dir(&pack_plugin_path);

    exec_pike(vec!["plugin", "new", PACK_PLUGIN_NAME], TESTS_DIR, &vec![]);

    // Change build script for sub plugin to test custom assets
    fs::copy(
        Path::new(TESTS_DIR)
            .parent()
            .unwrap()
            .join("assets")
            .join("custom_assets_build.rs"),
        pack_plugin_path.join("build.rs"),
    )
    .unwrap();

    // release build
    exec_pike(
        vec!["plugin", "pack"],
        Path::new(TESTS_DIR).join(PACK_PLUGIN_NAME),
        &vec![],
    );

    // check release archive
    let unzipped_dir = pack_plugin_path.join("unzipped_release");
    let base_file_path = unzipped_dir.join("test_pack_plugin").join("0.1.0");

    helpers::unpack_archive(
        &pack_plugin_path
            .join("target")
            .join("release")
            .join("test_pack_plugin-0.1.0.tar.gz"),
        &unzipped_dir,
    );

    assert!(base_file_path.join("libtest_pack_plugin.so").exists());
    assert!(base_file_path.join("manifest.yaml").exists());
    assert!(base_file_path.join("migrations").is_dir());
    assert!(base_file_path.join("Cargo.toml").exists());

    // debug build
    exec_pike(
        vec!["plugin", "pack"],
        Path::new(TESTS_DIR).join(PACK_PLUGIN_NAME),
        &vec!["--debug".into()],
    );

    // check debug archive
    let unzipped_dir = pack_plugin_path.join("unzipped_debug");
    let base_file_path = unzipped_dir.join("test_pack_plugin").join("0.1.0");

    helpers::unpack_archive(
        &pack_plugin_path
            .join("target")
            .join("debug")
            .join("test_pack_plugin-0.1.0.tar.gz"),
        &unzipped_dir,
    );

    assert!(base_file_path.join("libtest_pack_plugin.so").exists());
    assert!(base_file_path.join("manifest.yaml").exists());
    assert!(base_file_path.join("migrations").is_dir());
    assert!(base_file_path.join("Cargo.toml").exists());
}

#[test]
fn test_cargo_plugin_new() {
    let root_dir = PathBuf::new().join(PLUGIN_DIR);
    cleanup_dir(&root_dir);

    // Test creating simple plugin
    exec_pike(vec!["plugin", "new", "test-plugin"], TESTS_DIR, &vec![]);

    assert!(root_dir.join("picodata.yaml").exists());
    assert!(root_dir.join(".git").exists());
    assert!(root_dir.join("topology.toml").exists());
    assert!(root_dir.join("manifest.yaml.template").exists());

    cleanup_dir(&root_dir);

    // Test creating plugin without git
    exec_pike(
        vec!["plugin", "new", "test-plugin", "--without-git"],
        TESTS_DIR,
        &vec![],
    );

    assert!(!root_dir.join(".git").exists());

    cleanup_dir(&Path::new(PLUGIN_DIR).to_path_buf());

    // Test creating plugin as workspace
    exec_pike(
        vec!["plugin", "new", "test-plugin", "--workspace"],
        TESTS_DIR,
        &vec![],
    );

    let subcrate_path = Path::new(PLUGIN_DIR).join("test-plugin");
    assert!(subcrate_path.exists());

    assert!(root_dir.join(".cargo").join("config.toml").exists());
    assert!(!subcrate_path.join(".cargo").exists());

    assert!(root_dir.join("picodata.yaml").exists());
    assert!(!subcrate_path.join("picodata.yaml").exists());

    assert!(root_dir.join("topology.toml").exists());
    assert!(!subcrate_path.join("topology.toml").exists());

    assert!(root_dir.join(".git").exists());
    assert!(!subcrate_path.join(".git").exists());

    assert!(root_dir.join(".gitignore").exists());
    assert!(!subcrate_path.join(".gitignore").exists());

    assert!(root_dir.join("rust-toolchain.toml").exists());
    assert!(!subcrate_path.join("rust-toolchain.toml").exists());

    assert!(root_dir.join("tmp").exists());
    assert!(!subcrate_path.join("tmp").exists());

    let contents = fs::read_to_string(root_dir.join("Cargo.toml")).unwrap();
    assert!(contents.contains("[workspace]"));
}
