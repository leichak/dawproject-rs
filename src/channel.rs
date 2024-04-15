#![allow(unused)]
#![allow(clippy::large_enum_variant)]

use crate::add_one_get;

use {
    crate::{
        bool_parameter::BoolParameter,
        device_mods::{
            au_plugin::AuPlugin, builtin_device::BuiltinDevice, clap_plugin::ClapPlugin,
            compressor::Compressor, device::Device, equalizer::Equalizer, limiter::Limiter,
            noise_gate::NoiseGate, vst2_plugin::Vst2Plugin, vst3_plugin::Vst3Plugin,
        },
        fake_rng,
        mixer_role::MixerRoleEnum,
        real_parameter::RealParameter,
        send::Send,
        unit::Unit,
    },
    fake::{Dummy, Fake, Faker},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum DeviceTypes {
    Device(Device),
    Vst2Plugin(Vst2Plugin),
    Vst3Plugin(Vst3Plugin),
    ClapPlugin(ClapPlugin),
    BuiltinDevice(BuiltinDevice),
    Equalizer(Equalizer),
    Compressor(Compressor),
    NoiseGate(NoiseGate),
    Limiter(Limiter),
    AuPlugin(AuPlugin),
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub enum ChannelParameters {
    Volume(Option<RealParameter>),
    Pan(Option<RealParameter>),
    Mute(Option<BoolParameter>),
    Devices(Option<Devices>), //
    Sends(Option<Vec<Send>>),
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Devices {
    // this is weird
    #[serde(rename = "$value")]
    pub choice: Vec<DeviceTypes>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Dummy)]
pub struct Channel {
    // Extends lane
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "@name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "@color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(rename = "@comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(rename = "@role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<MixerRoleEnum>,
    #[serde(rename = "@audioChannels")]
    audio_channels: i32,
    #[serde(rename = "@solo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    solo: Option<bool>,
    #[serde(rename = "@destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "$value", default)]
    pub elements: Option<Vec<ChannelParameters>>,
}

impl Channel {
    pub fn new_test(volume_value: f64, pan_value: f64, role: MixerRoleEnum) -> Self {
        let mut volume = RealParameter::new_test(Unit::Linear);
        volume.value = Some(volume_value);
        let mut pan = RealParameter::new_test(Unit::Normalized);
        pan.value = Some(pan_value);

        Self {
            id: Some(format!("id_{}", add_one_get())),
            name: None,
            color: None,
            comment: None,
            role: Some(role),
            audio_channels: 2,
            solo: Some(false),
            destination: None,
            elements: Some(vec![]),
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

    use super::*;

    use {
        super::MixerRoleEnum,
        quick_xml::{se::to_string, DeError},
    };

    #[test]
    pub fn se_test() -> Result<(), Box<dyn Error>> {
        let mut o = Channel::new_fake();

        match to_string(&o) {
            Ok(o) => println!("{}", o),
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
