use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

use libfmod::{Error, LoadMemoryMode, SpeakerMode, StopMode, Studio};
use libfmod::ffi::{FMOD_INIT_NORMAL, FMOD_STUDIO_INIT_NORMAL, FMOD_STUDIO_LOAD_BANK_NORMAL};

#[test]
fn test_simple_events() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;

    let master = studio.load_bank_file("./data/Master.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    let strings = studio.load_bank_file("./data/Master.strings.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    let sfx = studio.load_bank_file("./data/SFX.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;

    let ambience_event = studio.get_event("event:/Ambience/Country")?;
    let ambience = ambience_event.create_instance()?;

    let cancellation_event = studio.get_event("event:/UI/Cancel")?;
    let cancellation = cancellation_event.create_instance()?;

    let explosion_event = studio.get_event("event:/Weapons/Explosion")?;
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
    studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;

    let master = studio.load_bank_file("./data/Master.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    let strings = studio.load_bank_file("./data/Master.strings.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    let mut sfx_file = File::open("./data/SFX.bank").unwrap();
    let mut buffer = Vec::new();
    sfx_file.read_to_end(&mut buffer).unwrap();
    let sfx = studio.load_bank_memory(&buffer, FMOD_STUDIO_LOAD_BANK_NORMAL)?;

    let ambience_event = studio.get_event("event:/Ambience/Country")?;
    let ambience = ambience_event.create_instance()?;
    ambience.start()?;

    for _ in 0..5 {
        studio.update()?;
        sleep(Duration::from_secs(1));
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
    studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;
    let strings = studio.load_bank_file("./data/Master.strings.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;

    let path = strings.get_path()?;
    assert_eq!(path, "bank:/Master.strings");

    studio.release()
}

#[test]
fn test_get_event_path() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;
    studio.load_bank_file("./data/Master.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    studio.load_bank_file("./data/Master.strings.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    studio.load_bank_file("./data/SFX.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    let event = studio.get_event("event:/Ambience/Country")?;

    let path = event.get_path()?;
    assert_eq!(path, "event:/Ambience/Country");

    studio.release()
}

#[test]
fn test_get_vca_path() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;
    studio.load_bank_file("./data/Master.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    studio.load_bank_file("./data/Master.strings.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
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
    studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;
    studio.load_bank_file("./data/Master.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    studio.load_bank_file("./data/Master.strings.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    let bus = studio.get_bus("bus:/SFX/Ambience")?;

    let path = bus.get_path()?;
    assert_eq!(path, "bus:/SFX/Ambience");

    studio.release()
}

#[test]
fn test_lookup_path() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    system.set_software_format(None, Some(SpeakerMode::Quad), None)?;
    studio.initialize(1024, FMOD_STUDIO_INIT_NORMAL, FMOD_INIT_NORMAL, None)?;
    studio.load_bank_file("./data/Master.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    let strings = studio.load_bank_file("./data/Master.strings.bank", FMOD_STUDIO_LOAD_BANK_NORMAL)?;
    let strings_id = strings.get_id()?;

    let path = studio.lookup_path(strings_id)?;
    assert_eq!(path, "bank:/Master.strings");

    studio.release()
}
