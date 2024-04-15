#![allow(unused)]

use {
    fake::Dummy,
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Audio,
    Automation,
    Notes,
    Video,
    Markers,
    Tracks,
}

#[cfg(test)]
mod tests {
    use {super::ContentType, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = ContentType::Audio;

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
