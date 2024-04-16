#![allow(unused)]

use {
    super::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct FileReference {
    #[serde(rename = "@path")]
    pub path: String,
    #[serde(rename = "@external")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
}

impl FileReference {
    pub fn new_test(path: String) -> Self {
        Self {
            path,
            external: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::FileReference, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = FileReference::new_test("/test/test/file.daw".to_string());

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
