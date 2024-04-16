#![allow(unused)]
use super::{
    audio::Audio, clip_slot::ClipSlot, clips::Clips, lanes::Lanes, markers::Markers, notes::Notes,
    points::Points, time_unit::TimeUnit, timeline::TimeLine, video::Video, warp::Warp,
    UpcastTimeline,
};

use {
    super::super::{add_one_get, fake_rng},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum WarpsSequenceEnum {
    Timeline(TimeLine),
    Lanes(Lanes),
    Notes(Notes),
    Clips(Clips),
    ClipSlot(ClipSlot),
    Markers(Markers),
    Warps(Warps),
    Audio(Audio),
    Video(Video),
    Points(Points),
    Warp(Warp),
}

type WarpsSequence = Vec<WarpsSequenceEnum>;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Warps {
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
    #[serde(rename = "@contentTimeUnit")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content_time_unit: Option<TimeUnit>,
    #[serde(rename = "$value", default)]
    pub warps_sequence: Option<WarpsSequence>,
}

impl Warps {
    pub fn new_test(time_unit: TimeUnit) -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            warps_sequence: Some(vec![]),
            content_time_unit: Some(time_unit),
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

impl UpcastTimeline for Warps {
    fn upcast(&self) -> TimeLine {
        TimeLine {
            id: self.id.clone(),
            name: self.name.clone(),
            color: self.color.clone(),
            comment: self.comment.clone(),
            track: self.track.clone(),
            time_unit: self.time_unit.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::TimeUnit, std::error::Error};

    use {super::Warps, quick_xml::se::to_string};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Warps::new_test(TimeUnit::Seconds);

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
