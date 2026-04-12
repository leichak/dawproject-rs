#![allow(unused)]
use super::add_one_get;

use super::{mixer_role::MixerRoleEnum, project::TrackChannelEnum};

use {
    super::{channel::Channel, content_type::ContentType, fake_rng},
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

pub type TrackChannel = Vec<TrackChannelEnum>;

type Content = Vec<ContentType>;

/// A track in the DAW project, containing channels and content type information.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Dummy)]
pub struct Track {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "@contentType")]
    content_type: Content,
    #[serde(rename = "@loaded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    loaded: Option<bool>,
    #[serde(rename = "$value")]
    pub track_channel: TrackChannel,
}

impl Track {
    pub fn new_test(
        name: String,
        content_type: Vec<ContentType>,
        mixer_role: MixerRoleEnum,
        volume: f64,
        pan: f64,
    ) -> Track {
        let channel = Channel::new_test(volume, pan, mixer_role);

        Track {
            id: Some(format!("id_{}", add_one_get())),
            name: Some(name),
            color: None,
            comment: None,
            track_channel: vec![TrackChannelEnum::Channel(channel)],
            content_type,
            loaded: Some(false),
        }
    }

    pub fn new_fake() -> Self {
        let o: Self = Faker.fake_with_rng(&mut fake_rng());
        o
    }

    pub fn get_channel(&mut self) -> Option<&mut Channel> {
        if let Some(TrackChannelEnum::Channel(c)) = self
            .track_channel
            .iter_mut()
            .find(|el| matches!(el, TrackChannelEnum::Channel(_c)))
        {
            return Some(c);
        }

        None
    }

    pub fn get_id(&self) -> String {
        self.id.clone().expect("Track ID was not set")
    }
}

pub struct TrackBuilder {
    name: Option<String>,
    color: Option<String>,
    content_type: Vec<ContentType>,
    channel: Option<Channel>,
    loaded: Option<bool>,
}

impl TrackBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn channel(mut self, channel: Channel) -> Self {
        self.channel = Some(channel);
        self
    }

    pub fn loaded(mut self, loaded: bool) -> Self {
        self.loaded = Some(loaded);
        self
    }

    pub fn build(self) -> Track {
        let mut track_channel = Vec::new();
        if let Some(ch) = self.channel {
            track_channel.push(TrackChannelEnum::Channel(ch));
        }

        Track {
            id: Some(format!("id_{}", add_one_get())),
            name: self.name,
            color: self.color,
            comment: None,
            content_type: self.content_type,
            loaded: self.loaded,
            track_channel,
        }
    }
}

impl Track {
    pub fn builder(content_type: Vec<ContentType>) -> TrackBuilder {
        TrackBuilder {
            name: None,
            color: None,
            content_type,
            channel: None,
            loaded: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, quick_xml::se, std::error::Error};

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let t = Track::new_test(
            "Test".to_string(),
            vec![ContentType::Audio],
            MixerRoleEnum::Effect,
            0.0,
            0.0,
        );

        match se::to_string(&t) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err.into()),
        }
    }
}
