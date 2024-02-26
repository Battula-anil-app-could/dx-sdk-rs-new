use crate::{framework_version, framework_versions, version::FrameworkVersion};

/// The last version to be used for upgrades and templates.
///
/// Should be edited every time a new version of the framework is released.
pub const LAST_VERSION: FrameworkVersion = framework_version!(0.14.0);

/// Indicates where to stop with the upgrades.
pub const LAST_UPGRADE_VERSION: FrameworkVersion = LAST_VERSION;

pub const LAST_TEMPLATE_VERSION: FrameworkVersion = LAST_VERSION;

#[rustfmt::skip]
/// Known versions for the upgrader.
pub const VERSIONS: &[FrameworkVersion] = framework_versions![
      0.9.2,
      0.9.3,
      0.9.4,
      0.9.5,
      0.9.6,
      0.9.7,
      0.9.8,
      0.9.9,
      0.10.0,
      0.10.1,
      0.10.2,
      0.10.3,
      0.10.4,
      0.10.5,
      0.10.7,
      0.10.6,
      0.10.7,
      0.10.8,
      0.10.9,
      0.11.0,
      0.11.1,
      0.11.2,
      0.11.3,
      0.11.4,
      0.11.5,
      0.11.6,
      0.11.7,
      0.11.8,
      0.11.9,
      0.12.0,
      0.12.1,
      0.12.2,
      0.12.3,
      0.12.4,
      0.12.5,
      0.12.6,
      0.12.7,
      0.12.8,
      0.12.9,
      0.13.0,
      0.13.1,
      0.13.2,
      0.13.3,
      0.13.4,
      0.13.5,
      0.13.6,
      0.13.7,
      0.13.8,
      0.13.9,
      0.14.0,
  ];

 #[rustfmt::skip]
 pub const CHECK_AFTER_UPGRADE_TO: &[FrameworkVersion] = framework_versions![
     0.9.2,
     0.9.3,
     0.9.6,
     0.9.7,
     0.9.9,
     0.10.0,
     0.10.2,
     0.10.4,
     0.10.5,
     0.10.7,
     0.11.8,
     0.11.9,
     0.12.4,
     0.12.5,
     0.13.1,
     0.13.4,
     0.13.5,
     0.13.7,
     0.13.8,
     0.13.9,
     0.14.0,
 ];

pub const LOWER_VERSION_WITH_TEMPLATE_TAG: FrameworkVersion = framework_version!(0.12.5);
pub const LOWER_VERSION_WITH_AUTOGENERATED_JSON: FrameworkVersion = framework_version!(0.13.1);
pub const LOWER_VERSION_WITH_AUTOGENERATED_WASM: FrameworkVersion = framework_version!(0.13.2);

pub fn parse_known_version(tag_str: &str) -> FrameworkVersion {
    let tag: FrameworkVersion = FrameworkVersion::from_string_template(tag_str);
    if VERSIONS.contains(&tag) {
        tag
    } else {
        panic!("Version unknown")
    }
}

/// We started supporting contract templates with version 0.12.5.
pub fn validate_template_tag(tag_str: &str) -> bool {
    let tag: FrameworkVersion = parse_known_version(tag_str);

    tag >= LOWER_VERSION_WITH_TEMPLATE_TAG
}

pub fn is_template_with_autogenerated_wasm(tag: FrameworkVersion) -> bool {
    tag >= LOWER_VERSION_WITH_AUTOGENERATED_WASM
}

pub fn is_template_with_autogenerated_json(tag: FrameworkVersion) -> bool {
    tag >= LOWER_VERSION_WITH_AUTOGENERATED_JSON
}

pub fn find_version_by_str(tag: &str) -> Option<&FrameworkVersion> {
    VERSIONS.iter().find(|&v| v.to_string() == tag)
}

pub struct VersionIterator {
    next_version: usize,
    last_version: FrameworkVersion,
}

impl VersionIterator {
    fn is_last_version(&self, version: &FrameworkVersion) -> bool {
        self.last_version == *version
    }
}

impl Iterator for VersionIterator {
    type Item = (&'static FrameworkVersion, &'static FrameworkVersion);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_version > 0 && self.next_version < VERSIONS.len() {
            let from_version = &VERSIONS[self.next_version - 1];

            if self.is_last_version(from_version) {
                None
            } else {
                let to_version = &VERSIONS[self.next_version];
                let result = (from_version, to_version);
                self.next_version += 1;
                Some(result)
            }
        } else {
            None
        }
    }
}

pub fn versions_iter(last_version: FrameworkVersion) -> VersionIterator {
    VersionIterator {
        next_version: 1,
        last_version,
    }
}

#[cfg(test)]
pub mod tests {

    use crate::version::is_sorted;

    use super::*;

    #[test]
    fn compare_versions_test() {
        let f1: FrameworkVersion = framework_version!(0.13.1);
        let f2: FrameworkVersion = framework_version!(0.12.2);

        assert!(f1 > f2);
    }

    #[test]
    fn framework_version_display_test() {
        assert_eq!(format!("Framework: {}", VERSIONS[0]), "Framework: 0.9.2");
    }

    #[test]
    fn template_versions_test() {
        assert!(validate_template_tag("0.12.5"));
        assert!(!validate_template_tag("0.12.4"));
    }

    #[test]
    fn template_versions_with_autogenerated_wasm_test() {
        assert!(is_template_with_autogenerated_wasm(framework_version!(
            0.13.2
        )));
        assert!(!is_template_with_autogenerated_wasm(framework_version!(
            0.13.1
        )));
    }

    #[test]
    fn template_versions_with_autogenerated_json_test() {
        assert!(is_template_with_autogenerated_json(framework_version!(
            0.13.1
        )));
        assert!(!is_template_with_autogenerated_json(framework_version!(
            0.12.5
        )));
    }

    #[test]
    fn find_version_by_str_test() {
        let version = find_version_by_str("0.9.2");
        match version {
            Some(v) => assert_eq!(VERSIONS[0], *v),
            None => unreachable!(),
        }
    }

   
}
