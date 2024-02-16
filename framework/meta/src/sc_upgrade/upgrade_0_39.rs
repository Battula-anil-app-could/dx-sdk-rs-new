use std::path::Path;

use crate::CargoTomlContents;
use colored::Colorize;
use ruplacer::Query;
use toml::{value::Table, Value};

use super::{
    folder_structure::{DirectoryToUpdate, DirectoryType},
    upgrade_common::{rename_files, replace_in_files},
};

#[rustfmt::skip]
pub const SCENARIO_FILE_PATTERNS: &[(&str, &str)] = &[
    ("denali_go", "scenario_go"), 
    ("denali_rs", "scenario_rs"),
];

/// All `0.38.0` to `0.39.0` transformations other than the version bump.
pub(crate) fn upgrade_39(dir: &DirectoryToUpdate) {
    if dir.dir_type == DirectoryType::Contract {
        v_0_39_prepare_meta(&dir.path);
        v_0_39_prepare_wasm(&dir.path);
    }
    v_0_39_replace_in_files(&dir.path);
    rename_files(dir.path.as_ref(), SCENARIO_FILE_PATTERNS);
}

fn v_0_39_prepare_meta(sc_crate_path: &Path) {
    let cargo_toml_path = sc_crate_path.join("meta/Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let mut meta_cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let deps = meta_cargo_toml.dependencies_mut();

    println!(
        "{}/dependencies/{}",
        cargo_toml_path.as_path().display(),
        "dharitri-wasm".red().strikethrough(),
    );
    deps.remove("dharitri-wasm");

    println!(
        "{}/dependencies/{}",
        cargo_toml_path.as_path().display(),
        "dharitri-wasm-debug".red().strikethrough(),
    );
    deps.remove("dharitri-wasm-debug");

    println!(
        "{}/dependencies/{}",
        cargo_toml_path.as_path().display(),
        "dharitri-sc-meta".green(),
    );
    let mut meta_dep = Table::new();
    meta_dep.insert("version".to_string(), Value::String("0.39.0".to_string()));
    deps.insert("dharitri-sc-meta".to_string(), Value::Table(meta_dep));

    meta_cargo_toml.save_to_file(&cargo_toml_path);
}

fn v_0_39_prepare_wasm(sc_crate_path: &Path) {
    let cargo_toml_path = sc_crate_path.join("wasm/Cargo.toml");
    assert!(
        cargo_toml_path.exists(),
        "SC crate Cargo.toml not found: {}",
        cargo_toml_path.display()
    );
    let mut meta_cargo_toml = CargoTomlContents::load_from_file(&cargo_toml_path);
    let deps = meta_cargo_toml.dependencies_mut();

    println!(
        "{}/dependencies/{}",
        cargo_toml_path.as_path().display(),
        "dharitri-wasm-output".red().strikethrough(),
    );
    deps.remove("dharitri-wasm-output");

    meta_cargo_toml.save_to_file(&cargo_toml_path);
}

fn v_0_39_replace_in_files(sc_crate_path: &Path) {
    replace_in_files(
        sc_crate_path,
        "*Cargo.toml",
        &[
            Query::substring("dharitri-wasm-debug", "dharitri-sc-scenario"),
            Query::substring("dharitri-wasm-modules", "dharitri-sc-modules"),
            Query::substring("dharitri-wasm-node", "dharitri-sc-wasm-adapter"),
            Query::substring("dharitri-wasm", "dharitri-sc"),
        ][..],
    );

    replace_in_files(
        sc_crate_path,
        "*rs",
        &[
            Query::substring("dharitri_codec", "codec"),
            Query::substring(
                "dharitri_wasm_debug::meta::perform",
                "dharitri_sc_meta::cli_main",
            ),
            Query::substring(
                "dharitri_wasm_debug::denali_go",
                "dharitri_sc_scenario::run_go",
            ),
            Query::substring(
                "dharitri_wasm_debug::denali_rs",
                "dharitri_sc_scenario::run_rs",
            ),
            Query::substring("dharitri_wasm_debug", "dharitri_sc_scenario"),
            Query::substring("dharitri_wasm_modules", "dharitri_sc_modules"),
            Query::substring("dharitri_wasm_node", "dharitri_sc_wasm_adapter"),
            Query::substring("dharitri_wasm", "dharitri_sc"),
            Query::substring("BlockchainMock", "ScenarioWorld"),
            Query::substring("testing_framework", "whitebox"),
            Query::substring("tx_mock", "whitebox"),
        ][..],
    );
}
