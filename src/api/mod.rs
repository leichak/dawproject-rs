#![allow(unused)]

mod application;
mod arrangement;
mod bool_parameter;
mod channel;
mod content_type;
mod daw_project;
mod device_mods;
mod enum_parameter;
mod expression_type;
mod file_reference;
mod integer_parameter;
mod interpolation;
mod lane;
mod meta_data;
mod mixer_role;
mod parameter;
mod project;
mod real_parameter;
mod scene;
mod send;
mod send_type;
mod time_signature_parameter;
mod timeline_mods;
mod track;
mod transport;
mod unit;
mod utility;

pub use crate::api::application::Application;
pub use crate::api::arrangement::Arrangement;
pub use crate::api::bool_parameter::BoolParameter;
pub use crate::api::channel::Channel;
pub use crate::api::content_type::ContentType;
pub use crate::api::daw_project::DawProject;
pub use crate::api::enum_parameter::EnumParameter;
pub use crate::api::expression_type::ExpressionType;
pub use crate::api::file_reference::FileReference;
pub use crate::api::integer_parameter::IntegerParameter;
pub use crate::api::interpolation::{Interpolation, InterpolationEnum};
pub use crate::api::lane::Lane;
pub use crate::api::meta_data::MetaData;
pub use crate::api::mixer_role::MixerRoleEnum;
pub use crate::api::parameter::Parameter;
pub use crate::api::project::Project;
pub use crate::api::project::Structure;
pub use crate::api::real_parameter::RealParameter;
pub use crate::api::scene::Scene;
pub use crate::api::send::Send;
pub use crate::api::send_type::{SendType, SendTypeEnum};
pub use crate::api::time_signature_parameter::TimeSignatureParameter;
pub use crate::api::track::Track;
pub use crate::api::transport::Transport;
pub use crate::api::unit::Unit;

pub use crate::api::device_mods::{
    au_plugin::AuPlugin, builtin_device::BuiltinDevice, clap_plugin::ClapPlugin,
    compressor::Compressor, device::Device, device_role::DeviceRole, eq_band::EqBand,
    eq_band_type::EqBandTypeEnum, equalizer::Equalizer, limiter::Limiter, noise_gate::NoiseGate,
    plugin::Plugin, vst2_plugin::Vst2Plugin, vst3_plugin::Vst3Plugin,
};
pub use crate::api::timeline_mods::{
    audio::Audio, automation_target::AutomationTarget, bool_point::BoolPoint, clip::Clip,
    clip_slot::ClipSlot, clips::Clips, enum_point::EnumPoint, integer_point::IntegerPoint,
    lanes::Lanes, marker::Marker, markers::Markers, media_file::MediaFile, note::Note,
    notes::Notes, point::Point, real_point::RealPoint, time_signature_point::TimeSignaturePoint,
    time_unit::TimeUnit, timeline::TimeLine, video::Video, warp::Warp, warps::Warps,
};

use fake::utils::AlwaysTrueRng;

pub fn fake_rng() -> AlwaysTrueRng {
    AlwaysTrueRng::default()
}

use std::sync::atomic::{AtomicI32, Ordering};

static ID_XML: AtomicI32 = AtomicI32::new(0);

fn add_one_get() -> i32 {
    ID_XML.fetch_add(1, Ordering::SeqCst)
}

pub fn reset_xml_id() {
    let _ = ID_XML.fetch_and(0, Ordering::SeqCst);
}

static XML: &str = r##"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Project version="1.0">
<Application name="Bitwig Studio" version="5.0"/>
<Transport>
<Tempo max="666.000000" min="20.000000" unit="bpm" value="149.000000" id="id0" name="Tempo"/>
<TimeSignature denominator="4" numerator="4" id="id1"/>
</Transport>
<Structure>
<Track contentType="notes" loaded="true" id="id2" name="Bass" color="#a2eabf">
  <Channel audioChannels="2" destination="id15" role="regular" solo="false" id="id3">
    <Devices>
      <ClapPlugin deviceID="org.surge-synth-team.surge-xt" deviceName="Surge XT" deviceRole="instrument" loaded="true" id="id7" name="Surge XT">
        <Parameters/>
        <Enabled value="true" id="id8" name="On/Off"/>
        <State path="plugins/d19b1f6e-bbb6-42fe-a6c9-54b41d97a05d.clap-preset"/>
      </ClapPlugin>
    </Devices>
    <Mute value="false" id="id6" name="Mute"/>
    <Pan max="1.000000" min="0.000000" unit="normalized" value="0.500000" id="id5" name="Pan"/>
    <Volume max="2.000000" min="0.000000" unit="linear" value="0.659140" id="id4" name="Volume"/>
  </Channel>
