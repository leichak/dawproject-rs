#![allow(unused)]

use super::{real_parameter::RealParameter, time_signature_parameter::TimeSignatureParameter};

use {
    super::fake_rng,
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum TransportSequence {
    Tempo(RealParameter),
    TimeSignature(TimeSignatureParameter),
}

type TransportSequenceVec = Vec<TransportSequence>;

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Transport {
    #[serde(rename = "$value", default)]
    pub sequence: TransportSequenceVec,
}

impl Transport {
    pub fn new_test() -> Self {
        Self { sequence: vec![] }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }
}

#[cfg(test)]
mod tests {
    use {super::Transport, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Transport::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
