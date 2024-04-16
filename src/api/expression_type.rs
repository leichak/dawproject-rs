#![allow(unused)]

use {
    super::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
#[serde(rename = "camelCase")]
pub enum ExpressionTypeEnum {
    Gain,

    Pan,

    Transpose,

    Timbre,

    Formant,

    Pressure,

    ChannelController,

    ChannelPressure,

    PolyPressure,

    PitchBend,

    ProgramChange,
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct ExpressionType {
    #[serde(rename = "$value", default)]
    pub expression_type: Vec<ExpressionTypeEnum>,
}

impl ExpressionType {
    pub fn new_test() -> Self {
        Self {
            expression_type: vec![],
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::ExpressionType, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = ExpressionType::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
