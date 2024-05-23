use libfmod::{Error, SpeakerMode, System};

#[test]
fn test_system_sample_rate_configuration() -> Result<(), Error> {
    let system = System::create()?;
    system.set_software_format(Some(24000), None, None)
}

#[test]
fn test_system_channel_count_configuration() -> Result<(), Error> {
    let system = System::create()?;
    system.set_software_format(None, Some(SpeakerMode::Mono), None)
}

#[test]
fn test_system_dsp_block_configuration() -> Result<(), Error> {
    let system = System::create()?;
    system.set_dsp_buffer_size(512, 2)
}

#[test]
fn test_cpu_usage() -> Result<(), Error> {
    let system = System::create()?;
    let usage = system.get_cpu_usage()?;
    println!("usage: {:?}", usage);
    system.release()
}
