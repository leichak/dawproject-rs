use {
    super::eq_band_type::EqBandTypeEnum,
    crate::{bool_parameter::BoolParameter, fake_rng, real_parameter::RealParameter},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
enum EqBandParamsEnum {
    Freq(RealParameter),
    Gain(RealParameter),
    Q(RealParameter),
    Enabled(BoolParameter),
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct EqBand {
    #[serde(rename = "@type")]
    eq_type: EqBandTypeEnum,
    #[serde(rename = "@order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<i32>,
    #[serde(rename = "$value", default)]
    eq_band_params: Vec<EqBandParamsEnum>,
}

impl EqBand {
    pub fn new_test() -> Self {
        Self {
            eq_band_params: vec![],
            eq_type: EqBandTypeEnum::BandPass,
            order: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::EqBand, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = EqBand::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
