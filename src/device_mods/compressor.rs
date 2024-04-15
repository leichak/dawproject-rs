#![allow(unused)]
use super::{device::DeviceElements, device_role::DeviceRole};

use {
    crate::{add_one_get, bool_parameter::BoolParameter, fake_rng, real_parameter::RealParameter},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
enum CompressorParamsEnum {
    Attack(RealParameter),
    AutoMakeup(BoolParameter),
    InputGain(RealParameter),
    OutputGain(RealParameter),
    Ratio(RealParameter),
    Release(RealParameter),
    Threshold(RealParameter),
}

type CompressorParams = Vec<CompressorParamsEnum>;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Compressor {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "$value", default)]
    device_elements: DeviceElements,
    #[serde(rename = "@deviceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    device_id: Option<String>,
    #[serde(rename = "@deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<String>,
    #[serde(rename = "@deviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    device_role: Option<DeviceRole>,
    #[serde(rename = "@deviceVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    device_vendor: Option<String>,
    #[serde(rename = "@loaded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    loaded: Option<bool>,
    #[serde(rename = "$value", default)]
    compressor_elements: Vec<CompressorParamsEnum>,
}

impl Compressor {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            device_elements: vec![],
            device_id: None,
            device_name: None,
            device_role: None,
            device_vendor: None,
            loaded: None,
            compressor_elements: vec![],
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Compressor, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Compressor::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
