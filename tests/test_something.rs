use libfmod::Error;

#[test]
fn test_something() -> Result<(), Error> {
    /*
    let system = System::create()?;
    system.set_output(Outputtype::Nosound)?;
    let output = system.get_output()?;
    println!("output: {:?}", output);
    let (guid, name, speakermode) = system.get_driver_info(0, 11)?;
    println!("di: {:?}, name: {:?}, speakermode: {:?}", guid, name, speakermode);
    let settings = system.get_advanced_settings()?;
    println!("settings: {:?}", settings);
    let usage = system.get_cpu_usage()?;
    println!("usage: {:?}", usage);

    let prop = ffi::FMOD_REVERB_PROPERTIES {
        DecayTime: 250.0,
        EarlyDelay: 2.0,
        LateDelay: 3.0,
        HFReference: 4.0,
        HFDecayRatio: 5.0,
        Diffusion: 6.0,
        Density: 7.0,
        LowShelfFrequency: 8.0,
        LowShelfGain: 9.0,
        HighCut: 1.0,
        EarlyLateMix: 2.0,
        WetLevel: 3.0
    };
    system.set_reverb_properties(0, &prop)?;
    let prop = system.get_reverb_properties(0)?;
    println!("prop: {:?}", prop);

    */
    Ok(())
}