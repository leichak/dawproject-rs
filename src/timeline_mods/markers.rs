#![allow(unused)]
use {
    crate::{
        add_one_get, fake_rng,
        timeline_mods::{marker::Marker, time_unit::TimeUnit},
        track::Track,
    },
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum MarkersTrackEnum {
    Track(Track),
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Markers {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "$value", default)]
    track: Option<MarkersTrackEnum>,
    #[serde(rename = "@timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markers: Option<Vec<Marker>>,
}

impl Markers {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            markers: Some(vec![]),
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Markers, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Markers::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
