#![allow(unused)]

use {
    crate::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
#[serde(rename_all = "lowercase")]
enum WarpSequenceEnum {
    Time(f64),
    ContentTime(f64),
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Warp {
    #[serde(rename = "@time")]
    time: f64,
    #[serde(rename = "@contentTime")]
    content_time: f64,
}

impl Warp {
    pub fn new_test(time: f64, content_time: f64) -> Self {
        Self { time, content_time }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Warp, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Warp::new_test(1.0, 1.0);

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
