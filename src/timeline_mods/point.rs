#![allow(unused)]

use {
    crate::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Point {
    #[serde(rename = "@time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl Point {
    pub fn new_test() -> Self {
        Self { time: None }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Point, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Point::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
