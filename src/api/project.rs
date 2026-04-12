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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Dummy)]
pub enum TrackChannelEnum {
    Track(Track),
    Channel(Channel),
}

/// Top-level structure containing tracks and channels.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Dummy)]
pub struct Structure {
    #[serde(rename = "$value", default)]
    pub sequence: Vec<TrackChannelEnum>,
}

/// Root element of a DAWproject file containing structure, arrangement, transport, and scenes.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Dummy)]
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

pub struct ProjectBuilder {
    version: String,
    application: Application,
    transport: Option<Transport>,
    structure: Option<Structure>,
    arrangement: Option<Arrangement>,
    scenes: Option<Vec<Scene>>,
}

impl ProjectBuilder {
    pub fn transport(mut self, transport: Transport) -> Self {
        self.transport = Some(transport);
        self
    }

    pub fn structure(mut self, structure: Structure) -> Self {
        self.structure = Some(structure);
        self
    }

    pub fn arrangement(mut self, arrangement: Arrangement) -> Self {
        self.arrangement = Some(arrangement);
        self
    }

    pub fn scenes(mut self, scenes: Vec<Scene>) -> Self {
        self.scenes = Some(scenes);
        self
    }

    pub fn build(self) -> Project {
        Project {
            version: self.version,
            application: self.application,
            transport: self.transport,
            structure: self.structure,
            arrangement: self.arrangement,
            scenes: self.scenes,
        }
    }
}

impl Project {
    pub fn builder(version: &str, application: Application) -> ProjectBuilder {
        ProjectBuilder {
            version: version.to_string(),
            application,
            transport: None,
            structure: None,
            arrangement: None,
            scenes: None,
        }
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
