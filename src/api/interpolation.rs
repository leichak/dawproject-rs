#![allow(unused)]

use {
    super::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
#[serde(rename_all = "lowercase")]
pub enum InterpolationEnum {
    Hold,
    Linear,
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Interpolation {
    #[serde(rename = "$value", default)]
    interpolation_type: Vec<InterpolationEnum>,
}

impl Interpolation {
    pub fn new_test(interpolation_type: InterpolationEnum) -> Self {
        Self {
            interpolation_type: vec![interpolation_type],
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{Interpolation, InterpolationEnum},
        quick_xml::se::to_string,
        std::error::Error,
    };

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Interpolation::new_test(InterpolationEnum::Linear);

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
