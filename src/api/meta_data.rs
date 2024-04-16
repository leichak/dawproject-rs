#![allow(unused)]

use {
    super::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};
#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
enum MetaDataEnum {
    Title(String),
    Artist(String),
    Album(String),
    OriginalArtist(String),
    Composer(String),
    Songwriter(String),
    Producer(String),
    Arranger(String),
    Year(String),
    Genre(String),
    Copyright(String),
    Website(String),
    Comment(String),
}

type MetaDataVec = Vec<MetaDataEnum>;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct MetaData {
    #[serde(rename = "$value", default)]
    meta_data: MetaDataVec,
}

impl MetaData {
    pub fn new_test() -> Self {
        MetaData {
            meta_data: Vec::new(),
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::MetaData, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = MetaData::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
