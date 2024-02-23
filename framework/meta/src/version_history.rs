/// The last version to be used for upgrades and templates.
///
/// Should be edited every time a new version of the framework is released.
pub const LAST_VERSION: &str = "0.13.5";

/// Indicates where to stop with the upgrades.
pub const LAST_UPGRADE_VERSION: &str = LAST_VERSION;

pub const LAST_TEMPLATE_VERSION: &str = LAST_VERSION;

/// Known versions for the upgrader.
#[rustfmt::skip]
pub const VERSIONS: &[&str] = &[
     "0.9.2",
     "0.9.3",
     "0.9.4",
     "0.9.5",
     "0.9.6",
     "0.9.7",
     "0.9.8",
     "0.9.9",
     "0.10.0",
     "0.10.1",
     "0.10.2",
     "0.10.3",
     "0.10.4",     
     "0.10.5",
     "0.10.6",
     "0.10.7",
     "0.10.8",
     "0.10.9",
     "0.11.0",
     "0.11.1",
     "0.11.2",
     "0.11.3",
     "0.11.4",
     "0.11.5",
     "0.11.6",
     "0.11.7",
     "0.11.8",
     "0.11.9",
     "0.12.0",
     "0.12.1",
     "0.12.2",
     "0.12.3",
     "0.12.4",
     "0.12.5",
     "0.12.6",
     "0.12.7",
     "0.12.8",
     "0.12.9",
     "0.13.0",
     "0.13.1",
     "0.13.2",
     "0.13.3",
     "0.13.4",
     "0.13.5",
 ];

/// We started supporting contract templates with version 0.43.0.
pub fn template_versions() -> &'static [&'static str] {
    &VERSIONS[33..]
}

pub fn validate_template_tag(tag: &str) -> bool {
    template_versions().iter().any(|&tt| tt == tag)
}

pub fn template_versions_with_autogenerated_wasm() -> &'static [&'static str] {
    &VERSIONS[40..]
}

pub fn is_template_with_autogenerated_wasm(tag: &str) -> bool {
    template_versions_with_autogenerated_wasm()
        .iter()
        .any(|&tt| tt == tag)
}

pub fn template_versions_with_autogenerated_json() -> &'static [&'static str] {
    &VERSIONS[39..]
}

pub fn is_template_with_autogenerated_json(tag: &str) -> bool {
    template_versions_with_autogenerated_json()
        .iter()
        .any(|&tt| tt == tag)
}

pub struct VersionIterator {
    next_version: usize,
    last_version: String,
}

impl VersionIterator {
    fn is_last_version(&self, version: &str) -> bool {
        self.last_version == version
    }
}

impl Iterator for VersionIterator {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_version > 0 && self.next_version < VERSIONS.len() {
            let from_version = VERSIONS[self.next_version - 1];

            if self.is_last_version(from_version) {
                None
            } else {
                let to_version = VERSIONS[self.next_version];
                let result = (from_version, to_version);
                self.next_version += 1;
                Some(result)
            }
        } else {
            None
        }
    }
}

pub fn versions_iter(last_version: String) -> VersionIterator {
    VersionIterator {
        next_version: 1,
        last_version,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn template_versions_test() {
        assert_eq!(template_versions()[0], "0.43.0");

        assert!(validate_template_tag("0.43.0"));
        assert!(!validate_template_tag("0.12.4"));
    }

    #[test]
    fn template_versions_with_autogenerated_wasm_test() {
        assert_eq!(template_versions_with_autogenerated_wasm()[0], "0.13.2");

        assert!(is_template_with_autogenerated_wasm("0.13.2"));
        assert!(!is_template_with_autogenerated_wasm("0.44.0"));
    }

    #[test]
    fn template_versions_with_autogenerated_json_test() {
        assert_eq!(template_versions_with_autogenerated_json()[0], "0.44.0");

        assert!(is_template_with_autogenerated_json("0.44.0"));
        assert!(!is_template_with_autogenerated_json("0.43.0"));
    }
}
