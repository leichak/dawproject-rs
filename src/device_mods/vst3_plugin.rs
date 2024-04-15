#![allow(unused)]
use super::{device::DeviceElements, device_role::DeviceRole};

use {
    crate::{add_one_get, fake_rng},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Vst3Plugin {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "$value", default)]
    pub device_elements: DeviceElements,
    #[serde(rename = "@deviceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    device_id: Option<String>,
    #[serde(rename = "@deviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "@deviceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_role: Option<DeviceRole>,
    #[serde(rename = "@deviceVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    device_vendor: Option<String>,
    #[serde(rename = "@loaded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    loaded: Option<bool>,
    #[serde(rename = "@pluginVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin_version: Option<String>,
}

impl Vst3Plugin {
    pub fn new_test() -> Self {
        Self {
            id: Some(format!("id{}", add_one_get())),
            device_elements: vec![],
            device_id: None,
            device_name: None,
            device_role: None,
            device_vendor: None,
            loaded: None,
            plugin_version: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Vst3Plugin, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Vst3Plugin::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
