use libfmod::ffi::FMOD_INIT_NORMAL;
use libfmod::{Error, ReverbProperties, System, Vector};

#[test]
fn test_3d_reverb() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    let reverb = system.create_reverb_3d()?;
    reverb.set_properties(ReverbProperties::underwater())?;
    let pos = Vector {
        x: -10.0,
        y: 0.0,
        z: 0.0,
    };
    let min_dist = 10.0;
    let max_dist = 20.0;
    reverb.set_3d_attributes(Some(pos), min_dist, max_dist)?;
    let listener = Vector {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    system.set_3d_listener_attributes(0, Some(listener), None, None, None)?;
    system.release()
}
