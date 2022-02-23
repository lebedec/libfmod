use std::ffi::c_void;

use libfmod::{AdvancedSettings, DspResampler, Error, ffi, OpenState, Studio, StudioAdvancedSettings, System};
use libfmod::ffi::{FMOD_DEFAULT, FMOD_INIT_NORMAL, FMOD_NONBLOCKING, FMOD_RESULT, FMOD_STUDIO_INIT_NORMAL, FMOD_SYSTEM_CALLBACK_PREUPDATE};

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
    let sound = system.create_sound("./data/heartbeat.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, false)?;
    while channel.is_playing()? {
        // do something else
    }
    system.release()
}

/*
#[test]
fn test_extended_sound_creating() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    let master = system.get_master_sound_group()?;
    let config = CreateSoundexInfo {
        length: 0,
        fileoffset: 0,
        numchannels: 0,
        defaultfrequency: 0,
        format: SoundFormat::None,
        decodebuffersize: 0,
        initialsubsound: 0,
        numsubsounds: 0,
        inclusionlist: vec![],
        inclusionlistnum: 0,
        pcmreadcallback: None,
        pcmsetposcallback: None,
        nonblockcallback: None,
        dlsname: "".to_string(),
        encryptionkey: "".to_string(),
        maxpolyphony: 0,
        userdata: null_mut(),
        suggestedsoundtype: SoundType::Unknown,
        fileuseropen: None,
        fileuserclose: None,
        fileuserread: None,
        fileuserseek: None,
        fileuserasyncread: None,
        fileuserasynccancel: None,
        fileuserdata: null_mut(),
        filebuffersize: 0,
        channelorder: ChannelOrder::Default,
        initialsoundgroup: master,
        initialseekposition: 0,
        initialseekpostype: 0,
        ignoresetfilesystem: 0,
        audioqueuepolicy: 0,
        minmidigranularity: 0,
        nonblockthreadid: 0,
        fsbguid: Guid {
            data_1: 0,
            data_2: 0,
            data_3: 0,
            data_4: [0, 0, 0, 0, 0, 0, 0, 0]
        }
    };
    let sound = system.create_sound("./data/heartbeat.ogg", FMOD_DEFAULT, Some(config))?;
    system.release()
}*/

#[test]
fn test_playing_steams() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    let sound = system.create_stream("./data/heartbeat.ogg", FMOD_DEFAULT, None)?;
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
    let sound = system.create_sound("./data/heartbeat.ogg", FMOD_NONBLOCKING, None)?;
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
        println!("system {} callback {} {}", system as usize, type_, type_ == FMOD_SYSTEM_CALLBACK_PREUPDATE);
        ffi::FMOD_OK
    }
    system.set_callback(Some(callback), FMOD_SYSTEM_CALLBACK_PREUPDATE)?;
    system.release()
}
