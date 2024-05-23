use libfmod::ffi::{FMOD_3D, FMOD_3D_LINEARROLLOFF, FMOD_DEFAULT, FMOD_INIT_NORMAL};
use libfmod::{Error, System, Vector};

#[test]
fn test_3d_sound() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;

    let doppler_scale = 1.0;
    let distance_factor = 1.0;
    let rolloff_scale = 1.0;
    system.set_3d_settings(doppler_scale, distance_factor, rolloff_scale)?;

    let sound = system.create_sound("./tests/data/Assets/1.ogg", FMOD_3D, None)?;
    sound.set_mode(FMOD_3D_LINEARROLLOFF)?;

    let channel = system.play_sound(sound, None, false)?;

    // game loop
    let mut position = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let velocity = Vector {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    while channel.is_playing()? {
        position.x += velocity.x;
        system.set_3d_listener_attributes(
            0,
            Some(position.clone()),
            Some(velocity.clone()),
            None,
            None,
        )?;
    }

    system.release()
}

#[test]
fn test_3d_optional_parameters() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    system.set_3d_settings(1.0, 1.0, 1.0)?;

    let position = Vector::new(0.1, 0.2, 0.3);
    let velocity = Vector::new(0.0, 0.5, 0.0);
    let forward = Vector::new(0.0, 0.0, 1.0); // default
    let up = Vector::new(0.0, 1.0, 0.0); // default
    system.set_3d_listener_attributes(0, Some(position), Some(velocity), None, None)?;

    let expected = (position, velocity, forward, up);
    let attributes = system.get_3d_listener_attributes(0)?;
    assert_eq!(attributes, expected, "position, velocity, forward, up");
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

    let sound = system.create_sound("./tests/data/Assets/2.ogg", FMOD_3D, None)?;

    let channel = system.play_sound(sound, None, false)?;

    // game loop
    let a = Vector {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let b = Vector {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    while channel.is_playing()? {
        system.set_3d_listener_attributes(0, None, Some(a.clone()), None, None)?;
        system.set_3d_listener_attributes(1, None, Some(b.clone()), None, None)?;
    }

    system.release()
}

#[test]
fn test_sound_custom_rolloff() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    let sound = system.create_sound("./tests/data/Assets/1.ogg", FMOD_DEFAULT, None)?;
    let curve = vec![
        Vector {
            x: 0.0,
            y: 0.75,
            z: 0.0,
        },
        Vector {
            x: 1.0,
            y: 0.25,
            z: 0.0,
        },
        Vector {
            x: 2.0,
            y: 0.25,
            z: 0.0,
        },
    ];
    sound.set_3d_custom_rolloff(curve)?;
    let rolloff = sound.get_3d_custom_rolloff()?;
    println!("rolloff: {:?}", rolloff);
    system.release()
}

#[test]
fn test_channel_custom_rolloff() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    let sound = system.create_sound("./tests/data/Assets/1.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, false)?;
    let curve = vec![
        Vector {
            x: 0.0,
            y: 0.75,
            z: 0.0,
        },
        Vector {
            x: 1.0,
            y: 0.25,
            z: 0.0,
        },
        Vector {
            x: 2.0,
            y: 0.25,
            z: 0.0,
        },
    ];
    channel.set_3d_custom_rolloff(curve)?;
    let rolloff = channel.get_3d_custom_rolloff()?;
    println!("rolloff: {:?}", rolloff);
    system.release()
}
