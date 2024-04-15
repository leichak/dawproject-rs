#![allow(unused)]
use {
    crate::{add_one_get, fake_rng},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

use super::{note::Note, time_unit::TimeUnit};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Notes {
    // Extends timeline
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // attribute
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>, // att
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>, // att
    #[serde(rename = "@track")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "$value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_sequence: Option<Vec<Note>>,
}

impl Notes {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            notes_sequence: Some(vec![]),
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use {super::Notes, quick_xml::se::to_string};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Notes::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
