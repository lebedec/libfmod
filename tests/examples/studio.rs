use std::thread::sleep;
use std::time::Duration;

use libfmod::{Error, SpeakerMode, StopMode, Studio};
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