#![allow(unused)]
use {
    crate::{
        add_one_get, fake_rng,
        timeline_mods::{lanes::Lanes, markers::Markers, points::Points},
    },
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum ArrangementSequenceEnum {
    Points(Points),

    Lanes(Lanes),
    Markers(Markers),
    TempoAutomation(Points),
}

type ArrangementSequence = Vec<ArrangementSequenceEnum>;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Arrangement {
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
    pub sequence: Option<ArrangementSequence>,
}

impl Arrangement {
    pub fn new_test() -> Self {
        Arrangement {
            id: Some(format!("id_{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            sequence: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Arrangement, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Arrangement::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
