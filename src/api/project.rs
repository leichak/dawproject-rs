#![allow(unused)]

use {
    super::{
        application::Application, arrangement::Arrangement, fake_rng, mixer_role::MixerRoleEnum,
        scene::Scene,
    },
    fake::{Dummy, Fake, Faker},
};

use super::transport::Transport;

use super::{channel::Channel, track::Track};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum TrackChannelEnum {
    Track(Track),
    Channel(Channel),
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Structure {
    #[serde(rename = "$value", default)]
    pub sequence: Vec<TrackChannelEnum>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Project {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "Application")]
    pub application: Application,
    #[serde(rename = "Transport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
    #[serde(rename = "Structure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<Structure>,
    #[serde(rename = "Arrangement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrangement: Option<Arrangement>,
    #[serde(rename = "Scenes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<Scene>>,
}

impl Structure {
    pub fn new_empty() -> Self {
        Structure {
            sequence: Vec::new(),
        }
    }
}

impl Project {
    pub fn new_test(name: String, version: f64) -> Self {
        Project {
            version: "1.0".to_string(),
            application: Application::new_name_ver(name, version),
            transport: None,
            structure: Some(Structure { sequence: vec![] }),
            arrangement: Some(Arrangement::new_fake()),
            scenes: None,
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }

    pub fn get_master_track(&self) -> Option<&Track> {
        if let Some(TrackChannelEnum::Track(t)) = self
            .structure
            .as_ref()
            .unwrap()
            .sequence
            .iter()
            .find(|el| match el {
                TrackChannelEnum::Track(t) => {
                    t.track_channel
                        .iter()
                        .filter(|el| match el {
                            TrackChannelEnum::Channel(c) => {
                                if c.role.is_some() {
                                    *c.role.as_ref().unwrap() == MixerRoleEnum::Master
                                } else {
                                    false
                                }
                            }
                            _ => false,
                        })
                        .count()
                        != 0
                }
                TrackChannelEnum::Channel(_) => false,
            })
        {
            return Some(t);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use {super::Project, quick_xml::se::to_string, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Project::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
