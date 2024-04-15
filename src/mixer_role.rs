#![allow(unused)]

use {
    fake::Dummy,
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Dummy)]
pub enum MixerRoleEnum {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "effect")]
    Effect,
    #[serde(rename = "submix")]
    SubMix,
    #[serde(rename = "vca")]
    Vca,
}

#[cfg(test)]
mod tests {
    use {super::MixerRoleEnum, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = MixerRoleEnum::Master;

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
