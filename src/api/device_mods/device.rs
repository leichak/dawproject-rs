#![allow(unused)]
use super::device_role::DeviceRole;

use {
    super::{
        super::add_one_get, super::bool_parameter::BoolParameter, super::fake_rng,
        super::file_reference::FileReference,
    },
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum DeviceElementsEnum {
    Parameters,
    Enabled(BoolParameter),
    State(FileReference),
}

pub type DeviceElements = Vec<DeviceElementsEnum>;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
enum Parameters {
    #[serde(rename = "parameter")]
    Parameter,
    #[serde(rename = "realParameter")]
    RealParameter,
    #[serde(rename = "boolParameter")]
    BoolParameter,
    #[serde(rename = "integerParameter")]
    IntegerParameter,
    #[serde(rename = "enumParameter")]
    EnumParameter,
    #[serde(rename = "timeSignatureParameter")]
    TimeSignatureParameter,
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Device {
    // Extends referenceable
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
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
    device_elements: DeviceElements,
}

impl Device {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            device_elements: vec![],
            device_id: None,
            device_name: None,
            device_role: None,
            device_vendor: None,
            loaded: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Device, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Device::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
