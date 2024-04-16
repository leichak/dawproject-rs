#![allow(unused)]

use {
    super::super::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct EnumPoint {
    #[serde(rename = "@time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "@value")]
    value: i32,
}

impl EnumPoint {
    pub fn new_test() -> Self {
        Self {
            time: Some("empty".to_string()),
            value: 0,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use {super::EnumPoint, quick_xml::se::to_string};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = EnumPoint::new_fake();

        println!("{:#?}", o);
        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
