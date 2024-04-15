#![allow(unused)]

use {
    fake::Dummy,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
#[serde(rename_all = "lowercase")]
pub enum Unit {
    Linear,
    Normalized,
    Percent,
    Decibel,
    Hertz,
    Semitones,
    Seconds,
    Beats,
    Bpm,
}

#[cfg(test)]
mod tests {
    use {super::Unit, std::error::Error};

    use quick_xml::se::to_string;

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o: Unit = Unit::Beats;

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
