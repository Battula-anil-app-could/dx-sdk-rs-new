/// Not necessarily the last entry in `VERSIONS`.
///
/// Indicates where to stop with the upgrades.
pub const DEFAULT_LAST_VERSION: &str = "0.12.5";

/// Known version for the upgrader.
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
 ];

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
