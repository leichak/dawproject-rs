#![allow(unused)]

use {
    super::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
#[serde(rename_all = "lowercase")]
pub enum SendTypeEnum {
    Pre,
    Post,
}

impl SendTypeEnum {
    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct SendType {
    #[serde(rename = "$value", default)]
    field: Vec<SendTypeEnum>,
}

impl SendType {
    pub fn new_test() -> Self {
        Self {
            field: vec![SendTypeEnum::Post],
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::SendType, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = SendType::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
