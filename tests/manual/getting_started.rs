use std::ffi::{c_void, CString};
use std::fs;
use std::ptr::null_mut;

use libfmod::ffi::{
    map_fmod_error, FMOD_CREATESOUNDEXINFO, FMOD_DEFAULT, FMOD_DSP_LOUDNESS_METER_INFO_TYPE,
    FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI, FMOD_INIT_NORMAL, FMOD_NONBLOCKING, FMOD_OPENMEMORY,
    FMOD_RESULT, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_USER_PROPERTY,
    FMOD_SYSTEM_CALLBACK_PREUPDATE,
};
use libfmod::{
    ffi, AdvancedSettings, DspResampler, Error, OpenState, Sound, Studio, StudioAdvancedSettings,
    System,
};

#[test]
fn test_core_system_initialization() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)
}

#[test]
fn test_core_system_advanced_settings() -> Result<(), Error> {
    let system = System::create()?;
    let settings = AdvancedSettings {
        max_mpeg_codecs: 32,
        max_adpcm_codecs: 32,
        max_xma_codecs: 32,
        max_vorbis_codecs: 32,
        max_at_9_codecs: 32,
        max_fadpcm_codecs: 32,
        max_pcm_codecs: 0,
        asio_num_channels: 0,
        asio_channel_list: vec![],
        asio_speaker_list: vec![],
        vol_0_virtualvol: 0.0,
        default_decode_buffer_size: 400,
        profile_port: 9164,
        geometry_max_fade_time: 500,
        distance_filter_center_freq: 1500.0,
        reverb_3_d_instance: 0,
        dsp_buffer_pool_size: 0,
        resampler_method: DspResampler::Spline,
        random_seed: 0,
        max_convolution_threads: 3,
        max_opus_codecs: 32,
    };
    system.set_advanced_settings(settings)?;
    let settings = system.get_advanced_settings()?;
    assert_eq!(settings.profile_port, 9164);
    assert_eq!(settings.resampler_method, DspResampler::Spline);
    system.release()
}

#[test]
fn test_studio_system_initialization() -> Result<(), Error> {
    let studio = Studio::create()?;
    studio.initialize(512, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)
}

#[test]
fn test_studio_system_advanced_settings() -> Result<(), Error> {
    let studio = Studio::create()?;
    let settings = StudioAdvancedSettings {
        commandqueuesize: 32768,
        handleinitialsize: 8192,
        studioupdateperiod: 20,
        idlesampledatapoolsize: 256,
        streamingscheduledelay: 10,
        encryptionkey: "secret".to_string(),
    };
    studio.set_advanced_settings(settings)?;
    let settings = studio.get_advanced_settings()?;
    println!("Settings: {:?}", settings);
    studio.release()
}

#[test]
fn test_playing_sound() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    let sound = system.create_sound("./tests/data/Assets/1.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, false)?;
    while channel.is_playing()? {
        // do something else
    }
    system.release()
}

#[test]
fn test_playing_sound_from_data() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    let data = fs::read("./tests/data/Assets/1.ogg").unwrap();
    // let sound = system.create_sound("./tests/data/Assets/1.ogg", FMOD_DEFAULT, None)?;

    let one = FMOD_DSP_LOUDNESS_METER_INFO_TYPE::default();
    let other: FMOD_DSP_LOUDNESS_METER_INFO_TYPE = unsafe { std::mem::zeroed() };

    println!(
        "one {} {:?}",
        one.loudnesshistogram.len(),
        one.loudnesshistogram
    );
    println!(
        "other {} {:?}",
        other.loudnesshistogram.len(),
        other.loudnesshistogram
    );

    let one = FMOD_STUDIO_USER_PROPERTY::default();
    let other: FMOD_STUDIO_USER_PROPERTY = unsafe { std::mem::zeroed() };
    println!("one {}", unsafe { one.union.intvalue });
    println!("other {}", unsafe { other.union.intvalue });

    let one = FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI::default();
    let other: FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI = unsafe { std::mem::zeroed() };
    println!("one {}", one.relative[0].up.x);
    println!("other {}", other.relative[0].up.x);

    let sound = unsafe {
        let mut sound = null_mut();
        let mut exinfo: FMOD_CREATESOUNDEXINFO = std::mem::zeroed();
        exinfo.cbsize = std::mem::size_of::<FMOD_CREATESOUNDEXINFO>() as i32;
        exinfo.length = data.len() as u32;
        match ffi::FMOD_System_CreateSound(
            system.as_mut_ptr(),
            data.as_ptr() as *const _,
            FMOD_OPENMEMORY,
            &mut exinfo as *mut _,
            &mut sound,
        ) {
            ffi::FMOD_OK => Ok(Sound::from(sound)),
            error => Err(Error::Fmod {
                function: "FMOD_System_CreateSound".to_string(),
                code: error,
                message: map_fmod_error(error).to_string(),
            }),
        }
    };
    let channel = system.play_sound(sound?, None, false)?;
    while channel.is_playing()? {
        // do something else
    }
    system.release()
}

#[test]
fn test_playing_streams() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    let sound = system.create_stream("./tests/data/Assets/2.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, false)?;
    while channel.is_playing()? {
        // do something else
    }
    system.release()
}

#[test]
fn test_background_loading() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    let sound = system.create_sound("./tests/data/Assets/1.ogg", FMOD_NONBLOCKING, None)?;
    let (state, filled, starving, busy) = sound.get_open_state()?;
    assert_eq!(state, OpenState::Loading);
    assert_eq!(filled, 0);
    assert_eq!(starving, false);
    assert_eq!(busy, false);
    system.release()
}

#[test]
fn test_system_pre_update_callback() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    unsafe extern "C" fn callback(
        system: *mut ffi::FMOD_SYSTEM,
        type_: ffi::FMOD_SYSTEM_CALLBACK_TYPE,
        _commanddata1: *mut c_void,
        _commanddata2: *mut c_void,
        _userdata: *mut c_void,
    ) -> FMOD_RESULT {
        println!(
            "system {} callback {} {}",
            system as usize,
            type_,
            type_ == FMOD_SYSTEM_CALLBACK_PREUPDATE
        );
        ffi::FMOD_OK
    }
    system.set_callback(Some(callback), FMOD_SYSTEM_CALLBACK_PREUPDATE)?;
    system.release()
}
