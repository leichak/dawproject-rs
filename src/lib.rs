//! # dawproject-rs
//!
//! Rust implementation of the [DAWproject](https://github.com/bitwig/dawproject) exchange format
//! for Digital Audio Workstations.
//!
//! A `.dawproject` file is a ZIP archive containing `project.xml` (the project structure)
//! and `metadata.xml` (song metadata), plus optional embedded audio files.
//!
//! All types are available through the [`api`] module.

pub mod api;

#[cfg(test)]
pub mod examples {
    use std::{collections::HashMap, path::Path};

    use crate::api::*;

    #[test]
    pub fn example_save_load() {
        let project = Project::new_fake();
        let meta_data = MetaData::new_fake();
        let embedded_files = HashMap::new();
        let path = Path::new("tests/test.dawproject");

        let _ = DawProject::save(project, meta_data, embedded_files, path);

        let _xmls = DawProject::load(path).unwrap();
    }
}
