#![allow(unused)]

use {
    super::lanes::ArrangementTypeChoiceEnum,
    crate::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
type NoteSequenceChoice = Vec<ArrangementTypeChoiceEnum>;
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub(crate) struct Note {
    #[serde(rename = "@time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    #[serde(rename = "@duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "@channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
    #[serde(rename = "@key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<i32>,
    #[serde(rename = "@velocity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vel: Option<f64>,
    #[serde(rename = "@releaseVelocity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<f64>,
    #[serde(rename = "$value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_sequence_choice: Option<NoteSequenceChoice>,
}

impl Note {
    pub fn new_test() -> Self {
        Self {
            notes_sequence_choice: None,
            time: None,
            duration: None,
            channel: None,
            key: None,
            vel: None,
            rel: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Note, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Note::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
