use libfmod::ffi::{
    FMOD_CHANNELCONTROL_DSP_HEAD, FMOD_CHANNELCONTROL_DSP_TAIL, FMOD_DEFAULT, FMOD_INIT_NORMAL,
};
use libfmod::{DspConnectionType, DspParameterFft, DspType, Error, System};

/// This unit produces an echo on the sound and fades out at the desired rate.
#[test]
fn test_dsp_effect_to_channel() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let sound = system.create_sound("./tests/data/Assets/1.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, false)?;
    let echo = system.create_dsp_by_type(DspType::Echo)?;
    channel.add_dsp(0, echo)?;
    while channel.is_playing()? {
        // do something else
    }
    system.release()
}

/// This dsp simply analyzes the signal and provides spectrum information back.
#[test]
fn test_fft_dsp() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let sound = system.create_sound("./tests/data/Assets/2.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, false)?;
    let dsp = system.create_dsp_by_type(DspType::Fft)?;
    channel.add_dsp(0, dsp)?;
    while channel.is_playing()? {}
    let fft = DspParameterFft::try_from(dsp)?;

    assert_eq!(fft.spectrum.len(), 2);
    assert_eq!(fft.length, 2048);
    assert_eq!(fft.spectrum[0].len(), 2048);
    assert_eq!(fft.spectrum[1].len(), 2048);

    system.release()
}

#[test]
fn test_fft_from_type_mismatched_dsp() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let sound = system.create_sound("./tests/data/Assets/1.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, false)?;
    let echo_dsp = system.create_dsp_by_type(DspType::Echo)?;
    channel.add_dsp(0, echo_dsp)?;
    let fft = DspParameterFft::try_from(echo_dsp);
    assert_eq!(
        "trying get FFT from DSP which not FFT",
        format!("{}", fft.err().unwrap())
    );

    system.release()
}

#[test]
fn test_dps_description_name() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let description = system.get_dsp_info_by_type(DspType::Echo)?;
    let bytes = description.name.map(|char| char as u8).to_vec();
    let name = String::from_utf8(bytes[..9].to_vec());

    assert_eq!(name, Ok("FMOD Echo".to_string()));

    system.release()
}

#[test]
fn test_channels_dsp() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let reverb = system.create_dsp_by_type(DspType::Sfxreverb)?;
    let master = system.get_master_channel_group()?;
    let tail = master.get_dsp(FMOD_CHANNELCONTROL_DSP_TAIL)?;
    tail.add_input(reverb, DspConnectionType::Send)?;
    reverb.set_active(true)?;

    let sound = system.create_sound("./tests/data/Assets/2.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, Some(master), true)?;
    let head = channel.get_dsp(FMOD_CHANNELCONTROL_DSP_HEAD)?;
    reverb.add_input(head, DspConnectionType::Send)?;

    channel.set_paused(false)?;
    while channel.is_playing()? {
        // do something else
    }

    system.release()
}

#[test]
fn test_controlling_mix_level() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let reverb = system.create_dsp_by_type(DspType::Sfxreverb)?;
    let sound = system.create_sound("./tests/data/Assets/1.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, true)?;
    let head = channel.get_dsp(FMOD_CHANNELCONTROL_DSP_HEAD)?;
    let connection = reverb.add_input(head, DspConnectionType::Send)?;
    connection.set_mix(0.0)?;

    system.release()
}

#[test]
fn test_output_pan_matrix() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let sound = system.create_sound("./tests/data/Assets/2.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, true)?;
    let head = channel.get_dsp(FMOD_CHANNELCONTROL_DSP_HEAD)?;

    let mut matrix = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    ];

    let (_, connection) = head.get_output(0)?;
    connection.set_mix_matrix(Some(matrix.as_mut_ptr()), 4, 4, None)?;
    // get mix matrix TODO
    system.release()
}
