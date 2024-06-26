#![allow(unused)]
use {
    super::{add_one_get, fake_rng},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct TimeSignatureParameter {
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "@parameterID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter_id: Option<i32>,
    #[serde(rename = "@numerator")]
    numerator: i32,
    #[serde(rename = "@denominator")]
    denominator: i32,
}

impl TimeSignatureParameter {
    pub fn new_required(numerator: i32, denominator: i32) -> Self {
        Self {
            name: None,
            color: None,
            comment: None,
            id: Some(format!("id{}", add_one_get())),
            parameter_id: None,
            numerator,
            denominator,
        }
    }

    pub fn new_test(numerator: i32, denominator: i32) -> Self {
        Self {
            name: None,
            color: None,
            comment: None,
            id: Some(format!("id{}", add_one_get())),
            parameter_id: None,
            numerator,
            denominator,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::TimeSignatureParameter, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = TimeSignatureParameter::new_test(1, 1);

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
