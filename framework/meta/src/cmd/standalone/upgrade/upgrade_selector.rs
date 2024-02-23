use crate::{
    cli_args::UpgradeArgs,
    folder_structure::{dir_pretty_print, RelevantDirectories, RelevantDirectory},
    version_history::{versions_iter, LAST_UPGRADE_VERSION, VERSIONS},
};

use super::{
    upgrade_0_31::upgrade_to_9_7,
    upgrade_0_32::upgrade_to_9_9,
    upgrade_0_39::{postprocessing_after_10_9, upgrade_to_10_9},
    upgrade_0_45::upgrade_to_13_2,
    upgrade_common::{cargo_check, version_bump_in_cargo_toml},
    upgrade_print::*,
};

pub fn upgrade_sc(args: &UpgradeArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    let last_version = args
        .override_target_version
        .clone()
        .unwrap_or_else(|| LAST_UPGRADE_VERSION.to_string());

    assert!(
        VERSIONS.contains(&last_version.as_str()),
        "Invalid requested version: {last_version}",
    );

    let mut dirs = RelevantDirectories::find_all(path, args.ignore.as_slice());
    println!(
        "Found {} directories to upgrade, out of which {} are contract crates.\n",
        dirs.len(),
        dirs.iter_contract_crates().count(),
    );
    dir_pretty_print(dirs.iter(), "", &|dir| {
        print_tree_dir_metadata(dir, last_version.as_str())
    });

    for (from_version, to_version) in versions_iter(last_version) {
        if dirs.count_for_version(from_version) == 0 {
            continue;
        }

        print_upgrading_all(from_version, to_version);
        dirs.start_upgrade(from_version, to_version);
        for dir in dirs.iter_version(from_version) {
            upgrade_function_selector(dir);
        }

        for dir in dirs.iter_version(from_version) {
            upgrade_post_processing(dir);
        }

        // // change the version in memory for the next iteration (dirs is not reloaded from disk)
        // dirs.update_versions_in_memory(from_version, to_version);
        dirs.finish_upgrade();
    }
}

fn upgrade_function_selector(dir: &RelevantDirectory) {
    if dir.upgrade_in_progress.is_some() {
        print_upgrading(dir);
    }

    match dir.upgrade_in_progress {
        Some((_, "0.9.7")) => {
            upgrade_to_9_7(dir);
        },
        Some((_, "0.9.9")) => {
            upgrade_to_9_9(dir);
        },
        Some((_, "0.10.9")) => {
            upgrade_to_10_9(dir);
        },
        Some((_, "0.13.2")) => {
        upgrade_to_13_2(dir);
        },
        Some((from_version, to_version)) => {
            version_bump_in_cargo_toml(&dir.path, from_version, to_version);
        },
        None => {},
    }
}



fn upgrade_post_processing(dir: &RelevantDirectory) {
    match dir.upgrade_in_progress {
        Some((_, "0.9.2")) | Some((_, "0.9.3")) | Some((_, "0.9.6")) | Some((_, "0.9.7"))
        | Some((_, "0.9.9")) | Some((_, "0.10.3")) | Some((_, "0.10.2")) | Some((_, "0.10.4"))
        | Some((_, "0.10.5")) | Some((_, "0.10.7")) | Some((_, "0.11.8")) | Some((_, "0.12.0"))
        | Some((_, "0.12.4")) | Some((_, "0.12.5")) | Some((_, "0.13.1")) | Some((_, "0.13.3")) => {
            print_post_processing(dir);
            cargo_check(dir);
        },
        Some((_, "0.10.9")) => {
            print_post_processing(dir);
            postprocessing_after_10_9(dir);
            cargo_check(dir);
        },
        _ => {},
    }
}