</Track>
<Track contentType="audio" loaded="true" id="id9" name="Drumloop" color="#b53bba">
  <Channel audioChannels="2" destination="id15" role="regular" solo="false" id="id10">
    <Mute value="false" id="id13" name="Mute"/>
    <Pan max="1.000000" min="0.000000" unit="normalized" value="0.500000" id="id12" name="Pan"/>
    <Volume max="2.000000" min="0.000000" unit="linear" value="0.177125" id="id11" name="Volume"/>
  </Channel>
</Track>
<Track contentType="audio notes" loaded="true" id="id14" name="Master">
  <Channel audioChannels="2" role="master" solo="false" id="id15">
    <Mute value="false" id="id18" name="Mute"/>
    <Pan max="1.000000" min="0.000000" unit="normalized" value="0.500000" id="id17" name="Pan"/>
    <Volume max="2.000000" min="0.000000" unit="linear" value="1.000000" id="id16" name="Volume"/>
  </Channel>
</Track>
</Structure>
<Arrangement id="id19">
<Lanes timeUnit="beats" id="id20">
  <Lanes track="id2" id="id21">
    <Clips id="id22">
      <Clip time="0.0" duration="8.0" playStart="0.0">
        <Notes id="id23">
          <Note time="0.000000" duration="0.250000" channel="0" key="65" vel="0.787402" rel="0.787402"/>
          <Note time="1.000000" duration="0.250000" channel="0" key="65" vel="0.787402" rel="0.787402"/>
          <Note time="4.000000" duration="0.250000" channel="0" key="65" vel="0.787402" rel="0.787402"/>
          <Note time="5.000000" duration="0.250000" channel="0" key="65" vel="0.787402" rel="0.787402"/>
          <Note time="0.500000" duration="0.250000" channel="0" key="64" vel="0.787402" rel="0.787402"/>
          <Note time="4.500000" duration="0.250000" channel="0" key="64" vel="0.787402" rel="0.787402"/>
          <Note time="1.500000" duration="2.500000" channel="0" key="53" vel="0.787402" rel="0.787402"/>
          <Note time="5.500000" duration="0.250000" channel="0" key="53" vel="0.787402" rel="0.787402"/>
          <Note time="6.000000" duration="2.000000" channel="0" key="53" vel="0.787402" rel="0.787402"/>
        </Notes>
      </Clip>
    </Clips>
  </Lanes>
  <Lanes track="id9" id="id24">
    <Clips id="id25">
      <Clip time="0.0" duration="8.00003433227539" playStart="0.0" loopStart="0.0" loopEnd="8.00003433227539" fadeTimeUnit="beats" fadeInTime="0.0" fadeOutTime="0.0" name="Drumfunk3 170bpm">
        <Clips id="id26">
          <Clip time="0.0" duration="8.00003433227539" contentTimeUnit="beats" playStart="0.0" fadeTimeUnit="beats" fadeInTime="0.0" fadeOutTime="0.0">
            <Warps contentTimeUnit="seconds" timeUnit="beats" id="id28">
              <Audio algorithm="stretch" channels="2" duration="2.823541666666667" sampleRate="48000" id="id27">
                <File path="audio/Drumfunk3 170bpm.wav"/>
              </Audio>
              <Warp time="0.0" contentTime="0.0"/>
              <Warp time="8.00003433227539" contentTime="2.823541666666667"/>
            </Warps>
          </Clip>
        </Clips>
      </Clip>
    </Clips>
  </Lanes>
  <Lanes track="id14" id="id29">
    <Clips id="id30"/>
  </Lanes>
