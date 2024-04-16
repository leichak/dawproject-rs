#![allow(unused)]

use {
    super::super::fake_rng,
    super::{lanes::ArrangementTypeChoiceEnum, time_unit::TimeUnit, timeline::TimeLine},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
type ClipSequenceChoice = Vec<ArrangementTypeChoiceEnum>;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum TimeLineEnum {
    TimeLine(TimeLine),
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Clip {
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "$value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_sequence_choice: Option<ClipSequenceChoice>,
    #[serde(rename = "@time")]
    pub time: f64,
    #[serde(rename = "@duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(rename = "@contentTimeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content_time_unit: Option<TimeUnit>,
    #[serde(rename = "@playStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_start: Option<f64>,
    #[serde(rename = "@playStop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_stop: Option<f64>,
    #[serde(rename = "@loopStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_start: Option<f64>,
    #[serde(rename = "@loopEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_end: Option<f64>,
    #[serde(rename = "@fadeTimeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_time_unit: Option<TimeUnit>,
    #[serde(rename = "@fadeInTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in_time: Option<f64>,
    #[serde(rename = "@fadeOutTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out_time: Option<f64>,
    #[serde(rename = "@reference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl Clip {
    pub fn new_test(_content: TimeLine, time: f64, duration: f64) -> Self {
        Clip {
            name: None,
            color: None,
            comment: None,
            notes_sequence_choice: None,
            time,
            duration: Some(duration),
            content_time_unit: None,
            play_start: None,
            play_stop: None,
            loop_start: None,
            loop_end: None,
            fade_time_unit: None,
            fade_in_time: None,
            fade_out_time: None,
            reference: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::super::timeline::TimeLine, std::error::Error};

    use {super::Clip, quick_xml::se::to_string};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Clip::new_test(TimeLine::new_fake(), 1.0, 0.0);

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
