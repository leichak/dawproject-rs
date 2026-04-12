#![allow(unused)]

use {
    super::super::fake_rng,
    super::{lanes::ArrangementTypeChoiceEnum, time_unit::TimeUnit, timeline::TimeLine},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
type ClipSequenceChoice = Vec<ArrangementTypeChoiceEnum>;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Dummy)]
pub enum TimeLineEnum {
    TimeLine(TimeLine),
}

/// A time-positioned region of content (notes, audio, automation) within an arrangement.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Dummy)]
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
    #[serde(rename = "@enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
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
            enable: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

pub struct ClipBuilder {
    time: f64,
    duration: Option<f64>,
    name: Option<String>,
    color: Option<String>,
    content_time_unit: Option<TimeUnit>,
    play_start: Option<f64>,
    play_stop: Option<f64>,
    loop_start: Option<f64>,
    loop_end: Option<f64>,
    reference: Option<String>,
    enable: Option<bool>,
}

impl ClipBuilder {
    pub fn duration(mut self, duration: f64) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn content_time_unit(mut self, unit: TimeUnit) -> Self {
        self.content_time_unit = Some(unit);
        self
    }

    pub fn play_start(mut self, play_start: f64) -> Self {
        self.play_start = Some(play_start);
        self
    }

    pub fn play_stop(mut self, play_stop: f64) -> Self {
        self.play_stop = Some(play_stop);
        self
    }

    pub fn loop_range(mut self, start: f64, end: f64) -> Self {
        self.loop_start = Some(start);
        self.loop_end = Some(end);
        self
    }

    pub fn reference(mut self, reference: &str) -> Self {
        self.reference = Some(reference.to_string());
        self
    }

    pub fn enable(mut self, enable: bool) -> Self {
        self.enable = Some(enable);
        self
    }

    pub fn build(self) -> Clip {
        Clip {
            name: self.name,
            color: self.color,
            comment: None,
            notes_sequence_choice: None,
            time: self.time,
            duration: self.duration,
            content_time_unit: self.content_time_unit,
            play_start: self.play_start,
            play_stop: self.play_stop,
            loop_start: self.loop_start,
            loop_end: self.loop_end,
            fade_time_unit: None,
            fade_in_time: None,
            fade_out_time: None,
            reference: self.reference,
            enable: self.enable,
        }
    }
}

impl Clip {
    pub fn builder(time: f64) -> ClipBuilder {
        ClipBuilder {
            time,
            duration: None,
            name: None,
            color: None,
            content_time_unit: None,
            play_start: None,
            play_stop: None,
            loop_start: None,
            loop_end: None,
            reference: None,
            enable: None,
        }
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
