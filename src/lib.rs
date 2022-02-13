use std::ptr::null_mut;
pub mod ffi;
#[derive(Debug)]
pub struct Error {
    pub function: String,
    pub code: i32,
    pub message: String,
}
macro_rules! err {
    ($ function : expr , $ code : expr) => {
        Error {
            function: $function.to_string(),
            code: $code,
            message: ffi::map_fmod_error($code).to_string(),
        }
    };
}
pub struct Channel {
    pointer: *mut ffi::FMOD_CHANNEL,
}
impl Channel {}
pub struct ChannelGroup {
    pointer: *mut ffi::FMOD_CHANNELGROUP,
}
impl ChannelGroup {}
pub struct Dsp {
    pointer: *mut ffi::FMOD_DSP,
}
impl Dsp {}
pub struct DspConnection {
    pointer: *mut ffi::FMOD_DSPCONNECTION,
}
impl DspConnection {}
pub struct Geometry {
    pointer: *mut ffi::FMOD_GEOMETRY,
}
impl Geometry {}
pub struct Reverb3d {
    pointer: *mut ffi::FMOD_REVERB3D,
}
impl Reverb3d {}
pub struct Sound {
    pointer: *mut ffi::FMOD_SOUND,
}
impl Sound {}
pub struct SoundGroup {
    pointer: *mut ffi::FMOD_SOUNDGROUP,
}
impl SoundGroup {}
pub struct Bank {
    pointer: *mut ffi::FMOD_STUDIO_BANK,
}
impl Bank {}
pub struct Bus {
    pointer: *mut ffi::FMOD_STUDIO_BUS,
}
impl Bus {}
pub struct CommandReplay {
    pointer: *mut ffi::FMOD_STUDIO_COMMANDREPLAY,
}
impl CommandReplay {}
pub struct EventDescription {
    pointer: *mut ffi::FMOD_STUDIO_EVENTDESCRIPTION,
}
impl EventDescription {}
pub struct EventInstance {
    pointer: *mut ffi::FMOD_STUDIO_EVENTINSTANCE,
}
impl EventInstance {}
pub struct Studio {
    pointer: *mut ffi::FMOD_STUDIO_SYSTEM,
}
impl Studio {
    pub fn create() -> Result<Self, Error> {
        let mut pointer = null_mut();
        let result = unsafe { ffi::FMOD_Studio_System_Create(&mut pointer, ffi::FMOD_VERSION) };
        if result == ffi::FMOD_OK {
            Ok(Self { pointer })
        } else {
            Err(err!("FMOD_Studio_System_Create", result))
        }
    }
}
pub struct Vca {
    pointer: *mut ffi::FMOD_STUDIO_VCA,
}
impl Vca {}
pub struct System {
    pointer: *mut ffi::FMOD_SYSTEM,
}
impl System {
    pub fn create() -> Result<Self, Error> {
        let mut pointer = null_mut();
        let result = unsafe { ffi::FMOD_System_Create(&mut pointer, ffi::FMOD_VERSION) };
        if result == ffi::FMOD_OK {
            Ok(Self { pointer })
        } else {
            Err(err!("FMOD_System_Create", result))
        }
    }
}
