#![allow(unused)]
use crate::{add_one_get, fake_rng};

use {
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct IntegerParameter {
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
    #[serde(rename = "@parameterID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_id: Option<i32>,
    #[serde(rename = "@max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<i32>,
    #[serde(rename = "@min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<i32>,
    #[serde(rename = "@value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<i32>,
}

impl IntegerParameter {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id_{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            parameter_id: None,
            max: None,
            min: None,
            value: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::IntegerParameter, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = IntegerParameter::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
