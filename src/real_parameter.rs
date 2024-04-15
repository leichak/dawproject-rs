#![allow(unused)]
use {
    crate::{add_one_get, fake_rng},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

use crate::unit::Unit;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct RealParameter {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "@parameterID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_id: Option<i32>,
    #[serde(rename = "@value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "@unit")]
    pub unit: Unit,
    #[serde(rename = "@min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,
    #[serde(rename = "@max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,
}

impl RealParameter {
    pub fn new_test(unit: Unit) -> RealParameter {
        RealParameter {
            id: Some(format!("id{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            parameter_id: None,
            value: None,
            unit,
            min: None,
            max: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {crate::unit::Unit, std::error::Error};

    use {super::RealParameter, quick_xml::se::to_string};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut pan = RealParameter::new_fake();

        match to_string(&pan) {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
