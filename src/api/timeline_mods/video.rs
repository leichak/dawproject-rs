#![allow(unused)]
use {
    super::super::{add_one_get, fake_rng, file_reference::FileReference},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

use super::time_unit::TimeUnit;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Video {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>, // attribute
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>, // att
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>, // att
    #[serde(rename = "@track")]
    #[serde(skip_serializing_if = "Option::is_none")]
    track: Option<String>,
    #[serde(rename = "@timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub time_unit: Option<TimeUnit>,
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    files_sequence: Option<Vec<FileReference>>,
    #[serde(rename = "@duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f64>,
    #[serde(rename = "@algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<String>,
    #[serde(rename = "@channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    channels: Option<i32>,
    #[serde(rename = "@sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<i32>,
}

impl Video {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            files_sequence: None,
            duration: None,
            algorithm: None,
            channels: None,
            sample_rate: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Video, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Video::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
