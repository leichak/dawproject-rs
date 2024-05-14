use {
    fake::Dummy,
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum EqBandTypeEnum {
    #[serde(rename = "highPass")]
    HighPass,
    #[serde(rename = "lowPass")]
    LowPass,
    #[serde(rename = "bandPass")]
    BandPass,
    #[serde(rename = "highShelf")]
    HighShelf,
    #[serde(rename = "lowShelf")]
    LowShelf,
    #[serde(rename = "bell")]
    Bell,
    #[serde(rename = "notch")]
    Notch,
}

#[cfg(test)]
mod tests {
    use {super::EqBandTypeEnum, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = EqBandTypeEnum::BandPass;

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
