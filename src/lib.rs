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
