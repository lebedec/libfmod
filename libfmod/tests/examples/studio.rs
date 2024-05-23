use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

use libfmod::{Error, Init, LoadBank, SpeakerMode, StopMode, Studio, StudioInit};

#[test]
fn test_simple_events() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;

    let master =
        studio.load_bank_file("./tests/data/Build/Desktop/Master.bank", LoadBank::NORMAL)?;
    let strings = studio.load_bank_file(
        "./tests/data/Build/Desktop/Master.strings.bank",
        LoadBank::NORMAL,
    )?;
    let sfx = studio.load_bank_file("./tests/data/Build/Desktop/SFX.bank", LoadBank::NORMAL)?;

    let ambience_event = studio.get_event("event:/events/1")?;
    let ambience = ambience_event.create_instance()?;

    let cancellation_event = studio.get_event("event:/events/2")?;
    let cancellation = cancellation_event.create_instance()?;

    let explosion_event = studio.get_event("event:/events/1")?;
    explosion_event.load_sample_data()?;

    for step in 0..5 {
        match step {
            0 => {
                ambience.start()?;
            }
            2 => {
                let explosion = explosion_event.create_instance()?;
                explosion.start()?;
                explosion.release()?;
            }
            3 => {
                cancellation.start()?;
            }
            4 => {
                ambience.stop(StopMode::AllowFadeout)?;
            }
            _ => {}
        }
        studio.update()?;
        sleep(Duration::from_secs(1));
    }

    sfx.unload()?;
    strings.unload()?;
    master.unload()?;

    studio.release()
}

#[test]
fn test_bank_loading_from_memory() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;
    let master =
        studio.load_bank_file("./tests/data/Build/Desktop/Master.bank", LoadBank::NORMAL)?;
    let strings = studio.load_bank_file(
        "./tests/data/Build/Desktop/Master.strings.bank",
        LoadBank::NORMAL,
    )?;
    let mut sfx_file = File::open("./tests/data/Build/Desktop/SFX.bank").unwrap();
    let mut buffer = Vec::new();
    sfx_file.read_to_end(&mut buffer).unwrap();
    let sfx = studio.load_bank_memory(&buffer, LoadBank::NORMAL)?;
    let my_event = studio.get_event("event:/events/1")?;
    let event = my_event.create_instance()?;
    event.start()?;
    for _ in 0..5 {
        studio.update()?;
        sleep(Duration::from_millis(500));
    }
    sfx.unload()?;
    strings.unload()?;
    master.unload()?;
    studio.release()
}

#[test]
fn test_get_bank_path() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;
    let strings = studio.load_bank_file(
        "./tests/data/Build/Desktop/Master.strings.bank",
        LoadBank::NORMAL,
    )?;

    let path = strings.get_path()?;
    assert_eq!(path, "bank:/Master.strings");

    studio.release()
}

#[test]
fn test_get_event_path() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;
    studio.load_bank_file("./tests/data/Build/Desktop/Master.bank", LoadBank::NORMAL)?;
    studio.load_bank_file(
        "./tests/data/Build/Desktop/Master.strings.bank",
        LoadBank::NORMAL,
    )?;
    studio.load_bank_file("./tests/data/Build/Desktop/SFX.bank", LoadBank::NORMAL)?;
    let event = studio.get_event("event:/events/1")?;

    let path = event.get_path()?;
    assert_eq!(path, "event:/events/1");

    studio.release()
}

#[test]
fn test_get_vca_path() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;
    studio.load_bank_file("./tests/data/Build/Desktop/Master.bank", LoadBank::NORMAL)?;
    studio.load_bank_file(
        "./tests/data/Build/Desktop/Master.strings.bank",
        LoadBank::NORMAL,
    )?;
    let vca = studio.get_vca("vca:/Environment")?;

    let path = vca.get_path()?;
    assert_eq!(path, "vca:/Environment");
    assert!(vca.is_valid());

    studio.release()
}

#[test]
fn test_get_bus_path() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;
    studio.load_bank_file("./tests/data/Build/Desktop/Master.bank", LoadBank::NORMAL)?;
    studio.load_bank_file(
        "./tests/data/Build/Desktop/Master.strings.bank",
        LoadBank::NORMAL,
    )?;
    let bus = studio.get_bus("bus:/MyGroup/Bus")?;

    let path = bus.get_path()?;
    assert_eq!(path, "bus:/MyGroup/Bus");

    studio.release()
}

#[test]
fn test_lookup_path() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;
    studio.load_bank_file("./tests/data/Build/Desktop/Master.bank", LoadBank::NORMAL)?;
    let strings = studio.load_bank_file(
        "./tests/data/Build/Desktop/Master.strings.bank",
        LoadBank::NORMAL,
    )?;
    let strings_id = strings.get_id()?;

    let path = studio.lookup_path(strings_id)?;
    assert_eq!(path, "bank:/Master.strings");

    studio.release()
}

#[test]
fn test_banks_list() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;
    studio.load_bank_file("./tests/data/Build/Desktop/Master.bank", LoadBank::NORMAL)?;
    studio.load_bank_file(
        "./tests/data/Build/Desktop/Master.strings.bank",
        LoadBank::NORMAL,
    )?;
    let banks_count = studio.get_bank_count()?;
    let banks = studio.get_bank_list(banks_count)?; // take all
    let banks: Vec<String> = banks.iter().map(|bank| bank.get_path().unwrap()).collect();

    assert_eq!(banks_count, 2);
    assert_eq!(
        banks,
        vec![
            "bank:/Master".to_string(),
            "bank:/Master.strings".to_string(),
        ]
    );

    studio.release()
}

#[test]
fn test_bank_events_list() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;
    studio.load_bank_file("./tests/data/Build/Desktop/Master.bank", LoadBank::NORMAL)?;
    studio.load_bank_file(
        "./tests/data/Build/Desktop/Master.strings.bank",
        LoadBank::NORMAL,
    )?;
    let sfx = studio.load_bank_file("./tests/data/Build/Desktop/SFX.bank", LoadBank::NORMAL)?;
    let count = sfx.get_event_count()?;
    let events = sfx.get_event_list(3)?; // take first 3
    let events: Vec<String> = events
        .iter()
        .map(|event| event.get_path().unwrap())
        .collect();

    assert_eq!(count, 2);
    assert_eq!(
        events,
        vec!["event:/events/1".to_string(), "event:/events/2".to_string()]
    );

    studio.release()
}
