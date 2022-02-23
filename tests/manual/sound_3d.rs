use libfmod::{Error, System, Vector};
use libfmod::ffi::{FMOD_3D, FMOD_3D_LINEARROLLOFF, FMOD_INIT_NORMAL};

#[test]
fn test_3d_sound() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let doppler_scale = 1.0;
    let distance_factor = 1.0;
    let rolloff_scale = 1.0;
    system.set_3d_settings(doppler_scale, distance_factor, rolloff_scale)?;

    let sound = system.create_sound("./data/heartbeat.ogg", FMOD_3D, None)?;
    sound.set_mode(FMOD_3D_LINEARROLLOFF)?;

    let channel = system.play_sound(sound, None, false)?;

    // game loop
    let mut position = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0
    };
    let velocity = Vector {
        x: -1.0,
        y: 0.0,
        z: 0.0
    };
    while channel.is_playing()? {
        position.x += velocity.x;
        system.set_3d_listener_attributes(0, Some(position.clone()), Some(velocity.clone()), None, None)?;
        system.update()?;
    }

    system.release()
}

#[test]
fn test_multiple_listeners() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let doppler_scale = 1.0;
    let distance_factor = 1.0;
    let rolloff_scale = 1.0;
    system.set_3d_settings(doppler_scale, distance_factor, rolloff_scale)?;
    system.set_3d_num_listeners(2)?;

    let sound = system.create_sound("./data/heartbeat.ogg", FMOD_3D, None)?;

    let channel = system.play_sound(sound, None, false)?;

    // game loop
    let a = Vector {
        x: 1.0,
        y: 0.0,
        z: 0.0
    };
    let b = Vector {
        x: -1.0,
        y: 0.0,
        z: 0.0
    };
    while channel.is_playing()? {
        system.set_3d_listener_attributes(0, None, Some(a.clone()), None, None)?;
        system.set_3d_listener_attributes(1, None, Some(b.clone()), None, None)?;
        system.update()?;
    }

    system.release()
}