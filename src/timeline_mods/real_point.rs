#![allow(unused)]

use {
    crate::{fake_rng, interpolation::InterpolationEnum},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct RealPoint {
    #[serde(rename = "@time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@interpolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<InterpolationEnum>,
}

impl RealPoint {
    pub fn new_test() -> Self {
        Self {
            time: None,
            value: None,
            interpolation: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::RealPoint, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = RealPoint::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
