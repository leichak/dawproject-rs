#![allow(unused)]

use {
    crate::{
        content_type::ContentType,
        mixer_role::MixerRoleEnum,
        timeline_mods::{audio::Audio, clip::Clip, clips::Clips, timeline::TimeLine, warp::Warp},
        track::Track,
    },
    std::{error::Error, fs::File, path::Path},
};

pub fn create_track(
    name: String,
    content_type: Vec<ContentType>,
    mixer_role: MixerRoleEnum,
    volume: f64,
    pan: f64,
) -> Track {
    Track::new_test(name, content_type, mixer_role, volume, pan)
}

pub fn create_audio(
    relative_path: String,
    sample_rate: i32,
    channels: i32,
    duration: f64,
) -> Audio {
    Audio::new_test(relative_path, sample_rate, channels, duration)
}

pub fn create_warp(time: f64, content_time: f64) -> Warp {
    Warp::new_test(time, content_time)
}

pub fn create_clip(content: TimeLine, time: f64, duration: f64) -> Clip {
    Clip::new_test(content, time, duration)
}

pub fn create_clips(clips: Vec<Clip>) -> Clips {
    Clips::new_test(clips)
}

pub fn create_file_path_absolute_string(file_path: String) -> Result<String, Box<dyn Error>> {
    match File::create(file_path.clone()) {
        Ok(f) => (),
        Err(err) => return Err(err.into()),
    };

    match std::fs::canonicalize(Path::new(&file_path)) {
        Ok(p) => return Ok(p.to_str().unwrap().to_string()),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use std::io;

    use super::*;

    pub fn test_cannoicalize() -> Result<(), Box<dyn Error>> {
        let sample = "white-glasses.wav".to_string();

        match super::create_file_path_absolute_string(sample.clone()) {
            Ok(s) => println!("{}", s),
            Err(err) => return Err(err.into()),
        }
        Ok(())
    }
}
