#![allow(unused)]

use {
    crate::{
        add_one_get, fake_rng, real_parameter::RealParameter, send_type::SendTypeEnum, unit::Unit,
    },
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Send {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "Volume")]
    volume: RealParameter,
    #[serde(rename = "Pan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pan: Option<RealParameter>,
    #[serde(rename = "@destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<String>,
    #[serde(rename = "@type")]
    send_type: SendTypeEnum,
}

impl Send {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            volume: RealParameter::new_test(Unit::Beats),
            pan: None,
            destination: None,
            send_type: SendTypeEnum::new_fake(),
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Send, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Send::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
