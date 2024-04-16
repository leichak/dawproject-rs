#![allow(unused)]

use {
    super::super::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
enum TimeSignaturePointEnum {
    #[serde(rename = "@numerator")]
    Numerator(i32),
    #[serde(rename = "@denominator")]
    Denominator(i32),
}

type TimeSignaturePointSequence = Vec<TimeSignaturePointEnum>;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct TimeSignaturePoint {
    #[serde(rename = "@time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,

    #[serde(rename = "$value", default)]
    real_point_sequence: Option<TimeSignaturePointSequence>,
}

impl TimeSignaturePoint {
    pub fn new_test() -> Self {
        Self {
            time: None,
            real_point_sequence: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::TimeSignaturePoint, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = TimeSignaturePoint::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