</Lanes>
</Arrangement>
<Scenes/>
</Project>
"##;

#[cfg(test)]
mod project_creator {

    use std::collections::HashMap;

    use std::{error::Error, path::Path};

    use crate::api::XML;

    use super::{
        super::api::device_mods::{
            device::DeviceElementsEnum, device_role::DeviceRole, vst3_plugin::Vst3Plugin,
        },
        arrangement::{Arrangement, ArrangementSequenceEnum},
        channel::{Channel, ChannelParameters, DeviceTypes, Devices},
        content_type::ContentType,
        daw_project::DawProject,
        expression_type::{ExpressionType, ExpressionTypeEnum},
        file_reference::FileReference,
        interpolation::InterpolationEnum,
        meta_data::MetaData,
        project::TrackChannelEnum,
        real_parameter::RealParameter,
    };

    use super::timeline_mods::{
        automation_target::AutomationTarget,
        clip::Clip,
        clips::{ClipEnum, Clips},
        lanes::{ArrangementTypeChoiceEnum, Lanes},
        marker::Marker,
        markers::Markers,
        note::Note,
        notes::Notes,
    };

    use super::timeline_mods::{
        points::{Points, PointsSequenceEnum, PointsTypeEnum},
        real_point::RealPoint,
        time_unit::TimeUnit,
    };

    use super::timeline_mods::{
        timeline::TimeLine,
        warps::{Warps, WarpsSequenceEnum},
        UpcastTimeline,
    };

    use {
        super::{
            project::Project,
            reset_xml_id,
            track::Track,
            transport::{Transport, TransportSequence},
            unit::Unit,
            utility::{self, create_track},
        },
        strum::{EnumIter, IntoEnumIterator, VariantNames},
    };

    #[test]
    fn fast_test() {
        let mut v: Vec<u8> = [67, 108, 97, 112, 80, 108, 117, 103, 105, 110].to_vec();
        println!("{}", String::from_utf8(v).unwrap());
    }

    #[test]
    fn load_daw_project_test() {
        use {super::project::Project, quick_xml::de::from_str};

        let obj: Project = from_str(XML).unwrap();

        println!("Deserialized object {:#?}", obj);
    }

    #[derive(PartialEq)]
    pub enum Features {
        CueMarkers,
        Clips,
        Audio,
        Notes,
        Automation,
        AliasClips,
        Plugins,
    }

    pub fn create_empty_project() -> Project {
        reset_xml_id();
        Project::new_test("Test".to_string(), 1.0)
    }

