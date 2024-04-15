#![allow(unused)]

use {
    crate::{expression_type::ExpressionTypeEnum, fake_rng},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct AutomationTarget {
    #[serde(rename = "@parameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(rename = "@expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<ExpressionTypeEnum>,
    #[serde(rename = "@channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
    #[serde(rename = "@key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<i32>,
    #[serde(rename = "@controller")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<i32>,
}

impl AutomationTarget {
    pub fn new_test() -> Self {
        Self {
            parameter: None,
            expression: None,
            channel: None,
            key: None,
            controller: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}
#[cfg(test)]
mod tests {
    use {super::AutomationTarget, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = AutomationTarget::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
