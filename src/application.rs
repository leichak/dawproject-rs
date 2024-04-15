#![allow(unused)]

use {
    crate::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Application {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@version")]
    pub version: String,
}

impl Application {
    pub fn new_empty() -> Self {
        Application {
            name: "".to_string(),
            version: "".to_string(),
        }
    }

    pub fn new_name_ver(name: String, version: f64) -> Self {
        Application {
            name,
            version: version.to_string(),
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Application, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Application::new_name_ver("test".to_string(), 1.0);

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
