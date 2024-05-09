#![allow(unused)]
use {
    super::{super::add_one_get, super::fake_rng, super::timeline_mods::time_unit::TimeUnit},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct TimeLine {
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
}

impl TimeLine {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::TimeLine, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = TimeLine::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