    pub fn create_dummy_project(num_tracks: i32, features: Vec<Features>) -> Project {
        let mut project = create_empty_project();

        let mut master_track = create_track(
            "Master".to_string(),
            vec![],
            super::mixer_role::MixerRoleEnum::Master,
            1.0,
            0.5,
        );

        let master_track_ref = &mut master_track;

        if features.contains(&Features::Plugins) {
            let mut device = Vst3Plugin::new_test();
            device.device_role = Some(DeviceRole::AudioFX);
            if let Some(state) = device
                .device_elements
                .iter_mut()
                .find(|el| matches!(el, DeviceElementsEnum::State(_)))
            {
                if let DeviceElementsEnum::State(state) = state {
                    state.path = "plugin-states/12323545.vstpreset".to_string();
                }
            } else {
                device
                    .device_elements
                    .push(DeviceElementsEnum::State(FileReference::new_test(
                        "plugin-states/12323545.vstpreset".to_string(),
                    )));
            }

            if let Some(c) = master_track_ref
                .track_channel
                .iter_mut()
                .find(|el| matches!(el, TrackChannelEnum::Channel(_)))
            {
                match c {
                    TrackChannelEnum::Channel(c) => {
                        if let Some(elements) = &mut c.elements {
                            if let Some(ChannelParameters::Devices(Some(devices))) = elements
                                .iter_mut()
                                .find(|el| matches!(el, ChannelParameters::Devices(_)))
                            {
                                devices.choice.push(DeviceTypes::Vst3Plugin(device));
                            }
                        } else {
                            let elements = vec![ChannelParameters::Devices(Some(Devices {
                                choice: vec![DeviceTypes::Vst3Plugin(device)],
                            }))];

                            c.elements = Some(elements);
                        }
                    }
                    _ => (),
                }
            } else {
                master_track_ref
                    .track_channel
                    .push(TrackChannelEnum::Channel(Channel::new_test(
                        1.0,
                        0.0,
                        super::mixer_role::MixerRoleEnum::Master,
                    )))
            }
        }

        project
            .structure
            .as_mut()
            .unwrap()
            .sequence
            .push(TrackChannelEnum::Track(master_track));

        let mut arrangement = Arrangement::new_test();
        if arrangement.sequence.is_none() {
            arrangement.sequence = Some(vec![]);
        }
        let mut arrangement_lanes = Lanes::new_test();

        arrangement_lanes.time_unit = Some(TimeUnit::Beats);

        if features.contains(&Features::CueMarkers) {
            let mut cue_markers = Markers::new_test();
            cue_markers
                .markers
                .as_mut()
                .unwrap()
                .push(Marker::new_test(0.0, "Verse".to_string()));
            cue_markers
                .markers
                .as_mut()
                .unwrap()
                .push(Marker::new_test(24.0, "Chorus".to_string()));
        }

        for i in 0..num_tracks {
            let mut track = utility::create_track(
                format!("Track {}", (i + 1)),
                vec![ContentType::Notes],
                super::mixer_role::MixerRoleEnum::Regular,
                1.0,
                0.5,
            );
            track.color = Some(format!("#{}{}{}{}{}{}", i, i, i, i, i, i).to_string());

            if let Some(TrackChannelEnum::Channel(c)) = &mut track
                .track_channel
                .iter_mut()
                .find(|el| matches!(el, TrackChannelEnum::Channel(_)))
            {
                if let Some(r) = project.get_master_track() {
                    c.destination = Some(r.get_id());
                }
            }

            let mut track_lanes = Lanes::new_test();

            if features.contains(&Features::Clips) {
                let mut clips = Clips::new_test(vec![]);

                let mut clip = Clip::new_test(TimeLine::new_test(), 0.1, 0.1);
                clip.name = Some(format!("Clip {}", i));
                clip.time = 8.0 * i as f64;
                clip.duration = Some(4.0);

                let mut notes = Notes::new_test();

                for j in 0..8 {
                    let mut note = Note::new_test();
                    note.key = Some(36 + 12 * (j % (1 + i)));
                    note.vel = Some(0.8);
                    note.rel = Some(0.5);
                    note.time = Some(0.5 * (j as f64));
                    note.duration = Some(0.5);
                    notes.notes_sequence.as_mut().unwrap().push(note);
                }

                if i == 0 && features.contains(&Features::Automation) {
                    let mut points = Points::new_test();

                    let mut automation_target = AutomationTarget::new_test();
                    automation_target.parameter = track
                        .track_channel
                        .iter()
                        .filter(|el| matches!(el, TrackChannelEnum::Channel(_)))
                        .take(1)
                        .map(|el| match el {
                            TrackChannelEnum::Channel(c) => c
                                .elements
                                .as_ref()
                                .unwrap()
                                .iter()
                                .filter(|el| matches!(el, ChannelParameters::Volume(_)))
                                .take(1)
                                .map(|el| match el {
                                    ChannelParameters::Volume(el) => {
                                        el.as_ref().unwrap().id.clone()
                                    }
                                    _ => Some("non existent".to_string()),
                                })
                                .collect(),
                            _ => Some("non existent".to_string()),
                        })
                        .collect();

                    points
                        .points
                        .as_mut()
                        .unwrap()
                        .push(PointsSequenceEnum::Target(automation_target));

                    if points.points.is_none() {
                        points.points = Some(vec![]);
                    }
                    let point = PointsSequenceEnum::PointType(PointsTypeEnum::RealPoint(
                        create_point(0.0, 0.0, InterpolationEnum::Linear),
                    ));
                    let point_1 = PointsSequenceEnum::PointType(PointsTypeEnum::RealPoint(
                        create_point(8.0, 1.0, InterpolationEnum::Linear),
                    ));

                    points
                        .points
                        .as_mut()
                        .unwrap()
                        .append(&mut vec![point, point_1]);

                    track_lanes
                        .lanes_sequence
                        .as_mut()
                        .unwrap()
                        .push(ArrangementTypeChoiceEnum::Points(points));
                }

                clips.clips.as_mut().unwrap().push(ClipEnum::Clip(clip));

                if features.contains(&Features::AliasClips) {
                    let mut clip_2 = Clip::new_test(TimeLine::new_test(), 0.0, 0.0);
                    clip_2.name = Some(format!("Alias Clip {}", i));
                    clip_2.time = 32.0 + 8.0 * i as f64;
                    clip_2.duration = Some(4.0);
                    clip_2.reference.clone_from(&notes.id);
                    clips.clips.as_mut().unwrap().push(ClipEnum::Clip(clip_2));
                }

                let clips = ArrangementTypeChoiceEnum::Clips(clips);
                track_lanes.lanes_sequence.as_mut().unwrap().push(clips);
                track_lanes.track = Some(track.get_id());
            }

            arrangement_lanes
                .lanes_sequence
                .as_mut()
                .unwrap()
                .push(ArrangementTypeChoiceEnum::Lanes(track_lanes));
        }

        arrangement
            .sequence
            .as_mut()
            .unwrap()
            .push(ArrangementSequenceEnum::Lanes(arrangement_lanes));

        project.arrangement = Some(arrangement);

        project
    }

