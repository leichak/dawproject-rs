#![allow(unused)]
use super::{
    super::unit::Unit, automation_target::AutomationTarget, bool_point::BoolPoint,
    enum_point::EnumPoint, integer_point::IntegerPoint, point::Point, real_point::RealPoint,
    time_signature_point::TimeSignaturePoint, time_unit::TimeUnit, timeline::TimeLine,
    UpcastTimeline,
};

use {
    crate::{add_one_get, fake_rng},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum PointsTypeEnum {
    Point(Point),
    RealPoint(RealPoint),
    EnumPoint(EnumPoint),
    BoolPoint(BoolPoint),
    IntegerPoint(IntegerPoint),
    TimeSignaturePoint(TimeSignaturePoint),
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum PointsSequenceEnum {
    Target(AutomationTarget),
    PointType(PointsTypeEnum),
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Points {
    // Extends timeline
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
    // Extension finish
    #[serde(rename = "$value", default)]
    pub points: Option<Vec<PointsSequenceEnum>>,
    #[serde(rename = "@unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
}

impl Points {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            track: None,
            time_unit: None,
            points: Some(vec![]),
            unit: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

impl UpcastTimeline for Points {
    // this is to emulate upcasting
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
    use {super::Points, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Points::new_fake();

        println!("{:#?}", o);

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
