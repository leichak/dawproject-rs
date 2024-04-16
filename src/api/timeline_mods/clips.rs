#![allow(unused)]

use super::clip::Clip;

use {
    super::super::{add_one_get, fake_rng},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

use super::time_unit::TimeUnit;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum ClipEnum {
    Clip(Clip),
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Clips {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "@track")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    #[serde(rename = "@timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "$value", default)]
    pub clips: Option<Vec<ClipEnum>>,
}

impl Clips {
    pub fn new_test(clips: Vec<Clip>) -> Self {
        Clips {
            id: Some(format!("id_{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            clips: Some(clips.into_iter().map(ClipEnum::Clip).collect()),
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Clips, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Clips::new_test(vec![]);

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