    fn create_point(time: f64, value: f64, interpolation: InterpolationEnum) -> RealPoint {
        RealPoint {
            time: Some(time.to_string()),
            value: Some(value.to_string()),
            interpolation: Some(interpolation),
        }
    }

    pub fn create_marker(time: f64, name: String) -> Marker {
        Marker::new_test(time, name)
    }

    #[test]
    pub fn save_daw_project() -> Result<(), Box<dyn Error>> {
        let features = vec![Features::Clips, Features::Notes, Features::Audio];
        let proj = create_dummy_project(3, features);
        let meta_data = MetaData::new_test();
        let embedded_files: HashMap<&Path, String> = HashMap::new();

        DawProject::save(
            proj,
            meta_data,
            embedded_files,
            Path::new("target/test.dawproject"),
        )?;

        Ok(())
    }

    #[test]
    pub fn validate_daw_project() -> Result<(), Box<dyn Error>> {
        let features = vec![Features::Clips, Features::Notes, Features::Audio];
        let _proj = create_dummy_project(3, features);

        Ok(())
    }

    #[test]
    pub fn validate_complex_project() -> Result<(), Box<dyn Error>> {
        let features = vec![Features::Clips, Features::Notes, Features::Audio];
        let _proj = create_dummy_project(3, features);

        Ok(())
    }

    #[test]
    pub fn save_and_load_daw_project() -> Result<(), Box<dyn Error>> {
        let features = vec![Features::Clips, Features::Notes, Features::Audio];
        let proj = create_dummy_project(3, features);
        let meta_data = MetaData::new_fake();
        let embedded_files: HashMap<&Path, String> = HashMap::new();
        let file = Path::new("tests/test.dawproject.zip");

        DawProject::save(proj, meta_data, embedded_files, file)?;

        let _loaded_project = DawProject::load(file)?;

        //TODO
        //assert_eq!(proj.structure.unwrap(), loaded_project.structure.unwrap());
        //assert_eq!(proj.scenes.unwrap(), loaded_project.scenes.unwrap());
        Ok(())
    }

    #[test]
    pub fn save_complex_daw_project() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[test]
    pub fn save_and_load_complex_daw_project() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[test]
    pub fn write_metadata_schema() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[test]
    pub fn write_project_schema() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[test]
    pub fn load_embedded_file() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[derive(EnumIter, VariantNames, Debug, PartialEq)]
    pub enum AudioScenario {
        Warped,
        RawBeats,
        RawSeconds,
        FileWithAbsolutePath,
        FileWithRelativePath,
    }

