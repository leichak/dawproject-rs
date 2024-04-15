#![allow(unused)]

use {
    crate::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct BoolPoint {
    #[serde(rename = "@time")]
    pub time: Vec<String>,

    #[serde(rename = "@value")]
    value: bool,
}

impl BoolPoint {
    pub fn new_test() -> Self {
        Self {
            time: vec![],
            value: false,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::BoolPoint, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = BoolPoint::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
