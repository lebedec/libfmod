use libfmod::{DspConnectionType, DspType, Error, System};
use libfmod::ffi::{FMOD_CHANNELCONTROL_DSP_HEAD, FMOD_CHANNELCONTROL_DSP_TAIL, FMOD_DEFAULT, FMOD_INIT_NORMAL};

#[test]
fn test_dsp_effect_to_channel() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let sound = system.create_sound("./data/heartbeat.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, false)?;
    let echo = system.create_dsp_by_type(DspType::Echo)?;
    channel.add_dsp(0, echo)?;
    while channel.is_playing()? {
        // do something else
    }
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

    let sound = system.create_sound("./data/heartbeat.ogg", FMOD_DEFAULT, None)?;
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
    let sound = system.create_sound("./data/heartbeat.ogg", FMOD_DEFAULT, None)?;
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

    let sound = system.create_sound("./data/heartbeat.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, true)?;
    let head = channel.get_dsp(FMOD_CHANNELCONTROL_DSP_HEAD)?;

    let mut matrix = [
        0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0
    ];

    let (_, connection) = head.get_output(0)?;
    connection.set_mix_matrix(Some(matrix.as_mut_ptr()), 4, 4, None)?;
    // get mix matrix TODO
    system.release()
}