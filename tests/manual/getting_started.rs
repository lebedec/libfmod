use std::ptr::null_mut;
use libfmod::{AdvancedSettings, DspResampler, Error, ffi, Studio, StudioAdvancedSettings, System};
use libfmod::ffi::{FMOD_DEFAULT, FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL};


#[test]
fn test_core_system_initialization() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, null_mut())
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
        profile_port: 9264,
        geometry_max_fade_time: 500,
        distance_filter_center_freq: 1500.0,
        reverb_3_dinstance: 0,
        dsp_buffer_pool_size: 0,
        resampler_method: DspResampler::Default,
        random_seed: 0,
        max_convolution_threads: 3,
        max_opus_codecs: 32
    };
    system.set_advanced_settings(settings)?;
    let settings = system.get_advanced_settings()?;
    println!("settings: {:?}", settings);
    Ok(())
}

#[test]
fn test_studio_system_initialization() -> Result<(), Error> {
    let studio = Studio::create()?;
    studio.initialize(512, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, null_mut())
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
        encryptionkey: "secret".to_string()
    };
    studio.set_advanced_settings(settings)?;
    // let settings = studio.get_advanced_settings()?; TODO: debug
    // println!("Settings: {:?}", settings);
    Ok(())
}

#[test]
fn test_playing_sound() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, null_mut())?;
    /*
    let sound = system.create_sound("sample.ogg", FMOD_DEFAULT)?;
    let channel = system.play_sound(sound, NOne, false)?;
    while channel.is_playing()? {

    }*/
    Ok(())
}