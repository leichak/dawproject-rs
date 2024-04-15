use {
    fake::Dummy,
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum DeviceRole {
    #[serde(rename = "instrument")]
    Instrument,
    #[serde(rename = "noteFX")]
    NoteFX,
    #[serde(rename = "audioFX")]
    AudioFX,
    #[serde(rename = "analyzer")]
    Analyzer,
}

#[cfg(test)]
mod tests {
    use {super::DeviceRole, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = DeviceRole::NoteFX;

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