    fn should_test_offset_and_fades(scenario: &AudioScenario) -> bool {
        !matches!(
            *scenario,
            AudioScenario::FileWithAbsolutePath | AudioScenario::FileWithRelativePath
        )
    }

    #[test]
    pub fn create_audio_example() -> Result<(), Box<dyn Error>> {
        for (scenario, name) in AudioScenario::iter().zip(AudioScenario::VARIANTS.iter()) {
            create_project_audio_example(0.0, 0.0, &scenario, name.to_string(), false)?;

            if should_test_offset_and_fades(&scenario) {
                create_project_audio_example(0.0, 0.0, &scenario, name.to_string(), true)?;
                create_project_audio_example(1.0, 0.0, &scenario, name.to_string(), false)?;
                create_project_audio_example(0.0, 1.0, &scenario, name.to_string(), false)?;
            }
        }

        Ok(())
    }

    pub fn create_project_audio_example(
        play_start_offset: f64,
        clip_time: f64,
        scenario: &AudioScenario,
        scenario_name: String,
        with_fades: bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut name = format!("Audio {}", scenario_name);
        if with_fades {
            name += "WithFades";
        }
        if play_start_offset != 0.0 {
            name += "Offset";
        }
        if clip_time != 0.0 {
            name += "Clipstart";
        }

        let mut project = create_empty_project();
        let master_track = create_track(
            "Master".to_string(),
            vec![],
            super::mixer_role::MixerRoleEnum::Master,
            1.0,
            0.5,
        );
        let mut audio_track = create_track(
            "Audio".to_string(),
            vec![ContentType::Audio],
            super::mixer_role::MixerRoleEnum::Regular,
            1.0,
            0.5,
        );

        if let Some(TrackChannelEnum::Channel(c)) = audio_track
            .track_channel
            .iter_mut()
            .find(|el| matches!(el, TrackChannelEnum::Channel(_)))
        {
            c.destination = Some(master_track.get_id())
        }

        let mut arrangement = Arrangement::new_fake();
        let mut transport = Transport::new_fake();
        let mut tempo = RealParameter::new_test(Unit::Bpm);
        tempo.value = Some(155.0);
        transport.sequence.push(TransportSequence::Tempo(tempo));
        if arrangement.sequence.is_none() {
            arrangement.sequence = Some(vec![]);
        }
        let mut arrangement_lanes = Lanes::new_fake();
        let arrangement_in_seconds = *scenario == AudioScenario::RawBeats;

        arrangement_lanes.time_unit = if arrangement_in_seconds {
            Some(TimeUnit::Seconds)
        } else {
            Some(TimeUnit::Beats)
        };

        let sample = "tests/white-glasses.wav".to_string();
        let mut audio_clip = Clip::new_test(TimeLine::new_fake(), 0.0, 0.0);
        let sample_duration = 3.097;
        let mut audio = utility::create_audio(sample.clone(), 44100, 2, sample_duration);

        if *scenario == AudioScenario::FileWithAbsolutePath {
            if audio.files_sequence.is_none() {
                audio.files_sequence = Some(vec![]);
            }

            let mut path = String::new();
            match utility::create_file_path_absolute_string(format!("./{}", sample.clone())) {
                Ok(p) => path = p,
                Err(err) => {
                    return Err(err);
                }
            }
            audio.files_sequence.as_mut().unwrap().push(FileReference {
                path,
                external: Some(true),
            });
        } else if *scenario == AudioScenario::FileWithRelativePath {
            if audio.files_sequence.is_none() {
                audio.files_sequence = Some(vec![]);
            }
            audio.files_sequence.as_mut().unwrap().push(FileReference {
                path: format!("test-data/{}", sample),
                external: Some(true),
            });
        }

        if *scenario == AudioScenario::Warped {
            let mut warps = Warps::new_test(TimeUnit::Beats);
            warps
                .warps_sequence
                .as_mut()
                .unwrap()
                .push(WarpsSequenceEnum::Warp(utility::create_warp(0.0, 0.0)));
            warps
                .warps_sequence
                .as_mut()
                .unwrap()
                .push(WarpsSequenceEnum::Warp(utility::create_warp(
                    8.0,
                    sample_duration,
                )));
            audio_clip = utility::create_clip(warps.upcast(), clip_time, 8.0);
            audio_clip.content_time_unit = Some(TimeUnit::Beats);
            audio_clip.play_start = Some(play_start_offset);
            if with_fades {
                audio_clip.fade_time_unit = Some(TimeUnit::Beats);
                audio_clip.fade_in_time = Some(0.25);
                audio_clip.fade_out_time = Some(0.25);
            }
        } else {
            audio_clip = utility::create_clip(audio.upcast(), clip_time, 8.0);
            audio_clip.content_time_unit = Some(TimeUnit::Beats);
            audio_clip.play_start = Some(play_start_offset);
            if with_fades {
                audio_clip.fade_time_unit = Some(TimeUnit::Beats);
                audio_clip.fade_in_time = Some(0.1);
                audio_clip.fade_out_time = Some(0.1);
            }
        }

        let mut clips = utility::create_clips(vec![audio_clip]);
        clips.track = Some(audio_track.get_id());
        arrangement_lanes
            .lanes_sequence
            .as_mut()
            .unwrap()
            .push(ArrangementTypeChoiceEnum::Clips(clips));

        project
            .structure
            .as_mut()
            .unwrap()
            .sequence
            .push(TrackChannelEnum::Track(master_track));
        project
            .structure
            .as_mut()
            .unwrap()
            .sequence
            .push(TrackChannelEnum::Track(audio_track));

        match save_test_project(project, name) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn create_midi_automation_example(
        name: String,
        in_clips: bool,
        is_pitch_bend: bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut project = create_empty_project();
        let master_track = create_track(
            "Master".to_string(),
            vec![],
            super::mixer_role::MixerRoleEnum::Master,
            1.0,
            0.5,
        );
        let mut instruments_track = create_track(
            "Notes".to_string(),
            vec![ContentType::Notes],
            super::mixer_role::MixerRoleEnum::Regular,
            1.0,
            0.5,
        );

        let mut arrangement = Arrangement::new_fake();
        let mut transport = Transport::new_fake();
        let mut tempo = RealParameter::new_test(Unit::Bpm);
        tempo.value = Some(123.0);
        transport.sequence.push(TransportSequence::Tempo(tempo));
        if arrangement.sequence.is_none() {
            arrangement.sequence = Some(vec![]);
        }
        let mut arrangement_lanes = Lanes::new_fake();
        arrangement_lanes.time_unit = Some(TimeUnit::Beats);

        let mut automation = Points::new_fake();
        automation.unit = Some(Unit::Normalized);

        if is_pitch_bend {
            let mut target = AutomationTarget::new_fake();
            let mut expression_type = ExpressionType::new_fake();
            expression_type
                .expression_type
                .push(ExpressionTypeEnum::PitchBend);

            target.expression = Some(ExpressionTypeEnum::PitchBend);
            target.channel = Some(0);
            automation
                .points
                .as_mut()
                .unwrap()
                .push(PointsSequenceEnum::Target(target));
        } else {
            let mut target = AutomationTarget::new_fake();
            let mut expression_type = ExpressionType::new_fake();
            expression_type
                .expression_type
                .push(ExpressionTypeEnum::ChannelController);
            target.controller = Some(1);
            target.expression = Some(ExpressionTypeEnum::PitchBend);
            target.channel = Some(0);
            automation
                .points
                .as_mut()
                .unwrap()
                .push(PointsSequenceEnum::Target(target));
        }

        let points_values = [
            (0.0, 0.0, InterpolationEnum::Linear),
            (1.0, 0.0, InterpolationEnum::Linear),
            (2.0, 0.5, InterpolationEnum::Linear),
            (3.0, 0.5, InterpolationEnum::Linear),
            (4.0, 1.0, InterpolationEnum::Linear),
            (5.0, 1.0, InterpolationEnum::Linear),
            (6.0, 0.5, InterpolationEnum::Linear),
            (7.0, 1.0, InterpolationEnum::Hold),
            (8.0, 0.5, InterpolationEnum::Hold),
        ];

        for (a, b, c) in points_values {
            automation
                .points
                .as_mut()
                .unwrap()
                .push(PointsSequenceEnum::PointType(PointsTypeEnum::RealPoint(
                    create_point(a, b, c),
                )));
        }

        if in_clips {
            let note_clip = utility::create_clip(automation.upcast(), 0.0, 8.0);
            let mut clips = utility::create_clips(vec![note_clip]);
            clips.track = Some(instruments_track.get_id());
            arrangement_lanes
                .lanes_sequence
                .as_mut()
                .unwrap()
                .push(ArrangementTypeChoiceEnum::Clips(clips));
        } else {
            automation.track = Some(instruments_track.get_id());
            arrangement_lanes
                .lanes_sequence
                .as_mut()
                .unwrap()
                .push(ArrangementTypeChoiceEnum::Points(automation));
        }

        if let Some(c) = instruments_track.get_channel() {
            c.destination = Some(master_track.get_id());
        }
        project
            .structure
            .as_mut()
            .unwrap()
            .sequence
            .push(TrackChannelEnum::Track(Track::new_test(
                "S".to_string(),
                vec![ContentType::Audio],
                super::mixer_role::MixerRoleEnum::Effect,
                0.0,
                0.0,
            )));
        project
            .structure
            .as_mut()
            .unwrap()
            .sequence
            .push(TrackChannelEnum::Track(instruments_track));
        project.arrangement = Some(arrangement);
        project.transport = Some(transport);

        save_test_project(project, name)
    }

    pub fn test_double_adapter() -> Result<(), Box<dyn Error>> {
        /*
        This JUnit test method testDoubleAdapter() verifies that a DoubleAdapter class correctly converts special
        Double values like -infinity and +infinity to their respective string representations, and vice versa,
        ensuring proper marshalling and unmarshalling functionality. The assertions confirm that -inf unmarshals
        to Double.NEGATIVE_INFINITY, inf unmarshals to Double.POSITIVE_INFINITY, Double.POSITIVE_INFINITY marshals to inf,
        and Double.NEGATIVE_INFINITY marshals to -inf.
         */
        Ok(())
    }

    pub fn save_test_project(project: Project, name: String) -> Result<(), Box<dyn Error>> {
        let meta_data = MetaData::new_fake();
        let embedded_files: HashMap<&Path, String> = HashMap::new();
        let path_file = format!("target/{}.dawproject", name);
        let path_file = Path::new(&path_file);

        let p = Project::new_test("test".to_string(), 2.11);

        DawProject::save(project, meta_data, embedded_files, path_file)?;

        Ok(())
    }

    #[test]
    pub fn create_midi_automation_in_clips_example() -> Result<(), Box<dyn Error>> {
        create_midi_automation_example("MIDI-CC1-AutomationOnTrack".to_string(), false, false)?;
        create_midi_automation_example("MIDI-CC1-AutomationInClips".to_string(), true, false)?;
        create_midi_automation_example(
            "MIDI-PitchBend-AutomationOnTrack".to_string(),
            false,
            true,
        )?;
        create_midi_automation_example("MIDI-PitchBend-AutomationInClips".to_string(), true, true)?;
        Ok(())
    }

    #[test]
    pub fn load_save_load_comparision_test() -> Result<(), Box<dyn Error>> {
        /// This test should load project exported with original dawproject in java,
        /// save and load again to compare results.
        use {super::project::Project, quick_xml::de::from_str};

        let obj1: Project = from_str(XML).unwrap();

        println!("Deserialized object {:#?}", obj1);

        let file = Path::new("tests/conform_test.dawproject.zip");

        let obj1_ser = DawProject::save(
            obj1,
            MetaData::new_fake(),
            std::collections::HashMap::new(),
            file,
        )?;

        // let obj2 = deserialize

        // compare obj1 and obj2

        Ok(())
    }
}
