#![allow(unused_unsafe)]

use std::ffi::{c_void, CStr, CString, IntoStringError, NulError};
use std::mem::size_of;
use std::os::raw::c_char;
use std::ptr::{null, null_mut};
use std::slice;

pub mod ffi;

#[derive(Debug)]
pub enum Error {
    Fmod {
        function: String,
        code: i32,
        message: String,
    },
    EnumBindgen {
        enumeration: String,
        value: String,
    },
    String(IntoStringError),
    StringNul(NulError),
    NotDspFft,
}

impl From<NulError> for Error {
    fn from(error: NulError) -> Self {
        Error::StringNul(error)
    }
}
macro_rules! err_fmod {
    ($ function : expr , $ code : expr) => {
        Error::Fmod {
            function: $function.to_string(),
            code: $code,
            message: ffi::map_fmod_error($code).to_string(),
        }
    };
}
macro_rules! err_enum {
    ($ enumeration : expr , $ value : expr) => {
        Error::EnumBindgen {
            enumeration: $enumeration.to_string(),
            value: $value.to_string(),
        }
    };
}
macro_rules! to_string {
    ($ ptr : expr) => {
        if $ptr.is_null() {
            Ok(String::new())
        } else {
            CString::from(CStr::from_ptr($ptr))
                .into_string()
                .map_err(Error::String)
        }
    };
}
macro_rules! to_vec {
    ($ ptr : expr , $ length : expr , $ closure : expr) => {
        slice::from_raw_parts($ptr, $length as usize)
            .to_vec()
            .into_iter()
            .map($closure)
            .collect::<Result<Vec<_>, Error>>()
    };
    ($ ptr : expr , $ length : expr) => {
        slice::from_raw_parts($ptr, $length as usize).to_vec()
    };
}
macro_rules! to_bool {
    ($ value : expr) => {
        match $value {
            1 => true,
            _ => false,
        }
    };
}
macro_rules! from_bool {
    ($ value : expr) => {
        match $value {
            true => 1,
            _ => 0,
        }
    };
}
pub fn attr3d_array8(
    values: Vec<Attributes3d>,
) -> [Attributes3d; ffi::FMOD_MAX_LISTENERS as usize] {
    values.try_into().expect("slice with incorrect length")
}

pub fn vec_as_mut_ptr<T, O, F>(values: Vec<T>, map: F) -> *mut O
    where
        F: FnMut(T) -> O,
{
    let mut values = values.into_iter().map(map).collect::<Vec<O>>();
    let pointer = values.as_mut_ptr();
    std::mem::forget(values);
    pointer
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadingState {
    Unloading,
    Unloaded,
    Loading,
    Loaded,
    Error,
}

impl From<LoadingState> for ffi::FMOD_STUDIO_LOADING_STATE {
    fn from(value: LoadingState) -> ffi::FMOD_STUDIO_LOADING_STATE {
        match value {
            LoadingState::Unloading => ffi::FMOD_STUDIO_LOADING_STATE_UNLOADING,
            LoadingState::Unloaded => ffi::FMOD_STUDIO_LOADING_STATE_UNLOADED,
            LoadingState::Loading => ffi::FMOD_STUDIO_LOADING_STATE_LOADING,
            LoadingState::Loaded => ffi::FMOD_STUDIO_LOADING_STATE_LOADED,
            LoadingState::Error => ffi::FMOD_STUDIO_LOADING_STATE_ERROR,
        }
    }
}

impl LoadingState {
    pub fn from(value: ffi::FMOD_STUDIO_LOADING_STATE) -> Result<LoadingState, Error> {
        match value {
            ffi::FMOD_STUDIO_LOADING_STATE_UNLOADING => Ok(LoadingState::Unloading),
            ffi::FMOD_STUDIO_LOADING_STATE_UNLOADED => Ok(LoadingState::Unloaded),
            ffi::FMOD_STUDIO_LOADING_STATE_LOADING => Ok(LoadingState::Loading),
            ffi::FMOD_STUDIO_LOADING_STATE_LOADED => Ok(LoadingState::Loaded),
            ffi::FMOD_STUDIO_LOADING_STATE_ERROR => Ok(LoadingState::Error),
            _ => Err(err_enum!("FMOD_STUDIO_LOADING_STATE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadMemoryMode {
    Memory,
    MemoryPoint,
}

impl From<LoadMemoryMode> for ffi::FMOD_STUDIO_LOAD_MEMORY_MODE {
    fn from(value: LoadMemoryMode) -> ffi::FMOD_STUDIO_LOAD_MEMORY_MODE {
        match value {
            LoadMemoryMode::Memory => ffi::FMOD_STUDIO_LOAD_MEMORY,
            LoadMemoryMode::MemoryPoint => ffi::FMOD_STUDIO_LOAD_MEMORY_POINT,
        }
    }
}

impl LoadMemoryMode {
    pub fn from(value: ffi::FMOD_STUDIO_LOAD_MEMORY_MODE) -> Result<LoadMemoryMode, Error> {
        match value {
            ffi::FMOD_STUDIO_LOAD_MEMORY => Ok(LoadMemoryMode::Memory),
            ffi::FMOD_STUDIO_LOAD_MEMORY_POINT => Ok(LoadMemoryMode::MemoryPoint),
            _ => Err(err_enum!("FMOD_STUDIO_LOAD_MEMORY_MODE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParameterType {
    GameControlled,
    AutomaticDistance,
    AutomaticEventConeAngle,
    AutomaticEventOrientation,
    AutomaticDirection,
    AutomaticElevation,
    AutomaticListenerOrientation,
    AutomaticSpeed,
    AutomaticSpeedAbsolute,
    AutomaticDistanceNormalized,
    Max,
}

impl From<ParameterType> for ffi::FMOD_STUDIO_PARAMETER_TYPE {
    fn from(value: ParameterType) -> ffi::FMOD_STUDIO_PARAMETER_TYPE {
        match value {
            ParameterType::GameControlled => ffi::FMOD_STUDIO_PARAMETER_GAME_CONTROLLED,
            ParameterType::AutomaticDistance => ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE,
            ParameterType::AutomaticEventConeAngle => {
                ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_CONE_ANGLE
            }
            ParameterType::AutomaticEventOrientation => {
                ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_ORIENTATION
            }
            ParameterType::AutomaticDirection => ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_DIRECTION,
            ParameterType::AutomaticElevation => ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_ELEVATION,
            ParameterType::AutomaticListenerOrientation => {
                ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_LISTENER_ORIENTATION
            }
            ParameterType::AutomaticSpeed => ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED,
            ParameterType::AutomaticSpeedAbsolute => {
                ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED_ABSOLUTE
            }
            ParameterType::AutomaticDistanceNormalized => {
                ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE_NORMALIZED
            }
            ParameterType::Max => ffi::FMOD_STUDIO_PARAMETER_MAX,
        }
    }
}

impl ParameterType {
    pub fn from(value: ffi::FMOD_STUDIO_PARAMETER_TYPE) -> Result<ParameterType, Error> {
        match value {
            ffi::FMOD_STUDIO_PARAMETER_GAME_CONTROLLED => Ok(ParameterType::GameControlled),
            ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE => Ok(ParameterType::AutomaticDistance),
            ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_CONE_ANGLE => {
                Ok(ParameterType::AutomaticEventConeAngle)
            }
            ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_EVENT_ORIENTATION => {
                Ok(ParameterType::AutomaticEventOrientation)
            }
            ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_DIRECTION => Ok(ParameterType::AutomaticDirection),
            ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_ELEVATION => Ok(ParameterType::AutomaticElevation),
            ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_LISTENER_ORIENTATION => {
                Ok(ParameterType::AutomaticListenerOrientation)
            }
            ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED => Ok(ParameterType::AutomaticSpeed),
            ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_SPEED_ABSOLUTE => {
                Ok(ParameterType::AutomaticSpeedAbsolute)
            }
            ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE_NORMALIZED => {
                Ok(ParameterType::AutomaticDistanceNormalized)
            }
            ffi::FMOD_STUDIO_PARAMETER_MAX => Ok(ParameterType::Max),
            _ => Err(err_enum!("FMOD_STUDIO_PARAMETER_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserPropertyType {
    Integer,
    Boolean,
    Float,
    String,
}

impl From<UserPropertyType> for ffi::FMOD_STUDIO_USER_PROPERTY_TYPE {
    fn from(value: UserPropertyType) -> ffi::FMOD_STUDIO_USER_PROPERTY_TYPE {
        match value {
            UserPropertyType::Integer => ffi::FMOD_STUDIO_USER_PROPERTY_TYPE_INTEGER,
            UserPropertyType::Boolean => ffi::FMOD_STUDIO_USER_PROPERTY_TYPE_BOOLEAN,
            UserPropertyType::Float => ffi::FMOD_STUDIO_USER_PROPERTY_TYPE_FLOAT,
            UserPropertyType::String => ffi::FMOD_STUDIO_USER_PROPERTY_TYPE_STRING,
        }
    }
}

impl UserPropertyType {
    pub fn from(value: ffi::FMOD_STUDIO_USER_PROPERTY_TYPE) -> Result<UserPropertyType, Error> {
        match value {
            ffi::FMOD_STUDIO_USER_PROPERTY_TYPE_INTEGER => Ok(UserPropertyType::Integer),
            ffi::FMOD_STUDIO_USER_PROPERTY_TYPE_BOOLEAN => Ok(UserPropertyType::Boolean),
            ffi::FMOD_STUDIO_USER_PROPERTY_TYPE_FLOAT => Ok(UserPropertyType::Float),
            ffi::FMOD_STUDIO_USER_PROPERTY_TYPE_STRING => Ok(UserPropertyType::String),
            _ => Err(err_enum!("FMOD_STUDIO_USER_PROPERTY_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventProperty {
    ChannelPriority,
    ScheduleDelay,
    ScheduleLookahead,
    MinimumDistance,
    MaximumDistance,
    Cooldown,
    Max,
}

impl From<EventProperty> for ffi::FMOD_STUDIO_EVENT_PROPERTY {
    fn from(value: EventProperty) -> ffi::FMOD_STUDIO_EVENT_PROPERTY {
        match value {
            EventProperty::ChannelPriority => ffi::FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY,
            EventProperty::ScheduleDelay => ffi::FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY,
            EventProperty::ScheduleLookahead => ffi::FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD,
            EventProperty::MinimumDistance => ffi::FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE,
            EventProperty::MaximumDistance => ffi::FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE,
            EventProperty::Cooldown => ffi::FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN,
            EventProperty::Max => ffi::FMOD_STUDIO_EVENT_PROPERTY_MAX,
        }
    }
}

impl EventProperty {
    pub fn from(value: ffi::FMOD_STUDIO_EVENT_PROPERTY) -> Result<EventProperty, Error> {
        match value {
            ffi::FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY => Ok(EventProperty::ChannelPriority),
            ffi::FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY => Ok(EventProperty::ScheduleDelay),
            ffi::FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD => {
                Ok(EventProperty::ScheduleLookahead)
            }
            ffi::FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE => Ok(EventProperty::MinimumDistance),
            ffi::FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE => Ok(EventProperty::MaximumDistance),
            ffi::FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN => Ok(EventProperty::Cooldown),
            ffi::FMOD_STUDIO_EVENT_PROPERTY_MAX => Ok(EventProperty::Max),
            _ => Err(err_enum!("FMOD_STUDIO_EVENT_PROPERTY", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Playing,
    Sustaining,
    Stopped,
    Starting,
    Stopping,
}

impl From<PlaybackState> for ffi::FMOD_STUDIO_PLAYBACK_STATE {
    fn from(value: PlaybackState) -> ffi::FMOD_STUDIO_PLAYBACK_STATE {
        match value {
            PlaybackState::Playing => ffi::FMOD_STUDIO_PLAYBACK_PLAYING,
            PlaybackState::Sustaining => ffi::FMOD_STUDIO_PLAYBACK_SUSTAINING,
            PlaybackState::Stopped => ffi::FMOD_STUDIO_PLAYBACK_STOPPED,
            PlaybackState::Starting => ffi::FMOD_STUDIO_PLAYBACK_STARTING,
            PlaybackState::Stopping => ffi::FMOD_STUDIO_PLAYBACK_STOPPING,
        }
    }
}

impl PlaybackState {
    pub fn from(value: ffi::FMOD_STUDIO_PLAYBACK_STATE) -> Result<PlaybackState, Error> {
        match value {
            ffi::FMOD_STUDIO_PLAYBACK_PLAYING => Ok(PlaybackState::Playing),
            ffi::FMOD_STUDIO_PLAYBACK_SUSTAINING => Ok(PlaybackState::Sustaining),
            ffi::FMOD_STUDIO_PLAYBACK_STOPPED => Ok(PlaybackState::Stopped),
            ffi::FMOD_STUDIO_PLAYBACK_STARTING => Ok(PlaybackState::Starting),
            ffi::FMOD_STUDIO_PLAYBACK_STOPPING => Ok(PlaybackState::Stopping),
            _ => Err(err_enum!("FMOD_STUDIO_PLAYBACK_STATE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StopMode {
    AllowFadeout,
    Immediate,
}

impl From<StopMode> for ffi::FMOD_STUDIO_STOP_MODE {
    fn from(value: StopMode) -> ffi::FMOD_STUDIO_STOP_MODE {
        match value {
            StopMode::AllowFadeout => ffi::FMOD_STUDIO_STOP_ALLOWFADEOUT,
            StopMode::Immediate => ffi::FMOD_STUDIO_STOP_IMMEDIATE,
        }
    }
}

impl StopMode {
    pub fn from(value: ffi::FMOD_STUDIO_STOP_MODE) -> Result<StopMode, Error> {
        match value {
            ffi::FMOD_STUDIO_STOP_ALLOWFADEOUT => Ok(StopMode::AllowFadeout),
            ffi::FMOD_STUDIO_STOP_IMMEDIATE => Ok(StopMode::Immediate),
            _ => Err(err_enum!("FMOD_STUDIO_STOP_MODE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstanceType {
    None,
    System,
    EventDescription,
    EventInstance,
    ParameterInstance,
    Bus,
    Vca,
    Bank,
    CommandReplay,
}

impl From<InstanceType> for ffi::FMOD_STUDIO_INSTANCETYPE {
    fn from(value: InstanceType) -> ffi::FMOD_STUDIO_INSTANCETYPE {
        match value {
            InstanceType::None => ffi::FMOD_STUDIO_INSTANCETYPE_NONE,
            InstanceType::System => ffi::FMOD_STUDIO_INSTANCETYPE_SYSTEM,
            InstanceType::EventDescription => ffi::FMOD_STUDIO_INSTANCETYPE_EVENTDESCRIPTION,
            InstanceType::EventInstance => ffi::FMOD_STUDIO_INSTANCETYPE_EVENTINSTANCE,
            InstanceType::ParameterInstance => ffi::FMOD_STUDIO_INSTANCETYPE_PARAMETERINSTANCE,
            InstanceType::Bus => ffi::FMOD_STUDIO_INSTANCETYPE_BUS,
            InstanceType::Vca => ffi::FMOD_STUDIO_INSTANCETYPE_VCA,
            InstanceType::Bank => ffi::FMOD_STUDIO_INSTANCETYPE_BANK,
            InstanceType::CommandReplay => ffi::FMOD_STUDIO_INSTANCETYPE_COMMANDREPLAY,
        }
    }
}

impl InstanceType {
    pub fn from(value: ffi::FMOD_STUDIO_INSTANCETYPE) -> Result<InstanceType, Error> {
        match value {
            ffi::FMOD_STUDIO_INSTANCETYPE_NONE => Ok(InstanceType::None),
            ffi::FMOD_STUDIO_INSTANCETYPE_SYSTEM => Ok(InstanceType::System),
            ffi::FMOD_STUDIO_INSTANCETYPE_EVENTDESCRIPTION => Ok(InstanceType::EventDescription),
            ffi::FMOD_STUDIO_INSTANCETYPE_EVENTINSTANCE => Ok(InstanceType::EventInstance),
            ffi::FMOD_STUDIO_INSTANCETYPE_PARAMETERINSTANCE => Ok(InstanceType::ParameterInstance),
            ffi::FMOD_STUDIO_INSTANCETYPE_BUS => Ok(InstanceType::Bus),
            ffi::FMOD_STUDIO_INSTANCETYPE_VCA => Ok(InstanceType::Vca),
            ffi::FMOD_STUDIO_INSTANCETYPE_BANK => Ok(InstanceType::Bank),
            ffi::FMOD_STUDIO_INSTANCETYPE_COMMANDREPLAY => Ok(InstanceType::CommandReplay),
            _ => Err(err_enum!("FMOD_STUDIO_INSTANCETYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThreadType {
    Mixer,
    Feeder,
    Stream,
    File,
    Nonblocking,
    Record,
    Geometry,
    Profiler,
    StudioUpdate,
    StudioLoadBank,
    StudioLoadSample,
    Convolution1,
    Convolution2,
    Max,
}

impl From<ThreadType> for ffi::FMOD_THREAD_TYPE {
    fn from(value: ThreadType) -> ffi::FMOD_THREAD_TYPE {
        match value {
            ThreadType::Mixer => ffi::FMOD_THREAD_TYPE_MIXER,
            ThreadType::Feeder => ffi::FMOD_THREAD_TYPE_FEEDER,
            ThreadType::Stream => ffi::FMOD_THREAD_TYPE_STREAM,
            ThreadType::File => ffi::FMOD_THREAD_TYPE_FILE,
            ThreadType::Nonblocking => ffi::FMOD_THREAD_TYPE_NONBLOCKING,
            ThreadType::Record => ffi::FMOD_THREAD_TYPE_RECORD,
            ThreadType::Geometry => ffi::FMOD_THREAD_TYPE_GEOMETRY,
            ThreadType::Profiler => ffi::FMOD_THREAD_TYPE_PROFILER,
            ThreadType::StudioUpdate => ffi::FMOD_THREAD_TYPE_STUDIO_UPDATE,
            ThreadType::StudioLoadBank => ffi::FMOD_THREAD_TYPE_STUDIO_LOAD_BANK,
            ThreadType::StudioLoadSample => ffi::FMOD_THREAD_TYPE_STUDIO_LOAD_SAMPLE,
            ThreadType::Convolution1 => ffi::FMOD_THREAD_TYPE_CONVOLUTION1,
            ThreadType::Convolution2 => ffi::FMOD_THREAD_TYPE_CONVOLUTION2,
            ThreadType::Max => ffi::FMOD_THREAD_TYPE_MAX,
        }
    }
}

impl ThreadType {
    pub fn from(value: ffi::FMOD_THREAD_TYPE) -> Result<ThreadType, Error> {
        match value {
            ffi::FMOD_THREAD_TYPE_MIXER => Ok(ThreadType::Mixer),
            ffi::FMOD_THREAD_TYPE_FEEDER => Ok(ThreadType::Feeder),
            ffi::FMOD_THREAD_TYPE_STREAM => Ok(ThreadType::Stream),
            ffi::FMOD_THREAD_TYPE_FILE => Ok(ThreadType::File),
            ffi::FMOD_THREAD_TYPE_NONBLOCKING => Ok(ThreadType::Nonblocking),
            ffi::FMOD_THREAD_TYPE_RECORD => Ok(ThreadType::Record),
            ffi::FMOD_THREAD_TYPE_GEOMETRY => Ok(ThreadType::Geometry),
            ffi::FMOD_THREAD_TYPE_PROFILER => Ok(ThreadType::Profiler),
            ffi::FMOD_THREAD_TYPE_STUDIO_UPDATE => Ok(ThreadType::StudioUpdate),
            ffi::FMOD_THREAD_TYPE_STUDIO_LOAD_BANK => Ok(ThreadType::StudioLoadBank),
            ffi::FMOD_THREAD_TYPE_STUDIO_LOAD_SAMPLE => Ok(ThreadType::StudioLoadSample),
            ffi::FMOD_THREAD_TYPE_CONVOLUTION1 => Ok(ThreadType::Convolution1),
            ffi::FMOD_THREAD_TYPE_CONVOLUTION2 => Ok(ThreadType::Convolution2),
            ffi::FMOD_THREAD_TYPE_MAX => Ok(ThreadType::Max),
            _ => Err(err_enum!("FMOD_THREAD_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FmodResult {
    Ok,
    ErrBadcommand,
    ErrChannelAlloc,
    ErrChannelStolen,
    ErrDma,
    ErrDspConnection,
    ErrDspDontprocess,
    ErrDspFormat,
    ErrDspInuse,
    ErrDspNotfound,
    ErrDspReserved,
    ErrDspSilence,
    ErrDspType,
    ErrFileBad,
    ErrFileCouldnotseek,
    ErrFileDiskejected,
    ErrFileEof,
    ErrFileEndofdata,
    ErrFileNotfound,
    ErrFormat,
    ErrHeaderMismatch,
    ErrHttp,
    ErrHttpAccess,
    ErrHttpProxyAuth,
    ErrHttpServerError,
    ErrHttpTimeout,
    ErrInitialization,
    ErrInitialized,
    ErrInternal,
    ErrInvalidFloat,
    ErrInvalidHandle,
    ErrInvalidParam,
    ErrInvalidPosition,
    ErrInvalidSpeaker,
    ErrInvalidSyncpoint,
    ErrInvalidThread,
    ErrInvalidVector,
    ErrMaxaudible,
    ErrMemory,
    ErrMemoryCantpoint,
    ErrNeeds3D,
    ErrNeedshardware,
    ErrNetConnect,
    ErrNetSocketError,
    ErrNetUrl,
    ErrNetWouldBlock,
    ErrNotready,
    ErrOutputAllocated,
    ErrOutputCreatebuffer,
    ErrOutputDrivercall,
    ErrOutputFormat,
    ErrOutputInit,
    ErrOutputNodrivers,
    ErrPlugin,
    ErrPluginMissing,
    ErrPluginResource,
    ErrPluginVersion,
    ErrRecord,
    ErrReverbChannelgroup,
    ErrReverbInstance,
    ErrSubsounds,
    ErrSubsoundAllocated,
    ErrSubsoundCantmove,
    ErrTagnotfound,
    ErrToomanychannels,
    ErrTruncated,
    ErrUnimplemented,
    ErrUninitialized,
    ErrUnsupported,
    ErrVersion,
    ErrEventAlreadyLoaded,
    ErrEventLiveupdateBusy,
    ErrEventLiveupdateMismatch,
    ErrEventLiveupdateTimeout,
    ErrEventNotfound,
    ErrStudioUninitialized,
    ErrStudioNotLoaded,
    ErrInvalidString,
    ErrAlreadyLocked,
    ErrNotLocked,
    ErrRecordDisconnected,
    ErrToomanysamples,
}

impl From<FmodResult> for ffi::FMOD_RESULT {
    fn from(value: FmodResult) -> ffi::FMOD_RESULT {
        match value {
            FmodResult::Ok => ffi::FMOD_OK,
            FmodResult::ErrBadcommand => ffi::FMOD_ERR_BADCOMMAND,
            FmodResult::ErrChannelAlloc => ffi::FMOD_ERR_CHANNEL_ALLOC,
            FmodResult::ErrChannelStolen => ffi::FMOD_ERR_CHANNEL_STOLEN,
            FmodResult::ErrDma => ffi::FMOD_ERR_DMA,
            FmodResult::ErrDspConnection => ffi::FMOD_ERR_DSP_CONNECTION,
            FmodResult::ErrDspDontprocess => ffi::FMOD_ERR_DSP_DONTPROCESS,
            FmodResult::ErrDspFormat => ffi::FMOD_ERR_DSP_FORMAT,
            FmodResult::ErrDspInuse => ffi::FMOD_ERR_DSP_INUSE,
            FmodResult::ErrDspNotfound => ffi::FMOD_ERR_DSP_NOTFOUND,
            FmodResult::ErrDspReserved => ffi::FMOD_ERR_DSP_RESERVED,
            FmodResult::ErrDspSilence => ffi::FMOD_ERR_DSP_SILENCE,
            FmodResult::ErrDspType => ffi::FMOD_ERR_DSP_TYPE,
            FmodResult::ErrFileBad => ffi::FMOD_ERR_FILE_BAD,
            FmodResult::ErrFileCouldnotseek => ffi::FMOD_ERR_FILE_COULDNOTSEEK,
            FmodResult::ErrFileDiskejected => ffi::FMOD_ERR_FILE_DISKEJECTED,
            FmodResult::ErrFileEof => ffi::FMOD_ERR_FILE_EOF,
            FmodResult::ErrFileEndofdata => ffi::FMOD_ERR_FILE_ENDOFDATA,
            FmodResult::ErrFileNotfound => ffi::FMOD_ERR_FILE_NOTFOUND,
            FmodResult::ErrFormat => ffi::FMOD_ERR_FORMAT,
            FmodResult::ErrHeaderMismatch => ffi::FMOD_ERR_HEADER_MISMATCH,
            FmodResult::ErrHttp => ffi::FMOD_ERR_HTTP,
            FmodResult::ErrHttpAccess => ffi::FMOD_ERR_HTTP_ACCESS,
            FmodResult::ErrHttpProxyAuth => ffi::FMOD_ERR_HTTP_PROXY_AUTH,
            FmodResult::ErrHttpServerError => ffi::FMOD_ERR_HTTP_SERVER_ERROR,
            FmodResult::ErrHttpTimeout => ffi::FMOD_ERR_HTTP_TIMEOUT,
            FmodResult::ErrInitialization => ffi::FMOD_ERR_INITIALIZATION,
            FmodResult::ErrInitialized => ffi::FMOD_ERR_INITIALIZED,
            FmodResult::ErrInternal => ffi::FMOD_ERR_INTERNAL,
            FmodResult::ErrInvalidFloat => ffi::FMOD_ERR_INVALID_FLOAT,
            FmodResult::ErrInvalidHandle => ffi::FMOD_ERR_INVALID_HANDLE,
            FmodResult::ErrInvalidParam => ffi::FMOD_ERR_INVALID_PARAM,
            FmodResult::ErrInvalidPosition => ffi::FMOD_ERR_INVALID_POSITION,
            FmodResult::ErrInvalidSpeaker => ffi::FMOD_ERR_INVALID_SPEAKER,
            FmodResult::ErrInvalidSyncpoint => ffi::FMOD_ERR_INVALID_SYNCPOINT,
            FmodResult::ErrInvalidThread => ffi::FMOD_ERR_INVALID_THREAD,
            FmodResult::ErrInvalidVector => ffi::FMOD_ERR_INVALID_VECTOR,
            FmodResult::ErrMaxaudible => ffi::FMOD_ERR_MAXAUDIBLE,
            FmodResult::ErrMemory => ffi::FMOD_ERR_MEMORY,
            FmodResult::ErrMemoryCantpoint => ffi::FMOD_ERR_MEMORY_CANTPOINT,
            FmodResult::ErrNeeds3D => ffi::FMOD_ERR_NEEDS3D,
            FmodResult::ErrNeedshardware => ffi::FMOD_ERR_NEEDSHARDWARE,
            FmodResult::ErrNetConnect => ffi::FMOD_ERR_NET_CONNECT,
            FmodResult::ErrNetSocketError => ffi::FMOD_ERR_NET_SOCKET_ERROR,
            FmodResult::ErrNetUrl => ffi::FMOD_ERR_NET_URL,
            FmodResult::ErrNetWouldBlock => ffi::FMOD_ERR_NET_WOULD_BLOCK,
            FmodResult::ErrNotready => ffi::FMOD_ERR_NOTREADY,
            FmodResult::ErrOutputAllocated => ffi::FMOD_ERR_OUTPUT_ALLOCATED,
            FmodResult::ErrOutputCreatebuffer => ffi::FMOD_ERR_OUTPUT_CREATEBUFFER,
            FmodResult::ErrOutputDrivercall => ffi::FMOD_ERR_OUTPUT_DRIVERCALL,
            FmodResult::ErrOutputFormat => ffi::FMOD_ERR_OUTPUT_FORMAT,
            FmodResult::ErrOutputInit => ffi::FMOD_ERR_OUTPUT_INIT,
            FmodResult::ErrOutputNodrivers => ffi::FMOD_ERR_OUTPUT_NODRIVERS,
            FmodResult::ErrPlugin => ffi::FMOD_ERR_PLUGIN,
            FmodResult::ErrPluginMissing => ffi::FMOD_ERR_PLUGIN_MISSING,
            FmodResult::ErrPluginResource => ffi::FMOD_ERR_PLUGIN_RESOURCE,
            FmodResult::ErrPluginVersion => ffi::FMOD_ERR_PLUGIN_VERSION,
            FmodResult::ErrRecord => ffi::FMOD_ERR_RECORD,
            FmodResult::ErrReverbChannelgroup => ffi::FMOD_ERR_REVERB_CHANNELGROUP,
            FmodResult::ErrReverbInstance => ffi::FMOD_ERR_REVERB_INSTANCE,
            FmodResult::ErrSubsounds => ffi::FMOD_ERR_SUBSOUNDS,
            FmodResult::ErrSubsoundAllocated => ffi::FMOD_ERR_SUBSOUND_ALLOCATED,
            FmodResult::ErrSubsoundCantmove => ffi::FMOD_ERR_SUBSOUND_CANTMOVE,
            FmodResult::ErrTagnotfound => ffi::FMOD_ERR_TAGNOTFOUND,
            FmodResult::ErrToomanychannels => ffi::FMOD_ERR_TOOMANYCHANNELS,
            FmodResult::ErrTruncated => ffi::FMOD_ERR_TRUNCATED,
            FmodResult::ErrUnimplemented => ffi::FMOD_ERR_UNIMPLEMENTED,
            FmodResult::ErrUninitialized => ffi::FMOD_ERR_UNINITIALIZED,
            FmodResult::ErrUnsupported => ffi::FMOD_ERR_UNSUPPORTED,
            FmodResult::ErrVersion => ffi::FMOD_ERR_VERSION,
            FmodResult::ErrEventAlreadyLoaded => ffi::FMOD_ERR_EVENT_ALREADY_LOADED,
            FmodResult::ErrEventLiveupdateBusy => ffi::FMOD_ERR_EVENT_LIVEUPDATE_BUSY,
            FmodResult::ErrEventLiveupdateMismatch => ffi::FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH,
            FmodResult::ErrEventLiveupdateTimeout => ffi::FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT,
            FmodResult::ErrEventNotfound => ffi::FMOD_ERR_EVENT_NOTFOUND,
            FmodResult::ErrStudioUninitialized => ffi::FMOD_ERR_STUDIO_UNINITIALIZED,
            FmodResult::ErrStudioNotLoaded => ffi::FMOD_ERR_STUDIO_NOT_LOADED,
            FmodResult::ErrInvalidString => ffi::FMOD_ERR_INVALID_STRING,
            FmodResult::ErrAlreadyLocked => ffi::FMOD_ERR_ALREADY_LOCKED,
            FmodResult::ErrNotLocked => ffi::FMOD_ERR_NOT_LOCKED,
            FmodResult::ErrRecordDisconnected => ffi::FMOD_ERR_RECORD_DISCONNECTED,
            FmodResult::ErrToomanysamples => ffi::FMOD_ERR_TOOMANYSAMPLES,
        }
    }
}

impl FmodResult {
    pub fn from(value: ffi::FMOD_RESULT) -> Result<FmodResult, Error> {
        match value {
            ffi::FMOD_OK => Ok(FmodResult::Ok),
            ffi::FMOD_ERR_BADCOMMAND => Ok(FmodResult::ErrBadcommand),
            ffi::FMOD_ERR_CHANNEL_ALLOC => Ok(FmodResult::ErrChannelAlloc),
            ffi::FMOD_ERR_CHANNEL_STOLEN => Ok(FmodResult::ErrChannelStolen),
            ffi::FMOD_ERR_DMA => Ok(FmodResult::ErrDma),
            ffi::FMOD_ERR_DSP_CONNECTION => Ok(FmodResult::ErrDspConnection),
            ffi::FMOD_ERR_DSP_DONTPROCESS => Ok(FmodResult::ErrDspDontprocess),
            ffi::FMOD_ERR_DSP_FORMAT => Ok(FmodResult::ErrDspFormat),
            ffi::FMOD_ERR_DSP_INUSE => Ok(FmodResult::ErrDspInuse),
            ffi::FMOD_ERR_DSP_NOTFOUND => Ok(FmodResult::ErrDspNotfound),
            ffi::FMOD_ERR_DSP_RESERVED => Ok(FmodResult::ErrDspReserved),
            ffi::FMOD_ERR_DSP_SILENCE => Ok(FmodResult::ErrDspSilence),
            ffi::FMOD_ERR_DSP_TYPE => Ok(FmodResult::ErrDspType),
            ffi::FMOD_ERR_FILE_BAD => Ok(FmodResult::ErrFileBad),
            ffi::FMOD_ERR_FILE_COULDNOTSEEK => Ok(FmodResult::ErrFileCouldnotseek),
            ffi::FMOD_ERR_FILE_DISKEJECTED => Ok(FmodResult::ErrFileDiskejected),
            ffi::FMOD_ERR_FILE_EOF => Ok(FmodResult::ErrFileEof),
            ffi::FMOD_ERR_FILE_ENDOFDATA => Ok(FmodResult::ErrFileEndofdata),
            ffi::FMOD_ERR_FILE_NOTFOUND => Ok(FmodResult::ErrFileNotfound),
            ffi::FMOD_ERR_FORMAT => Ok(FmodResult::ErrFormat),
            ffi::FMOD_ERR_HEADER_MISMATCH => Ok(FmodResult::ErrHeaderMismatch),
            ffi::FMOD_ERR_HTTP => Ok(FmodResult::ErrHttp),
            ffi::FMOD_ERR_HTTP_ACCESS => Ok(FmodResult::ErrHttpAccess),
            ffi::FMOD_ERR_HTTP_PROXY_AUTH => Ok(FmodResult::ErrHttpProxyAuth),
            ffi::FMOD_ERR_HTTP_SERVER_ERROR => Ok(FmodResult::ErrHttpServerError),
            ffi::FMOD_ERR_HTTP_TIMEOUT => Ok(FmodResult::ErrHttpTimeout),
            ffi::FMOD_ERR_INITIALIZATION => Ok(FmodResult::ErrInitialization),
            ffi::FMOD_ERR_INITIALIZED => Ok(FmodResult::ErrInitialized),
            ffi::FMOD_ERR_INTERNAL => Ok(FmodResult::ErrInternal),
            ffi::FMOD_ERR_INVALID_FLOAT => Ok(FmodResult::ErrInvalidFloat),
            ffi::FMOD_ERR_INVALID_HANDLE => Ok(FmodResult::ErrInvalidHandle),
            ffi::FMOD_ERR_INVALID_PARAM => Ok(FmodResult::ErrInvalidParam),
            ffi::FMOD_ERR_INVALID_POSITION => Ok(FmodResult::ErrInvalidPosition),
            ffi::FMOD_ERR_INVALID_SPEAKER => Ok(FmodResult::ErrInvalidSpeaker),
            ffi::FMOD_ERR_INVALID_SYNCPOINT => Ok(FmodResult::ErrInvalidSyncpoint),
            ffi::FMOD_ERR_INVALID_THREAD => Ok(FmodResult::ErrInvalidThread),
            ffi::FMOD_ERR_INVALID_VECTOR => Ok(FmodResult::ErrInvalidVector),
            ffi::FMOD_ERR_MAXAUDIBLE => Ok(FmodResult::ErrMaxaudible),
            ffi::FMOD_ERR_MEMORY => Ok(FmodResult::ErrMemory),
            ffi::FMOD_ERR_MEMORY_CANTPOINT => Ok(FmodResult::ErrMemoryCantpoint),
            ffi::FMOD_ERR_NEEDS3D => Ok(FmodResult::ErrNeeds3D),
            ffi::FMOD_ERR_NEEDSHARDWARE => Ok(FmodResult::ErrNeedshardware),
            ffi::FMOD_ERR_NET_CONNECT => Ok(FmodResult::ErrNetConnect),
            ffi::FMOD_ERR_NET_SOCKET_ERROR => Ok(FmodResult::ErrNetSocketError),
            ffi::FMOD_ERR_NET_URL => Ok(FmodResult::ErrNetUrl),
            ffi::FMOD_ERR_NET_WOULD_BLOCK => Ok(FmodResult::ErrNetWouldBlock),
            ffi::FMOD_ERR_NOTREADY => Ok(FmodResult::ErrNotready),
            ffi::FMOD_ERR_OUTPUT_ALLOCATED => Ok(FmodResult::ErrOutputAllocated),
            ffi::FMOD_ERR_OUTPUT_CREATEBUFFER => Ok(FmodResult::ErrOutputCreatebuffer),
            ffi::FMOD_ERR_OUTPUT_DRIVERCALL => Ok(FmodResult::ErrOutputDrivercall),
            ffi::FMOD_ERR_OUTPUT_FORMAT => Ok(FmodResult::ErrOutputFormat),
            ffi::FMOD_ERR_OUTPUT_INIT => Ok(FmodResult::ErrOutputInit),
            ffi::FMOD_ERR_OUTPUT_NODRIVERS => Ok(FmodResult::ErrOutputNodrivers),
            ffi::FMOD_ERR_PLUGIN => Ok(FmodResult::ErrPlugin),
            ffi::FMOD_ERR_PLUGIN_MISSING => Ok(FmodResult::ErrPluginMissing),
            ffi::FMOD_ERR_PLUGIN_RESOURCE => Ok(FmodResult::ErrPluginResource),
            ffi::FMOD_ERR_PLUGIN_VERSION => Ok(FmodResult::ErrPluginVersion),
            ffi::FMOD_ERR_RECORD => Ok(FmodResult::ErrRecord),
            ffi::FMOD_ERR_REVERB_CHANNELGROUP => Ok(FmodResult::ErrReverbChannelgroup),
            ffi::FMOD_ERR_REVERB_INSTANCE => Ok(FmodResult::ErrReverbInstance),
            ffi::FMOD_ERR_SUBSOUNDS => Ok(FmodResult::ErrSubsounds),
            ffi::FMOD_ERR_SUBSOUND_ALLOCATED => Ok(FmodResult::ErrSubsoundAllocated),
            ffi::FMOD_ERR_SUBSOUND_CANTMOVE => Ok(FmodResult::ErrSubsoundCantmove),
            ffi::FMOD_ERR_TAGNOTFOUND => Ok(FmodResult::ErrTagnotfound),
            ffi::FMOD_ERR_TOOMANYCHANNELS => Ok(FmodResult::ErrToomanychannels),
            ffi::FMOD_ERR_TRUNCATED => Ok(FmodResult::ErrTruncated),
            ffi::FMOD_ERR_UNIMPLEMENTED => Ok(FmodResult::ErrUnimplemented),
            ffi::FMOD_ERR_UNINITIALIZED => Ok(FmodResult::ErrUninitialized),
            ffi::FMOD_ERR_UNSUPPORTED => Ok(FmodResult::ErrUnsupported),
            ffi::FMOD_ERR_VERSION => Ok(FmodResult::ErrVersion),
            ffi::FMOD_ERR_EVENT_ALREADY_LOADED => Ok(FmodResult::ErrEventAlreadyLoaded),
            ffi::FMOD_ERR_EVENT_LIVEUPDATE_BUSY => Ok(FmodResult::ErrEventLiveupdateBusy),
            ffi::FMOD_ERR_EVENT_LIVEUPDATE_MISMATCH => Ok(FmodResult::ErrEventLiveupdateMismatch),
            ffi::FMOD_ERR_EVENT_LIVEUPDATE_TIMEOUT => Ok(FmodResult::ErrEventLiveupdateTimeout),
            ffi::FMOD_ERR_EVENT_NOTFOUND => Ok(FmodResult::ErrEventNotfound),
            ffi::FMOD_ERR_STUDIO_UNINITIALIZED => Ok(FmodResult::ErrStudioUninitialized),
            ffi::FMOD_ERR_STUDIO_NOT_LOADED => Ok(FmodResult::ErrStudioNotLoaded),
            ffi::FMOD_ERR_INVALID_STRING => Ok(FmodResult::ErrInvalidString),
            ffi::FMOD_ERR_ALREADY_LOCKED => Ok(FmodResult::ErrAlreadyLocked),
            ffi::FMOD_ERR_NOT_LOCKED => Ok(FmodResult::ErrNotLocked),
            ffi::FMOD_ERR_RECORD_DISCONNECTED => Ok(FmodResult::ErrRecordDisconnected),
            ffi::FMOD_ERR_TOOMANYSAMPLES => Ok(FmodResult::ErrToomanysamples),
            _ => Err(err_enum!("FMOD_RESULT", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelControlType {
    Channel,
    ChannelGroup,
    Max,
}

impl From<ChannelControlType> for ffi::FMOD_CHANNELCONTROL_TYPE {
    fn from(value: ChannelControlType) -> ffi::FMOD_CHANNELCONTROL_TYPE {
        match value {
            ChannelControlType::Channel => ffi::FMOD_CHANNELCONTROL_CHANNEL,
            ChannelControlType::ChannelGroup => ffi::FMOD_CHANNELCONTROL_CHANNELGROUP,
            ChannelControlType::Max => ffi::FMOD_CHANNELCONTROL_MAX,
        }
    }
}

impl ChannelControlType {
    pub fn from(value: ffi::FMOD_CHANNELCONTROL_TYPE) -> Result<ChannelControlType, Error> {
        match value {
            ffi::FMOD_CHANNELCONTROL_CHANNEL => Ok(ChannelControlType::Channel),
            ffi::FMOD_CHANNELCONTROL_CHANNELGROUP => Ok(ChannelControlType::ChannelGroup),
            ffi::FMOD_CHANNELCONTROL_MAX => Ok(ChannelControlType::Max),
            _ => Err(err_enum!("FMOD_CHANNELCONTROL_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputType {
    Autodetect,
    Unknown,
    NoSound,
    WavWriter,
    NoSoundNrt,
    WavWriterNrt,
    Wasapi,
    Asio,
    PulseAudio,
    Alsa,
    CoreAudio,
    AudioTrack,
    OpenSL,
    AudioOut,
    Audio3D,
    WebAudio,
    NnAudio,
    Winsonic,
    AAudio,
    AudioWorklet,
    Max,
}

impl From<OutputType> for ffi::FMOD_OUTPUTTYPE {
    fn from(value: OutputType) -> ffi::FMOD_OUTPUTTYPE {
        match value {
            OutputType::Autodetect => ffi::FMOD_OUTPUTTYPE_AUTODETECT,
            OutputType::Unknown => ffi::FMOD_OUTPUTTYPE_UNKNOWN,
            OutputType::NoSound => ffi::FMOD_OUTPUTTYPE_NOSOUND,
            OutputType::WavWriter => ffi::FMOD_OUTPUTTYPE_WAVWRITER,
            OutputType::NoSoundNrt => ffi::FMOD_OUTPUTTYPE_NOSOUND_NRT,
            OutputType::WavWriterNrt => ffi::FMOD_OUTPUTTYPE_WAVWRITER_NRT,
            OutputType::Wasapi => ffi::FMOD_OUTPUTTYPE_WASAPI,
            OutputType::Asio => ffi::FMOD_OUTPUTTYPE_ASIO,
            OutputType::PulseAudio => ffi::FMOD_OUTPUTTYPE_PULSEAUDIO,
            OutputType::Alsa => ffi::FMOD_OUTPUTTYPE_ALSA,
            OutputType::CoreAudio => ffi::FMOD_OUTPUTTYPE_COREAUDIO,
            OutputType::AudioTrack => ffi::FMOD_OUTPUTTYPE_AUDIOTRACK,
            OutputType::OpenSL => ffi::FMOD_OUTPUTTYPE_OPENSL,
            OutputType::AudioOut => ffi::FMOD_OUTPUTTYPE_AUDIOOUT,
            OutputType::Audio3D => ffi::FMOD_OUTPUTTYPE_AUDIO3D,
            OutputType::WebAudio => ffi::FMOD_OUTPUTTYPE_WEBAUDIO,
            OutputType::NnAudio => ffi::FMOD_OUTPUTTYPE_NNAUDIO,
            OutputType::Winsonic => ffi::FMOD_OUTPUTTYPE_WINSONIC,
            OutputType::AAudio => ffi::FMOD_OUTPUTTYPE_AAUDIO,
            OutputType::AudioWorklet => ffi::FMOD_OUTPUTTYPE_AUDIOWORKLET,
            OutputType::Max => ffi::FMOD_OUTPUTTYPE_MAX,
        }
    }
}

impl OutputType {
    pub fn from(value: ffi::FMOD_OUTPUTTYPE) -> Result<OutputType, Error> {
        match value {
            ffi::FMOD_OUTPUTTYPE_AUTODETECT => Ok(OutputType::Autodetect),
            ffi::FMOD_OUTPUTTYPE_UNKNOWN => Ok(OutputType::Unknown),
            ffi::FMOD_OUTPUTTYPE_NOSOUND => Ok(OutputType::NoSound),
            ffi::FMOD_OUTPUTTYPE_WAVWRITER => Ok(OutputType::WavWriter),
            ffi::FMOD_OUTPUTTYPE_NOSOUND_NRT => Ok(OutputType::NoSoundNrt),
            ffi::FMOD_OUTPUTTYPE_WAVWRITER_NRT => Ok(OutputType::WavWriterNrt),
            ffi::FMOD_OUTPUTTYPE_WASAPI => Ok(OutputType::Wasapi),
            ffi::FMOD_OUTPUTTYPE_ASIO => Ok(OutputType::Asio),
            ffi::FMOD_OUTPUTTYPE_PULSEAUDIO => Ok(OutputType::PulseAudio),
            ffi::FMOD_OUTPUTTYPE_ALSA => Ok(OutputType::Alsa),
            ffi::FMOD_OUTPUTTYPE_COREAUDIO => Ok(OutputType::CoreAudio),
            ffi::FMOD_OUTPUTTYPE_AUDIOTRACK => Ok(OutputType::AudioTrack),
            ffi::FMOD_OUTPUTTYPE_OPENSL => Ok(OutputType::OpenSL),
            ffi::FMOD_OUTPUTTYPE_AUDIOOUT => Ok(OutputType::AudioOut),
            ffi::FMOD_OUTPUTTYPE_AUDIO3D => Ok(OutputType::Audio3D),
            ffi::FMOD_OUTPUTTYPE_WEBAUDIO => Ok(OutputType::WebAudio),
            ffi::FMOD_OUTPUTTYPE_NNAUDIO => Ok(OutputType::NnAudio),
            ffi::FMOD_OUTPUTTYPE_WINSONIC => Ok(OutputType::Winsonic),
            ffi::FMOD_OUTPUTTYPE_AAUDIO => Ok(OutputType::AAudio),
            ffi::FMOD_OUTPUTTYPE_AUDIOWORKLET => Ok(OutputType::AudioWorklet),
            ffi::FMOD_OUTPUTTYPE_MAX => Ok(OutputType::Max),
            _ => Err(err_enum!("FMOD_OUTPUTTYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebugMode {
    Tty,
    File,
    Callback,
}

impl From<DebugMode> for ffi::FMOD_DEBUG_MODE {
    fn from(value: DebugMode) -> ffi::FMOD_DEBUG_MODE {
        match value {
            DebugMode::Tty => ffi::FMOD_DEBUG_MODE_TTY,
            DebugMode::File => ffi::FMOD_DEBUG_MODE_FILE,
            DebugMode::Callback => ffi::FMOD_DEBUG_MODE_CALLBACK,
        }
    }
}

impl DebugMode {
    pub fn from(value: ffi::FMOD_DEBUG_MODE) -> Result<DebugMode, Error> {
        match value {
            ffi::FMOD_DEBUG_MODE_TTY => Ok(DebugMode::Tty),
            ffi::FMOD_DEBUG_MODE_FILE => Ok(DebugMode::File),
            ffi::FMOD_DEBUG_MODE_CALLBACK => Ok(DebugMode::Callback),
            _ => Err(err_enum!("FMOD_DEBUG_MODE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpeakerMode {
    Default,
    Raw,
    Mono,
    Stereo,
    Quad,
    Surround,
    Mode5Point1,
    Mode7Point1,
    Mode7Point1Point4,
    Max,
}

impl From<SpeakerMode> for ffi::FMOD_SPEAKERMODE {
    fn from(value: SpeakerMode) -> ffi::FMOD_SPEAKERMODE {
        match value {
            SpeakerMode::Default => ffi::FMOD_SPEAKERMODE_DEFAULT,
            SpeakerMode::Raw => ffi::FMOD_SPEAKERMODE_RAW,
            SpeakerMode::Mono => ffi::FMOD_SPEAKERMODE_MONO,
            SpeakerMode::Stereo => ffi::FMOD_SPEAKERMODE_STEREO,
            SpeakerMode::Quad => ffi::FMOD_SPEAKERMODE_QUAD,
            SpeakerMode::Surround => ffi::FMOD_SPEAKERMODE_SURROUND,
            SpeakerMode::Mode5Point1 => ffi::FMOD_SPEAKERMODE_5POINT1,
            SpeakerMode::Mode7Point1 => ffi::FMOD_SPEAKERMODE_7POINT1,
            SpeakerMode::Mode7Point1Point4 => ffi::FMOD_SPEAKERMODE_7POINT1POINT4,
            SpeakerMode::Max => ffi::FMOD_SPEAKERMODE_MAX,
        }
    }
}

impl SpeakerMode {
    pub fn from(value: ffi::FMOD_SPEAKERMODE) -> Result<SpeakerMode, Error> {
        match value {
            ffi::FMOD_SPEAKERMODE_DEFAULT => Ok(SpeakerMode::Default),
            ffi::FMOD_SPEAKERMODE_RAW => Ok(SpeakerMode::Raw),
            ffi::FMOD_SPEAKERMODE_MONO => Ok(SpeakerMode::Mono),
            ffi::FMOD_SPEAKERMODE_STEREO => Ok(SpeakerMode::Stereo),
            ffi::FMOD_SPEAKERMODE_QUAD => Ok(SpeakerMode::Quad),
            ffi::FMOD_SPEAKERMODE_SURROUND => Ok(SpeakerMode::Surround),
            ffi::FMOD_SPEAKERMODE_5POINT1 => Ok(SpeakerMode::Mode5Point1),
            ffi::FMOD_SPEAKERMODE_7POINT1 => Ok(SpeakerMode::Mode7Point1),
            ffi::FMOD_SPEAKERMODE_7POINT1POINT4 => Ok(SpeakerMode::Mode7Point1Point4),
            ffi::FMOD_SPEAKERMODE_MAX => Ok(SpeakerMode::Max),
            _ => Err(err_enum!("FMOD_SPEAKERMODE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Speaker {
    None,
    FrontLeft,
    FrontRight,
    FrontCenter,
    LowFrequency,
    SurroundLeft,
    SurroundRight,
    BackLeft,
    BackRight,
    TopFrontLeft,
    TopFrontRight,
    TopBackLeft,
    TopBackRight,
    Max,
}

impl From<Speaker> for ffi::FMOD_SPEAKER {
    fn from(value: Speaker) -> ffi::FMOD_SPEAKER {
        match value {
            Speaker::None => ffi::FMOD_SPEAKER_NONE,
            Speaker::FrontLeft => ffi::FMOD_SPEAKER_FRONT_LEFT,
            Speaker::FrontRight => ffi::FMOD_SPEAKER_FRONT_RIGHT,
            Speaker::FrontCenter => ffi::FMOD_SPEAKER_FRONT_CENTER,
            Speaker::LowFrequency => ffi::FMOD_SPEAKER_LOW_FREQUENCY,
            Speaker::SurroundLeft => ffi::FMOD_SPEAKER_SURROUND_LEFT,
            Speaker::SurroundRight => ffi::FMOD_SPEAKER_SURROUND_RIGHT,
            Speaker::BackLeft => ffi::FMOD_SPEAKER_BACK_LEFT,
            Speaker::BackRight => ffi::FMOD_SPEAKER_BACK_RIGHT,
            Speaker::TopFrontLeft => ffi::FMOD_SPEAKER_TOP_FRONT_LEFT,
            Speaker::TopFrontRight => ffi::FMOD_SPEAKER_TOP_FRONT_RIGHT,
            Speaker::TopBackLeft => ffi::FMOD_SPEAKER_TOP_BACK_LEFT,
            Speaker::TopBackRight => ffi::FMOD_SPEAKER_TOP_BACK_RIGHT,
            Speaker::Max => ffi::FMOD_SPEAKER_MAX,
        }
    }
}

impl Speaker {
    pub fn from(value: ffi::FMOD_SPEAKER) -> Result<Speaker, Error> {
        match value {
            ffi::FMOD_SPEAKER_NONE => Ok(Speaker::None),
            ffi::FMOD_SPEAKER_FRONT_LEFT => Ok(Speaker::FrontLeft),
            ffi::FMOD_SPEAKER_FRONT_RIGHT => Ok(Speaker::FrontRight),
            ffi::FMOD_SPEAKER_FRONT_CENTER => Ok(Speaker::FrontCenter),
            ffi::FMOD_SPEAKER_LOW_FREQUENCY => Ok(Speaker::LowFrequency),
            ffi::FMOD_SPEAKER_SURROUND_LEFT => Ok(Speaker::SurroundLeft),
            ffi::FMOD_SPEAKER_SURROUND_RIGHT => Ok(Speaker::SurroundRight),
            ffi::FMOD_SPEAKER_BACK_LEFT => Ok(Speaker::BackLeft),
            ffi::FMOD_SPEAKER_BACK_RIGHT => Ok(Speaker::BackRight),
            ffi::FMOD_SPEAKER_TOP_FRONT_LEFT => Ok(Speaker::TopFrontLeft),
            ffi::FMOD_SPEAKER_TOP_FRONT_RIGHT => Ok(Speaker::TopFrontRight),
            ffi::FMOD_SPEAKER_TOP_BACK_LEFT => Ok(Speaker::TopBackLeft),
            ffi::FMOD_SPEAKER_TOP_BACK_RIGHT => Ok(Speaker::TopBackRight),
            ffi::FMOD_SPEAKER_MAX => Ok(Speaker::Max),
            _ => Err(err_enum!("FMOD_SPEAKER", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelOrder {
    Default,
    WaveFormat,
    ProTools,
    AllMono,
    AllStereo,
    Alsa,
    Max,
}

impl From<ChannelOrder> for ffi::FMOD_CHANNELORDER {
    fn from(value: ChannelOrder) -> ffi::FMOD_CHANNELORDER {
        match value {
            ChannelOrder::Default => ffi::FMOD_CHANNELORDER_DEFAULT,
            ChannelOrder::WaveFormat => ffi::FMOD_CHANNELORDER_WAVEFORMAT,
            ChannelOrder::ProTools => ffi::FMOD_CHANNELORDER_PROTOOLS,
            ChannelOrder::AllMono => ffi::FMOD_CHANNELORDER_ALLMONO,
            ChannelOrder::AllStereo => ffi::FMOD_CHANNELORDER_ALLSTEREO,
            ChannelOrder::Alsa => ffi::FMOD_CHANNELORDER_ALSA,
            ChannelOrder::Max => ffi::FMOD_CHANNELORDER_MAX,
        }
    }
}

impl ChannelOrder {
    pub fn from(value: ffi::FMOD_CHANNELORDER) -> Result<ChannelOrder, Error> {
        match value {
            ffi::FMOD_CHANNELORDER_DEFAULT => Ok(ChannelOrder::Default),
            ffi::FMOD_CHANNELORDER_WAVEFORMAT => Ok(ChannelOrder::WaveFormat),
            ffi::FMOD_CHANNELORDER_PROTOOLS => Ok(ChannelOrder::ProTools),
            ffi::FMOD_CHANNELORDER_ALLMONO => Ok(ChannelOrder::AllMono),
            ffi::FMOD_CHANNELORDER_ALLSTEREO => Ok(ChannelOrder::AllStereo),
            ffi::FMOD_CHANNELORDER_ALSA => Ok(ChannelOrder::Alsa),
            ffi::FMOD_CHANNELORDER_MAX => Ok(ChannelOrder::Max),
            _ => Err(err_enum!("FMOD_CHANNELORDER", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PluginType {
    Output,
    Codec,
    Dsp,
    Max,
}

impl From<PluginType> for ffi::FMOD_PLUGINTYPE {
    fn from(value: PluginType) -> ffi::FMOD_PLUGINTYPE {
        match value {
            PluginType::Output => ffi::FMOD_PLUGINTYPE_OUTPUT,
            PluginType::Codec => ffi::FMOD_PLUGINTYPE_CODEC,
            PluginType::Dsp => ffi::FMOD_PLUGINTYPE_DSP,
            PluginType::Max => ffi::FMOD_PLUGINTYPE_MAX,
        }
    }
}

impl PluginType {
    pub fn from(value: ffi::FMOD_PLUGINTYPE) -> Result<PluginType, Error> {
        match value {
            ffi::FMOD_PLUGINTYPE_OUTPUT => Ok(PluginType::Output),
            ffi::FMOD_PLUGINTYPE_CODEC => Ok(PluginType::Codec),
            ffi::FMOD_PLUGINTYPE_DSP => Ok(PluginType::Dsp),
            ffi::FMOD_PLUGINTYPE_MAX => Ok(PluginType::Max),
            _ => Err(err_enum!("FMOD_PLUGINTYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoundType {
    Unknown,
    Aiff,
    Asf,
    Dls,
    Flac,
    Fsb,
    It,
    Midi,
    Mod,
    Mpeg,
    OggVorbis,
    Playlist,
    Raw,
    S3M,
    User,
    Wav,
    Xm,
    Xma,
    AudioQueue,
    At9,
    Vorbis,
    MediaFoundation,
    Mediacodec,
    Fadpcm,
    Opus,
    Max,
}

impl From<SoundType> for ffi::FMOD_SOUND_TYPE {
    fn from(value: SoundType) -> ffi::FMOD_SOUND_TYPE {
        match value {
            SoundType::Unknown => ffi::FMOD_SOUND_TYPE_UNKNOWN,
            SoundType::Aiff => ffi::FMOD_SOUND_TYPE_AIFF,
            SoundType::Asf => ffi::FMOD_SOUND_TYPE_ASF,
            SoundType::Dls => ffi::FMOD_SOUND_TYPE_DLS,
            SoundType::Flac => ffi::FMOD_SOUND_TYPE_FLAC,
            SoundType::Fsb => ffi::FMOD_SOUND_TYPE_FSB,
            SoundType::It => ffi::FMOD_SOUND_TYPE_IT,
            SoundType::Midi => ffi::FMOD_SOUND_TYPE_MIDI,
            SoundType::Mod => ffi::FMOD_SOUND_TYPE_MOD,
            SoundType::Mpeg => ffi::FMOD_SOUND_TYPE_MPEG,
            SoundType::OggVorbis => ffi::FMOD_SOUND_TYPE_OGGVORBIS,
            SoundType::Playlist => ffi::FMOD_SOUND_TYPE_PLAYLIST,
            SoundType::Raw => ffi::FMOD_SOUND_TYPE_RAW,
            SoundType::S3M => ffi::FMOD_SOUND_TYPE_S3M,
            SoundType::User => ffi::FMOD_SOUND_TYPE_USER,
            SoundType::Wav => ffi::FMOD_SOUND_TYPE_WAV,
            SoundType::Xm => ffi::FMOD_SOUND_TYPE_XM,
            SoundType::Xma => ffi::FMOD_SOUND_TYPE_XMA,
            SoundType::AudioQueue => ffi::FMOD_SOUND_TYPE_AUDIOQUEUE,
            SoundType::At9 => ffi::FMOD_SOUND_TYPE_AT9,
            SoundType::Vorbis => ffi::FMOD_SOUND_TYPE_VORBIS,
            SoundType::MediaFoundation => ffi::FMOD_SOUND_TYPE_MEDIA_FOUNDATION,
            SoundType::Mediacodec => ffi::FMOD_SOUND_TYPE_MEDIACODEC,
            SoundType::Fadpcm => ffi::FMOD_SOUND_TYPE_FADPCM,
            SoundType::Opus => ffi::FMOD_SOUND_TYPE_OPUS,
            SoundType::Max => ffi::FMOD_SOUND_TYPE_MAX,
        }
    }
}

impl SoundType {
    pub fn from(value: ffi::FMOD_SOUND_TYPE) -> Result<SoundType, Error> {
        match value {
            ffi::FMOD_SOUND_TYPE_UNKNOWN => Ok(SoundType::Unknown),
            ffi::FMOD_SOUND_TYPE_AIFF => Ok(SoundType::Aiff),
            ffi::FMOD_SOUND_TYPE_ASF => Ok(SoundType::Asf),
            ffi::FMOD_SOUND_TYPE_DLS => Ok(SoundType::Dls),
            ffi::FMOD_SOUND_TYPE_FLAC => Ok(SoundType::Flac),
            ffi::FMOD_SOUND_TYPE_FSB => Ok(SoundType::Fsb),
            ffi::FMOD_SOUND_TYPE_IT => Ok(SoundType::It),
            ffi::FMOD_SOUND_TYPE_MIDI => Ok(SoundType::Midi),
            ffi::FMOD_SOUND_TYPE_MOD => Ok(SoundType::Mod),
            ffi::FMOD_SOUND_TYPE_MPEG => Ok(SoundType::Mpeg),
            ffi::FMOD_SOUND_TYPE_OGGVORBIS => Ok(SoundType::OggVorbis),
            ffi::FMOD_SOUND_TYPE_PLAYLIST => Ok(SoundType::Playlist),
            ffi::FMOD_SOUND_TYPE_RAW => Ok(SoundType::Raw),
            ffi::FMOD_SOUND_TYPE_S3M => Ok(SoundType::S3M),
            ffi::FMOD_SOUND_TYPE_USER => Ok(SoundType::User),
            ffi::FMOD_SOUND_TYPE_WAV => Ok(SoundType::Wav),
            ffi::FMOD_SOUND_TYPE_XM => Ok(SoundType::Xm),
            ffi::FMOD_SOUND_TYPE_XMA => Ok(SoundType::Xma),
            ffi::FMOD_SOUND_TYPE_AUDIOQUEUE => Ok(SoundType::AudioQueue),
            ffi::FMOD_SOUND_TYPE_AT9 => Ok(SoundType::At9),
            ffi::FMOD_SOUND_TYPE_VORBIS => Ok(SoundType::Vorbis),
            ffi::FMOD_SOUND_TYPE_MEDIA_FOUNDATION => Ok(SoundType::MediaFoundation),
            ffi::FMOD_SOUND_TYPE_MEDIACODEC => Ok(SoundType::Mediacodec),
            ffi::FMOD_SOUND_TYPE_FADPCM => Ok(SoundType::Fadpcm),
            ffi::FMOD_SOUND_TYPE_OPUS => Ok(SoundType::Opus),
            ffi::FMOD_SOUND_TYPE_MAX => Ok(SoundType::Max),
            _ => Err(err_enum!("FMOD_SOUND_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoundFormat {
    None,
    Pcm8,
    Pcm16,
    Pcm24,
    Pcm32,
    PcmFloat,
    Bitstream,
    Max,
}

impl From<SoundFormat> for ffi::FMOD_SOUND_FORMAT {
    fn from(value: SoundFormat) -> ffi::FMOD_SOUND_FORMAT {
        match value {
            SoundFormat::None => ffi::FMOD_SOUND_FORMAT_NONE,
            SoundFormat::Pcm8 => ffi::FMOD_SOUND_FORMAT_PCM8,
            SoundFormat::Pcm16 => ffi::FMOD_SOUND_FORMAT_PCM16,
            SoundFormat::Pcm24 => ffi::FMOD_SOUND_FORMAT_PCM24,
            SoundFormat::Pcm32 => ffi::FMOD_SOUND_FORMAT_PCM32,
            SoundFormat::PcmFloat => ffi::FMOD_SOUND_FORMAT_PCMFLOAT,
            SoundFormat::Bitstream => ffi::FMOD_SOUND_FORMAT_BITSTREAM,
            SoundFormat::Max => ffi::FMOD_SOUND_FORMAT_MAX,
        }
    }
}

impl SoundFormat {
    pub fn from(value: ffi::FMOD_SOUND_FORMAT) -> Result<SoundFormat, Error> {
        match value {
            ffi::FMOD_SOUND_FORMAT_NONE => Ok(SoundFormat::None),
            ffi::FMOD_SOUND_FORMAT_PCM8 => Ok(SoundFormat::Pcm8),
            ffi::FMOD_SOUND_FORMAT_PCM16 => Ok(SoundFormat::Pcm16),
            ffi::FMOD_SOUND_FORMAT_PCM24 => Ok(SoundFormat::Pcm24),
            ffi::FMOD_SOUND_FORMAT_PCM32 => Ok(SoundFormat::Pcm32),
            ffi::FMOD_SOUND_FORMAT_PCMFLOAT => Ok(SoundFormat::PcmFloat),
            ffi::FMOD_SOUND_FORMAT_BITSTREAM => Ok(SoundFormat::Bitstream),
            ffi::FMOD_SOUND_FORMAT_MAX => Ok(SoundFormat::Max),
            _ => Err(err_enum!("FMOD_SOUND_FORMAT", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpenState {
    Ready,
    Loading,
    Error,
    Connecting,
    Buffering,
    Seeking,
    Playing,
    SetPosition,
    Max,
}

impl From<OpenState> for ffi::FMOD_OPENSTATE {
    fn from(value: OpenState) -> ffi::FMOD_OPENSTATE {
        match value {
            OpenState::Ready => ffi::FMOD_OPENSTATE_READY,
            OpenState::Loading => ffi::FMOD_OPENSTATE_LOADING,
            OpenState::Error => ffi::FMOD_OPENSTATE_ERROR,
            OpenState::Connecting => ffi::FMOD_OPENSTATE_CONNECTING,
            OpenState::Buffering => ffi::FMOD_OPENSTATE_BUFFERING,
            OpenState::Seeking => ffi::FMOD_OPENSTATE_SEEKING,
            OpenState::Playing => ffi::FMOD_OPENSTATE_PLAYING,
            OpenState::SetPosition => ffi::FMOD_OPENSTATE_SETPOSITION,
            OpenState::Max => ffi::FMOD_OPENSTATE_MAX,
        }
    }
}

impl OpenState {
    pub fn from(value: ffi::FMOD_OPENSTATE) -> Result<OpenState, Error> {
        match value {
            ffi::FMOD_OPENSTATE_READY => Ok(OpenState::Ready),
            ffi::FMOD_OPENSTATE_LOADING => Ok(OpenState::Loading),
            ffi::FMOD_OPENSTATE_ERROR => Ok(OpenState::Error),
            ffi::FMOD_OPENSTATE_CONNECTING => Ok(OpenState::Connecting),
            ffi::FMOD_OPENSTATE_BUFFERING => Ok(OpenState::Buffering),
            ffi::FMOD_OPENSTATE_SEEKING => Ok(OpenState::Seeking),
            ffi::FMOD_OPENSTATE_PLAYING => Ok(OpenState::Playing),
            ffi::FMOD_OPENSTATE_SETPOSITION => Ok(OpenState::SetPosition),
            ffi::FMOD_OPENSTATE_MAX => Ok(OpenState::Max),
            _ => Err(err_enum!("FMOD_OPENSTATE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoundGroupBehavior {
    Fail,
    Mute,
    StealLowest,
    Max,
}

impl From<SoundGroupBehavior> for ffi::FMOD_SOUNDGROUP_BEHAVIOR {
    fn from(value: SoundGroupBehavior) -> ffi::FMOD_SOUNDGROUP_BEHAVIOR {
        match value {
            SoundGroupBehavior::Fail => ffi::FMOD_SOUNDGROUP_BEHAVIOR_FAIL,
            SoundGroupBehavior::Mute => ffi::FMOD_SOUNDGROUP_BEHAVIOR_MUTE,
            SoundGroupBehavior::StealLowest => ffi::FMOD_SOUNDGROUP_BEHAVIOR_STEALLOWEST,
            SoundGroupBehavior::Max => ffi::FMOD_SOUNDGROUP_BEHAVIOR_MAX,
        }
    }
}

impl SoundGroupBehavior {
    pub fn from(value: ffi::FMOD_SOUNDGROUP_BEHAVIOR) -> Result<SoundGroupBehavior, Error> {
        match value {
            ffi::FMOD_SOUNDGROUP_BEHAVIOR_FAIL => Ok(SoundGroupBehavior::Fail),
            ffi::FMOD_SOUNDGROUP_BEHAVIOR_MUTE => Ok(SoundGroupBehavior::Mute),
            ffi::FMOD_SOUNDGROUP_BEHAVIOR_STEALLOWEST => Ok(SoundGroupBehavior::StealLowest),
            ffi::FMOD_SOUNDGROUP_BEHAVIOR_MAX => Ok(SoundGroupBehavior::Max),
            _ => Err(err_enum!("FMOD_SOUNDGROUP_BEHAVIOR", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelControlCallbackType {
    End,
    VirtualVoice,
    SyncPoint,
    Occlusion,
    Max,
}

impl From<ChannelControlCallbackType> for ffi::FMOD_CHANNELCONTROL_CALLBACK_TYPE {
    fn from(value: ChannelControlCallbackType) -> ffi::FMOD_CHANNELCONTROL_CALLBACK_TYPE {
        match value {
            ChannelControlCallbackType::End => ffi::FMOD_CHANNELCONTROL_CALLBACK_END,
            ChannelControlCallbackType::VirtualVoice => {
                ffi::FMOD_CHANNELCONTROL_CALLBACK_VIRTUALVOICE
            }
            ChannelControlCallbackType::SyncPoint => ffi::FMOD_CHANNELCONTROL_CALLBACK_SYNCPOINT,
            ChannelControlCallbackType::Occlusion => ffi::FMOD_CHANNELCONTROL_CALLBACK_OCCLUSION,
            ChannelControlCallbackType::Max => ffi::FMOD_CHANNELCONTROL_CALLBACK_MAX,
        }
    }
}

impl ChannelControlCallbackType {
    pub fn from(
        value: ffi::FMOD_CHANNELCONTROL_CALLBACK_TYPE,
    ) -> Result<ChannelControlCallbackType, Error> {
        match value {
            ffi::FMOD_CHANNELCONTROL_CALLBACK_END => Ok(ChannelControlCallbackType::End),
            ffi::FMOD_CHANNELCONTROL_CALLBACK_VIRTUALVOICE => {
                Ok(ChannelControlCallbackType::VirtualVoice)
            }
            ffi::FMOD_CHANNELCONTROL_CALLBACK_SYNCPOINT => {
                Ok(ChannelControlCallbackType::SyncPoint)
            }
            ffi::FMOD_CHANNELCONTROL_CALLBACK_OCCLUSION => {
                Ok(ChannelControlCallbackType::Occlusion)
            }
            ffi::FMOD_CHANNELCONTROL_CALLBACK_MAX => Ok(ChannelControlCallbackType::Max),
            _ => Err(err_enum!("FMOD_CHANNELCONTROL_CALLBACK_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelControlDspIndex {
    Head,
    Fader,
    Tail,
}

impl From<ChannelControlDspIndex> for ffi::FMOD_CHANNELCONTROL_DSP_INDEX {
    fn from(value: ChannelControlDspIndex) -> ffi::FMOD_CHANNELCONTROL_DSP_INDEX {
        match value {
            ChannelControlDspIndex::Head => ffi::FMOD_CHANNELCONTROL_DSP_HEAD,
            ChannelControlDspIndex::Fader => ffi::FMOD_CHANNELCONTROL_DSP_FADER,
            ChannelControlDspIndex::Tail => ffi::FMOD_CHANNELCONTROL_DSP_TAIL,
        }
    }
}

impl ChannelControlDspIndex {
    pub fn from(
        value: ffi::FMOD_CHANNELCONTROL_DSP_INDEX,
    ) -> Result<ChannelControlDspIndex, Error> {
        match value {
            ffi::FMOD_CHANNELCONTROL_DSP_HEAD => Ok(ChannelControlDspIndex::Head),
            ffi::FMOD_CHANNELCONTROL_DSP_FADER => Ok(ChannelControlDspIndex::Fader),
            ffi::FMOD_CHANNELCONTROL_DSP_TAIL => Ok(ChannelControlDspIndex::Tail),
            _ => Err(err_enum!("FMOD_CHANNELCONTROL_DSP_INDEX", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorCallbackInstancetype {
    None,
    System,
    Channel,
    ChannelGroup,
    ChannelControl,
    Sound,
    SoundGroup,
    Dsp,
    DspConnection,
    Geometry,
    Reverb3d,
    Studio,
    EventDescription,
    EventInstance,
    ParameterInstance,
    Bus,
    Vca,
    Bank,
    CommandReplay,
}

impl From<ErrorCallbackInstancetype> for ffi::FMOD_ERRORCALLBACK_INSTANCETYPE {
    fn from(value: ErrorCallbackInstancetype) -> ffi::FMOD_ERRORCALLBACK_INSTANCETYPE {
        match value {
            ErrorCallbackInstancetype::None => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_NONE,
            ErrorCallbackInstancetype::System => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_SYSTEM,
            ErrorCallbackInstancetype::Channel => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNEL,
            ErrorCallbackInstancetype::ChannelGroup => {
                ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNELGROUP
            }
            ErrorCallbackInstancetype::ChannelControl => {
                ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNELCONTROL
            }
            ErrorCallbackInstancetype::Sound => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_SOUND,
            ErrorCallbackInstancetype::SoundGroup => {
                ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_SOUNDGROUP
            }
            ErrorCallbackInstancetype::Dsp => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_DSP,
            ErrorCallbackInstancetype::DspConnection => {
                ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_DSPCONNECTION
            }
            ErrorCallbackInstancetype::Geometry => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_GEOMETRY,
            ErrorCallbackInstancetype::Reverb3d => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_REVERB3D,
            ErrorCallbackInstancetype::Studio => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_SYSTEM,
            ErrorCallbackInstancetype::EventDescription => {
                ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_EVENTDESCRIPTION
            }
            ErrorCallbackInstancetype::EventInstance => {
                ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_EVENTINSTANCE
            }
            ErrorCallbackInstancetype::ParameterInstance => {
                ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_PARAMETERINSTANCE
            }
            ErrorCallbackInstancetype::Bus => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_BUS,
            ErrorCallbackInstancetype::Vca => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_VCA,
            ErrorCallbackInstancetype::Bank => ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_BANK,
            ErrorCallbackInstancetype::CommandReplay => {
                ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_COMMANDREPLAY
            }
        }
    }
}

impl ErrorCallbackInstancetype {
    pub fn from(
        value: ffi::FMOD_ERRORCALLBACK_INSTANCETYPE,
    ) -> Result<ErrorCallbackInstancetype, Error> {
        match value {
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_NONE => Ok(ErrorCallbackInstancetype::None),
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_SYSTEM => Ok(ErrorCallbackInstancetype::System),
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNEL => Ok(ErrorCallbackInstancetype::Channel),
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNELGROUP => {
                Ok(ErrorCallbackInstancetype::ChannelGroup)
            }
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNELCONTROL => {
                Ok(ErrorCallbackInstancetype::ChannelControl)
            }
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_SOUND => Ok(ErrorCallbackInstancetype::Sound),
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_SOUNDGROUP => {
                Ok(ErrorCallbackInstancetype::SoundGroup)
            }
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_DSP => Ok(ErrorCallbackInstancetype::Dsp),
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_DSPCONNECTION => {
                Ok(ErrorCallbackInstancetype::DspConnection)
            }
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_GEOMETRY => {
                Ok(ErrorCallbackInstancetype::Geometry)
            }
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_REVERB3D => {
                Ok(ErrorCallbackInstancetype::Reverb3d)
            }
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_SYSTEM => {
                Ok(ErrorCallbackInstancetype::Studio)
            }
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_EVENTDESCRIPTION => {
                Ok(ErrorCallbackInstancetype::EventDescription)
            }
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_EVENTINSTANCE => {
                Ok(ErrorCallbackInstancetype::EventInstance)
            }
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_PARAMETERINSTANCE => {
                Ok(ErrorCallbackInstancetype::ParameterInstance)
            }
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_BUS => Ok(ErrorCallbackInstancetype::Bus),
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_VCA => Ok(ErrorCallbackInstancetype::Vca),
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_BANK => Ok(ErrorCallbackInstancetype::Bank),
            ffi::FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_COMMANDREPLAY => {
                Ok(ErrorCallbackInstancetype::CommandReplay)
            }
            _ => Err(err_enum!("FMOD_ERRORCALLBACK_INSTANCETYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspResampler {
    Default,
    NoInterp,
    Linear,
    Cubic,
    Spline,
    Max,
}

impl From<DspResampler> for ffi::FMOD_DSP_RESAMPLER {
    fn from(value: DspResampler) -> ffi::FMOD_DSP_RESAMPLER {
        match value {
            DspResampler::Default => ffi::FMOD_DSP_RESAMPLER_DEFAULT,
            DspResampler::NoInterp => ffi::FMOD_DSP_RESAMPLER_NOINTERP,
            DspResampler::Linear => ffi::FMOD_DSP_RESAMPLER_LINEAR,
            DspResampler::Cubic => ffi::FMOD_DSP_RESAMPLER_CUBIC,
            DspResampler::Spline => ffi::FMOD_DSP_RESAMPLER_SPLINE,
            DspResampler::Max => ffi::FMOD_DSP_RESAMPLER_MAX,
        }
    }
}

impl DspResampler {
    pub fn from(value: ffi::FMOD_DSP_RESAMPLER) -> Result<DspResampler, Error> {
        match value {
            ffi::FMOD_DSP_RESAMPLER_DEFAULT => Ok(DspResampler::Default),
            ffi::FMOD_DSP_RESAMPLER_NOINTERP => Ok(DspResampler::NoInterp),
            ffi::FMOD_DSP_RESAMPLER_LINEAR => Ok(DspResampler::Linear),
            ffi::FMOD_DSP_RESAMPLER_CUBIC => Ok(DspResampler::Cubic),
            ffi::FMOD_DSP_RESAMPLER_SPLINE => Ok(DspResampler::Spline),
            ffi::FMOD_DSP_RESAMPLER_MAX => Ok(DspResampler::Max),
            _ => Err(err_enum!("FMOD_DSP_RESAMPLER", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspConnectionType {
    Standard,
    Sidechain,
    Send,
    SendSidechain,
    Max,
}

impl From<DspConnectionType> for ffi::FMOD_DSPCONNECTION_TYPE {
    fn from(value: DspConnectionType) -> ffi::FMOD_DSPCONNECTION_TYPE {
        match value {
            DspConnectionType::Standard => ffi::FMOD_DSPCONNECTION_TYPE_STANDARD,
            DspConnectionType::Sidechain => ffi::FMOD_DSPCONNECTION_TYPE_SIDECHAIN,
            DspConnectionType::Send => ffi::FMOD_DSPCONNECTION_TYPE_SEND,
            DspConnectionType::SendSidechain => ffi::FMOD_DSPCONNECTION_TYPE_SEND_SIDECHAIN,
            DspConnectionType::Max => ffi::FMOD_DSPCONNECTION_TYPE_MAX,
        }
    }
}

impl DspConnectionType {
    pub fn from(value: ffi::FMOD_DSPCONNECTION_TYPE) -> Result<DspConnectionType, Error> {
        match value {
            ffi::FMOD_DSPCONNECTION_TYPE_STANDARD => Ok(DspConnectionType::Standard),
            ffi::FMOD_DSPCONNECTION_TYPE_SIDECHAIN => Ok(DspConnectionType::Sidechain),
            ffi::FMOD_DSPCONNECTION_TYPE_SEND => Ok(DspConnectionType::Send),
            ffi::FMOD_DSPCONNECTION_TYPE_SEND_SIDECHAIN => Ok(DspConnectionType::SendSidechain),
            ffi::FMOD_DSPCONNECTION_TYPE_MAX => Ok(DspConnectionType::Max),
            _ => Err(err_enum!("FMOD_DSPCONNECTION_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TagType {
    Unknown,
    Id3V1,
    Id3V2,
    VorbisComment,
    Shoutcast,
    Icecast,
    Asf,
    Midi,
    Playlist,
    Fmod,
    User,
    Max,
}

impl From<TagType> for ffi::FMOD_TAGTYPE {
    fn from(value: TagType) -> ffi::FMOD_TAGTYPE {
        match value {
            TagType::Unknown => ffi::FMOD_TAGTYPE_UNKNOWN,
            TagType::Id3V1 => ffi::FMOD_TAGTYPE_ID3V1,
            TagType::Id3V2 => ffi::FMOD_TAGTYPE_ID3V2,
            TagType::VorbisComment => ffi::FMOD_TAGTYPE_VORBISCOMMENT,
            TagType::Shoutcast => ffi::FMOD_TAGTYPE_SHOUTCAST,
            TagType::Icecast => ffi::FMOD_TAGTYPE_ICECAST,
            TagType::Asf => ffi::FMOD_TAGTYPE_ASF,
            TagType::Midi => ffi::FMOD_TAGTYPE_MIDI,
            TagType::Playlist => ffi::FMOD_TAGTYPE_PLAYLIST,
            TagType::Fmod => ffi::FMOD_TAGTYPE_FMOD,
            TagType::User => ffi::FMOD_TAGTYPE_USER,
            TagType::Max => ffi::FMOD_TAGTYPE_MAX,
        }
    }
}

impl TagType {
    pub fn from(value: ffi::FMOD_TAGTYPE) -> Result<TagType, Error> {
        match value {
            ffi::FMOD_TAGTYPE_UNKNOWN => Ok(TagType::Unknown),
            ffi::FMOD_TAGTYPE_ID3V1 => Ok(TagType::Id3V1),
            ffi::FMOD_TAGTYPE_ID3V2 => Ok(TagType::Id3V2),
            ffi::FMOD_TAGTYPE_VORBISCOMMENT => Ok(TagType::VorbisComment),
            ffi::FMOD_TAGTYPE_SHOUTCAST => Ok(TagType::Shoutcast),
            ffi::FMOD_TAGTYPE_ICECAST => Ok(TagType::Icecast),
            ffi::FMOD_TAGTYPE_ASF => Ok(TagType::Asf),
            ffi::FMOD_TAGTYPE_MIDI => Ok(TagType::Midi),
            ffi::FMOD_TAGTYPE_PLAYLIST => Ok(TagType::Playlist),
            ffi::FMOD_TAGTYPE_FMOD => Ok(TagType::Fmod),
            ffi::FMOD_TAGTYPE_USER => Ok(TagType::User),
            ffi::FMOD_TAGTYPE_MAX => Ok(TagType::Max),
            _ => Err(err_enum!("FMOD_TAGTYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TagDataType {
    Binary,
    Int,
    Float,
    String,
    StringUtf16,
    StringUtf16Be,
    StringUtf8,
    Max,
}

impl From<TagDataType> for ffi::FMOD_TAGDATATYPE {
    fn from(value: TagDataType) -> ffi::FMOD_TAGDATATYPE {
        match value {
            TagDataType::Binary => ffi::FMOD_TAGDATATYPE_BINARY,
            TagDataType::Int => ffi::FMOD_TAGDATATYPE_INT,
            TagDataType::Float => ffi::FMOD_TAGDATATYPE_FLOAT,
            TagDataType::String => ffi::FMOD_TAGDATATYPE_STRING,
            TagDataType::StringUtf16 => ffi::FMOD_TAGDATATYPE_STRING_UTF16,
            TagDataType::StringUtf16Be => ffi::FMOD_TAGDATATYPE_STRING_UTF16BE,
            TagDataType::StringUtf8 => ffi::FMOD_TAGDATATYPE_STRING_UTF8,
            TagDataType::Max => ffi::FMOD_TAGDATATYPE_MAX,
        }
    }
}

impl TagDataType {
    pub fn from(value: ffi::FMOD_TAGDATATYPE) -> Result<TagDataType, Error> {
        match value {
            ffi::FMOD_TAGDATATYPE_BINARY => Ok(TagDataType::Binary),
            ffi::FMOD_TAGDATATYPE_INT => Ok(TagDataType::Int),
            ffi::FMOD_TAGDATATYPE_FLOAT => Ok(TagDataType::Float),
            ffi::FMOD_TAGDATATYPE_STRING => Ok(TagDataType::String),
            ffi::FMOD_TAGDATATYPE_STRING_UTF16 => Ok(TagDataType::StringUtf16),
            ffi::FMOD_TAGDATATYPE_STRING_UTF16BE => Ok(TagDataType::StringUtf16Be),
            ffi::FMOD_TAGDATATYPE_STRING_UTF8 => Ok(TagDataType::StringUtf8),
            ffi::FMOD_TAGDATATYPE_MAX => Ok(TagDataType::Max),
            _ => Err(err_enum!("FMOD_TAGDATATYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortType {
    Music,
    CopyrightMusic,
    Voice,
    Controller,
    Personal,
    Vibration,
    Aux,
    Max,
}

impl From<PortType> for ffi::FMOD_PORT_TYPE {
    fn from(value: PortType) -> ffi::FMOD_PORT_TYPE {
        match value {
            PortType::Music => ffi::FMOD_PORT_TYPE_MUSIC,
            PortType::CopyrightMusic => ffi::FMOD_PORT_TYPE_COPYRIGHT_MUSIC,
            PortType::Voice => ffi::FMOD_PORT_TYPE_VOICE,
            PortType::Controller => ffi::FMOD_PORT_TYPE_CONTROLLER,
            PortType::Personal => ffi::FMOD_PORT_TYPE_PERSONAL,
            PortType::Vibration => ffi::FMOD_PORT_TYPE_VIBRATION,
            PortType::Aux => ffi::FMOD_PORT_TYPE_AUX,
            PortType::Max => ffi::FMOD_PORT_TYPE_MAX,
        }
    }
}

impl PortType {
    pub fn from(value: ffi::FMOD_PORT_TYPE) -> Result<PortType, Error> {
        match value {
            ffi::FMOD_PORT_TYPE_MUSIC => Ok(PortType::Music),
            ffi::FMOD_PORT_TYPE_COPYRIGHT_MUSIC => Ok(PortType::CopyrightMusic),
            ffi::FMOD_PORT_TYPE_VOICE => Ok(PortType::Voice),
            ffi::FMOD_PORT_TYPE_CONTROLLER => Ok(PortType::Controller),
            ffi::FMOD_PORT_TYPE_PERSONAL => Ok(PortType::Personal),
            ffi::FMOD_PORT_TYPE_VIBRATION => Ok(PortType::Vibration),
            ffi::FMOD_PORT_TYPE_AUX => Ok(PortType::Aux),
            ffi::FMOD_PORT_TYPE_MAX => Ok(PortType::Max),
            _ => Err(err_enum!("FMOD_PORT_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspProcessOperation {
    Perform,
    Query,
}

impl From<DspProcessOperation> for ffi::FMOD_DSP_PROCESS_OPERATION {
    fn from(value: DspProcessOperation) -> ffi::FMOD_DSP_PROCESS_OPERATION {
        match value {
            DspProcessOperation::Perform => ffi::FMOD_DSP_PROCESS_PERFORM,
            DspProcessOperation::Query => ffi::FMOD_DSP_PROCESS_QUERY,
        }
    }
}

impl DspProcessOperation {
    pub fn from(value: ffi::FMOD_DSP_PROCESS_OPERATION) -> Result<DspProcessOperation, Error> {
        match value {
            ffi::FMOD_DSP_PROCESS_PERFORM => Ok(DspProcessOperation::Perform),
            ffi::FMOD_DSP_PROCESS_QUERY => Ok(DspProcessOperation::Query),
            _ => Err(err_enum!("FMOD_DSP_PROCESS_OPERATION", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspPanSurroundFlags {
    Default,
    RotationNotBiased,
}

impl From<DspPanSurroundFlags> for ffi::FMOD_DSP_PAN_SURROUND_FLAGS {
    fn from(value: DspPanSurroundFlags) -> ffi::FMOD_DSP_PAN_SURROUND_FLAGS {
        match value {
            DspPanSurroundFlags::Default => ffi::FMOD_DSP_PAN_SURROUND_DEFAULT,
            DspPanSurroundFlags::RotationNotBiased => {
                ffi::FMOD_DSP_PAN_SURROUND_ROTATION_NOT_BIASED
            }
        }
    }
}

impl DspPanSurroundFlags {
    pub fn from(value: ffi::FMOD_DSP_PAN_SURROUND_FLAGS) -> Result<DspPanSurroundFlags, Error> {
        match value {
            ffi::FMOD_DSP_PAN_SURROUND_DEFAULT => Ok(DspPanSurroundFlags::Default),
            ffi::FMOD_DSP_PAN_SURROUND_ROTATION_NOT_BIASED => {
                Ok(DspPanSurroundFlags::RotationNotBiased)
            }
            _ => Err(err_enum!("FMOD_DSP_PAN_SURROUND_FLAGS", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspParameterType {
    Float,
    Int,
    Bool,
    Data,
    Max,
}

impl From<DspParameterType> for ffi::FMOD_DSP_PARAMETER_TYPE {
    fn from(value: DspParameterType) -> ffi::FMOD_DSP_PARAMETER_TYPE {
        match value {
            DspParameterType::Float => ffi::FMOD_DSP_PARAMETER_TYPE_FLOAT,
            DspParameterType::Int => ffi::FMOD_DSP_PARAMETER_TYPE_INT,
            DspParameterType::Bool => ffi::FMOD_DSP_PARAMETER_TYPE_BOOL,
            DspParameterType::Data => ffi::FMOD_DSP_PARAMETER_TYPE_DATA,
            DspParameterType::Max => ffi::FMOD_DSP_PARAMETER_TYPE_MAX,
        }
    }
}

impl DspParameterType {
    pub fn from(value: ffi::FMOD_DSP_PARAMETER_TYPE) -> Result<DspParameterType, Error> {
        match value {
            ffi::FMOD_DSP_PARAMETER_TYPE_FLOAT => Ok(DspParameterType::Float),
            ffi::FMOD_DSP_PARAMETER_TYPE_INT => Ok(DspParameterType::Int),
            ffi::FMOD_DSP_PARAMETER_TYPE_BOOL => Ok(DspParameterType::Bool),
            ffi::FMOD_DSP_PARAMETER_TYPE_DATA => Ok(DspParameterType::Data),
            ffi::FMOD_DSP_PARAMETER_TYPE_MAX => Ok(DspParameterType::Max),
            _ => Err(err_enum!("FMOD_DSP_PARAMETER_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspParameterFloatMappingType {
    Linear,
    Auto,
    PiecewiseLinear,
}

impl From<DspParameterFloatMappingType> for ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE {
    fn from(value: DspParameterFloatMappingType) -> ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE {
        match value {
            DspParameterFloatMappingType::Linear => {
                ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE_LINEAR
            }
            DspParameterFloatMappingType::Auto => ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE_AUTO,
            DspParameterFloatMappingType::PiecewiseLinear => {
                ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE_PIECEWISE_LINEAR
            }
        }
    }
}

impl DspParameterFloatMappingType {
    pub fn from(
        value: ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE,
    ) -> Result<DspParameterFloatMappingType, Error> {
        match value {
            ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE_LINEAR => {
                Ok(DspParameterFloatMappingType::Linear)
            }
            ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE_AUTO => {
                Ok(DspParameterFloatMappingType::Auto)
            }
            ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE_PIECEWISE_LINEAR => {
                Ok(DspParameterFloatMappingType::PiecewiseLinear)
            }
            _ => Err(err_enum!("FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspParameterDataType {
    User,
    OverallGain,
    Attributes3D,
    Sidechain,
    Fft,
    AttributesMulti3D,
    AttenuationRange,
}

impl From<DspParameterDataType> for ffi::FMOD_DSP_PARAMETER_DATA_TYPE {
    fn from(value: DspParameterDataType) -> ffi::FMOD_DSP_PARAMETER_DATA_TYPE {
        match value {
            DspParameterDataType::User => ffi::FMOD_DSP_PARAMETER_DATA_TYPE_USER,
            DspParameterDataType::OverallGain => ffi::FMOD_DSP_PARAMETER_DATA_TYPE_OVERALLGAIN,
            DspParameterDataType::Attributes3D => ffi::FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES,
            DspParameterDataType::Sidechain => ffi::FMOD_DSP_PARAMETER_DATA_TYPE_SIDECHAIN,
            DspParameterDataType::Fft => ffi::FMOD_DSP_PARAMETER_DATA_TYPE_FFT,
            DspParameterDataType::AttributesMulti3D => {
                ffi::FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES_MULTI
            }
            DspParameterDataType::AttenuationRange => {
                ffi::FMOD_DSP_PARAMETER_DATA_TYPE_ATTENUATION_RANGE
            }
        }
    }
}

impl DspParameterDataType {
    pub fn from(value: ffi::FMOD_DSP_PARAMETER_DATA_TYPE) -> Result<DspParameterDataType, Error> {
        match value {
            ffi::FMOD_DSP_PARAMETER_DATA_TYPE_USER => Ok(DspParameterDataType::User),
            ffi::FMOD_DSP_PARAMETER_DATA_TYPE_OVERALLGAIN => Ok(DspParameterDataType::OverallGain),
            ffi::FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES => {
                Ok(DspParameterDataType::Attributes3D)
            }
            ffi::FMOD_DSP_PARAMETER_DATA_TYPE_SIDECHAIN => Ok(DspParameterDataType::Sidechain),
            ffi::FMOD_DSP_PARAMETER_DATA_TYPE_FFT => Ok(DspParameterDataType::Fft),
            ffi::FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES_MULTI => {
                Ok(DspParameterDataType::AttributesMulti3D)
            }
            ffi::FMOD_DSP_PARAMETER_DATA_TYPE_ATTENUATION_RANGE => {
                Ok(DspParameterDataType::AttenuationRange)
            }
            _ => Err(err_enum!("FMOD_DSP_PARAMETER_DATA_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspType {
    Unknown,
    Mixer,
    Oscillator,
    Lowpass,
    Itlowpass,
    Highpass,
    Echo,
    Fader,
    Flange,
    Distortion,
    Normalize,
    Limiter,
    Parameq,
    Pitchshift,
    Chorus,
    Vstplugin,
    Winampplugin,
    Itecho,
    Compressor,
    Sfxreverb,
    LowpassSimple,
    Delay,
    Tremolo,
    Ladspaplugin,
    Send,
    Return,
    HighpassSimple,
    Pan,
    ThreeEq,
    Fft,
    LoudnessMeter,
    Envelopefollower,
    Convolutionreverb,
    Channelmix,
    Transceiver,
    Objectpan,
    MultibandEq,
    Max,
}

impl From<DspType> for ffi::FMOD_DSP_TYPE {
    fn from(value: DspType) -> ffi::FMOD_DSP_TYPE {
        match value {
            DspType::Unknown => ffi::FMOD_DSP_TYPE_UNKNOWN,
            DspType::Mixer => ffi::FMOD_DSP_TYPE_MIXER,
            DspType::Oscillator => ffi::FMOD_DSP_TYPE_OSCILLATOR,
            DspType::Lowpass => ffi::FMOD_DSP_TYPE_LOWPASS,
            DspType::Itlowpass => ffi::FMOD_DSP_TYPE_ITLOWPASS,
            DspType::Highpass => ffi::FMOD_DSP_TYPE_HIGHPASS,
            DspType::Echo => ffi::FMOD_DSP_TYPE_ECHO,
            DspType::Fader => ffi::FMOD_DSP_TYPE_FADER,
            DspType::Flange => ffi::FMOD_DSP_TYPE_FLANGE,
            DspType::Distortion => ffi::FMOD_DSP_TYPE_DISTORTION,
            DspType::Normalize => ffi::FMOD_DSP_TYPE_NORMALIZE,
            DspType::Limiter => ffi::FMOD_DSP_TYPE_LIMITER,
            DspType::Parameq => ffi::FMOD_DSP_TYPE_PARAMEQ,
            DspType::Pitchshift => ffi::FMOD_DSP_TYPE_PITCHSHIFT,
            DspType::Chorus => ffi::FMOD_DSP_TYPE_CHORUS,
            DspType::Vstplugin => ffi::FMOD_DSP_TYPE_VSTPLUGIN,
            DspType::Winampplugin => ffi::FMOD_DSP_TYPE_WINAMPPLUGIN,
            DspType::Itecho => ffi::FMOD_DSP_TYPE_ITECHO,
            DspType::Compressor => ffi::FMOD_DSP_TYPE_COMPRESSOR,
            DspType::Sfxreverb => ffi::FMOD_DSP_TYPE_SFXREVERB,
            DspType::LowpassSimple => ffi::FMOD_DSP_TYPE_LOWPASS_SIMPLE,
            DspType::Delay => ffi::FMOD_DSP_TYPE_DELAY,
            DspType::Tremolo => ffi::FMOD_DSP_TYPE_TREMOLO,
            DspType::Ladspaplugin => ffi::FMOD_DSP_TYPE_LADSPAPLUGIN,
            DspType::Send => ffi::FMOD_DSP_TYPE_SEND,
            DspType::Return => ffi::FMOD_DSP_TYPE_RETURN,
            DspType::HighpassSimple => ffi::FMOD_DSP_TYPE_HIGHPASS_SIMPLE,
            DspType::Pan => ffi::FMOD_DSP_TYPE_PAN,
            DspType::ThreeEq => ffi::FMOD_DSP_TYPE_THREE_EQ,
            DspType::Fft => ffi::FMOD_DSP_TYPE_FFT,
            DspType::LoudnessMeter => ffi::FMOD_DSP_TYPE_LOUDNESS_METER,
            DspType::Envelopefollower => ffi::FMOD_DSP_TYPE_ENVELOPEFOLLOWER,
            DspType::Convolutionreverb => ffi::FMOD_DSP_TYPE_CONVOLUTIONREVERB,
            DspType::Channelmix => ffi::FMOD_DSP_TYPE_CHANNELMIX,
            DspType::Transceiver => ffi::FMOD_DSP_TYPE_TRANSCEIVER,
            DspType::Objectpan => ffi::FMOD_DSP_TYPE_OBJECTPAN,
            DspType::MultibandEq => ffi::FMOD_DSP_TYPE_MULTIBAND_EQ,
            DspType::Max => ffi::FMOD_DSP_TYPE_MAX,
        }
    }
}

impl DspType {
    pub fn from(value: ffi::FMOD_DSP_TYPE) -> Result<DspType, Error> {
        match value {
            ffi::FMOD_DSP_TYPE_UNKNOWN => Ok(DspType::Unknown),
            ffi::FMOD_DSP_TYPE_MIXER => Ok(DspType::Mixer),
            ffi::FMOD_DSP_TYPE_OSCILLATOR => Ok(DspType::Oscillator),
            ffi::FMOD_DSP_TYPE_LOWPASS => Ok(DspType::Lowpass),
            ffi::FMOD_DSP_TYPE_ITLOWPASS => Ok(DspType::Itlowpass),
            ffi::FMOD_DSP_TYPE_HIGHPASS => Ok(DspType::Highpass),
            ffi::FMOD_DSP_TYPE_ECHO => Ok(DspType::Echo),
            ffi::FMOD_DSP_TYPE_FADER => Ok(DspType::Fader),
            ffi::FMOD_DSP_TYPE_FLANGE => Ok(DspType::Flange),
            ffi::FMOD_DSP_TYPE_DISTORTION => Ok(DspType::Distortion),
            ffi::FMOD_DSP_TYPE_NORMALIZE => Ok(DspType::Normalize),
            ffi::FMOD_DSP_TYPE_LIMITER => Ok(DspType::Limiter),
            ffi::FMOD_DSP_TYPE_PARAMEQ => Ok(DspType::Parameq),
            ffi::FMOD_DSP_TYPE_PITCHSHIFT => Ok(DspType::Pitchshift),
            ffi::FMOD_DSP_TYPE_CHORUS => Ok(DspType::Chorus),
            ffi::FMOD_DSP_TYPE_VSTPLUGIN => Ok(DspType::Vstplugin),
            ffi::FMOD_DSP_TYPE_WINAMPPLUGIN => Ok(DspType::Winampplugin),
            ffi::FMOD_DSP_TYPE_ITECHO => Ok(DspType::Itecho),
            ffi::FMOD_DSP_TYPE_COMPRESSOR => Ok(DspType::Compressor),
            ffi::FMOD_DSP_TYPE_SFXREVERB => Ok(DspType::Sfxreverb),
            ffi::FMOD_DSP_TYPE_LOWPASS_SIMPLE => Ok(DspType::LowpassSimple),
            ffi::FMOD_DSP_TYPE_DELAY => Ok(DspType::Delay),
            ffi::FMOD_DSP_TYPE_TREMOLO => Ok(DspType::Tremolo),
            ffi::FMOD_DSP_TYPE_LADSPAPLUGIN => Ok(DspType::Ladspaplugin),
            ffi::FMOD_DSP_TYPE_SEND => Ok(DspType::Send),
            ffi::FMOD_DSP_TYPE_RETURN => Ok(DspType::Return),
            ffi::FMOD_DSP_TYPE_HIGHPASS_SIMPLE => Ok(DspType::HighpassSimple),
            ffi::FMOD_DSP_TYPE_PAN => Ok(DspType::Pan),
            ffi::FMOD_DSP_TYPE_THREE_EQ => Ok(DspType::ThreeEq),
            ffi::FMOD_DSP_TYPE_FFT => Ok(DspType::Fft),
            ffi::FMOD_DSP_TYPE_LOUDNESS_METER => Ok(DspType::LoudnessMeter),
            ffi::FMOD_DSP_TYPE_ENVELOPEFOLLOWER => Ok(DspType::Envelopefollower),
            ffi::FMOD_DSP_TYPE_CONVOLUTIONREVERB => Ok(DspType::Convolutionreverb),
            ffi::FMOD_DSP_TYPE_CHANNELMIX => Ok(DspType::Channelmix),
            ffi::FMOD_DSP_TYPE_TRANSCEIVER => Ok(DspType::Transceiver),
            ffi::FMOD_DSP_TYPE_OBJECTPAN => Ok(DspType::Objectpan),
            ffi::FMOD_DSP_TYPE_MULTIBAND_EQ => Ok(DspType::MultibandEq),
            ffi::FMOD_DSP_TYPE_MAX => Ok(DspType::Max),
            _ => Err(err_enum!("FMOD_DSP_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspOscillator {
    Type,
    Rate,
}

impl From<DspOscillator> for ffi::FMOD_DSP_OSCILLATOR {
    fn from(value: DspOscillator) -> ffi::FMOD_DSP_OSCILLATOR {
        match value {
            DspOscillator::Type => ffi::FMOD_DSP_OSCILLATOR_TYPE,
            DspOscillator::Rate => ffi::FMOD_DSP_OSCILLATOR_RATE,
        }
    }
}

impl DspOscillator {
    pub fn from(value: ffi::FMOD_DSP_OSCILLATOR) -> Result<DspOscillator, Error> {
        match value {
            ffi::FMOD_DSP_OSCILLATOR_TYPE => Ok(DspOscillator::Type),
            ffi::FMOD_DSP_OSCILLATOR_RATE => Ok(DspOscillator::Rate),
            _ => Err(err_enum!("FMOD_DSP_OSCILLATOR", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspLowPass {
    Cutoff,
    Resonance,
}

impl From<DspLowPass> for ffi::FMOD_DSP_LOWPASS {
    fn from(value: DspLowPass) -> ffi::FMOD_DSP_LOWPASS {
        match value {
            DspLowPass::Cutoff => ffi::FMOD_DSP_LOWPASS_CUTOFF,
            DspLowPass::Resonance => ffi::FMOD_DSP_LOWPASS_RESONANCE,
        }
    }
}

impl DspLowPass {
    pub fn from(value: ffi::FMOD_DSP_LOWPASS) -> Result<DspLowPass, Error> {
        match value {
            ffi::FMOD_DSP_LOWPASS_CUTOFF => Ok(DspLowPass::Cutoff),
            ffi::FMOD_DSP_LOWPASS_RESONANCE => Ok(DspLowPass::Resonance),
            _ => Err(err_enum!("FMOD_DSP_LOWPASS", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspItLowPass {
    Cutoff,
    Resonance,
}

impl From<DspItLowPass> for ffi::FMOD_DSP_ITLOWPASS {
    fn from(value: DspItLowPass) -> ffi::FMOD_DSP_ITLOWPASS {
        match value {
            DspItLowPass::Cutoff => ffi::FMOD_DSP_ITLOWPASS_CUTOFF,
            DspItLowPass::Resonance => ffi::FMOD_DSP_ITLOWPASS_RESONANCE,
        }
    }
}

impl DspItLowPass {
    pub fn from(value: ffi::FMOD_DSP_ITLOWPASS) -> Result<DspItLowPass, Error> {
        match value {
            ffi::FMOD_DSP_ITLOWPASS_CUTOFF => Ok(DspItLowPass::Cutoff),
            ffi::FMOD_DSP_ITLOWPASS_RESONANCE => Ok(DspItLowPass::Resonance),
            _ => Err(err_enum!("FMOD_DSP_ITLOWPASS", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspHighPass {
    Cutoff,
    Resonance,
}

impl From<DspHighPass> for ffi::FMOD_DSP_HIGHPASS {
    fn from(value: DspHighPass) -> ffi::FMOD_DSP_HIGHPASS {
        match value {
            DspHighPass::Cutoff => ffi::FMOD_DSP_HIGHPASS_CUTOFF,
            DspHighPass::Resonance => ffi::FMOD_DSP_HIGHPASS_RESONANCE,
        }
    }
}

impl DspHighPass {
    pub fn from(value: ffi::FMOD_DSP_HIGHPASS) -> Result<DspHighPass, Error> {
        match value {
            ffi::FMOD_DSP_HIGHPASS_CUTOFF => Ok(DspHighPass::Cutoff),
            ffi::FMOD_DSP_HIGHPASS_RESONANCE => Ok(DspHighPass::Resonance),
            _ => Err(err_enum!("FMOD_DSP_HIGHPASS", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspEcho {
    Delay,
    Feedback,
    DryLevel,
    WetLevel,
}

impl From<DspEcho> for ffi::FMOD_DSP_ECHO {
    fn from(value: DspEcho) -> ffi::FMOD_DSP_ECHO {
        match value {
            DspEcho::Delay => ffi::FMOD_DSP_ECHO_DELAY,
            DspEcho::Feedback => ffi::FMOD_DSP_ECHO_FEEDBACK,
            DspEcho::DryLevel => ffi::FMOD_DSP_ECHO_DRYLEVEL,
            DspEcho::WetLevel => ffi::FMOD_DSP_ECHO_WETLEVEL,
        }
    }
}

impl DspEcho {
    pub fn from(value: ffi::FMOD_DSP_ECHO) -> Result<DspEcho, Error> {
        match value {
            ffi::FMOD_DSP_ECHO_DELAY => Ok(DspEcho::Delay),
            ffi::FMOD_DSP_ECHO_FEEDBACK => Ok(DspEcho::Feedback),
            ffi::FMOD_DSP_ECHO_DRYLEVEL => Ok(DspEcho::DryLevel),
            ffi::FMOD_DSP_ECHO_WETLEVEL => Ok(DspEcho::WetLevel),
            _ => Err(err_enum!("FMOD_DSP_ECHO", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspFader {
    Gain,
    OverallGain,
}

impl From<DspFader> for ffi::FMOD_DSP_FADER {
    fn from(value: DspFader) -> ffi::FMOD_DSP_FADER {
        match value {
            DspFader::Gain => ffi::FMOD_DSP_FADER_GAIN,
            DspFader::OverallGain => ffi::FMOD_DSP_FADER_OVERALL_GAIN,
        }
    }
}

impl DspFader {
    pub fn from(value: ffi::FMOD_DSP_FADER) -> Result<DspFader, Error> {
        match value {
            ffi::FMOD_DSP_FADER_GAIN => Ok(DspFader::Gain),
            ffi::FMOD_DSP_FADER_OVERALL_GAIN => Ok(DspFader::OverallGain),
            _ => Err(err_enum!("FMOD_DSP_FADER", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspFlange {
    Mix,
    Depth,
    Rate,
}

impl From<DspFlange> for ffi::FMOD_DSP_FLANGE {
    fn from(value: DspFlange) -> ffi::FMOD_DSP_FLANGE {
        match value {
            DspFlange::Mix => ffi::FMOD_DSP_FLANGE_MIX,
            DspFlange::Depth => ffi::FMOD_DSP_FLANGE_DEPTH,
            DspFlange::Rate => ffi::FMOD_DSP_FLANGE_RATE,
        }
    }
}

impl DspFlange {
    pub fn from(value: ffi::FMOD_DSP_FLANGE) -> Result<DspFlange, Error> {
        match value {
            ffi::FMOD_DSP_FLANGE_MIX => Ok(DspFlange::Mix),
            ffi::FMOD_DSP_FLANGE_DEPTH => Ok(DspFlange::Depth),
            ffi::FMOD_DSP_FLANGE_RATE => Ok(DspFlange::Rate),
            _ => Err(err_enum!("FMOD_DSP_FLANGE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspDistortion {
    Level,
}

impl From<DspDistortion> for ffi::FMOD_DSP_DISTORTION {
    fn from(value: DspDistortion) -> ffi::FMOD_DSP_DISTORTION {
        match value {
            DspDistortion::Level => ffi::FMOD_DSP_DISTORTION_LEVEL,
        }
    }
}

impl DspDistortion {
    pub fn from(value: ffi::FMOD_DSP_DISTORTION) -> Result<DspDistortion, Error> {
        match value {
            ffi::FMOD_DSP_DISTORTION_LEVEL => Ok(DspDistortion::Level),
            _ => Err(err_enum!("FMOD_DSP_DISTORTION", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspNormalize {
    FadeTime,
    Threshold,
    MaxAmp,
}

impl From<DspNormalize> for ffi::FMOD_DSP_NORMALIZE {
    fn from(value: DspNormalize) -> ffi::FMOD_DSP_NORMALIZE {
        match value {
            DspNormalize::FadeTime => ffi::FMOD_DSP_NORMALIZE_FADETIME,
            DspNormalize::Threshold => ffi::FMOD_DSP_NORMALIZE_THRESHOLD,
            DspNormalize::MaxAmp => ffi::FMOD_DSP_NORMALIZE_MAXAMP,
        }
    }
}

impl DspNormalize {
    pub fn from(value: ffi::FMOD_DSP_NORMALIZE) -> Result<DspNormalize, Error> {
        match value {
            ffi::FMOD_DSP_NORMALIZE_FADETIME => Ok(DspNormalize::FadeTime),
            ffi::FMOD_DSP_NORMALIZE_THRESHOLD => Ok(DspNormalize::Threshold),
            ffi::FMOD_DSP_NORMALIZE_MAXAMP => Ok(DspNormalize::MaxAmp),
            _ => Err(err_enum!("FMOD_DSP_NORMALIZE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspLimiter {
    ReleaseTime,
    Ceiling,
    MaximizerGain,
    Mode,
}

impl From<DspLimiter> for ffi::FMOD_DSP_LIMITER {
    fn from(value: DspLimiter) -> ffi::FMOD_DSP_LIMITER {
        match value {
            DspLimiter::ReleaseTime => ffi::FMOD_DSP_LIMITER_RELEASETIME,
            DspLimiter::Ceiling => ffi::FMOD_DSP_LIMITER_CEILING,
            DspLimiter::MaximizerGain => ffi::FMOD_DSP_LIMITER_MAXIMIZERGAIN,
            DspLimiter::Mode => ffi::FMOD_DSP_LIMITER_MODE,
        }
    }
}

impl DspLimiter {
    pub fn from(value: ffi::FMOD_DSP_LIMITER) -> Result<DspLimiter, Error> {
        match value {
            ffi::FMOD_DSP_LIMITER_RELEASETIME => Ok(DspLimiter::ReleaseTime),
            ffi::FMOD_DSP_LIMITER_CEILING => Ok(DspLimiter::Ceiling),
            ffi::FMOD_DSP_LIMITER_MAXIMIZERGAIN => Ok(DspLimiter::MaximizerGain),
            ffi::FMOD_DSP_LIMITER_MODE => Ok(DspLimiter::Mode),
            _ => Err(err_enum!("FMOD_DSP_LIMITER", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspParameq {
    Center,
    Bandwidth,
    Gain,
}

impl From<DspParameq> for ffi::FMOD_DSP_PARAMEQ {
    fn from(value: DspParameq) -> ffi::FMOD_DSP_PARAMEQ {
        match value {
            DspParameq::Center => ffi::FMOD_DSP_PARAMEQ_CENTER,
            DspParameq::Bandwidth => ffi::FMOD_DSP_PARAMEQ_BANDWIDTH,
            DspParameq::Gain => ffi::FMOD_DSP_PARAMEQ_GAIN,
        }
    }
}

impl DspParameq {
    pub fn from(value: ffi::FMOD_DSP_PARAMEQ) -> Result<DspParameq, Error> {
        match value {
            ffi::FMOD_DSP_PARAMEQ_CENTER => Ok(DspParameq::Center),
            ffi::FMOD_DSP_PARAMEQ_BANDWIDTH => Ok(DspParameq::Bandwidth),
            ffi::FMOD_DSP_PARAMEQ_GAIN => Ok(DspParameq::Gain),
            _ => Err(err_enum!("FMOD_DSP_PARAMEQ", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspMultibandEq {
    AFilter,
    AFrequency,
    AQ,
    AGain,
    BFilter,
    BFrequency,
    BQ,
    BGain,
    CFilter,
    CFrequency,
    CQ,
    CGain,
    DFilter,
    DFrequency,
    DQ,
    DGain,
    EFilter,
    EFrequency,
    EQ,
    EGain,
}

impl From<DspMultibandEq> for ffi::FMOD_DSP_MULTIBAND_EQ {
    fn from(value: DspMultibandEq) -> ffi::FMOD_DSP_MULTIBAND_EQ {
        match value {
            DspMultibandEq::AFilter => ffi::FMOD_DSP_MULTIBAND_EQ_A_FILTER,
            DspMultibandEq::AFrequency => ffi::FMOD_DSP_MULTIBAND_EQ_A_FREQUENCY,
            DspMultibandEq::AQ => ffi::FMOD_DSP_MULTIBAND_EQ_A_Q,
            DspMultibandEq::AGain => ffi::FMOD_DSP_MULTIBAND_EQ_A_GAIN,
            DspMultibandEq::BFilter => ffi::FMOD_DSP_MULTIBAND_EQ_B_FILTER,
            DspMultibandEq::BFrequency => ffi::FMOD_DSP_MULTIBAND_EQ_B_FREQUENCY,
            DspMultibandEq::BQ => ffi::FMOD_DSP_MULTIBAND_EQ_B_Q,
            DspMultibandEq::BGain => ffi::FMOD_DSP_MULTIBAND_EQ_B_GAIN,
            DspMultibandEq::CFilter => ffi::FMOD_DSP_MULTIBAND_EQ_C_FILTER,
            DspMultibandEq::CFrequency => ffi::FMOD_DSP_MULTIBAND_EQ_C_FREQUENCY,
            DspMultibandEq::CQ => ffi::FMOD_DSP_MULTIBAND_EQ_C_Q,
            DspMultibandEq::CGain => ffi::FMOD_DSP_MULTIBAND_EQ_C_GAIN,
            DspMultibandEq::DFilter => ffi::FMOD_DSP_MULTIBAND_EQ_D_FILTER,
            DspMultibandEq::DFrequency => ffi::FMOD_DSP_MULTIBAND_EQ_D_FREQUENCY,
            DspMultibandEq::DQ => ffi::FMOD_DSP_MULTIBAND_EQ_D_Q,
            DspMultibandEq::DGain => ffi::FMOD_DSP_MULTIBAND_EQ_D_GAIN,
            DspMultibandEq::EFilter => ffi::FMOD_DSP_MULTIBAND_EQ_E_FILTER,
            DspMultibandEq::EFrequency => ffi::FMOD_DSP_MULTIBAND_EQ_E_FREQUENCY,
            DspMultibandEq::EQ => ffi::FMOD_DSP_MULTIBAND_EQ_E_Q,
            DspMultibandEq::EGain => ffi::FMOD_DSP_MULTIBAND_EQ_E_GAIN,
        }
    }
}

impl DspMultibandEq {
    pub fn from(value: ffi::FMOD_DSP_MULTIBAND_EQ) -> Result<DspMultibandEq, Error> {
        match value {
            ffi::FMOD_DSP_MULTIBAND_EQ_A_FILTER => Ok(DspMultibandEq::AFilter),
            ffi::FMOD_DSP_MULTIBAND_EQ_A_FREQUENCY => Ok(DspMultibandEq::AFrequency),
            ffi::FMOD_DSP_MULTIBAND_EQ_A_Q => Ok(DspMultibandEq::AQ),
            ffi::FMOD_DSP_MULTIBAND_EQ_A_GAIN => Ok(DspMultibandEq::AGain),
            ffi::FMOD_DSP_MULTIBAND_EQ_B_FILTER => Ok(DspMultibandEq::BFilter),
            ffi::FMOD_DSP_MULTIBAND_EQ_B_FREQUENCY => Ok(DspMultibandEq::BFrequency),
            ffi::FMOD_DSP_MULTIBAND_EQ_B_Q => Ok(DspMultibandEq::BQ),
            ffi::FMOD_DSP_MULTIBAND_EQ_B_GAIN => Ok(DspMultibandEq::BGain),
            ffi::FMOD_DSP_MULTIBAND_EQ_C_FILTER => Ok(DspMultibandEq::CFilter),
            ffi::FMOD_DSP_MULTIBAND_EQ_C_FREQUENCY => Ok(DspMultibandEq::CFrequency),
            ffi::FMOD_DSP_MULTIBAND_EQ_C_Q => Ok(DspMultibandEq::CQ),
            ffi::FMOD_DSP_MULTIBAND_EQ_C_GAIN => Ok(DspMultibandEq::CGain),
            ffi::FMOD_DSP_MULTIBAND_EQ_D_FILTER => Ok(DspMultibandEq::DFilter),
            ffi::FMOD_DSP_MULTIBAND_EQ_D_FREQUENCY => Ok(DspMultibandEq::DFrequency),
            ffi::FMOD_DSP_MULTIBAND_EQ_D_Q => Ok(DspMultibandEq::DQ),
            ffi::FMOD_DSP_MULTIBAND_EQ_D_GAIN => Ok(DspMultibandEq::DGain),
            ffi::FMOD_DSP_MULTIBAND_EQ_E_FILTER => Ok(DspMultibandEq::EFilter),
            ffi::FMOD_DSP_MULTIBAND_EQ_E_FREQUENCY => Ok(DspMultibandEq::EFrequency),
            ffi::FMOD_DSP_MULTIBAND_EQ_E_Q => Ok(DspMultibandEq::EQ),
            ffi::FMOD_DSP_MULTIBAND_EQ_E_GAIN => Ok(DspMultibandEq::EGain),
            _ => Err(err_enum!("FMOD_DSP_MULTIBAND_EQ", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspMultibandEqFilterType {
    Disabled,
    Lowpass12Db,
    Lowpass24Db,
    Lowpass48Db,
    Highpass12Db,
    Highpass24Db,
    Highpass48Db,
    LowShelf,
    HighShelf,
    Peaking,
    Bandpass,
    Notch,
    AllPass,
}

impl From<DspMultibandEqFilterType> for ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_TYPE {
    fn from(value: DspMultibandEqFilterType) -> ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_TYPE {
        match value {
            DspMultibandEqFilterType::Disabled => ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_DISABLED,
            DspMultibandEqFilterType::Lowpass12Db => ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_LOWPASS_12DB,
            DspMultibandEqFilterType::Lowpass24Db => ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_LOWPASS_24DB,
            DspMultibandEqFilterType::Lowpass48Db => ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_LOWPASS_48DB,
            DspMultibandEqFilterType::Highpass12Db => {
                ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHPASS_12DB
            }
            DspMultibandEqFilterType::Highpass24Db => {
                ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHPASS_24DB
            }
            DspMultibandEqFilterType::Highpass48Db => {
                ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHPASS_48DB
            }
            DspMultibandEqFilterType::LowShelf => ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_LOWSHELF,
            DspMultibandEqFilterType::HighShelf => ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHSHELF,
            DspMultibandEqFilterType::Peaking => ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_PEAKING,
            DspMultibandEqFilterType::Bandpass => ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_BANDPASS,
            DspMultibandEqFilterType::Notch => ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_NOTCH,
            DspMultibandEqFilterType::AllPass => ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_ALLPASS,
        }
    }
}

impl DspMultibandEqFilterType {
    pub fn from(
        value: ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_TYPE,
    ) -> Result<DspMultibandEqFilterType, Error> {
        match value {
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_DISABLED => Ok(DspMultibandEqFilterType::Disabled),
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_LOWPASS_12DB => {
                Ok(DspMultibandEqFilterType::Lowpass12Db)
            }
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_LOWPASS_24DB => {
                Ok(DspMultibandEqFilterType::Lowpass24Db)
            }
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_LOWPASS_48DB => {
                Ok(DspMultibandEqFilterType::Lowpass48Db)
            }
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHPASS_12DB => {
                Ok(DspMultibandEqFilterType::Highpass12Db)
            }
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHPASS_24DB => {
                Ok(DspMultibandEqFilterType::Highpass24Db)
            }
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHPASS_48DB => {
                Ok(DspMultibandEqFilterType::Highpass48Db)
            }
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_LOWSHELF => Ok(DspMultibandEqFilterType::LowShelf),
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHSHELF => Ok(DspMultibandEqFilterType::HighShelf),
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_PEAKING => Ok(DspMultibandEqFilterType::Peaking),
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_BANDPASS => Ok(DspMultibandEqFilterType::Bandpass),
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_NOTCH => Ok(DspMultibandEqFilterType::Notch),
            ffi::FMOD_DSP_MULTIBAND_EQ_FILTER_ALLPASS => Ok(DspMultibandEqFilterType::AllPass),
            _ => Err(err_enum!("FMOD_DSP_MULTIBAND_EQ_FILTER_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspPitchShift {
    Pitch,
    FftSize,
    Overlap,
    MaxChannels,
}

impl From<DspPitchShift> for ffi::FMOD_DSP_PITCHSHIFT {
    fn from(value: DspPitchShift) -> ffi::FMOD_DSP_PITCHSHIFT {
        match value {
            DspPitchShift::Pitch => ffi::FMOD_DSP_PITCHSHIFT_PITCH,
            DspPitchShift::FftSize => ffi::FMOD_DSP_PITCHSHIFT_FFTSIZE,
            DspPitchShift::Overlap => ffi::FMOD_DSP_PITCHSHIFT_OVERLAP,
            DspPitchShift::MaxChannels => ffi::FMOD_DSP_PITCHSHIFT_MAXCHANNELS,
        }
    }
}

impl DspPitchShift {
    pub fn from(value: ffi::FMOD_DSP_PITCHSHIFT) -> Result<DspPitchShift, Error> {
        match value {
            ffi::FMOD_DSP_PITCHSHIFT_PITCH => Ok(DspPitchShift::Pitch),
            ffi::FMOD_DSP_PITCHSHIFT_FFTSIZE => Ok(DspPitchShift::FftSize),
            ffi::FMOD_DSP_PITCHSHIFT_OVERLAP => Ok(DspPitchShift::Overlap),
            ffi::FMOD_DSP_PITCHSHIFT_MAXCHANNELS => Ok(DspPitchShift::MaxChannels),
            _ => Err(err_enum!("FMOD_DSP_PITCHSHIFT", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspChorus {
    Mix,
    Rate,
    Depth,
}

impl From<DspChorus> for ffi::FMOD_DSP_CHORUS {
    fn from(value: DspChorus) -> ffi::FMOD_DSP_CHORUS {
        match value {
            DspChorus::Mix => ffi::FMOD_DSP_CHORUS_MIX,
            DspChorus::Rate => ffi::FMOD_DSP_CHORUS_RATE,
            DspChorus::Depth => ffi::FMOD_DSP_CHORUS_DEPTH,
        }
    }
}

impl DspChorus {
    pub fn from(value: ffi::FMOD_DSP_CHORUS) -> Result<DspChorus, Error> {
        match value {
            ffi::FMOD_DSP_CHORUS_MIX => Ok(DspChorus::Mix),
            ffi::FMOD_DSP_CHORUS_RATE => Ok(DspChorus::Rate),
            ffi::FMOD_DSP_CHORUS_DEPTH => Ok(DspChorus::Depth),
            _ => Err(err_enum!("FMOD_DSP_CHORUS", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspItEcho {
    WetDryMix,
    Feedback,
    LeftDelay,
    RightDelay,
    PanDelay,
}

impl From<DspItEcho> for ffi::FMOD_DSP_ITECHO {
    fn from(value: DspItEcho) -> ffi::FMOD_DSP_ITECHO {
        match value {
            DspItEcho::WetDryMix => ffi::FMOD_DSP_ITECHO_WETDRYMIX,
            DspItEcho::Feedback => ffi::FMOD_DSP_ITECHO_FEEDBACK,
            DspItEcho::LeftDelay => ffi::FMOD_DSP_ITECHO_LEFTDELAY,
            DspItEcho::RightDelay => ffi::FMOD_DSP_ITECHO_RIGHTDELAY,
            DspItEcho::PanDelay => ffi::FMOD_DSP_ITECHO_PANDELAY,
        }
    }
}

impl DspItEcho {
    pub fn from(value: ffi::FMOD_DSP_ITECHO) -> Result<DspItEcho, Error> {
        match value {
            ffi::FMOD_DSP_ITECHO_WETDRYMIX => Ok(DspItEcho::WetDryMix),
            ffi::FMOD_DSP_ITECHO_FEEDBACK => Ok(DspItEcho::Feedback),
            ffi::FMOD_DSP_ITECHO_LEFTDELAY => Ok(DspItEcho::LeftDelay),
            ffi::FMOD_DSP_ITECHO_RIGHTDELAY => Ok(DspItEcho::RightDelay),
            ffi::FMOD_DSP_ITECHO_PANDELAY => Ok(DspItEcho::PanDelay),
            _ => Err(err_enum!("FMOD_DSP_ITECHO", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspCompressor {
    Threshold,
    Ratio,
    Attack,
    Release,
    GainMakeup,
    UseSidechain,
    Linked,
}

impl From<DspCompressor> for ffi::FMOD_DSP_COMPRESSOR {
    fn from(value: DspCompressor) -> ffi::FMOD_DSP_COMPRESSOR {
        match value {
            DspCompressor::Threshold => ffi::FMOD_DSP_COMPRESSOR_THRESHOLD,
            DspCompressor::Ratio => ffi::FMOD_DSP_COMPRESSOR_RATIO,
            DspCompressor::Attack => ffi::FMOD_DSP_COMPRESSOR_ATTACK,
            DspCompressor::Release => ffi::FMOD_DSP_COMPRESSOR_RELEASE,
            DspCompressor::GainMakeup => ffi::FMOD_DSP_COMPRESSOR_GAINMAKEUP,
            DspCompressor::UseSidechain => ffi::FMOD_DSP_COMPRESSOR_USESIDECHAIN,
            DspCompressor::Linked => ffi::FMOD_DSP_COMPRESSOR_LINKED,
        }
    }
}

impl DspCompressor {
    pub fn from(value: ffi::FMOD_DSP_COMPRESSOR) -> Result<DspCompressor, Error> {
        match value {
            ffi::FMOD_DSP_COMPRESSOR_THRESHOLD => Ok(DspCompressor::Threshold),
            ffi::FMOD_DSP_COMPRESSOR_RATIO => Ok(DspCompressor::Ratio),
            ffi::FMOD_DSP_COMPRESSOR_ATTACK => Ok(DspCompressor::Attack),
            ffi::FMOD_DSP_COMPRESSOR_RELEASE => Ok(DspCompressor::Release),
            ffi::FMOD_DSP_COMPRESSOR_GAINMAKEUP => Ok(DspCompressor::GainMakeup),
            ffi::FMOD_DSP_COMPRESSOR_USESIDECHAIN => Ok(DspCompressor::UseSidechain),
            ffi::FMOD_DSP_COMPRESSOR_LINKED => Ok(DspCompressor::Linked),
            _ => Err(err_enum!("FMOD_DSP_COMPRESSOR", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspSfxReverb {
    DecayTime,
    EarlyDelay,
    LateDelay,
    HfReference,
    HfDecayRatio,
    Diffusion,
    Density,
    LowShelfFrequency,
    LowShelfGain,
    HighCut,
    EarlyLateMix,
    WetLevel,
    DryLevel,
}

impl From<DspSfxReverb> for ffi::FMOD_DSP_SFXREVERB {
    fn from(value: DspSfxReverb) -> ffi::FMOD_DSP_SFXREVERB {
        match value {
            DspSfxReverb::DecayTime => ffi::FMOD_DSP_SFXREVERB_DECAYTIME,
            DspSfxReverb::EarlyDelay => ffi::FMOD_DSP_SFXREVERB_EARLYDELAY,
            DspSfxReverb::LateDelay => ffi::FMOD_DSP_SFXREVERB_LATEDELAY,
            DspSfxReverb::HfReference => ffi::FMOD_DSP_SFXREVERB_HFREFERENCE,
            DspSfxReverb::HfDecayRatio => ffi::FMOD_DSP_SFXREVERB_HFDECAYRATIO,
            DspSfxReverb::Diffusion => ffi::FMOD_DSP_SFXREVERB_DIFFUSION,
            DspSfxReverb::Density => ffi::FMOD_DSP_SFXREVERB_DENSITY,
            DspSfxReverb::LowShelfFrequency => ffi::FMOD_DSP_SFXREVERB_LOWSHELFFREQUENCY,
            DspSfxReverb::LowShelfGain => ffi::FMOD_DSP_SFXREVERB_LOWSHELFGAIN,
            DspSfxReverb::HighCut => ffi::FMOD_DSP_SFXREVERB_HIGHCUT,
            DspSfxReverb::EarlyLateMix => ffi::FMOD_DSP_SFXREVERB_EARLYLATEMIX,
            DspSfxReverb::WetLevel => ffi::FMOD_DSP_SFXREVERB_WETLEVEL,
            DspSfxReverb::DryLevel => ffi::FMOD_DSP_SFXREVERB_DRYLEVEL,
        }
    }
}

impl DspSfxReverb {
    pub fn from(value: ffi::FMOD_DSP_SFXREVERB) -> Result<DspSfxReverb, Error> {
        match value {
            ffi::FMOD_DSP_SFXREVERB_DECAYTIME => Ok(DspSfxReverb::DecayTime),
            ffi::FMOD_DSP_SFXREVERB_EARLYDELAY => Ok(DspSfxReverb::EarlyDelay),
            ffi::FMOD_DSP_SFXREVERB_LATEDELAY => Ok(DspSfxReverb::LateDelay),
            ffi::FMOD_DSP_SFXREVERB_HFREFERENCE => Ok(DspSfxReverb::HfReference),
            ffi::FMOD_DSP_SFXREVERB_HFDECAYRATIO => Ok(DspSfxReverb::HfDecayRatio),
            ffi::FMOD_DSP_SFXREVERB_DIFFUSION => Ok(DspSfxReverb::Diffusion),
            ffi::FMOD_DSP_SFXREVERB_DENSITY => Ok(DspSfxReverb::Density),
            ffi::FMOD_DSP_SFXREVERB_LOWSHELFFREQUENCY => Ok(DspSfxReverb::LowShelfFrequency),
            ffi::FMOD_DSP_SFXREVERB_LOWSHELFGAIN => Ok(DspSfxReverb::LowShelfGain),
            ffi::FMOD_DSP_SFXREVERB_HIGHCUT => Ok(DspSfxReverb::HighCut),
            ffi::FMOD_DSP_SFXREVERB_EARLYLATEMIX => Ok(DspSfxReverb::EarlyLateMix),
            ffi::FMOD_DSP_SFXREVERB_WETLEVEL => Ok(DspSfxReverb::WetLevel),
            ffi::FMOD_DSP_SFXREVERB_DRYLEVEL => Ok(DspSfxReverb::DryLevel),
            _ => Err(err_enum!("FMOD_DSP_SFXREVERB", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspLowPassSimple {
    Cutoff,
}

impl From<DspLowPassSimple> for ffi::FMOD_DSP_LOWPASS_SIMPLE {
    fn from(value: DspLowPassSimple) -> ffi::FMOD_DSP_LOWPASS_SIMPLE {
        match value {
            DspLowPassSimple::Cutoff => ffi::FMOD_DSP_LOWPASS_SIMPLE_CUTOFF,
        }
    }
}

impl DspLowPassSimple {
    pub fn from(value: ffi::FMOD_DSP_LOWPASS_SIMPLE) -> Result<DspLowPassSimple, Error> {
        match value {
            ffi::FMOD_DSP_LOWPASS_SIMPLE_CUTOFF => Ok(DspLowPassSimple::Cutoff),
            _ => Err(err_enum!("FMOD_DSP_LOWPASS_SIMPLE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspDelay {
    Ch0,
    Ch1,
    Ch2,
    Ch3,
    Ch4,
    Ch5,
    Ch6,
    Ch7,
    Ch8,
    Ch9,
    Ch10,
    Ch11,
    Ch12,
    Ch13,
    Ch14,
    Ch15,
    MaxDelay,
}

impl From<DspDelay> for ffi::FMOD_DSP_DELAY {
    fn from(value: DspDelay) -> ffi::FMOD_DSP_DELAY {
        match value {
            DspDelay::Ch0 => ffi::FMOD_DSP_DELAY_CH0,
            DspDelay::Ch1 => ffi::FMOD_DSP_DELAY_CH1,
            DspDelay::Ch2 => ffi::FMOD_DSP_DELAY_CH2,
            DspDelay::Ch3 => ffi::FMOD_DSP_DELAY_CH3,
            DspDelay::Ch4 => ffi::FMOD_DSP_DELAY_CH4,
            DspDelay::Ch5 => ffi::FMOD_DSP_DELAY_CH5,
            DspDelay::Ch6 => ffi::FMOD_DSP_DELAY_CH6,
            DspDelay::Ch7 => ffi::FMOD_DSP_DELAY_CH7,
            DspDelay::Ch8 => ffi::FMOD_DSP_DELAY_CH8,
            DspDelay::Ch9 => ffi::FMOD_DSP_DELAY_CH9,
            DspDelay::Ch10 => ffi::FMOD_DSP_DELAY_CH10,
            DspDelay::Ch11 => ffi::FMOD_DSP_DELAY_CH11,
            DspDelay::Ch12 => ffi::FMOD_DSP_DELAY_CH12,
            DspDelay::Ch13 => ffi::FMOD_DSP_DELAY_CH13,
            DspDelay::Ch14 => ffi::FMOD_DSP_DELAY_CH14,
            DspDelay::Ch15 => ffi::FMOD_DSP_DELAY_CH15,
            DspDelay::MaxDelay => ffi::FMOD_DSP_DELAY_MAXDELAY,
        }
    }
}

impl DspDelay {
    pub fn from(value: ffi::FMOD_DSP_DELAY) -> Result<DspDelay, Error> {
        match value {
            ffi::FMOD_DSP_DELAY_CH0 => Ok(DspDelay::Ch0),
            ffi::FMOD_DSP_DELAY_CH1 => Ok(DspDelay::Ch1),
            ffi::FMOD_DSP_DELAY_CH2 => Ok(DspDelay::Ch2),
            ffi::FMOD_DSP_DELAY_CH3 => Ok(DspDelay::Ch3),
            ffi::FMOD_DSP_DELAY_CH4 => Ok(DspDelay::Ch4),
            ffi::FMOD_DSP_DELAY_CH5 => Ok(DspDelay::Ch5),
            ffi::FMOD_DSP_DELAY_CH6 => Ok(DspDelay::Ch6),
            ffi::FMOD_DSP_DELAY_CH7 => Ok(DspDelay::Ch7),
            ffi::FMOD_DSP_DELAY_CH8 => Ok(DspDelay::Ch8),
            ffi::FMOD_DSP_DELAY_CH9 => Ok(DspDelay::Ch9),
            ffi::FMOD_DSP_DELAY_CH10 => Ok(DspDelay::Ch10),
            ffi::FMOD_DSP_DELAY_CH11 => Ok(DspDelay::Ch11),
            ffi::FMOD_DSP_DELAY_CH12 => Ok(DspDelay::Ch12),
            ffi::FMOD_DSP_DELAY_CH13 => Ok(DspDelay::Ch13),
            ffi::FMOD_DSP_DELAY_CH14 => Ok(DspDelay::Ch14),
            ffi::FMOD_DSP_DELAY_CH15 => Ok(DspDelay::Ch15),
            ffi::FMOD_DSP_DELAY_MAXDELAY => Ok(DspDelay::MaxDelay),
            _ => Err(err_enum!("FMOD_DSP_DELAY", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspTremolo {
    Frequency,
    Depth,
    Shape,
    Skew,
    Duty,
    Square,
    Phase,
    Spread,
}

impl From<DspTremolo> for ffi::FMOD_DSP_TREMOLO {
    fn from(value: DspTremolo) -> ffi::FMOD_DSP_TREMOLO {
        match value {
            DspTremolo::Frequency => ffi::FMOD_DSP_TREMOLO_FREQUENCY,
            DspTremolo::Depth => ffi::FMOD_DSP_TREMOLO_DEPTH,
            DspTremolo::Shape => ffi::FMOD_DSP_TREMOLO_SHAPE,
            DspTremolo::Skew => ffi::FMOD_DSP_TREMOLO_SKEW,
            DspTremolo::Duty => ffi::FMOD_DSP_TREMOLO_DUTY,
            DspTremolo::Square => ffi::FMOD_DSP_TREMOLO_SQUARE,
            DspTremolo::Phase => ffi::FMOD_DSP_TREMOLO_PHASE,
            DspTremolo::Spread => ffi::FMOD_DSP_TREMOLO_SPREAD,
        }
    }
}

impl DspTremolo {
    pub fn from(value: ffi::FMOD_DSP_TREMOLO) -> Result<DspTremolo, Error> {
        match value {
            ffi::FMOD_DSP_TREMOLO_FREQUENCY => Ok(DspTremolo::Frequency),
            ffi::FMOD_DSP_TREMOLO_DEPTH => Ok(DspTremolo::Depth),
            ffi::FMOD_DSP_TREMOLO_SHAPE => Ok(DspTremolo::Shape),
            ffi::FMOD_DSP_TREMOLO_SKEW => Ok(DspTremolo::Skew),
            ffi::FMOD_DSP_TREMOLO_DUTY => Ok(DspTremolo::Duty),
            ffi::FMOD_DSP_TREMOLO_SQUARE => Ok(DspTremolo::Square),
            ffi::FMOD_DSP_TREMOLO_PHASE => Ok(DspTremolo::Phase),
            ffi::FMOD_DSP_TREMOLO_SPREAD => Ok(DspTremolo::Spread),
            _ => Err(err_enum!("FMOD_DSP_TREMOLO", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspSend {
    ReturnId,
    Level,
}

impl From<DspSend> for ffi::FMOD_DSP_SEND {
    fn from(value: DspSend) -> ffi::FMOD_DSP_SEND {
        match value {
            DspSend::ReturnId => ffi::FMOD_DSP_SEND_RETURNID,
            DspSend::Level => ffi::FMOD_DSP_SEND_LEVEL,
        }
    }
}

impl DspSend {
    pub fn from(value: ffi::FMOD_DSP_SEND) -> Result<DspSend, Error> {
        match value {
            ffi::FMOD_DSP_SEND_RETURNID => Ok(DspSend::ReturnId),
            ffi::FMOD_DSP_SEND_LEVEL => Ok(DspSend::Level),
            _ => Err(err_enum!("FMOD_DSP_SEND", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspReturn {
    Id,
    InputSpeakerMode,
}

impl From<DspReturn> for ffi::FMOD_DSP_RETURN {
    fn from(value: DspReturn) -> ffi::FMOD_DSP_RETURN {
        match value {
            DspReturn::Id => ffi::FMOD_DSP_RETURN_ID,
            DspReturn::InputSpeakerMode => ffi::FMOD_DSP_RETURN_INPUT_SPEAKER_MODE,
        }
    }
}

impl DspReturn {
    pub fn from(value: ffi::FMOD_DSP_RETURN) -> Result<DspReturn, Error> {
        match value {
            ffi::FMOD_DSP_RETURN_ID => Ok(DspReturn::Id),
            ffi::FMOD_DSP_RETURN_INPUT_SPEAKER_MODE => Ok(DspReturn::InputSpeakerMode),
            _ => Err(err_enum!("FMOD_DSP_RETURN", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspHighpassSimple {
    Cutoff,
}

impl From<DspHighpassSimple> for ffi::FMOD_DSP_HIGHPASS_SIMPLE {
    fn from(value: DspHighpassSimple) -> ffi::FMOD_DSP_HIGHPASS_SIMPLE {
        match value {
            DspHighpassSimple::Cutoff => ffi::FMOD_DSP_HIGHPASS_SIMPLE_CUTOFF,
        }
    }
}

impl DspHighpassSimple {
    pub fn from(value: ffi::FMOD_DSP_HIGHPASS_SIMPLE) -> Result<DspHighpassSimple, Error> {
        match value {
            ffi::FMOD_DSP_HIGHPASS_SIMPLE_CUTOFF => Ok(DspHighpassSimple::Cutoff),
            _ => Err(err_enum!("FMOD_DSP_HIGHPASS_SIMPLE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspPan2DStereoModeType {
    Distributed,
    Discrete,
}

impl From<DspPan2DStereoModeType> for ffi::FMOD_DSP_PAN_2D_STEREO_MODE_TYPE {
    fn from(value: DspPan2DStereoModeType) -> ffi::FMOD_DSP_PAN_2D_STEREO_MODE_TYPE {
        match value {
            DspPan2DStereoModeType::Distributed => ffi::FMOD_DSP_PAN_2D_STEREO_MODE_DISTRIBUTED,
            DspPan2DStereoModeType::Discrete => ffi::FMOD_DSP_PAN_2D_STEREO_MODE_DISCRETE,
        }
    }
}

impl DspPan2DStereoModeType {
    pub fn from(
        value: ffi::FMOD_DSP_PAN_2D_STEREO_MODE_TYPE,
    ) -> Result<DspPan2DStereoModeType, Error> {
        match value {
            ffi::FMOD_DSP_PAN_2D_STEREO_MODE_DISTRIBUTED => Ok(DspPan2DStereoModeType::Distributed),
            ffi::FMOD_DSP_PAN_2D_STEREO_MODE_DISCRETE => Ok(DspPan2DStereoModeType::Discrete),
            _ => Err(err_enum!("FMOD_DSP_PAN_2D_STEREO_MODE_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspPanModeType {
    Mono,
    Stereo,
    Surround,
}

impl From<DspPanModeType> for ffi::FMOD_DSP_PAN_MODE_TYPE {
    fn from(value: DspPanModeType) -> ffi::FMOD_DSP_PAN_MODE_TYPE {
        match value {
            DspPanModeType::Mono => ffi::FMOD_DSP_PAN_MODE_MONO,
            DspPanModeType::Stereo => ffi::FMOD_DSP_PAN_MODE_STEREO,
            DspPanModeType::Surround => ffi::FMOD_DSP_PAN_MODE_SURROUND,
        }
    }
}

impl DspPanModeType {
    pub fn from(value: ffi::FMOD_DSP_PAN_MODE_TYPE) -> Result<DspPanModeType, Error> {
        match value {
            ffi::FMOD_DSP_PAN_MODE_MONO => Ok(DspPanModeType::Mono),
            ffi::FMOD_DSP_PAN_MODE_STEREO => Ok(DspPanModeType::Stereo),
            ffi::FMOD_DSP_PAN_MODE_SURROUND => Ok(DspPanModeType::Surround),
            _ => Err(err_enum!("FMOD_DSP_PAN_MODE_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspPan3DRolloffType {
    LinearSquared,
    Linear,
    Inverse,
    InverseTapered,
    Custom,
}

impl From<DspPan3DRolloffType> for ffi::FMOD_DSP_PAN_3D_ROLLOFF_TYPE {
    fn from(value: DspPan3DRolloffType) -> ffi::FMOD_DSP_PAN_3D_ROLLOFF_TYPE {
        match value {
            DspPan3DRolloffType::LinearSquared => ffi::FMOD_DSP_PAN_3D_ROLLOFF_LINEARSQUARED,
            DspPan3DRolloffType::Linear => ffi::FMOD_DSP_PAN_3D_ROLLOFF_LINEAR,
            DspPan3DRolloffType::Inverse => ffi::FMOD_DSP_PAN_3D_ROLLOFF_INVERSE,
            DspPan3DRolloffType::InverseTapered => ffi::FMOD_DSP_PAN_3D_ROLLOFF_INVERSETAPERED,
            DspPan3DRolloffType::Custom => ffi::FMOD_DSP_PAN_3D_ROLLOFF_CUSTOM,
        }
    }
}

impl DspPan3DRolloffType {
    pub fn from(value: ffi::FMOD_DSP_PAN_3D_ROLLOFF_TYPE) -> Result<DspPan3DRolloffType, Error> {
        match value {
            ffi::FMOD_DSP_PAN_3D_ROLLOFF_LINEARSQUARED => Ok(DspPan3DRolloffType::LinearSquared),
            ffi::FMOD_DSP_PAN_3D_ROLLOFF_LINEAR => Ok(DspPan3DRolloffType::Linear),
            ffi::FMOD_DSP_PAN_3D_ROLLOFF_INVERSE => Ok(DspPan3DRolloffType::Inverse),
            ffi::FMOD_DSP_PAN_3D_ROLLOFF_INVERSETAPERED => Ok(DspPan3DRolloffType::InverseTapered),
            ffi::FMOD_DSP_PAN_3D_ROLLOFF_CUSTOM => Ok(DspPan3DRolloffType::Custom),
            _ => Err(err_enum!("FMOD_DSP_PAN_3D_ROLLOFF_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspPan3DExtentModeType {
    Auto,
    User,
    Off,
}

impl From<DspPan3DExtentModeType> for ffi::FMOD_DSP_PAN_3D_EXTENT_MODE_TYPE {
    fn from(value: DspPan3DExtentModeType) -> ffi::FMOD_DSP_PAN_3D_EXTENT_MODE_TYPE {
        match value {
            DspPan3DExtentModeType::Auto => ffi::FMOD_DSP_PAN_3D_EXTENT_MODE_AUTO,
            DspPan3DExtentModeType::User => ffi::FMOD_DSP_PAN_3D_EXTENT_MODE_USER,
            DspPan3DExtentModeType::Off => ffi::FMOD_DSP_PAN_3D_EXTENT_MODE_OFF,
        }
    }
}

impl DspPan3DExtentModeType {
    pub fn from(
        value: ffi::FMOD_DSP_PAN_3D_EXTENT_MODE_TYPE,
    ) -> Result<DspPan3DExtentModeType, Error> {
        match value {
            ffi::FMOD_DSP_PAN_3D_EXTENT_MODE_AUTO => Ok(DspPan3DExtentModeType::Auto),
            ffi::FMOD_DSP_PAN_3D_EXTENT_MODE_USER => Ok(DspPan3DExtentModeType::User),
            ffi::FMOD_DSP_PAN_3D_EXTENT_MODE_OFF => Ok(DspPan3DExtentModeType::Off),
            _ => Err(err_enum!("FMOD_DSP_PAN_3D_EXTENT_MODE_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspPan {
    Mode,
    StereoPosition2D,
    Direction2D,
    Extent2D,
    Rotation2D,
    LfeLevel2D,
    StereoMode2D,
    StereoSeparation2D,
    StereoAxis2D,
    EnabledSpeakers,
    Position3D,
    Rolloff3D,
    MinDistance3D,
    MaxDistance3D,
    ExtentMode3D,
    SoundSize3D,
    MinExtent3D,
    PanBlend3D,
    LfeUpmixEnabled,
    OverallGain,
    SurroundSpeakerMode,
    HeightBlend2D,
    AttenuationRange,
    OverrideRange,
}

impl From<DspPan> for ffi::FMOD_DSP_PAN {
    fn from(value: DspPan) -> ffi::FMOD_DSP_PAN {
        match value {
            DspPan::Mode => ffi::FMOD_DSP_PAN_MODE,
            DspPan::StereoPosition2D => ffi::FMOD_DSP_PAN_2D_STEREO_POSITION,
            DspPan::Direction2D => ffi::FMOD_DSP_PAN_2D_DIRECTION,
            DspPan::Extent2D => ffi::FMOD_DSP_PAN_2D_EXTENT,
            DspPan::Rotation2D => ffi::FMOD_DSP_PAN_2D_ROTATION,
            DspPan::LfeLevel2D => ffi::FMOD_DSP_PAN_2D_LFE_LEVEL,
            DspPan::StereoMode2D => ffi::FMOD_DSP_PAN_2D_STEREO_MODE,
            DspPan::StereoSeparation2D => ffi::FMOD_DSP_PAN_2D_STEREO_SEPARATION,
            DspPan::StereoAxis2D => ffi::FMOD_DSP_PAN_2D_STEREO_AXIS,
            DspPan::EnabledSpeakers => ffi::FMOD_DSP_PAN_ENABLED_SPEAKERS,
            DspPan::Position3D => ffi::FMOD_DSP_PAN_3D_POSITION,
            DspPan::Rolloff3D => ffi::FMOD_DSP_PAN_3D_ROLLOFF,
            DspPan::MinDistance3D => ffi::FMOD_DSP_PAN_3D_MIN_DISTANCE,
            DspPan::MaxDistance3D => ffi::FMOD_DSP_PAN_3D_MAX_DISTANCE,
            DspPan::ExtentMode3D => ffi::FMOD_DSP_PAN_3D_EXTENT_MODE,
            DspPan::SoundSize3D => ffi::FMOD_DSP_PAN_3D_SOUND_SIZE,
            DspPan::MinExtent3D => ffi::FMOD_DSP_PAN_3D_MIN_EXTENT,
            DspPan::PanBlend3D => ffi::FMOD_DSP_PAN_3D_PAN_BLEND,
            DspPan::LfeUpmixEnabled => ffi::FMOD_DSP_PAN_LFE_UPMIX_ENABLED,
            DspPan::OverallGain => ffi::FMOD_DSP_PAN_OVERALL_GAIN,
            DspPan::SurroundSpeakerMode => ffi::FMOD_DSP_PAN_SURROUND_SPEAKER_MODE,
            DspPan::HeightBlend2D => ffi::FMOD_DSP_PAN_2D_HEIGHT_BLEND,
            DspPan::AttenuationRange => ffi::FMOD_DSP_PAN_ATTENUATION_RANGE,
            DspPan::OverrideRange => ffi::FMOD_DSP_PAN_OVERRIDE_RANGE,
        }
    }
}

impl DspPan {
    pub fn from(value: ffi::FMOD_DSP_PAN) -> Result<DspPan, Error> {
        match value {
            ffi::FMOD_DSP_PAN_MODE => Ok(DspPan::Mode),
            ffi::FMOD_DSP_PAN_2D_STEREO_POSITION => Ok(DspPan::StereoPosition2D),
            ffi::FMOD_DSP_PAN_2D_DIRECTION => Ok(DspPan::Direction2D),
            ffi::FMOD_DSP_PAN_2D_EXTENT => Ok(DspPan::Extent2D),
            ffi::FMOD_DSP_PAN_2D_ROTATION => Ok(DspPan::Rotation2D),
            ffi::FMOD_DSP_PAN_2D_LFE_LEVEL => Ok(DspPan::LfeLevel2D),
            ffi::FMOD_DSP_PAN_2D_STEREO_MODE => Ok(DspPan::StereoMode2D),
            ffi::FMOD_DSP_PAN_2D_STEREO_SEPARATION => Ok(DspPan::StereoSeparation2D),
            ffi::FMOD_DSP_PAN_2D_STEREO_AXIS => Ok(DspPan::StereoAxis2D),
            ffi::FMOD_DSP_PAN_ENABLED_SPEAKERS => Ok(DspPan::EnabledSpeakers),
            ffi::FMOD_DSP_PAN_3D_POSITION => Ok(DspPan::Position3D),
            ffi::FMOD_DSP_PAN_3D_ROLLOFF => Ok(DspPan::Rolloff3D),
            ffi::FMOD_DSP_PAN_3D_MIN_DISTANCE => Ok(DspPan::MinDistance3D),
            ffi::FMOD_DSP_PAN_3D_MAX_DISTANCE => Ok(DspPan::MaxDistance3D),
            ffi::FMOD_DSP_PAN_3D_EXTENT_MODE => Ok(DspPan::ExtentMode3D),
            ffi::FMOD_DSP_PAN_3D_SOUND_SIZE => Ok(DspPan::SoundSize3D),
            ffi::FMOD_DSP_PAN_3D_MIN_EXTENT => Ok(DspPan::MinExtent3D),
            ffi::FMOD_DSP_PAN_3D_PAN_BLEND => Ok(DspPan::PanBlend3D),
            ffi::FMOD_DSP_PAN_LFE_UPMIX_ENABLED => Ok(DspPan::LfeUpmixEnabled),
            ffi::FMOD_DSP_PAN_OVERALL_GAIN => Ok(DspPan::OverallGain),
            ffi::FMOD_DSP_PAN_SURROUND_SPEAKER_MODE => Ok(DspPan::SurroundSpeakerMode),
            ffi::FMOD_DSP_PAN_2D_HEIGHT_BLEND => Ok(DspPan::HeightBlend2D),
            ffi::FMOD_DSP_PAN_ATTENUATION_RANGE => Ok(DspPan::AttenuationRange),
            ffi::FMOD_DSP_PAN_OVERRIDE_RANGE => Ok(DspPan::OverrideRange),
            _ => Err(err_enum!("FMOD_DSP_PAN", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspThreeEqCrossoverSlopeType {
    Slope12Db,
    Slope24Db,
    Slope48Db,
}

impl From<DspThreeEqCrossoverSlopeType> for ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_TYPE {
    fn from(value: DspThreeEqCrossoverSlopeType) -> ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_TYPE {
        match value {
            DspThreeEqCrossoverSlopeType::Slope12Db => ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_12DB,
            DspThreeEqCrossoverSlopeType::Slope24Db => ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_24DB,
            DspThreeEqCrossoverSlopeType::Slope48Db => ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_48DB,
        }
    }
}

impl DspThreeEqCrossoverSlopeType {
    pub fn from(
        value: ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_TYPE,
    ) -> Result<DspThreeEqCrossoverSlopeType, Error> {
        match value {
            ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_12DB => {
                Ok(DspThreeEqCrossoverSlopeType::Slope12Db)
            }
            ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_24DB => {
                Ok(DspThreeEqCrossoverSlopeType::Slope24Db)
            }
            ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_48DB => {
                Ok(DspThreeEqCrossoverSlopeType::Slope48Db)
            }
            _ => Err(err_enum!("FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspThreeEq {
    LowGain,
    MidGain,
    HighGain,
    LowCrossover,
    HightCorssover,
    CrossoverSlope,
}

impl From<DspThreeEq> for ffi::FMOD_DSP_THREE_EQ {
    fn from(value: DspThreeEq) -> ffi::FMOD_DSP_THREE_EQ {
        match value {
            DspThreeEq::LowGain => ffi::FMOD_DSP_THREE_EQ_LOWGAIN,
            DspThreeEq::MidGain => ffi::FMOD_DSP_THREE_EQ_MIDGAIN,
            DspThreeEq::HighGain => ffi::FMOD_DSP_THREE_EQ_HIGHGAIN,
            DspThreeEq::LowCrossover => ffi::FMOD_DSP_THREE_EQ_LOWCROSSOVER,
            DspThreeEq::HightCorssover => ffi::FMOD_DSP_THREE_EQ_HIGHCROSSOVER,
            DspThreeEq::CrossoverSlope => ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE,
        }
    }
}

impl DspThreeEq {
    pub fn from(value: ffi::FMOD_DSP_THREE_EQ) -> Result<DspThreeEq, Error> {
        match value {
            ffi::FMOD_DSP_THREE_EQ_LOWGAIN => Ok(DspThreeEq::LowGain),
            ffi::FMOD_DSP_THREE_EQ_MIDGAIN => Ok(DspThreeEq::MidGain),
            ffi::FMOD_DSP_THREE_EQ_HIGHGAIN => Ok(DspThreeEq::HighGain),
            ffi::FMOD_DSP_THREE_EQ_LOWCROSSOVER => Ok(DspThreeEq::LowCrossover),
            ffi::FMOD_DSP_THREE_EQ_HIGHCROSSOVER => Ok(DspThreeEq::HightCorssover),
            ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE => Ok(DspThreeEq::CrossoverSlope),
            _ => Err(err_enum!("FMOD_DSP_THREE_EQ", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspFftWindow {
    Rect,
    Triangle,
    Hamming,
    Hanning,
    BlackMan,
    BlackManHarris,
}

impl From<DspFftWindow> for ffi::FMOD_DSP_FFT_WINDOW {
    fn from(value: DspFftWindow) -> ffi::FMOD_DSP_FFT_WINDOW {
        match value {
            DspFftWindow::Rect => ffi::FMOD_DSP_FFT_WINDOW_RECT,
            DspFftWindow::Triangle => ffi::FMOD_DSP_FFT_WINDOW_TRIANGLE,
            DspFftWindow::Hamming => ffi::FMOD_DSP_FFT_WINDOW_HAMMING,
            DspFftWindow::Hanning => ffi::FMOD_DSP_FFT_WINDOW_HANNING,
            DspFftWindow::BlackMan => ffi::FMOD_DSP_FFT_WINDOW_BLACKMAN,
            DspFftWindow::BlackManHarris => ffi::FMOD_DSP_FFT_WINDOW_BLACKMANHARRIS,
        }
    }
}

impl DspFftWindow {
    pub fn from(value: ffi::FMOD_DSP_FFT_WINDOW) -> Result<DspFftWindow, Error> {
        match value {
            ffi::FMOD_DSP_FFT_WINDOW_RECT => Ok(DspFftWindow::Rect),
            ffi::FMOD_DSP_FFT_WINDOW_TRIANGLE => Ok(DspFftWindow::Triangle),
            ffi::FMOD_DSP_FFT_WINDOW_HAMMING => Ok(DspFftWindow::Hamming),
            ffi::FMOD_DSP_FFT_WINDOW_HANNING => Ok(DspFftWindow::Hanning),
            ffi::FMOD_DSP_FFT_WINDOW_BLACKMAN => Ok(DspFftWindow::BlackMan),
            ffi::FMOD_DSP_FFT_WINDOW_BLACKMANHARRIS => Ok(DspFftWindow::BlackManHarris),
            _ => Err(err_enum!("FMOD_DSP_FFT_WINDOW", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspFft {
    WindowSize,
    WindowType,
    SpectrumData,
    DominantFreq,
}

impl From<DspFft> for ffi::FMOD_DSP_FFT {
    fn from(value: DspFft) -> ffi::FMOD_DSP_FFT {
        match value {
            DspFft::WindowSize => ffi::FMOD_DSP_FFT_WINDOWSIZE,
            DspFft::WindowType => ffi::FMOD_DSP_FFT_WINDOWTYPE,
            DspFft::SpectrumData => ffi::FMOD_DSP_FFT_SPECTRUMDATA,
            DspFft::DominantFreq => ffi::FMOD_DSP_FFT_DOMINANT_FREQ,
        }
    }
}

impl DspFft {
    pub fn from(value: ffi::FMOD_DSP_FFT) -> Result<DspFft, Error> {
        match value {
            ffi::FMOD_DSP_FFT_WINDOWSIZE => Ok(DspFft::WindowSize),
            ffi::FMOD_DSP_FFT_WINDOWTYPE => Ok(DspFft::WindowType),
            ffi::FMOD_DSP_FFT_SPECTRUMDATA => Ok(DspFft::SpectrumData),
            ffi::FMOD_DSP_FFT_DOMINANT_FREQ => Ok(DspFft::DominantFreq),
            _ => Err(err_enum!("FMOD_DSP_FFT", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspLoudnessMeter {
    State,
    Weighting,
    Info,
}

impl From<DspLoudnessMeter> for ffi::FMOD_DSP_LOUDNESS_METER {
    fn from(value: DspLoudnessMeter) -> ffi::FMOD_DSP_LOUDNESS_METER {
        match value {
            DspLoudnessMeter::State => ffi::FMOD_DSP_LOUDNESS_METER_STATE,
            DspLoudnessMeter::Weighting => ffi::FMOD_DSP_LOUDNESS_METER_WEIGHTING,
            DspLoudnessMeter::Info => ffi::FMOD_DSP_LOUDNESS_METER_INFO,
        }
    }
}

impl DspLoudnessMeter {
    pub fn from(value: ffi::FMOD_DSP_LOUDNESS_METER) -> Result<DspLoudnessMeter, Error> {
        match value {
            ffi::FMOD_DSP_LOUDNESS_METER_STATE => Ok(DspLoudnessMeter::State),
            ffi::FMOD_DSP_LOUDNESS_METER_WEIGHTING => Ok(DspLoudnessMeter::Weighting),
            ffi::FMOD_DSP_LOUDNESS_METER_INFO => Ok(DspLoudnessMeter::Info),
            _ => Err(err_enum!("FMOD_DSP_LOUDNESS_METER", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspLoudnessMeterStateType {
    ResetIntegrated,
    ResetMaxpeak,
    ResetAll,
    Paused,
    Analyzing,
}

impl From<DspLoudnessMeterStateType> for ffi::FMOD_DSP_LOUDNESS_METER_STATE_TYPE {
    fn from(value: DspLoudnessMeterStateType) -> ffi::FMOD_DSP_LOUDNESS_METER_STATE_TYPE {
        match value {
            DspLoudnessMeterStateType::ResetIntegrated => {
                ffi::FMOD_DSP_LOUDNESS_METER_STATE_RESET_INTEGRATED
            }
            DspLoudnessMeterStateType::ResetMaxpeak => {
                ffi::FMOD_DSP_LOUDNESS_METER_STATE_RESET_MAXPEAK
            }
            DspLoudnessMeterStateType::ResetAll => ffi::FMOD_DSP_LOUDNESS_METER_STATE_RESET_ALL,
            DspLoudnessMeterStateType::Paused => ffi::FMOD_DSP_LOUDNESS_METER_STATE_PAUSED,
            DspLoudnessMeterStateType::Analyzing => ffi::FMOD_DSP_LOUDNESS_METER_STATE_ANALYZING,
        }
    }
}

impl DspLoudnessMeterStateType {
    pub fn from(
        value: ffi::FMOD_DSP_LOUDNESS_METER_STATE_TYPE,
    ) -> Result<DspLoudnessMeterStateType, Error> {
        match value {
            ffi::FMOD_DSP_LOUDNESS_METER_STATE_RESET_INTEGRATED => {
                Ok(DspLoudnessMeterStateType::ResetIntegrated)
            }
            ffi::FMOD_DSP_LOUDNESS_METER_STATE_RESET_MAXPEAK => {
                Ok(DspLoudnessMeterStateType::ResetMaxpeak)
            }
            ffi::FMOD_DSP_LOUDNESS_METER_STATE_RESET_ALL => Ok(DspLoudnessMeterStateType::ResetAll),
            ffi::FMOD_DSP_LOUDNESS_METER_STATE_PAUSED => Ok(DspLoudnessMeterStateType::Paused),
            ffi::FMOD_DSP_LOUDNESS_METER_STATE_ANALYZING => {
                Ok(DspLoudnessMeterStateType::Analyzing)
            }
            _ => Err(err_enum!("FMOD_DSP_LOUDNESS_METER_STATE_TYPE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspEnvelopeFollower {
    Attack,
    Release,
    Envelope,
    UseSidechain,
}

impl From<DspEnvelopeFollower> for ffi::FMOD_DSP_ENVELOPEFOLLOWER {
    fn from(value: DspEnvelopeFollower) -> ffi::FMOD_DSP_ENVELOPEFOLLOWER {
        match value {
            DspEnvelopeFollower::Attack => ffi::FMOD_DSP_ENVELOPEFOLLOWER_ATTACK,
            DspEnvelopeFollower::Release => ffi::FMOD_DSP_ENVELOPEFOLLOWER_RELEASE,
            DspEnvelopeFollower::Envelope => ffi::FMOD_DSP_ENVELOPEFOLLOWER_ENVELOPE,
            DspEnvelopeFollower::UseSidechain => ffi::FMOD_DSP_ENVELOPEFOLLOWER_USESIDECHAIN,
        }
    }
}

impl DspEnvelopeFollower {
    pub fn from(value: ffi::FMOD_DSP_ENVELOPEFOLLOWER) -> Result<DspEnvelopeFollower, Error> {
        match value {
            ffi::FMOD_DSP_ENVELOPEFOLLOWER_ATTACK => Ok(DspEnvelopeFollower::Attack),
            ffi::FMOD_DSP_ENVELOPEFOLLOWER_RELEASE => Ok(DspEnvelopeFollower::Release),
            ffi::FMOD_DSP_ENVELOPEFOLLOWER_ENVELOPE => Ok(DspEnvelopeFollower::Envelope),
            ffi::FMOD_DSP_ENVELOPEFOLLOWER_USESIDECHAIN => Ok(DspEnvelopeFollower::UseSidechain),
            _ => Err(err_enum!("FMOD_DSP_ENVELOPEFOLLOWER", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspConvolutionReverb {
    ParamIr,
    ParamWet,
    ParamDry,
    ParamLinked,
}

impl From<DspConvolutionReverb> for ffi::FMOD_DSP_CONVOLUTION_REVERB {
    fn from(value: DspConvolutionReverb) -> ffi::FMOD_DSP_CONVOLUTION_REVERB {
        match value {
            DspConvolutionReverb::ParamIr => ffi::FMOD_DSP_CONVOLUTION_REVERB_PARAM_IR,
            DspConvolutionReverb::ParamWet => ffi::FMOD_DSP_CONVOLUTION_REVERB_PARAM_WET,
            DspConvolutionReverb::ParamDry => ffi::FMOD_DSP_CONVOLUTION_REVERB_PARAM_DRY,
            DspConvolutionReverb::ParamLinked => ffi::FMOD_DSP_CONVOLUTION_REVERB_PARAM_LINKED,
        }
    }
}

impl DspConvolutionReverb {
    pub fn from(value: ffi::FMOD_DSP_CONVOLUTION_REVERB) -> Result<DspConvolutionReverb, Error> {
        match value {
            ffi::FMOD_DSP_CONVOLUTION_REVERB_PARAM_IR => Ok(DspConvolutionReverb::ParamIr),
            ffi::FMOD_DSP_CONVOLUTION_REVERB_PARAM_WET => Ok(DspConvolutionReverb::ParamWet),
            ffi::FMOD_DSP_CONVOLUTION_REVERB_PARAM_DRY => Ok(DspConvolutionReverb::ParamDry),
            ffi::FMOD_DSP_CONVOLUTION_REVERB_PARAM_LINKED => Ok(DspConvolutionReverb::ParamLinked),
            _ => Err(err_enum!("FMOD_DSP_CONVOLUTION_REVERB", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspChannelMixOutput {
    Default,
    AllMono,
    AllStereo,
    AllQuad,
    All5Point1,
    All7Point1,
    AllLfe,
    All7Point1Point4,
}

impl From<DspChannelMixOutput> for ffi::FMOD_DSP_CHANNELMIX_OUTPUT {
    fn from(value: DspChannelMixOutput) -> ffi::FMOD_DSP_CHANNELMIX_OUTPUT {
        match value {
            DspChannelMixOutput::Default => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_DEFAULT,
            DspChannelMixOutput::AllMono => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALLMONO,
            DspChannelMixOutput::AllStereo => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALLSTEREO,
            DspChannelMixOutput::AllQuad => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALLQUAD,
            DspChannelMixOutput::All5Point1 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALL5POINT1,
            DspChannelMixOutput::All7Point1 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALL7POINT1,
            DspChannelMixOutput::AllLfe => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALLLFE,
            DspChannelMixOutput::All7Point1Point4 => {
                ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALL7POINT1POINT4
            }
        }
    }
}

impl DspChannelMixOutput {
    pub fn from(value: ffi::FMOD_DSP_CHANNELMIX_OUTPUT) -> Result<DspChannelMixOutput, Error> {
        match value {
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_DEFAULT => Ok(DspChannelMixOutput::Default),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALLMONO => Ok(DspChannelMixOutput::AllMono),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALLSTEREO => Ok(DspChannelMixOutput::AllStereo),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALLQUAD => Ok(DspChannelMixOutput::AllQuad),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALL5POINT1 => Ok(DspChannelMixOutput::All5Point1),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALL7POINT1 => Ok(DspChannelMixOutput::All7Point1),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALLLFE => Ok(DspChannelMixOutput::AllLfe),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_ALL7POINT1POINT4 => {
                Ok(DspChannelMixOutput::All7Point1Point4)
            }
            _ => Err(err_enum!("FMOD_DSP_CHANNELMIX_OUTPUT", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspChannelMix {
    OutputGrouping,
    GainCh0,
    GainCh1,
    GainCh2,
    GainCh3,
    GainCh4,
    GainCh5,
    GainCh6,
    GainCh7,
    GainCh8,
    GainCh9,
    GainCh10,
    GainCh11,
    GainCh12,
    GainCh13,
    GainCh14,
    GainCh15,
    GainCh16,
    GainCh17,
    GainCh18,
    GainCh19,
    GainCh20,
    GainCh21,
    GainCh22,
    GainCh23,
    GainCh24,
    GainCh25,
    GainCh26,
    GainCh27,
    GainCh28,
    GainCh29,
    GainCh30,
    GainCh31,
    OutputCh0,
    OutputCh1,
    OutputCh2,
    OutputCh3,
    OutputCh4,
    OutputCh5,
    OutputCh6,
    OutputCh7,
    OutputCh8,
    OutputCh9,
    OutputCh10,
    OutputCh11,
    OutputCh12,
    OutputCh13,
    OutputCh14,
    OutputCh15,
    OutputCh16,
    OutputCh17,
    OutputCh18,
    OutputCh19,
    OutputCh20,
    OutputCh21,
    OutputCh22,
    OutputCh23,
    OutputCh24,
    OutputCh25,
    OutputCh26,
    OutputCh27,
    OutputCh28,
    OutputCh29,
    OutputCh30,
    OutputCh31,
}

impl From<DspChannelMix> for ffi::FMOD_DSP_CHANNELMIX {
    fn from(value: DspChannelMix) -> ffi::FMOD_DSP_CHANNELMIX {
        match value {
            DspChannelMix::OutputGrouping => ffi::FMOD_DSP_CHANNELMIX_OUTPUTGROUPING,
            DspChannelMix::GainCh0 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH0,
            DspChannelMix::GainCh1 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH1,
            DspChannelMix::GainCh2 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH2,
            DspChannelMix::GainCh3 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH3,
            DspChannelMix::GainCh4 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH4,
            DspChannelMix::GainCh5 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH5,
            DspChannelMix::GainCh6 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH6,
            DspChannelMix::GainCh7 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH7,
            DspChannelMix::GainCh8 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH8,
            DspChannelMix::GainCh9 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH9,
            DspChannelMix::GainCh10 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH10,
            DspChannelMix::GainCh11 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH11,
            DspChannelMix::GainCh12 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH12,
            DspChannelMix::GainCh13 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH13,
            DspChannelMix::GainCh14 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH14,
            DspChannelMix::GainCh15 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH15,
            DspChannelMix::GainCh16 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH16,
            DspChannelMix::GainCh17 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH17,
            DspChannelMix::GainCh18 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH18,
            DspChannelMix::GainCh19 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH19,
            DspChannelMix::GainCh20 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH20,
            DspChannelMix::GainCh21 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH21,
            DspChannelMix::GainCh22 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH22,
            DspChannelMix::GainCh23 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH23,
            DspChannelMix::GainCh24 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH24,
            DspChannelMix::GainCh25 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH25,
            DspChannelMix::GainCh26 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH26,
            DspChannelMix::GainCh27 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH27,
            DspChannelMix::GainCh28 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH28,
            DspChannelMix::GainCh29 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH29,
            DspChannelMix::GainCh30 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH30,
            DspChannelMix::GainCh31 => ffi::FMOD_DSP_CHANNELMIX_GAIN_CH31,
            DspChannelMix::OutputCh0 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH0,
            DspChannelMix::OutputCh1 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH1,
            DspChannelMix::OutputCh2 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH2,
            DspChannelMix::OutputCh3 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH3,
            DspChannelMix::OutputCh4 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH4,
            DspChannelMix::OutputCh5 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH5,
            DspChannelMix::OutputCh6 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH6,
            DspChannelMix::OutputCh7 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH7,
            DspChannelMix::OutputCh8 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH8,
            DspChannelMix::OutputCh9 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH9,
            DspChannelMix::OutputCh10 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH10,
            DspChannelMix::OutputCh11 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH11,
            DspChannelMix::OutputCh12 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH12,
            DspChannelMix::OutputCh13 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH13,
            DspChannelMix::OutputCh14 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH14,
            DspChannelMix::OutputCh15 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH15,
            DspChannelMix::OutputCh16 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH16,
            DspChannelMix::OutputCh17 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH17,
            DspChannelMix::OutputCh18 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH18,
            DspChannelMix::OutputCh19 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH19,
            DspChannelMix::OutputCh20 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH20,
            DspChannelMix::OutputCh21 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH21,
            DspChannelMix::OutputCh22 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH22,
            DspChannelMix::OutputCh23 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH23,
            DspChannelMix::OutputCh24 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH24,
            DspChannelMix::OutputCh25 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH25,
            DspChannelMix::OutputCh26 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH26,
            DspChannelMix::OutputCh27 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH27,
            DspChannelMix::OutputCh28 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH28,
            DspChannelMix::OutputCh29 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH29,
            DspChannelMix::OutputCh30 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH30,
            DspChannelMix::OutputCh31 => ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH31,
        }
    }
}

impl DspChannelMix {
    pub fn from(value: ffi::FMOD_DSP_CHANNELMIX) -> Result<DspChannelMix, Error> {
        match value {
            ffi::FMOD_DSP_CHANNELMIX_OUTPUTGROUPING => Ok(DspChannelMix::OutputGrouping),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH0 => Ok(DspChannelMix::GainCh0),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH1 => Ok(DspChannelMix::GainCh1),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH2 => Ok(DspChannelMix::GainCh2),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH3 => Ok(DspChannelMix::GainCh3),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH4 => Ok(DspChannelMix::GainCh4),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH5 => Ok(DspChannelMix::GainCh5),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH6 => Ok(DspChannelMix::GainCh6),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH7 => Ok(DspChannelMix::GainCh7),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH8 => Ok(DspChannelMix::GainCh8),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH9 => Ok(DspChannelMix::GainCh9),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH10 => Ok(DspChannelMix::GainCh10),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH11 => Ok(DspChannelMix::GainCh11),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH12 => Ok(DspChannelMix::GainCh12),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH13 => Ok(DspChannelMix::GainCh13),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH14 => Ok(DspChannelMix::GainCh14),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH15 => Ok(DspChannelMix::GainCh15),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH16 => Ok(DspChannelMix::GainCh16),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH17 => Ok(DspChannelMix::GainCh17),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH18 => Ok(DspChannelMix::GainCh18),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH19 => Ok(DspChannelMix::GainCh19),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH20 => Ok(DspChannelMix::GainCh20),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH21 => Ok(DspChannelMix::GainCh21),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH22 => Ok(DspChannelMix::GainCh22),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH23 => Ok(DspChannelMix::GainCh23),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH24 => Ok(DspChannelMix::GainCh24),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH25 => Ok(DspChannelMix::GainCh25),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH26 => Ok(DspChannelMix::GainCh26),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH27 => Ok(DspChannelMix::GainCh27),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH28 => Ok(DspChannelMix::GainCh28),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH29 => Ok(DspChannelMix::GainCh29),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH30 => Ok(DspChannelMix::GainCh30),
            ffi::FMOD_DSP_CHANNELMIX_GAIN_CH31 => Ok(DspChannelMix::GainCh31),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH0 => Ok(DspChannelMix::OutputCh0),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH1 => Ok(DspChannelMix::OutputCh1),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH2 => Ok(DspChannelMix::OutputCh2),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH3 => Ok(DspChannelMix::OutputCh3),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH4 => Ok(DspChannelMix::OutputCh4),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH5 => Ok(DspChannelMix::OutputCh5),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH6 => Ok(DspChannelMix::OutputCh6),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH7 => Ok(DspChannelMix::OutputCh7),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH8 => Ok(DspChannelMix::OutputCh8),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH9 => Ok(DspChannelMix::OutputCh9),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH10 => Ok(DspChannelMix::OutputCh10),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH11 => Ok(DspChannelMix::OutputCh11),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH12 => Ok(DspChannelMix::OutputCh12),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH13 => Ok(DspChannelMix::OutputCh13),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH14 => Ok(DspChannelMix::OutputCh14),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH15 => Ok(DspChannelMix::OutputCh15),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH16 => Ok(DspChannelMix::OutputCh16),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH17 => Ok(DspChannelMix::OutputCh17),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH18 => Ok(DspChannelMix::OutputCh18),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH19 => Ok(DspChannelMix::OutputCh19),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH20 => Ok(DspChannelMix::OutputCh20),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH21 => Ok(DspChannelMix::OutputCh21),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH22 => Ok(DspChannelMix::OutputCh22),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH23 => Ok(DspChannelMix::OutputCh23),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH24 => Ok(DspChannelMix::OutputCh24),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH25 => Ok(DspChannelMix::OutputCh25),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH26 => Ok(DspChannelMix::OutputCh26),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH27 => Ok(DspChannelMix::OutputCh27),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH28 => Ok(DspChannelMix::OutputCh28),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH29 => Ok(DspChannelMix::OutputCh29),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH30 => Ok(DspChannelMix::OutputCh30),
            ffi::FMOD_DSP_CHANNELMIX_OUTPUT_CH31 => Ok(DspChannelMix::OutputCh31),
            _ => Err(err_enum!("FMOD_DSP_CHANNELMIX", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspTransceiverSpeakerMode {
    Auto,
    Mono,
    Stereo,
    Surround,
}

impl From<DspTransceiverSpeakerMode> for ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE {
    fn from(value: DspTransceiverSpeakerMode) -> ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE {
        match value {
            DspTransceiverSpeakerMode::Auto => ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE_AUTO,
            DspTransceiverSpeakerMode::Mono => ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE_MONO,
            DspTransceiverSpeakerMode::Stereo => ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE_STEREO,
            DspTransceiverSpeakerMode::Surround => ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE_SURROUND,
        }
    }
}

impl DspTransceiverSpeakerMode {
    pub fn from(
        value: ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE,
    ) -> Result<DspTransceiverSpeakerMode, Error> {
        match value {
            ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE_AUTO => Ok(DspTransceiverSpeakerMode::Auto),
            ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE_MONO => Ok(DspTransceiverSpeakerMode::Mono),
            ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE_STEREO => Ok(DspTransceiverSpeakerMode::Stereo),
            ffi::FMOD_DSP_TRANSCEIVER_SPEAKERMODE_SURROUND => {
                Ok(DspTransceiverSpeakerMode::Surround)
            }
            _ => Err(err_enum!("FMOD_DSP_TRANSCEIVER_SPEAKERMODE", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspTransceiver {
    Transmit,
    Gain,
    Channel,
    TransmitSpeakerMode,
}

impl From<DspTransceiver> for ffi::FMOD_DSP_TRANSCEIVER {
    fn from(value: DspTransceiver) -> ffi::FMOD_DSP_TRANSCEIVER {
        match value {
            DspTransceiver::Transmit => ffi::FMOD_DSP_TRANSCEIVER_TRANSMIT,
            DspTransceiver::Gain => ffi::FMOD_DSP_TRANSCEIVER_GAIN,
            DspTransceiver::Channel => ffi::FMOD_DSP_TRANSCEIVER_CHANNEL,
            DspTransceiver::TransmitSpeakerMode => ffi::FMOD_DSP_TRANSCEIVER_TRANSMITSPEAKERMODE,
        }
    }
}

impl DspTransceiver {
    pub fn from(value: ffi::FMOD_DSP_TRANSCEIVER) -> Result<DspTransceiver, Error> {
        match value {
            ffi::FMOD_DSP_TRANSCEIVER_TRANSMIT => Ok(DspTransceiver::Transmit),
            ffi::FMOD_DSP_TRANSCEIVER_GAIN => Ok(DspTransceiver::Gain),
            ffi::FMOD_DSP_TRANSCEIVER_CHANNEL => Ok(DspTransceiver::Channel),
            ffi::FMOD_DSP_TRANSCEIVER_TRANSMITSPEAKERMODE => {
                Ok(DspTransceiver::TransmitSpeakerMode)
            }
            _ => Err(err_enum!("FMOD_DSP_TRANSCEIVER", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspObjectPan {
    Position3D,
    Rolloff3D,
    MinDistance3D,
    MaxDistance3D,
    ExtentMode3D,
    SoundSize3D,
    MinExtent3D,
    OverallGain,
    OutputGain,
    AttenuationRange,
    OverrideRange,
}

impl From<DspObjectPan> for ffi::FMOD_DSP_OBJECTPAN {
    fn from(value: DspObjectPan) -> ffi::FMOD_DSP_OBJECTPAN {
        match value {
            DspObjectPan::Position3D => ffi::FMOD_DSP_OBJECTPAN_3D_POSITION,
            DspObjectPan::Rolloff3D => ffi::FMOD_DSP_OBJECTPAN_3D_ROLLOFF,
            DspObjectPan::MinDistance3D => ffi::FMOD_DSP_OBJECTPAN_3D_MIN_DISTANCE,
            DspObjectPan::MaxDistance3D => ffi::FMOD_DSP_OBJECTPAN_3D_MAX_DISTANCE,
            DspObjectPan::ExtentMode3D => ffi::FMOD_DSP_OBJECTPAN_3D_EXTENT_MODE,
            DspObjectPan::SoundSize3D => ffi::FMOD_DSP_OBJECTPAN_3D_SOUND_SIZE,
            DspObjectPan::MinExtent3D => ffi::FMOD_DSP_OBJECTPAN_3D_MIN_EXTENT,
            DspObjectPan::OverallGain => ffi::FMOD_DSP_OBJECTPAN_OVERALL_GAIN,
            DspObjectPan::OutputGain => ffi::FMOD_DSP_OBJECTPAN_OUTPUTGAIN,
            DspObjectPan::AttenuationRange => ffi::FMOD_DSP_OBJECTPAN_ATTENUATION_RANGE,
            DspObjectPan::OverrideRange => ffi::FMOD_DSP_OBJECTPAN_OVERRIDE_RANGE,
        }
    }
}

impl DspObjectPan {
    pub fn from(value: ffi::FMOD_DSP_OBJECTPAN) -> Result<DspObjectPan, Error> {
        match value {
            ffi::FMOD_DSP_OBJECTPAN_3D_POSITION => Ok(DspObjectPan::Position3D),
            ffi::FMOD_DSP_OBJECTPAN_3D_ROLLOFF => Ok(DspObjectPan::Rolloff3D),
            ffi::FMOD_DSP_OBJECTPAN_3D_MIN_DISTANCE => Ok(DspObjectPan::MinDistance3D),
            ffi::FMOD_DSP_OBJECTPAN_3D_MAX_DISTANCE => Ok(DspObjectPan::MaxDistance3D),
            ffi::FMOD_DSP_OBJECTPAN_3D_EXTENT_MODE => Ok(DspObjectPan::ExtentMode3D),
            ffi::FMOD_DSP_OBJECTPAN_3D_SOUND_SIZE => Ok(DspObjectPan::SoundSize3D),
            ffi::FMOD_DSP_OBJECTPAN_3D_MIN_EXTENT => Ok(DspObjectPan::MinExtent3D),
            ffi::FMOD_DSP_OBJECTPAN_OVERALL_GAIN => Ok(DspObjectPan::OverallGain),
            ffi::FMOD_DSP_OBJECTPAN_OUTPUTGAIN => Ok(DspObjectPan::OutputGain),
            ffi::FMOD_DSP_OBJECTPAN_ATTENUATION_RANGE => Ok(DspObjectPan::AttenuationRange),
            ffi::FMOD_DSP_OBJECTPAN_OVERRIDE_RANGE => Ok(DspObjectPan::OverrideRange),
            _ => Err(err_enum!("FMOD_DSP_OBJECTPAN", value)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BankInfo {
    pub size: i32,
    pub userdata: *mut c_void,
    pub userdatalength: i32,
    pub opencallback: ffi::FMOD_FILE_OPEN_CALLBACK,
    pub closecallback: ffi::FMOD_FILE_CLOSE_CALLBACK,
    pub readcallback: ffi::FMOD_FILE_READ_CALLBACK,
    pub seekcallback: ffi::FMOD_FILE_SEEK_CALLBACK,
}

impl TryFrom<ffi::FMOD_STUDIO_BANK_INFO> for BankInfo {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_BANK_INFO) -> Result<Self, Self::Error> {
        unsafe {
            Ok(BankInfo {
                size: value.size,
                userdata: value.userdata,
                userdatalength: value.userdatalength,
                opencallback: value.opencallback,
                closecallback: value.closecallback,
                readcallback: value.readcallback,
                seekcallback: value.seekcallback,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_BANK_INFO> for BankInfo {
    fn into(self) -> ffi::FMOD_STUDIO_BANK_INFO {
        ffi::FMOD_STUDIO_BANK_INFO {
            size: self.size,
            userdata: self.userdata,
            userdatalength: self.userdatalength,
            opencallback: self.opencallback,
            closecallback: self.closecallback,
            readcallback: self.readcallback,
            seekcallback: self.seekcallback,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParameterId {
    pub data_1: u32,
    pub data_2: u32,
}

impl TryFrom<ffi::FMOD_STUDIO_PARAMETER_ID> for ParameterId {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_PARAMETER_ID) -> Result<Self, Self::Error> {
        unsafe {
            Ok(ParameterId {
                data_1: value.data1,
                data_2: value.data2,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_PARAMETER_ID> for ParameterId {
    fn into(self) -> ffi::FMOD_STUDIO_PARAMETER_ID {
        ffi::FMOD_STUDIO_PARAMETER_ID {
            data1: self.data_1,
            data2: self.data_2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParameterDescription {
    pub name: String,
    pub id: ParameterId,
    pub minimum: f32,
    pub maximum: f32,
    pub defaultvalue: f32,
    pub type_: ParameterType,
    pub flags: ffi::FMOD_STUDIO_PARAMETER_FLAGS,
    pub guid: Guid,
}

impl TryFrom<ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION> for ParameterDescription {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION) -> Result<Self, Self::Error> {
        unsafe {
            Ok(ParameterDescription {
                name: to_string!(value.name)?,
                id: ParameterId::try_from(value.id)?,
                minimum: value.minimum,
                maximum: value.maximum,
                defaultvalue: value.defaultvalue,
                type_: ParameterType::from(value.type_)?,
                flags: value.flags,
                guid: Guid::try_from(value.guid)?,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION> for ParameterDescription {
    fn into(self) -> ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION {
        ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION {
            name: self.name.as_ptr().cast(),
            id: self.id.into(),
            minimum: self.minimum,
            maximum: self.maximum,
            defaultvalue: self.defaultvalue,
            type_: self.type_.into(),
            flags: self.flags,
            guid: self.guid.into(),
        }
    }
}

#[derive(Clone)]
pub struct UserProperty {
    pub name: String,
    pub type_: UserPropertyType,
    pub union: ffi::FMOD_STUDIO_USER_PROPERTY_UNION,
}

impl TryFrom<ffi::FMOD_STUDIO_USER_PROPERTY> for UserProperty {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_USER_PROPERTY) -> Result<Self, Self::Error> {
        unsafe {
            Ok(UserProperty {
                name: to_string!(value.name)?,
                type_: UserPropertyType::from(value.type_)?,
                union: value.union,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_USER_PROPERTY> for UserProperty {
    fn into(self) -> ffi::FMOD_STUDIO_USER_PROPERTY {
        ffi::FMOD_STUDIO_USER_PROPERTY {
            name: self.name.as_ptr().cast(),
            type_: self.type_.into(),
            union: self.union,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProgrammerSoundProperties {
    pub name: String,
    pub sound: Sound,
    pub subsound_index: i32,
}

impl TryFrom<ffi::FMOD_STUDIO_PROGRAMMER_SOUND_PROPERTIES> for ProgrammerSoundProperties {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_PROGRAMMER_SOUND_PROPERTIES) -> Result<Self, Self::Error> {
        unsafe {
            Ok(ProgrammerSoundProperties {
                name: to_string!(value.name)?,
                sound: Sound::from(value.sound),
                subsound_index: value.subsoundIndex,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_PROGRAMMER_SOUND_PROPERTIES> for ProgrammerSoundProperties {
    fn into(self) -> ffi::FMOD_STUDIO_PROGRAMMER_SOUND_PROPERTIES {
        ffi::FMOD_STUDIO_PROGRAMMER_SOUND_PROPERTIES {
            name: self.name.as_ptr().cast(),
            sound: self.sound.as_mut_ptr(),
            subsoundIndex: self.subsound_index,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PluginInstanceProperties {
    pub name: String,
    pub dsp: Dsp,
}

impl TryFrom<ffi::FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES> for PluginInstanceProperties {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES) -> Result<Self, Self::Error> {
        unsafe {
            Ok(PluginInstanceProperties {
                name: to_string!(value.name)?,
                dsp: Dsp::from(value.dsp),
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES> for PluginInstanceProperties {
    fn into(self) -> ffi::FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES {
        ffi::FMOD_STUDIO_PLUGIN_INSTANCE_PROPERTIES {
            name: self.name.as_ptr().cast(),
            dsp: self.dsp.as_mut_ptr(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimelineMarkerProperties {
    pub name: String,
    pub position: i32,
}

impl TryFrom<ffi::FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES> for TimelineMarkerProperties {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES) -> Result<Self, Self::Error> {
        unsafe {
            Ok(TimelineMarkerProperties {
                name: to_string!(value.name)?,
                position: value.position,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES> for TimelineMarkerProperties {
    fn into(self) -> ffi::FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES {
        ffi::FMOD_STUDIO_TIMELINE_MARKER_PROPERTIES {
            name: self.name.as_ptr().cast(),
            position: self.position,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimelineBeatProperties {
    pub bar: i32,
    pub beat: i32,
    pub position: i32,
    pub tempo: f32,
    pub timesignatureupper: i32,
    pub timesignaturelower: i32,
}

impl TryFrom<ffi::FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES> for TimelineBeatProperties {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES) -> Result<Self, Self::Error> {
        unsafe {
            Ok(TimelineBeatProperties {
                bar: value.bar,
                beat: value.beat,
                position: value.position,
                tempo: value.tempo,
                timesignatureupper: value.timesignatureupper,
                timesignaturelower: value.timesignaturelower,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES> for TimelineBeatProperties {
    fn into(self) -> ffi::FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES {
        ffi::FMOD_STUDIO_TIMELINE_BEAT_PROPERTIES {
            bar: self.bar,
            beat: self.beat,
            position: self.position,
            tempo: self.tempo,
            timesignatureupper: self.timesignatureupper,
            timesignaturelower: self.timesignaturelower,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimelineNestedBeatProperties {
    pub eventid: Guid,
    pub properties: TimelineBeatProperties,
}

impl TryFrom<ffi::FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES> for TimelineNestedBeatProperties {
    type Error = Error;
    fn try_from(
        value: ffi::FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES,
    ) -> Result<Self, Self::Error> {
        unsafe {
            Ok(TimelineNestedBeatProperties {
                eventid: Guid::try_from(value.eventid)?,
                properties: TimelineBeatProperties::try_from(value.properties)?,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES> for TimelineNestedBeatProperties {
    fn into(self) -> ffi::FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES {
        ffi::FMOD_STUDIO_TIMELINE_NESTED_BEAT_PROPERTIES {
            eventid: self.eventid.into(),
            properties: self.properties.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StudioAdvancedSettings {
    pub commandqueuesize: u32,
    pub handleinitialsize: u32,
    pub studioupdateperiod: i32,
    pub idlesampledatapoolsize: i32,
    pub streamingscheduledelay: u32,
    pub encryptionkey: String,
}

impl TryFrom<ffi::FMOD_STUDIO_ADVANCEDSETTINGS> for StudioAdvancedSettings {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_ADVANCEDSETTINGS) -> Result<Self, Self::Error> {
        unsafe {
            Ok(StudioAdvancedSettings {
                commandqueuesize: value.commandqueuesize,
                handleinitialsize: value.handleinitialsize,
                studioupdateperiod: value.studioupdateperiod,
                idlesampledatapoolsize: value.idlesampledatapoolsize,
                streamingscheduledelay: value.streamingscheduledelay,
                encryptionkey: to_string!(value.encryptionkey)?,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_ADVANCEDSETTINGS> for StudioAdvancedSettings {
    fn into(self) -> ffi::FMOD_STUDIO_ADVANCEDSETTINGS {
        ffi::FMOD_STUDIO_ADVANCEDSETTINGS {
            cbsize: size_of::<ffi::FMOD_STUDIO_ADVANCEDSETTINGS>() as i32,
            commandqueuesize: self.commandqueuesize,
            handleinitialsize: self.handleinitialsize,
            studioupdateperiod: self.studioupdateperiod,
            idlesampledatapoolsize: self.idlesampledatapoolsize,
            streamingscheduledelay: self.streamingscheduledelay,
            encryptionkey: self.encryptionkey.as_ptr().cast(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StudioCpuUsage {
    pub update: f32,
}

impl TryFrom<ffi::FMOD_STUDIO_CPU_USAGE> for StudioCpuUsage {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_CPU_USAGE) -> Result<Self, Self::Error> {
        unsafe {
            Ok(StudioCpuUsage {
                update: value.update,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_CPU_USAGE> for StudioCpuUsage {
    fn into(self) -> ffi::FMOD_STUDIO_CPU_USAGE {
        ffi::FMOD_STUDIO_CPU_USAGE {
            update: self.update,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BufferInfo {
    pub currentusage: i32,
    pub peakusage: i32,
    pub capacity: i32,
    pub stallcount: i32,
    pub stalltime: f32,
}

impl TryFrom<ffi::FMOD_STUDIO_BUFFER_INFO> for BufferInfo {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_BUFFER_INFO) -> Result<Self, Self::Error> {
        unsafe {
            Ok(BufferInfo {
                currentusage: value.currentusage,
                peakusage: value.peakusage,
                capacity: value.capacity,
                stallcount: value.stallcount,
                stalltime: value.stalltime,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_BUFFER_INFO> for BufferInfo {
    fn into(self) -> ffi::FMOD_STUDIO_BUFFER_INFO {
        ffi::FMOD_STUDIO_BUFFER_INFO {
            currentusage: self.currentusage,
            peakusage: self.peakusage,
            capacity: self.capacity,
            stallcount: self.stallcount,
            stalltime: self.stalltime,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BufferUsage {
    pub studiocommandqueue: BufferInfo,
    pub studiohandle: BufferInfo,
}

impl TryFrom<ffi::FMOD_STUDIO_BUFFER_USAGE> for BufferUsage {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_BUFFER_USAGE) -> Result<Self, Self::Error> {
        unsafe {
            Ok(BufferUsage {
                studiocommandqueue: BufferInfo::try_from(value.studiocommandqueue)?,
                studiohandle: BufferInfo::try_from(value.studiohandle)?,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_BUFFER_USAGE> for BufferUsage {
    fn into(self) -> ffi::FMOD_STUDIO_BUFFER_USAGE {
        ffi::FMOD_STUDIO_BUFFER_USAGE {
            studiocommandqueue: self.studiocommandqueue.into(),
            studiohandle: self.studiohandle.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SoundInfo {
    pub name_or_data: String,
    pub mode: ffi::FMOD_MODE,
    pub exinfo: CreateSoundexInfo,
    pub subsoundindex: i32,
}

impl TryFrom<ffi::FMOD_STUDIO_SOUND_INFO> for SoundInfo {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_SOUND_INFO) -> Result<Self, Self::Error> {
        unsafe {
            Ok(SoundInfo {
                name_or_data: to_string!(value.name_or_data)?,
                mode: value.mode,
                exinfo: CreateSoundexInfo::try_from(value.exinfo)?,
                subsoundindex: value.subsoundindex,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_SOUND_INFO> for SoundInfo {
    fn into(self) -> ffi::FMOD_STUDIO_SOUND_INFO {
        ffi::FMOD_STUDIO_SOUND_INFO {
            name_or_data: self.name_or_data.as_ptr().cast(),
            mode: self.mode,
            exinfo: self.exinfo.into(),
            subsoundindex: self.subsoundindex,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub commandname: String,
    pub parentcommandindex: i32,
    pub framenumber: i32,
    pub frametime: f32,
    pub instancetype: InstanceType,
    pub outputtype: InstanceType,
    pub instancehandle: u32,
    pub outputhandle: u32,
}

impl TryFrom<ffi::FMOD_STUDIO_COMMAND_INFO> for CommandInfo {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_COMMAND_INFO) -> Result<Self, Self::Error> {
        unsafe {
            Ok(CommandInfo {
                commandname: to_string!(value.commandname)?,
                parentcommandindex: value.parentcommandindex,
                framenumber: value.framenumber,
                frametime: value.frametime,
                instancetype: InstanceType::from(value.instancetype)?,
                outputtype: InstanceType::from(value.outputtype)?,
                instancehandle: value.instancehandle,
                outputhandle: value.outputhandle,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_COMMAND_INFO> for CommandInfo {
    fn into(self) -> ffi::FMOD_STUDIO_COMMAND_INFO {
        ffi::FMOD_STUDIO_COMMAND_INFO {
            commandname: self.commandname.as_ptr().cast(),
            parentcommandindex: self.parentcommandindex,
            framenumber: self.framenumber,
            frametime: self.frametime,
            instancetype: self.instancetype.into(),
            outputtype: self.outputtype.into(),
            instancehandle: self.instancehandle,
            outputhandle: self.outputhandle,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub exclusive: i32,
    pub inclusive: i32,
    pub sampledata: i32,
}

impl TryFrom<ffi::FMOD_STUDIO_MEMORY_USAGE> for MemoryUsage {
    type Error = Error;
    fn try_from(value: ffi::FMOD_STUDIO_MEMORY_USAGE) -> Result<Self, Self::Error> {
        unsafe {
            Ok(MemoryUsage {
                exclusive: value.exclusive,
                inclusive: value.inclusive,
                sampledata: value.sampledata,
            })
        }
    }
}

impl Into<ffi::FMOD_STUDIO_MEMORY_USAGE> for MemoryUsage {
    fn into(self) -> ffi::FMOD_STUDIO_MEMORY_USAGE {
        ffi::FMOD_STUDIO_MEMORY_USAGE {
            exclusive: self.exclusive,
            inclusive: self.inclusive,
            sampledata: self.sampledata,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AsyncReadInfo {
    pub handle: *mut c_void,
    pub offset: u32,
    pub sizebytes: u32,
    pub priority: i32,
    pub userdata: *mut c_void,
    pub buffer: *mut c_void,
    pub bytesread: u32,
    pub done: ffi::FMOD_FILE_ASYNCDONE_FUNC,
}

impl TryFrom<ffi::FMOD_ASYNCREADINFO> for AsyncReadInfo {
    type Error = Error;
    fn try_from(value: ffi::FMOD_ASYNCREADINFO) -> Result<Self, Self::Error> {
        unsafe {
            Ok(AsyncReadInfo {
                handle: value.handle,
                offset: value.offset,
                sizebytes: value.sizebytes,
                priority: value.priority,
                userdata: value.userdata,
                buffer: value.buffer,
                bytesread: value.bytesread,
                done: value.done,
            })
        }
    }
}

impl Into<ffi::FMOD_ASYNCREADINFO> for AsyncReadInfo {
    fn into(self) -> ffi::FMOD_ASYNCREADINFO {
        ffi::FMOD_ASYNCREADINFO {
            handle: self.handle,
            offset: self.offset,
            sizebytes: self.sizebytes,
            priority: self.priority,
            userdata: self.userdata,
            buffer: self.buffer,
            bytesread: self.bytesread,
            done: self.done,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TryFrom<ffi::FMOD_VECTOR> for Vector {
    type Error = Error;
    fn try_from(value: ffi::FMOD_VECTOR) -> Result<Self, Self::Error> {
        unsafe {
            Ok(Vector {
                x: value.x,
                y: value.y,
                z: value.z,
            })
        }
    }
}

impl Into<ffi::FMOD_VECTOR> for Vector {
    fn into(self) -> ffi::FMOD_VECTOR {
        ffi::FMOD_VECTOR {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attributes3d {
    pub position: Vector,
    pub velocity: Vector,
    pub forward: Vector,
    pub up: Vector,
}

impl TryFrom<ffi::FMOD_3D_ATTRIBUTES> for Attributes3d {
    type Error = Error;
    fn try_from(value: ffi::FMOD_3D_ATTRIBUTES) -> Result<Self, Self::Error> {
        unsafe {
            Ok(Attributes3d {
                position: Vector::try_from(value.position)?,
                velocity: Vector::try_from(value.velocity)?,
                forward: Vector::try_from(value.forward)?,
                up: Vector::try_from(value.up)?,
            })
        }
    }
}

impl Into<ffi::FMOD_3D_ATTRIBUTES> for Attributes3d {
    fn into(self) -> ffi::FMOD_3D_ATTRIBUTES {
        ffi::FMOD_3D_ATTRIBUTES {
            position: self.position.into(),
            velocity: self.velocity.into(),
            forward: self.forward.into(),
            up: self.up.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Guid {
    pub data_1: u32,
    pub data_2: u16,
    pub data_3: u16,
    pub data_4: [u8; 8 as usize],
}

impl TryFrom<ffi::FMOD_GUID> for Guid {
    type Error = Error;
    fn try_from(value: ffi::FMOD_GUID) -> Result<Self, Self::Error> {
        unsafe {
            Ok(Guid {
                data_1: value.Data1,
                data_2: value.Data2,
                data_3: value.Data3,
                data_4: value.Data4,
            })
        }
    }
}

impl Into<ffi::FMOD_GUID> for Guid {
    fn into(self) -> ffi::FMOD_GUID {
        ffi::FMOD_GUID {
            Data1: self.data_1,
            Data2: self.data_2,
            Data3: self.data_3,
            Data4: self.data_4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PluginList {
    pub type_: PluginType,
    pub description: *mut c_void,
}

impl TryFrom<ffi::FMOD_PLUGINLIST> for PluginList {
    type Error = Error;
    fn try_from(value: ffi::FMOD_PLUGINLIST) -> Result<Self, Self::Error> {
        unsafe {
            Ok(PluginList {
                type_: PluginType::from(value.type_)?,
                description: value.description,
            })
        }
    }
}

impl Into<ffi::FMOD_PLUGINLIST> for PluginList {
    fn into(self) -> ffi::FMOD_PLUGINLIST {
        ffi::FMOD_PLUGINLIST {
            type_: self.type_.into(),
            description: self.description,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdvancedSettings {
    pub max_mpeg_codecs: i32,
    pub max_adpcm_codecs: i32,
    pub max_xma_codecs: i32,
    pub max_vorbis_codecs: i32,
    pub max_at_9_codecs: i32,
    pub max_fadpcm_codecs: i32,
    pub max_pcm_codecs: i32,
    pub asio_num_channels: i32,
    pub asio_channel_list: Vec<String>,
    pub asio_speaker_list: Vec<Speaker>,
    pub vol_0_virtualvol: f32,
    pub default_decode_buffer_size: u32,
    pub profile_port: u16,
    pub geometry_max_fade_time: u32,
    pub distance_filter_center_freq: f32,
    pub reverb_3_d_instance: i32,
    pub dsp_buffer_pool_size: i32,
    pub resampler_method: DspResampler,
    pub random_seed: u32,
    pub max_convolution_threads: i32,
    pub max_opus_codecs: i32,
}

impl TryFrom<ffi::FMOD_ADVANCEDSETTINGS> for AdvancedSettings {
    type Error = Error;
    fn try_from(value: ffi::FMOD_ADVANCEDSETTINGS) -> Result<Self, Self::Error> {
        unsafe {
            Ok(AdvancedSettings {
                max_mpeg_codecs: value.maxMPEGCodecs,
                max_adpcm_codecs: value.maxADPCMCodecs,
                max_xma_codecs: value.maxXMACodecs,
                max_vorbis_codecs: value.maxVorbisCodecs,
                max_at_9_codecs: value.maxAT9Codecs,
                max_fadpcm_codecs: value.maxFADPCMCodecs,
                max_pcm_codecs: value.maxPCMCodecs,
                asio_num_channels: value.ASIONumChannels,
                asio_channel_list: to_vec!(
                    value.ASIOChannelList,
                    value.ASIONumChannels,
                    |ptr| to_string!(ptr)
                )?,
                asio_speaker_list: to_vec!(
                    value.ASIOSpeakerList,
                    value.ASIONumChannels,
                    Speaker::from
                )?,
                vol_0_virtualvol: value.vol0virtualvol,
                default_decode_buffer_size: value.defaultDecodeBufferSize,
                profile_port: value.profilePort,
                geometry_max_fade_time: value.geometryMaxFadeTime,
                distance_filter_center_freq: value.distanceFilterCenterFreq,
                reverb_3_d_instance: value.reverb3Dinstance,
                dsp_buffer_pool_size: value.DSPBufferPoolSize,
                resampler_method: DspResampler::from(value.resamplerMethod)?,
                random_seed: value.randomSeed,
                max_convolution_threads: value.maxConvolutionThreads,
                max_opus_codecs: value.maxOpusCodecs,
            })
        }
    }
}

impl Into<ffi::FMOD_ADVANCEDSETTINGS> for AdvancedSettings {
    fn into(self) -> ffi::FMOD_ADVANCEDSETTINGS {
        ffi::FMOD_ADVANCEDSETTINGS {
            cbSize: size_of::<ffi::FMOD_ADVANCEDSETTINGS>() as i32,
            maxMPEGCodecs: self.max_mpeg_codecs,
            maxADPCMCodecs: self.max_adpcm_codecs,
            maxXMACodecs: self.max_xma_codecs,
            maxVorbisCodecs: self.max_vorbis_codecs,
            maxAT9Codecs: self.max_at_9_codecs,
            maxFADPCMCodecs: self.max_fadpcm_codecs,
            maxPCMCodecs: self.max_pcm_codecs,
            ASIONumChannels: self.asio_num_channels,
            ASIOChannelList: self
                .asio_channel_list
                .into_iter()
                .map(|val| val.as_ptr())
                .collect::<Vec<_>>()
                .as_mut_ptr()
                .cast(),
            ASIOSpeakerList: self
                .asio_speaker_list
                .into_iter()
                .map(|val| val.into())
                .collect::<Vec<_>>()
                .as_mut_ptr(),
            vol0virtualvol: self.vol_0_virtualvol,
            defaultDecodeBufferSize: self.default_decode_buffer_size,
            profilePort: self.profile_port,
            geometryMaxFadeTime: self.geometry_max_fade_time,
            distanceFilterCenterFreq: self.distance_filter_center_freq,
            reverb3Dinstance: self.reverb_3_d_instance,
            DSPBufferPoolSize: self.dsp_buffer_pool_size,
            resamplerMethod: self.resampler_method.into(),
            randomSeed: self.random_seed,
            maxConvolutionThreads: self.max_convolution_threads,
            maxOpusCodecs: self.max_opus_codecs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub type_: TagType,
    pub datatype: TagDataType,
    pub name: String,
    pub data: *mut c_void,
    pub datalen: u32,
    pub updated: ffi::FMOD_BOOL,
}

impl TryFrom<ffi::FMOD_TAG> for Tag {
    type Error = Error;
    fn try_from(value: ffi::FMOD_TAG) -> Result<Self, Self::Error> {
        unsafe {
            Ok(Tag {
                type_: TagType::from(value.type_)?,
                datatype: TagDataType::from(value.datatype)?,
                name: to_string!(value.name)?,
                data: value.data,
                datalen: value.datalen,
                updated: value.updated,
            })
        }
    }
}

impl Into<ffi::FMOD_TAG> for Tag {
    fn into(self) -> ffi::FMOD_TAG {
        ffi::FMOD_TAG {
            type_: self.type_.into(),
            datatype: self.datatype.into(),
            name: self.name.as_ptr() as *mut _,
            data: self.data,
            datalen: self.datalen,
            updated: self.updated,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateSoundexInfo {
    pub length: u32,
    pub fileoffset: u32,
    pub numchannels: i32,
    pub defaultfrequency: i32,
    pub format: SoundFormat,
    pub decodebuffersize: u32,
    pub initialsubsound: i32,
    pub numsubsounds: i32,
    pub inclusionlist: Vec<i32>,
    pub inclusionlistnum: i32,
    pub pcmreadcallback: ffi::FMOD_SOUND_PCMREAD_CALLBACK,
    pub pcmsetposcallback: ffi::FMOD_SOUND_PCMSETPOS_CALLBACK,
    pub nonblockcallback: ffi::FMOD_SOUND_NONBLOCK_CALLBACK,
    pub dlsname: String,
    pub encryptionkey: String,
    pub maxpolyphony: i32,
    pub userdata: *mut c_void,
    pub suggestedsoundtype: SoundType,
    pub fileuseropen: ffi::FMOD_FILE_OPEN_CALLBACK,
    pub fileuserclose: ffi::FMOD_FILE_CLOSE_CALLBACK,
    pub fileuserread: ffi::FMOD_FILE_READ_CALLBACK,
    pub fileuserseek: ffi::FMOD_FILE_SEEK_CALLBACK,
    pub fileuserasyncread: ffi::FMOD_FILE_ASYNCREAD_CALLBACK,
    pub fileuserasynccancel: ffi::FMOD_FILE_ASYNCCANCEL_CALLBACK,
    pub fileuserdata: *mut c_void,
    pub filebuffersize: i32,
    pub channelorder: ChannelOrder,
    pub initialsoundgroup: SoundGroup,
    pub initialseekposition: u32,
    pub initialseekpostype: ffi::FMOD_TIMEUNIT,
    pub ignoresetfilesystem: i32,
    pub audioqueuepolicy: u32,
    pub minmidigranularity: u32,
    pub nonblockthreadid: i32,
    pub fsbguid: Guid,
}

impl TryFrom<ffi::FMOD_CREATESOUNDEXINFO> for CreateSoundexInfo {
    type Error = Error;
    fn try_from(value: ffi::FMOD_CREATESOUNDEXINFO) -> Result<Self, Self::Error> {
        unsafe {
            Ok(CreateSoundexInfo {
                length: value.length,
                fileoffset: value.fileoffset,
                numchannels: value.numchannels,
                defaultfrequency: value.defaultfrequency,
                format: SoundFormat::from(value.format)?,
                decodebuffersize: value.decodebuffersize,
                initialsubsound: value.initialsubsound,
                numsubsounds: value.numsubsounds,
                inclusionlist: to_vec!(value.inclusionlist, value.inclusionlistnum),
                inclusionlistnum: value.inclusionlistnum,
                pcmreadcallback: value.pcmreadcallback,
                pcmsetposcallback: value.pcmsetposcallback,
                nonblockcallback: value.nonblockcallback,
                dlsname: to_string!(value.dlsname)?,
                encryptionkey: to_string!(value.encryptionkey)?,
                maxpolyphony: value.maxpolyphony,
                userdata: value.userdata,
                suggestedsoundtype: SoundType::from(value.suggestedsoundtype)?,
                fileuseropen: value.fileuseropen,
                fileuserclose: value.fileuserclose,
                fileuserread: value.fileuserread,
                fileuserseek: value.fileuserseek,
                fileuserasyncread: value.fileuserasyncread,
                fileuserasynccancel: value.fileuserasynccancel,
                fileuserdata: value.fileuserdata,
                filebuffersize: value.filebuffersize,
                channelorder: ChannelOrder::from(value.channelorder)?,
                initialsoundgroup: SoundGroup::from(value.initialsoundgroup),
                initialseekposition: value.initialseekposition,
                initialseekpostype: value.initialseekpostype,
                ignoresetfilesystem: value.ignoresetfilesystem,
                audioqueuepolicy: value.audioqueuepolicy,
                minmidigranularity: value.minmidigranularity,
                nonblockthreadid: value.nonblockthreadid,
                fsbguid: Guid::try_from(*value.fsbguid)?,
            })
        }
    }
}

impl Into<ffi::FMOD_CREATESOUNDEXINFO> for CreateSoundexInfo {
    fn into(self) -> ffi::FMOD_CREATESOUNDEXINFO {
        ffi::FMOD_CREATESOUNDEXINFO {
            cbsize: size_of::<ffi::FMOD_CREATESOUNDEXINFO>() as i32,
            length: self.length,
            fileoffset: self.fileoffset,
            numchannels: self.numchannels,
            defaultfrequency: self.defaultfrequency,
            format: self.format.into(),
            decodebuffersize: self.decodebuffersize,
            initialsubsound: self.initialsubsound,
            numsubsounds: self.numsubsounds,
            inclusionlist: self.inclusionlist.as_ptr() as *mut _,
            inclusionlistnum: self.inclusionlistnum,
            pcmreadcallback: self.pcmreadcallback,
            pcmsetposcallback: self.pcmsetposcallback,
            nonblockcallback: self.nonblockcallback,
            dlsname: self.dlsname.as_ptr().cast(),
            encryptionkey: self.encryptionkey.as_ptr().cast(),
            maxpolyphony: self.maxpolyphony,
            userdata: self.userdata,
            suggestedsoundtype: self.suggestedsoundtype.into(),
            fileuseropen: self.fileuseropen,
            fileuserclose: self.fileuserclose,
            fileuserread: self.fileuserread,
            fileuserseek: self.fileuserseek,
            fileuserasyncread: self.fileuserasyncread,
            fileuserasynccancel: self.fileuserasynccancel,
            fileuserdata: self.fileuserdata,
            filebuffersize: self.filebuffersize,
            channelorder: self.channelorder.into(),
            initialsoundgroup: self.initialsoundgroup.as_mut_ptr(),
            initialseekposition: self.initialseekposition,
            initialseekpostype: self.initialseekpostype,
            ignoresetfilesystem: self.ignoresetfilesystem,
            audioqueuepolicy: self.audioqueuepolicy,
            minmidigranularity: self.minmidigranularity,
            nonblockthreadid: self.nonblockthreadid,
            fsbguid: &mut self.fsbguid.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReverbProperties {
    pub decay_time: f32,
    pub early_delay: f32,
    pub late_delay: f32,
    pub hf_reference: f32,
    pub hf_decay_ratio: f32,
    pub diffusion: f32,
    pub density: f32,
    pub low_shelf_frequency: f32,
    pub low_shelf_gain: f32,
    pub high_cut: f32,
    pub early_late_mix: f32,
    pub wet_level: f32,
}

impl ReverbProperties {
    #[inline]
    pub fn off() -> Self {
        Self::try_from(ffi::FMOD_PRESET_OFF).unwrap()
    }
    #[inline]
    pub fn generic() -> Self {
        Self::try_from(ffi::FMOD_PRESET_GENERIC).unwrap()
    }
    #[inline]
    pub fn paddedcell() -> Self {
        Self::try_from(ffi::FMOD_PRESET_PADDEDCELL).unwrap()
    }
    #[inline]
    pub fn room() -> Self {
        Self::try_from(ffi::FMOD_PRESET_ROOM).unwrap()
    }
    #[inline]
    pub fn bathroom() -> Self {
        Self::try_from(ffi::FMOD_PRESET_BATHROOM).unwrap()
    }
    #[inline]
    pub fn livingroom() -> Self {
        Self::try_from(ffi::FMOD_PRESET_LIVINGROOM).unwrap()
    }
    #[inline]
    pub fn stoneroom() -> Self {
        Self::try_from(ffi::FMOD_PRESET_STONEROOM).unwrap()
    }
    #[inline]
    pub fn auditorium() -> Self {
        Self::try_from(ffi::FMOD_PRESET_AUDITORIUM).unwrap()
    }
    #[inline]
    pub fn concerthall() -> Self {
        Self::try_from(ffi::FMOD_PRESET_CONCERTHALL).unwrap()
    }
    #[inline]
    pub fn cave() -> Self {
        Self::try_from(ffi::FMOD_PRESET_CAVE).unwrap()
    }
    #[inline]
    pub fn arena() -> Self {
        Self::try_from(ffi::FMOD_PRESET_ARENA).unwrap()
    }
    #[inline]
    pub fn hangar() -> Self {
        Self::try_from(ffi::FMOD_PRESET_HANGAR).unwrap()
    }
    #[inline]
    pub fn carpettedhallway() -> Self {
        Self::try_from(ffi::FMOD_PRESET_CARPETTEDHALLWAY).unwrap()
    }
    #[inline]
    pub fn hallway() -> Self {
        Self::try_from(ffi::FMOD_PRESET_HALLWAY).unwrap()
    }
    #[inline]
    pub fn stonecorridor() -> Self {
        Self::try_from(ffi::FMOD_PRESET_STONECORRIDOR).unwrap()
    }
    #[inline]
    pub fn alley() -> Self {
        Self::try_from(ffi::FMOD_PRESET_ALLEY).unwrap()
    }
    #[inline]
    pub fn forest() -> Self {
        Self::try_from(ffi::FMOD_PRESET_FOREST).unwrap()
    }
    #[inline]
    pub fn city() -> Self {
        Self::try_from(ffi::FMOD_PRESET_CITY).unwrap()
    }
    #[inline]
    pub fn mountains() -> Self {
        Self::try_from(ffi::FMOD_PRESET_MOUNTAINS).unwrap()
    }
    #[inline]
    pub fn quarry() -> Self {
        Self::try_from(ffi::FMOD_PRESET_QUARRY).unwrap()
    }
    #[inline]
    pub fn plain() -> Self {
        Self::try_from(ffi::FMOD_PRESET_PLAIN).unwrap()
    }
    #[inline]
    pub fn parkinglot() -> Self {
        Self::try_from(ffi::FMOD_PRESET_PARKINGLOT).unwrap()
    }
    #[inline]
    pub fn sewerpipe() -> Self {
        Self::try_from(ffi::FMOD_PRESET_SEWERPIPE).unwrap()
    }
    #[inline]
    pub fn underwater() -> Self {
        Self::try_from(ffi::FMOD_PRESET_UNDERWATER).unwrap()
    }
}

impl TryFrom<ffi::FMOD_REVERB_PROPERTIES> for ReverbProperties {
    type Error = Error;
    fn try_from(value: ffi::FMOD_REVERB_PROPERTIES) -> Result<Self, Self::Error> {
        unsafe {
            Ok(ReverbProperties {
                decay_time: value.DecayTime,
                early_delay: value.EarlyDelay,
                late_delay: value.LateDelay,
                hf_reference: value.HFReference,
                hf_decay_ratio: value.HFDecayRatio,
                diffusion: value.Diffusion,
                density: value.Density,
                low_shelf_frequency: value.LowShelfFrequency,
                low_shelf_gain: value.LowShelfGain,
                high_cut: value.HighCut,
                early_late_mix: value.EarlyLateMix,
                wet_level: value.WetLevel,
            })
        }
    }
}

impl Into<ffi::FMOD_REVERB_PROPERTIES> for ReverbProperties {
    fn into(self) -> ffi::FMOD_REVERB_PROPERTIES {
        ffi::FMOD_REVERB_PROPERTIES {
            DecayTime: self.decay_time,
            EarlyDelay: self.early_delay,
            LateDelay: self.late_delay,
            HFReference: self.hf_reference,
            HFDecayRatio: self.hf_decay_ratio,
            Diffusion: self.diffusion,
            Density: self.density,
            LowShelfFrequency: self.low_shelf_frequency,
            LowShelfGain: self.low_shelf_gain,
            HighCut: self.high_cut,
            EarlyLateMix: self.early_late_mix,
            WetLevel: self.wet_level,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ErrorCallbackInfo {
    pub result: FmodResult,
    pub instancetype: ErrorCallbackInstancetype,
    pub instance: *mut c_void,
    pub functionname: String,
    pub functionparams: String,
}

impl TryFrom<ffi::FMOD_ERRORCALLBACK_INFO> for ErrorCallbackInfo {
    type Error = Error;
    fn try_from(value: ffi::FMOD_ERRORCALLBACK_INFO) -> Result<Self, Self::Error> {
        unsafe {
            Ok(ErrorCallbackInfo {
                result: FmodResult::from(value.result)?,
                instancetype: ErrorCallbackInstancetype::from(value.instancetype)?,
                instance: value.instance,
                functionname: to_string!(value.functionname)?,
                functionparams: to_string!(value.functionparams)?,
            })
        }
    }
}

impl Into<ffi::FMOD_ERRORCALLBACK_INFO> for ErrorCallbackInfo {
    fn into(self) -> ffi::FMOD_ERRORCALLBACK_INFO {
        ffi::FMOD_ERRORCALLBACK_INFO {
            result: self.result.into(),
            instancetype: self.instancetype.into(),
            instance: self.instance,
            functionname: self.functionname.as_ptr().cast(),
            functionparams: self.functionparams.as_ptr().cast(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CpuUsage {
    pub dsp: f32,
    pub stream: f32,
    pub geometry: f32,
    pub update: f32,
    pub convolution_1: f32,
    pub convolution_2: f32,
}

impl TryFrom<ffi::FMOD_CPU_USAGE> for CpuUsage {
    type Error = Error;
    fn try_from(value: ffi::FMOD_CPU_USAGE) -> Result<Self, Self::Error> {
        unsafe {
            Ok(CpuUsage {
                dsp: value.dsp,
                stream: value.stream,
                geometry: value.geometry,
                update: value.update,
                convolution_1: value.convolution1,
                convolution_2: value.convolution2,
            })
        }
    }
}

impl Into<ffi::FMOD_CPU_USAGE> for CpuUsage {
    fn into(self) -> ffi::FMOD_CPU_USAGE {
        ffi::FMOD_CPU_USAGE {
            dsp: self.dsp,
            stream: self.stream,
            geometry: self.geometry,
            update: self.update,
            convolution1: self.convolution_1,
            convolution2: self.convolution_2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodecDescription {
    pub apiversion: u32,
    pub name: String,
    pub version: u32,
    pub defaultasstream: i32,
    pub timeunits: ffi::FMOD_TIMEUNIT,
    pub open: ffi::FMOD_CODEC_OPEN_CALLBACK,
    pub close: ffi::FMOD_CODEC_CLOSE_CALLBACK,
    pub read: ffi::FMOD_CODEC_READ_CALLBACK,
    pub getlength: ffi::FMOD_CODEC_GETLENGTH_CALLBACK,
    pub setposition: ffi::FMOD_CODEC_SETPOSITION_CALLBACK,
    pub getposition: ffi::FMOD_CODEC_GETPOSITION_CALLBACK,
    pub soundcreate: ffi::FMOD_CODEC_SOUNDCREATE_CALLBACK,
    pub getwaveformat: ffi::FMOD_CODEC_GETWAVEFORMAT_CALLBACK,
}

impl TryFrom<ffi::FMOD_CODEC_DESCRIPTION> for CodecDescription {
    type Error = Error;
    fn try_from(value: ffi::FMOD_CODEC_DESCRIPTION) -> Result<Self, Self::Error> {
        unsafe {
            Ok(CodecDescription {
                apiversion: value.apiversion,
                name: to_string!(value.name)?,
                version: value.version,
                defaultasstream: value.defaultasstream,
                timeunits: value.timeunits,
                open: value.open,
                close: value.close,
                read: value.read,
                getlength: value.getlength,
                setposition: value.setposition,
                getposition: value.getposition,
                soundcreate: value.soundcreate,
                getwaveformat: value.getwaveformat,
            })
        }
    }
}

impl Into<ffi::FMOD_CODEC_DESCRIPTION> for CodecDescription {
    fn into(self) -> ffi::FMOD_CODEC_DESCRIPTION {
        ffi::FMOD_CODEC_DESCRIPTION {
            apiversion: self.apiversion,
            name: self.name.as_ptr().cast(),
            version: self.version,
            defaultasstream: self.defaultasstream,
            timeunits: self.timeunits,
            open: self.open,
            close: self.close,
            read: self.read,
            getlength: self.getlength,
            setposition: self.setposition,
            getposition: self.getposition,
            soundcreate: self.soundcreate,
            getwaveformat: self.getwaveformat,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodecWaveformat {
    pub name: String,
    pub format: SoundFormat,
    pub channels: i32,
    pub frequency: i32,
    pub lengthbytes: u32,
    pub lengthpcm: u32,
    pub pcmblocksize: u32,
    pub loopstart: i32,
    pub loopend: i32,
    pub mode: ffi::FMOD_MODE,
    pub channelmask: ffi::FMOD_CHANNELMASK,
    pub channelorder: ChannelOrder,
    pub peakvolume: f32,
}

impl TryFrom<ffi::FMOD_CODEC_WAVEFORMAT> for CodecWaveformat {
    type Error = Error;
    fn try_from(value: ffi::FMOD_CODEC_WAVEFORMAT) -> Result<Self, Self::Error> {
        unsafe {
            Ok(CodecWaveformat {
                name: to_string!(value.name)?,
                format: SoundFormat::from(value.format)?,
                channels: value.channels,
                frequency: value.frequency,
                lengthbytes: value.lengthbytes,
                lengthpcm: value.lengthpcm,
                pcmblocksize: value.pcmblocksize,
                loopstart: value.loopstart,
                loopend: value.loopend,
                mode: value.mode,
                channelmask: value.channelmask,
                channelorder: ChannelOrder::from(value.channelorder)?,
                peakvolume: value.peakvolume,
            })
        }
    }
}

impl Into<ffi::FMOD_CODEC_WAVEFORMAT> for CodecWaveformat {
    fn into(self) -> ffi::FMOD_CODEC_WAVEFORMAT {
        ffi::FMOD_CODEC_WAVEFORMAT {
            name: self.name.as_ptr().cast(),
            format: self.format.into(),
            channels: self.channels,
            frequency: self.frequency,
            lengthbytes: self.lengthbytes,
            lengthpcm: self.lengthpcm,
            pcmblocksize: self.pcmblocksize,
            loopstart: self.loopstart,
            loopend: self.loopend,
            mode: self.mode,
            channelmask: self.channelmask,
            channelorder: self.channelorder.into(),
            peakvolume: self.peakvolume,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodecStateFunctions {
    pub metadata: ffi::FMOD_CODEC_METADATA_FUNC,
    pub alloc: ffi::FMOD_CODEC_ALLOC_FUNC,
    pub free: ffi::FMOD_CODEC_FREE_FUNC,
    pub log: ffi::FMOD_CODEC_LOG_FUNC,
    pub read: ffi::FMOD_CODEC_FILE_READ_FUNC,
    pub seek: ffi::FMOD_CODEC_FILE_SEEK_FUNC,
    pub tell: ffi::FMOD_CODEC_FILE_TELL_FUNC,
    pub size: ffi::FMOD_CODEC_FILE_SIZE_FUNC,
}

impl TryFrom<ffi::FMOD_CODEC_STATE_FUNCTIONS> for CodecStateFunctions {
    type Error = Error;
    fn try_from(value: ffi::FMOD_CODEC_STATE_FUNCTIONS) -> Result<Self, Self::Error> {
        unsafe {
            Ok(CodecStateFunctions {
                metadata: value.metadata,
                alloc: value.alloc,
                free: value.free,
                log: value.log,
                read: value.read,
                seek: value.seek,
                tell: value.tell,
                size: value.size,
            })
        }
    }
}

impl Into<ffi::FMOD_CODEC_STATE_FUNCTIONS> for CodecStateFunctions {
    fn into(self) -> ffi::FMOD_CODEC_STATE_FUNCTIONS {
        ffi::FMOD_CODEC_STATE_FUNCTIONS {
            metadata: self.metadata,
            alloc: self.alloc,
            free: self.free,
            log: self.log,
            read: self.read,
            seek: self.seek,
            tell: self.tell,
            size: self.size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodecState {
    pub plugindata: *mut c_void,
    pub waveformat: CodecWaveformat,
    pub functions: CodecStateFunctions,
    pub numsubsounds: i32,
}

impl TryFrom<ffi::FMOD_CODEC_STATE> for CodecState {
    type Error = Error;
    fn try_from(value: ffi::FMOD_CODEC_STATE) -> Result<Self, Self::Error> {
        unsafe {
            Ok(CodecState {
                plugindata: value.plugindata,
                waveformat: CodecWaveformat::try_from(*value.waveformat)?,
                functions: CodecStateFunctions::try_from(*value.functions)?,
                numsubsounds: value.numsubsounds,
            })
        }
    }
}

impl Into<ffi::FMOD_CODEC_STATE> for CodecState {
    fn into(self) -> ffi::FMOD_CODEC_STATE {
        ffi::FMOD_CODEC_STATE {
            plugindata: self.plugindata,
            waveformat: &mut self.waveformat.into(),
            functions: &mut self.functions.into(),
            numsubsounds: self.numsubsounds,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputDescription {
    pub apiversion: u32,
    pub name: String,
    pub version: u32,
    pub method: ffi::FMOD_OUTPUT_METHOD,
    pub getnumdrivers: ffi::FMOD_OUTPUT_GETNUMDRIVERS_CALLBACK,
    pub getdriverinfo: ffi::FMOD_OUTPUT_GETDRIVERINFO_CALLBACK,
    pub init: ffi::FMOD_OUTPUT_INIT_CALLBACK,
    pub start: ffi::FMOD_OUTPUT_START_CALLBACK,
    pub stop: ffi::FMOD_OUTPUT_STOP_CALLBACK,
    pub close: ffi::FMOD_OUTPUT_CLOSE_CALLBACK,
    pub update: ffi::FMOD_OUTPUT_UPDATE_CALLBACK,
    pub gethandle: ffi::FMOD_OUTPUT_GETHANDLE_CALLBACK,
    pub mixer: ffi::FMOD_OUTPUT_MIXER_CALLBACK,
    pub object_3_dgetinfo: ffi::FMOD_OUTPUT_OBJECT3DGETINFO_CALLBACK,
    pub object_3_dalloc: ffi::FMOD_OUTPUT_OBJECT3DALLOC_CALLBACK,
    pub object_3_dfree: ffi::FMOD_OUTPUT_OBJECT3DFREE_CALLBACK,
    pub object_3_dupdate: ffi::FMOD_OUTPUT_OBJECT3DUPDATE_CALLBACK,
    pub openport: ffi::FMOD_OUTPUT_OPENPORT_CALLBACK,
    pub closeport: ffi::FMOD_OUTPUT_CLOSEPORT_CALLBACK,
    pub devicelistchanged: ffi::FMOD_OUTPUT_DEVICELISTCHANGED_CALLBACK,
}

impl TryFrom<ffi::FMOD_OUTPUT_DESCRIPTION> for OutputDescription {
    type Error = Error;
    fn try_from(value: ffi::FMOD_OUTPUT_DESCRIPTION) -> Result<Self, Self::Error> {
        unsafe {
            Ok(OutputDescription {
                apiversion: value.apiversion,
                name: to_string!(value.name)?,
                version: value.version,
                method: value.method,
                getnumdrivers: value.getnumdrivers,
                getdriverinfo: value.getdriverinfo,
                init: value.init,
                start: value.start,
                stop: value.stop,
                close: value.close,
                update: value.update,
                gethandle: value.gethandle,
                mixer: value.mixer,
                object_3_dgetinfo: value.object3dgetinfo,
                object_3_dalloc: value.object3dalloc,
                object_3_dfree: value.object3dfree,
                object_3_dupdate: value.object3dupdate,
                openport: value.openport,
                closeport: value.closeport,
                devicelistchanged: value.devicelistchanged,
            })
        }
    }
}

impl Into<ffi::FMOD_OUTPUT_DESCRIPTION> for OutputDescription {
    fn into(self) -> ffi::FMOD_OUTPUT_DESCRIPTION {
        ffi::FMOD_OUTPUT_DESCRIPTION {
            apiversion: self.apiversion,
            name: self.name.as_ptr().cast(),
            version: self.version,
            method: self.method,
            getnumdrivers: self.getnumdrivers,
            getdriverinfo: self.getdriverinfo,
            init: self.init,
            start: self.start,
            stop: self.stop,
            close: self.close,
            update: self.update,
            gethandle: self.gethandle,
            mixer: self.mixer,
            object3dgetinfo: self.object_3_dgetinfo,
            object3dalloc: self.object_3_dalloc,
            object3dfree: self.object_3_dfree,
            object3dupdate: self.object_3_dupdate,
            openport: self.openport,
            closeport: self.closeport,
            devicelistchanged: self.devicelistchanged,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputState {
    pub plugindata: *mut c_void,
    pub readfrommixer: ffi::FMOD_OUTPUT_READFROMMIXER_FUNC,
    pub alloc: ffi::FMOD_OUTPUT_ALLOC_FUNC,
    pub free: ffi::FMOD_OUTPUT_FREE_FUNC,
    pub log: ffi::FMOD_OUTPUT_LOG_FUNC,
    pub copyport: ffi::FMOD_OUTPUT_COPYPORT_FUNC,
    pub requestreset: ffi::FMOD_OUTPUT_REQUESTRESET_FUNC,
}

impl TryFrom<ffi::FMOD_OUTPUT_STATE> for OutputState {
    type Error = Error;
    fn try_from(value: ffi::FMOD_OUTPUT_STATE) -> Result<Self, Self::Error> {
        unsafe {
            Ok(OutputState {
                plugindata: value.plugindata,
                readfrommixer: value.readfrommixer,
                alloc: value.alloc,
                free: value.free,
                log: value.log,
                copyport: value.copyport,
                requestreset: value.requestreset,
            })
        }
    }
}

impl Into<ffi::FMOD_OUTPUT_STATE> for OutputState {
    fn into(self) -> ffi::FMOD_OUTPUT_STATE {
        ffi::FMOD_OUTPUT_STATE {
            plugindata: self.plugindata,
            readfrommixer: self.readfrommixer,
            alloc: self.alloc,
            free: self.free,
            log: self.log,
            copyport: self.copyport,
            requestreset: self.requestreset,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputObject3Dinfo {
    pub buffer: Vec<f32>,
    pub bufferlength: u32,
    pub position: Vector,
    pub gain: f32,
    pub spread: f32,
    pub priority: f32,
}

impl TryFrom<ffi::FMOD_OUTPUT_OBJECT3DINFO> for OutputObject3Dinfo {
    type Error = Error;
    fn try_from(value: ffi::FMOD_OUTPUT_OBJECT3DINFO) -> Result<Self, Self::Error> {
        unsafe {
            Ok(OutputObject3Dinfo {
                buffer: to_vec!(value.buffer, value.bufferlength),
                bufferlength: value.bufferlength,
                position: Vector::try_from(value.position)?,
                gain: value.gain,
                spread: value.spread,
                priority: value.priority,
            })
        }
    }
}

impl Into<ffi::FMOD_OUTPUT_OBJECT3DINFO> for OutputObject3Dinfo {
    fn into(self) -> ffi::FMOD_OUTPUT_OBJECT3DINFO {
        ffi::FMOD_OUTPUT_OBJECT3DINFO {
            buffer: self.buffer.as_ptr() as *mut _,
            bufferlength: self.bufferlength,
            position: self.position.into(),
            gain: self.gain,
            spread: self.spread,
            priority: self.priority,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspBufferArray {
    pub numbuffers: i32,
    pub buffernumchannels: Vec<i32>,
    pub bufferchannelmask: Vec<ffi::FMOD_CHANNELMASK>,
    pub buffers: Vec<f32>,
    pub speakermode: SpeakerMode,
}

impl TryFrom<ffi::FMOD_DSP_BUFFER_ARRAY> for DspBufferArray {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_BUFFER_ARRAY) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspBufferArray {
                numbuffers: value.numbuffers,
                buffernumchannels: to_vec!(value.buffernumchannels, value.numbuffers),
                bufferchannelmask: to_vec!(value.bufferchannelmask, value.numbuffers),
                buffers: to_vec!(value.buffers, value.numbuffers, |ptr| Ok(*ptr))?,
                speakermode: SpeakerMode::from(value.speakermode)?,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_BUFFER_ARRAY> for DspBufferArray {
    fn into(self) -> ffi::FMOD_DSP_BUFFER_ARRAY {
        ffi::FMOD_DSP_BUFFER_ARRAY {
            numbuffers: self.numbuffers,
            buffernumchannels: self.buffernumchannels.as_ptr() as *mut _,
            bufferchannelmask: self.bufferchannelmask.as_ptr() as *mut _,
            buffers: self.buffers.as_ptr() as *mut _,
            speakermode: self.speakermode.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Complex {
    pub real: f32,
    pub imag: f32,
}

impl TryFrom<ffi::FMOD_COMPLEX> for Complex {
    type Error = Error;
    fn try_from(value: ffi::FMOD_COMPLEX) -> Result<Self, Self::Error> {
        unsafe {
            Ok(Complex {
                real: value.real,
                imag: value.imag,
            })
        }
    }
}

impl Into<ffi::FMOD_COMPLEX> for Complex {
    fn into(self) -> ffi::FMOD_COMPLEX {
        ffi::FMOD_COMPLEX {
            real: self.real,
            imag: self.imag,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterFloatMappingPiecewiseLinear {
    pub numpoints: i32,
    pub pointparamvalues: Vec<f32>,
    pub pointpositions: Vec<f32>,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR>
for DspParameterFloatMappingPiecewiseLinear
{
    type Error = Error;
    fn try_from(
        value: ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR,
    ) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterFloatMappingPiecewiseLinear {
                numpoints: value.numpoints,
                pointparamvalues: to_vec!(value.pointparamvalues, value.numpoints),
                pointpositions: to_vec!(value.pointpositions, value.numpoints),
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR>
for DspParameterFloatMappingPiecewiseLinear
{
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR {
        ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR {
            numpoints: self.numpoints,
            pointparamvalues: self.pointparamvalues.as_ptr() as *mut _,
            pointpositions: self.pointpositions.as_ptr() as *mut _,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterFloatMapping {
    pub type_: DspParameterFloatMappingType,
    pub piecewiselinearmapping: DspParameterFloatMappingPiecewiseLinear,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING> for DspParameterFloatMapping {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterFloatMapping {
                type_: DspParameterFloatMappingType::from(value.type_)?,
                piecewiselinearmapping: DspParameterFloatMappingPiecewiseLinear::try_from(
                    value.piecewiselinearmapping,
                )?,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING> for DspParameterFloatMapping {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING {
        ffi::FMOD_DSP_PARAMETER_FLOAT_MAPPING {
            type_: self.type_.into(),
            piecewiselinearmapping: self.piecewiselinearmapping.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterDescFloat {
    pub min: f32,
    pub max: f32,
    pub defaultval: f32,
    pub mapping: DspParameterFloatMapping,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_DESC_FLOAT> for DspParameterDescFloat {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_DESC_FLOAT) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterDescFloat {
                min: value.min,
                max: value.max,
                defaultval: value.defaultval,
                mapping: DspParameterFloatMapping::try_from(value.mapping)?,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_DESC_FLOAT> for DspParameterDescFloat {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_DESC_FLOAT {
        ffi::FMOD_DSP_PARAMETER_DESC_FLOAT {
            min: self.min,
            max: self.max,
            defaultval: self.defaultval,
            mapping: self.mapping.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterDescInt {
    pub min: i32,
    pub max: i32,
    pub defaultval: i32,
    pub goestoinf: ffi::FMOD_BOOL,
    pub valuenames: Vec<String>,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_DESC_INT> for DspParameterDescInt {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_DESC_INT) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterDescInt {
                min: value.min,
                max: value.max,
                defaultval: value.defaultval,
                goestoinf: value.goestoinf,
                valuenames: vec![],
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_DESC_INT> for DspParameterDescInt {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_DESC_INT {
        ffi::FMOD_DSP_PARAMETER_DESC_INT {
            min: self.min,
            max: self.max,
            defaultval: self.defaultval,
            goestoinf: self.goestoinf,
            valuenames: self.valuenames.as_ptr() as *mut _,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterDescBool {
    pub defaultval: ffi::FMOD_BOOL,
    pub valuenames: Vec<String>,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_DESC_BOOL> for DspParameterDescBool {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_DESC_BOOL) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterDescBool {
                defaultval: value.defaultval,
                valuenames: vec![],
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_DESC_BOOL> for DspParameterDescBool {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_DESC_BOOL {
        ffi::FMOD_DSP_PARAMETER_DESC_BOOL {
            defaultval: self.defaultval,
            valuenames: self.valuenames.as_ptr() as *mut _,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterDescData {
    pub datatype: i32,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_DESC_DATA> for DspParameterDescData {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_DESC_DATA) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterDescData {
                datatype: value.datatype,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_DESC_DATA> for DspParameterDescData {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_DESC_DATA {
        ffi::FMOD_DSP_PARAMETER_DESC_DATA {
            datatype: self.datatype,
        }
    }
}

#[derive(Clone)]
pub struct DspParameterDesc {
    pub type_: DspParameterType,
    pub name: [c_char; 16 as usize],
    pub label: [c_char; 16 as usize],
    pub description: String,
    pub union: ffi::FMOD_DSP_PARAMETER_DESC_UNION,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_DESC> for DspParameterDesc {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_DESC) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterDesc {
                type_: DspParameterType::from(value.type_)?,
                name: value.name,
                label: value.label,
                description: to_string!(value.description)?,
                union: value.union,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_DESC> for DspParameterDesc {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_DESC {
        ffi::FMOD_DSP_PARAMETER_DESC {
            type_: self.type_.into(),
            name: self.name,
            label: self.label,
            description: self.description.as_ptr().cast(),
            union: self.union,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterOverallgain {
    pub linear_gain: f32,
    pub linear_gain_additive: f32,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_OVERALLGAIN> for DspParameterOverallgain {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_OVERALLGAIN) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterOverallgain {
                linear_gain: value.linear_gain,
                linear_gain_additive: value.linear_gain_additive,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_OVERALLGAIN> for DspParameterOverallgain {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_OVERALLGAIN {
        ffi::FMOD_DSP_PARAMETER_OVERALLGAIN {
            linear_gain: self.linear_gain,
            linear_gain_additive: self.linear_gain_additive,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterAttributes3d {
    pub relative: Attributes3d,
    pub absolute: Attributes3d,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_3DATTRIBUTES> for DspParameterAttributes3d {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_3DATTRIBUTES) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterAttributes3d {
                relative: Attributes3d::try_from(value.relative)?,
                absolute: Attributes3d::try_from(value.absolute)?,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_3DATTRIBUTES> for DspParameterAttributes3d {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_3DATTRIBUTES {
        ffi::FMOD_DSP_PARAMETER_3DATTRIBUTES {
            relative: self.relative.into(),
            absolute: self.absolute.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterAttributes3dMulti {
    pub numlisteners: i32,
    pub relative: [Attributes3d; ffi::FMOD_MAX_LISTENERS as usize],
    pub weight: [f32; ffi::FMOD_MAX_LISTENERS as usize],
    pub absolute: Attributes3d,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI> for DspParameterAttributes3dMulti {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterAttributes3dMulti {
                numlisteners: value.numlisteners,
                relative: attr3d_array8(
                    value
                        .relative
                        .map(Attributes3d::try_from)
                        .into_iter()
                        .collect::<Result<Vec<Attributes3d>, Error>>()?,
                ),
                weight: value.weight,
                absolute: Attributes3d::try_from(value.absolute)?,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI> for DspParameterAttributes3dMulti {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI {
        ffi::FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI {
            numlisteners: self.numlisteners,
            relative: self.relative.map(Attributes3d::into),
            weight: self.weight,
            absolute: self.absolute.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterAttenuationRange {
    pub min: f32,
    pub max: f32,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_ATTENUATION_RANGE> for DspParameterAttenuationRange {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_ATTENUATION_RANGE) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterAttenuationRange {
                min: value.min,
                max: value.max,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_ATTENUATION_RANGE> for DspParameterAttenuationRange {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_ATTENUATION_RANGE {
        ffi::FMOD_DSP_PARAMETER_ATTENUATION_RANGE {
            min: self.min,
            max: self.max,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterSidechain {
    pub sidechainenable: ffi::FMOD_BOOL,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_SIDECHAIN> for DspParameterSidechain {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_SIDECHAIN) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterSidechain {
                sidechainenable: value.sidechainenable,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_SIDECHAIN> for DspParameterSidechain {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_SIDECHAIN {
        ffi::FMOD_DSP_PARAMETER_SIDECHAIN {
            sidechainenable: self.sidechainenable,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspParameterFft {
    pub length: i32,
    pub spectrum: Vec<Vec<f32>>,
}

impl TryFrom<ffi::FMOD_DSP_PARAMETER_FFT> for DspParameterFft {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_PARAMETER_FFT) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspParameterFft {
                length: value.length,
                spectrum: to_vec!(value.spectrum.as_ptr(), value.numchannels, |ptr| Ok(
                    to_vec!(ptr, value.length)
                ))?,
            })
        }
    }
}

impl TryFrom<Dsp> for DspParameterFft {
    type Error = Error;
    fn try_from(dsp: Dsp) -> Result<Self, Self::Error> {
        match dsp.get_type() {
            Ok(DspType::Fft) => {
                let (ptr, _, _) = dsp.get_parameter_data(ffi::FMOD_DSP_FFT_SPECTRUMDATA, 0)?;
                let fft = unsafe { *(ptr as *const ffi::FMOD_DSP_PARAMETER_FFT) };
                DspParameterFft::try_from(fft)
            }
            _ => Err(Error::NotDspFft),
        }
    }
}

impl Into<ffi::FMOD_DSP_PARAMETER_FFT> for DspParameterFft {
    fn into(self) -> ffi::FMOD_DSP_PARAMETER_FFT {
        ffi::FMOD_DSP_PARAMETER_FFT {
            length: self.length,
            numchannels: self.spectrum.len() as i32,
            spectrum: [null_mut(); 32],
        }
    }
}

#[derive(Clone)]
pub struct DspDescription {
    pub pluginsdkversion: u32,
    pub name: [c_char; 32 as usize],
    pub version: u32,
    pub numinputbuffers: i32,
    pub numoutputbuffers: i32,
    pub create: ffi::FMOD_DSP_CREATE_CALLBACK,
    pub release: ffi::FMOD_DSP_RELEASE_CALLBACK,
    pub reset: ffi::FMOD_DSP_RESET_CALLBACK,
    pub read: ffi::FMOD_DSP_READ_CALLBACK,
    pub process: ffi::FMOD_DSP_PROCESS_CALLBACK,
    pub setposition: ffi::FMOD_DSP_SETPOSITION_CALLBACK,
    pub paramdesc: Vec<DspParameterDesc>,
    pub setparameterfloat: ffi::FMOD_DSP_SETPARAM_FLOAT_CALLBACK,
    pub setparameterint: ffi::FMOD_DSP_SETPARAM_INT_CALLBACK,
    pub setparameterbool: ffi::FMOD_DSP_SETPARAM_BOOL_CALLBACK,
    pub setparameterdata: ffi::FMOD_DSP_SETPARAM_DATA_CALLBACK,
    pub getparameterfloat: ffi::FMOD_DSP_GETPARAM_FLOAT_CALLBACK,
    pub getparameterint: ffi::FMOD_DSP_GETPARAM_INT_CALLBACK,
    pub getparameterbool: ffi::FMOD_DSP_GETPARAM_BOOL_CALLBACK,
    pub getparameterdata: ffi::FMOD_DSP_GETPARAM_DATA_CALLBACK,
    pub shouldiprocess: ffi::FMOD_DSP_SHOULDIPROCESS_CALLBACK,
    pub userdata: *mut c_void,
    pub sys_register: ffi::FMOD_DSP_SYSTEM_REGISTER_CALLBACK,
    pub sys_deregister: ffi::FMOD_DSP_SYSTEM_DEREGISTER_CALLBACK,
    pub sys_mix: ffi::FMOD_DSP_SYSTEM_MIX_CALLBACK,
}

impl TryFrom<ffi::FMOD_DSP_DESCRIPTION> for DspDescription {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_DESCRIPTION) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspDescription {
                pluginsdkversion: value.pluginsdkversion,
                name: value.name,
                version: value.version,
                numinputbuffers: value.numinputbuffers,
                numoutputbuffers: value.numoutputbuffers,
                create: value.create,
                release: value.release,
                reset: value.reset,
                read: value.read,
                process: value.process,
                setposition: value.setposition,
                paramdesc: to_vec!(
                    *value.paramdesc,
                    value.numparameters,
                    DspParameterDesc::try_from
                )?,
                setparameterfloat: value.setparameterfloat,
                setparameterint: value.setparameterint,
                setparameterbool: value.setparameterbool,
                setparameterdata: value.setparameterdata,
                getparameterfloat: value.getparameterfloat,
                getparameterint: value.getparameterint,
                getparameterbool: value.getparameterbool,
                getparameterdata: value.getparameterdata,
                shouldiprocess: value.shouldiprocess,
                userdata: value.userdata,
                sys_register: value.sys_register,
                sys_deregister: value.sys_deregister,
                sys_mix: value.sys_mix,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_DESCRIPTION> for DspDescription {
    fn into(self) -> ffi::FMOD_DSP_DESCRIPTION {
        ffi::FMOD_DSP_DESCRIPTION {
            pluginsdkversion: self.pluginsdkversion,
            name: self.name,
            version: self.version,
            numinputbuffers: self.numinputbuffers,
            numoutputbuffers: self.numoutputbuffers,
            create: self.create,
            release: self.release,
            reset: self.reset,
            read: self.read,
            process: self.process,
            setposition: self.setposition,
            numparameters: self.paramdesc.len() as i32,
            paramdesc: &mut vec_as_mut_ptr(self.paramdesc, |param| param.into()),
            setparameterfloat: self.setparameterfloat,
            setparameterint: self.setparameterint,
            setparameterbool: self.setparameterbool,
            setparameterdata: self.setparameterdata,
            getparameterfloat: self.getparameterfloat,
            getparameterint: self.getparameterint,
            getparameterbool: self.getparameterbool,
            getparameterdata: self.getparameterdata,
            shouldiprocess: self.shouldiprocess,
            userdata: self.userdata,
            sys_register: self.sys_register,
            sys_deregister: self.sys_deregister,
            sys_mix: self.sys_mix,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspStateDftFunctions {
    pub fftreal: ffi::FMOD_DSP_DFT_FFTREAL_FUNC,
    pub inversefftreal: ffi::FMOD_DSP_DFT_IFFTREAL_FUNC,
}

impl TryFrom<ffi::FMOD_DSP_STATE_DFT_FUNCTIONS> for DspStateDftFunctions {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_STATE_DFT_FUNCTIONS) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspStateDftFunctions {
                fftreal: value.fftreal,
                inversefftreal: value.inversefftreal,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_STATE_DFT_FUNCTIONS> for DspStateDftFunctions {
    fn into(self) -> ffi::FMOD_DSP_STATE_DFT_FUNCTIONS {
        ffi::FMOD_DSP_STATE_DFT_FUNCTIONS {
            fftreal: self.fftreal,
            inversefftreal: self.inversefftreal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspStatePanFunctions {
    pub summonomatrix: ffi::FMOD_DSP_PAN_SUMMONOMATRIX_FUNC,
    pub sumstereomatrix: ffi::FMOD_DSP_PAN_SUMSTEREOMATRIX_FUNC,
    pub sumsurroundmatrix: ffi::FMOD_DSP_PAN_SUMSURROUNDMATRIX_FUNC,
    pub summonotosurroundmatrix: ffi::FMOD_DSP_PAN_SUMMONOTOSURROUNDMATRIX_FUNC,
    pub sumstereotosurroundmatrix: ffi::FMOD_DSP_PAN_SUMSTEREOTOSURROUNDMATRIX_FUNC,
    pub getrolloffgain: ffi::FMOD_DSP_PAN_GETROLLOFFGAIN_FUNC,
}

impl TryFrom<ffi::FMOD_DSP_STATE_PAN_FUNCTIONS> for DspStatePanFunctions {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_STATE_PAN_FUNCTIONS) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspStatePanFunctions {
                summonomatrix: value.summonomatrix,
                sumstereomatrix: value.sumstereomatrix,
                sumsurroundmatrix: value.sumsurroundmatrix,
                summonotosurroundmatrix: value.summonotosurroundmatrix,
                sumstereotosurroundmatrix: value.sumstereotosurroundmatrix,
                getrolloffgain: value.getrolloffgain,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_STATE_PAN_FUNCTIONS> for DspStatePanFunctions {
    fn into(self) -> ffi::FMOD_DSP_STATE_PAN_FUNCTIONS {
        ffi::FMOD_DSP_STATE_PAN_FUNCTIONS {
            summonomatrix: self.summonomatrix,
            sumstereomatrix: self.sumstereomatrix,
            sumsurroundmatrix: self.sumsurroundmatrix,
            summonotosurroundmatrix: self.summonotosurroundmatrix,
            sumstereotosurroundmatrix: self.sumstereotosurroundmatrix,
            getrolloffgain: self.getrolloffgain,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspStateFunctions {
    pub alloc: ffi::FMOD_DSP_ALLOC_FUNC,
    pub realloc: ffi::FMOD_DSP_REALLOC_FUNC,
    pub free: ffi::FMOD_DSP_FREE_FUNC,
    pub getsamplerate: ffi::FMOD_DSP_GETSAMPLERATE_FUNC,
    pub getblocksize: ffi::FMOD_DSP_GETBLOCKSIZE_FUNC,
    pub dft: DspStateDftFunctions,
    pub pan: DspStatePanFunctions,
    pub getspeakermode: ffi::FMOD_DSP_GETSPEAKERMODE_FUNC,
    pub getclock: ffi::FMOD_DSP_GETCLOCK_FUNC,
    pub getlistenerattributes: ffi::FMOD_DSP_GETLISTENERATTRIBUTES_FUNC,
    pub log: ffi::FMOD_DSP_LOG_FUNC,
    pub getuserdata: ffi::FMOD_DSP_GETUSERDATA_FUNC,
}

impl TryFrom<ffi::FMOD_DSP_STATE_FUNCTIONS> for DspStateFunctions {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_STATE_FUNCTIONS) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspStateFunctions {
                alloc: value.alloc,
                realloc: value.realloc,
                free: value.free,
                getsamplerate: value.getsamplerate,
                getblocksize: value.getblocksize,
                dft: DspStateDftFunctions::try_from(*value.dft)?,
                pan: DspStatePanFunctions::try_from(*value.pan)?,
                getspeakermode: value.getspeakermode,
                getclock: value.getclock,
                getlistenerattributes: value.getlistenerattributes,
                log: value.log,
                getuserdata: value.getuserdata,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_STATE_FUNCTIONS> for DspStateFunctions {
    fn into(self) -> ffi::FMOD_DSP_STATE_FUNCTIONS {
        ffi::FMOD_DSP_STATE_FUNCTIONS {
            alloc: self.alloc,
            realloc: self.realloc,
            free: self.free,
            getsamplerate: self.getsamplerate,
            getblocksize: self.getblocksize,
            dft: &mut self.dft.into(),
            pan: &mut self.pan.into(),
            getspeakermode: self.getspeakermode,
            getclock: self.getclock,
            getlistenerattributes: self.getlistenerattributes,
            log: self.log,
            getuserdata: self.getuserdata,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspState {
    pub instance: *mut c_void,
    pub plugindata: *mut c_void,
    pub channelmask: ffi::FMOD_CHANNELMASK,
    pub source_speakermode: SpeakerMode,
    pub sidechaindata: Vec<f32>,
    pub sidechainchannels: i32,
    pub functions: DspStateFunctions,
    pub systemobject: i32,
}

impl TryFrom<ffi::FMOD_DSP_STATE> for DspState {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_STATE) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspState {
                instance: value.instance,
                plugindata: value.plugindata,
                channelmask: value.channelmask,
                source_speakermode: SpeakerMode::from(value.source_speakermode)?,
                sidechaindata: to_vec!(value.sidechaindata, value.sidechainchannels),
                sidechainchannels: value.sidechainchannels,
                functions: DspStateFunctions::try_from(*value.functions)?,
                systemobject: value.systemobject,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_STATE> for DspState {
    fn into(self) -> ffi::FMOD_DSP_STATE {
        ffi::FMOD_DSP_STATE {
            instance: self.instance,
            plugindata: self.plugindata,
            channelmask: self.channelmask,
            source_speakermode: self.source_speakermode.into(),
            sidechaindata: self.sidechaindata.as_ptr() as *mut _,
            sidechainchannels: self.sidechainchannels,
            functions: &mut self.functions.into(),
            systemobject: self.systemobject,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspMeteringInfo {
    pub numsamples: i32,
    pub peaklevel: [f32; 32 as usize],
    pub rmslevel: [f32; 32 as usize],
    pub numchannels: i16,
}

impl TryFrom<ffi::FMOD_DSP_METERING_INFO> for DspMeteringInfo {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_METERING_INFO) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspMeteringInfo {
                numsamples: value.numsamples,
                peaklevel: value.peaklevel,
                rmslevel: value.rmslevel,
                numchannels: value.numchannels,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_METERING_INFO> for DspMeteringInfo {
    fn into(self) -> ffi::FMOD_DSP_METERING_INFO {
        ffi::FMOD_DSP_METERING_INFO {
            numsamples: self.numsamples,
            peaklevel: self.peaklevel,
            rmslevel: self.rmslevel,
            numchannels: self.numchannels,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspLoudnessMeterInfoType {
    pub momentaryloudness: f32,
    pub shorttermloudness: f32,
    pub integratedloudness: f32,
    pub loudness_10_thpercentile: f32,
    pub loudness_95_thpercentile: f32,
    pub loudnesshistogram: [f32; ffi::FMOD_DSP_LOUDNESS_METER_HISTOGRAM_SAMPLES as usize],
    pub maxtruepeak: f32,
    pub maxmomentaryloudness: f32,
}

impl TryFrom<ffi::FMOD_DSP_LOUDNESS_METER_INFO_TYPE> for DspLoudnessMeterInfoType {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_LOUDNESS_METER_INFO_TYPE) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspLoudnessMeterInfoType {
                momentaryloudness: value.momentaryloudness,
                shorttermloudness: value.shorttermloudness,
                integratedloudness: value.integratedloudness,
                loudness_10_thpercentile: value.loudness10thpercentile,
                loudness_95_thpercentile: value.loudness95thpercentile,
                loudnesshistogram: value.loudnesshistogram,
                maxtruepeak: value.maxtruepeak,
                maxmomentaryloudness: value.maxmomentaryloudness,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_LOUDNESS_METER_INFO_TYPE> for DspLoudnessMeterInfoType {
    fn into(self) -> ffi::FMOD_DSP_LOUDNESS_METER_INFO_TYPE {
        ffi::FMOD_DSP_LOUDNESS_METER_INFO_TYPE {
            momentaryloudness: self.momentaryloudness,
            shorttermloudness: self.shorttermloudness,
            integratedloudness: self.integratedloudness,
            loudness10thpercentile: self.loudness_10_thpercentile,
            loudness95thpercentile: self.loudness_95_thpercentile,
            loudnesshistogram: self.loudnesshistogram,
            maxtruepeak: self.maxtruepeak,
            maxmomentaryloudness: self.maxmomentaryloudness,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DspLoudnessMeterWeightingType {
    pub channelweight: [f32; 32 as usize],
}

impl TryFrom<ffi::FMOD_DSP_LOUDNESS_METER_WEIGHTING_TYPE> for DspLoudnessMeterWeightingType {
    type Error = Error;
    fn try_from(value: ffi::FMOD_DSP_LOUDNESS_METER_WEIGHTING_TYPE) -> Result<Self, Self::Error> {
        unsafe {
            Ok(DspLoudnessMeterWeightingType {
                channelweight: value.channelweight,
            })
        }
    }
}

impl Into<ffi::FMOD_DSP_LOUDNESS_METER_WEIGHTING_TYPE> for DspLoudnessMeterWeightingType {
    fn into(self) -> ffi::FMOD_DSP_LOUDNESS_METER_WEIGHTING_TYPE {
        ffi::FMOD_DSP_LOUDNESS_METER_WEIGHTING_TYPE {
            channelweight: self.channelweight,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Channel {
    pointer: *mut ffi::FMOD_CHANNEL,
}

impl Channel {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_CHANNEL) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_CHANNEL {
        self.pointer
    }
    pub fn get_system_object(&self) -> Result<System, Error> {
        unsafe {
            let mut system = null_mut();
            match ffi::FMOD_Channel_GetSystemObject(self.pointer, &mut system) {
                ffi::FMOD_OK => Ok(System::from(system)),
                error => Err(err_fmod!("FMOD_Channel_GetSystemObject", error)),
            }
        }
    }
    pub fn stop(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_Stop(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Stop", error)),
            }
        }
    }
    pub fn set_paused(&self, paused: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetPaused(self.pointer, from_bool!(paused)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetPaused", error)),
            }
        }
    }
    pub fn get_paused(&self) -> Result<bool, Error> {
        unsafe {
            let mut paused = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Channel_GetPaused(self.pointer, &mut paused) {
                ffi::FMOD_OK => Ok(to_bool!(paused)),
                error => Err(err_fmod!("FMOD_Channel_GetPaused", error)),
            }
        }
    }
    pub fn set_volume(&self, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetVolume(self.pointer, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetVolume", error)),
            }
        }
    }
    pub fn get_volume(&self) -> Result<f32, Error> {
        unsafe {
            let mut volume = f32::default();
            match ffi::FMOD_Channel_GetVolume(self.pointer, &mut volume) {
                ffi::FMOD_OK => Ok(volume),
                error => Err(err_fmod!("FMOD_Channel_GetVolume", error)),
            }
        }
    }
    pub fn set_volume_ramp(&self, ramp: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetVolumeRamp(self.pointer, from_bool!(ramp)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetVolumeRamp", error)),
            }
        }
    }
    pub fn get_volume_ramp(&self) -> Result<bool, Error> {
        unsafe {
            let mut ramp = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Channel_GetVolumeRamp(self.pointer, &mut ramp) {
                ffi::FMOD_OK => Ok(to_bool!(ramp)),
                error => Err(err_fmod!("FMOD_Channel_GetVolumeRamp", error)),
            }
        }
    }
    pub fn get_audibility(&self) -> Result<f32, Error> {
        unsafe {
            let mut audibility = f32::default();
            match ffi::FMOD_Channel_GetAudibility(self.pointer, &mut audibility) {
                ffi::FMOD_OK => Ok(audibility),
                error => Err(err_fmod!("FMOD_Channel_GetAudibility", error)),
            }
        }
    }
    pub fn set_pitch(&self, pitch: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetPitch(self.pointer, pitch) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetPitch", error)),
            }
        }
    }
    pub fn get_pitch(&self) -> Result<f32, Error> {
        unsafe {
            let mut pitch = f32::default();
            match ffi::FMOD_Channel_GetPitch(self.pointer, &mut pitch) {
                ffi::FMOD_OK => Ok(pitch),
                error => Err(err_fmod!("FMOD_Channel_GetPitch", error)),
            }
        }
    }
    pub fn set_mute(&self, mute: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetMute(self.pointer, from_bool!(mute)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetMute", error)),
            }
        }
    }
    pub fn get_mute(&self) -> Result<bool, Error> {
        unsafe {
            let mut mute = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Channel_GetMute(self.pointer, &mut mute) {
                ffi::FMOD_OK => Ok(to_bool!(mute)),
                error => Err(err_fmod!("FMOD_Channel_GetMute", error)),
            }
        }
    }
    pub fn set_reverb_properties(&self, instance: i32, wet: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetReverbProperties(self.pointer, instance, wet) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetReverbProperties", error)),
            }
        }
    }
    pub fn get_reverb_properties(&self, instance: i32) -> Result<f32, Error> {
        unsafe {
            let mut wet = f32::default();
            match ffi::FMOD_Channel_GetReverbProperties(self.pointer, instance, &mut wet) {
                ffi::FMOD_OK => Ok(wet),
                error => Err(err_fmod!("FMOD_Channel_GetReverbProperties", error)),
            }
        }
    }
    pub fn set_low_pass_gain(&self, gain: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetLowPassGain(self.pointer, gain) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetLowPassGain", error)),
            }
        }
    }
    pub fn get_low_pass_gain(&self) -> Result<f32, Error> {
        unsafe {
            let mut gain = f32::default();
            match ffi::FMOD_Channel_GetLowPassGain(self.pointer, &mut gain) {
                ffi::FMOD_OK => Ok(gain),
                error => Err(err_fmod!("FMOD_Channel_GetLowPassGain", error)),
            }
        }
    }
    pub fn set_mode(&self, mode: ffi::FMOD_MODE) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetMode(self.pointer, mode) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetMode", error)),
            }
        }
    }
    pub fn get_mode(&self) -> Result<ffi::FMOD_MODE, Error> {
        unsafe {
            let mut mode = ffi::FMOD_MODE::default();
            match ffi::FMOD_Channel_GetMode(self.pointer, &mut mode) {
                ffi::FMOD_OK => Ok(mode),
                error => Err(err_fmod!("FMOD_Channel_GetMode", error)),
            }
        }
    }
    pub fn set_callback(&self, callback: ffi::FMOD_CHANNELCONTROL_CALLBACK) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetCallback(self.pointer, callback) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetCallback", error)),
            }
        }
    }
    pub fn is_playing(&self) -> Result<bool, Error> {
        unsafe {
            let mut isplaying = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Channel_IsPlaying(self.pointer, &mut isplaying) {
                ffi::FMOD_OK => Ok(to_bool!(isplaying)),
                error => Err(err_fmod!("FMOD_Channel_IsPlaying", error)),
            }
        }
    }
    pub fn set_pan(&self, pan: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetPan(self.pointer, pan) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetPan", error)),
            }
        }
    }
    pub fn set_mix_levels_output(
        &self,
        frontleft: f32,
        frontright: f32,
        center: f32,
        lfe: f32,
        surroundleft: f32,
        surroundright: f32,
        backleft: f32,
        backright: f32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetMixLevelsOutput(
                self.pointer,
                frontleft,
                frontright,
                center,
                lfe,
                surroundleft,
                surroundright,
                backleft,
                backright,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetMixLevelsOutput", error)),
            }
        }
    }
    pub fn set_mix_levels_input(&self, levels: *mut f32, numlevels: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetMixLevelsInput(self.pointer, levels, numlevels) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetMixLevelsInput", error)),
            }
        }
    }
    pub fn set_mix_matrix(
        &self,
        matrix: Option<*mut f32>,
        outchannels: i32,
        inchannels: i32,
        inchannel_hop: Option<i32>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetMixMatrix(
                self.pointer,
                matrix.unwrap_or(null_mut()),
                outchannels,
                inchannels,
                inchannel_hop.unwrap_or(0),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetMixMatrix", error)),
            }
        }
    }
    pub fn get_mix_matrix(&self, inchannel_hop: i32) -> Result<(f32, i32, i32), Error> {
        unsafe {
            let mut matrix = f32::default();
            let mut outchannels = i32::default();
            let mut inchannels = i32::default();
            match ffi::FMOD_Channel_GetMixMatrix(
                self.pointer,
                &mut matrix,
                &mut outchannels,
                &mut inchannels,
                inchannel_hop,
            ) {
                ffi::FMOD_OK => Ok((matrix, outchannels, inchannels)),
                error => Err(err_fmod!("FMOD_Channel_GetMixMatrix", error)),
            }
        }
    }
    pub fn get_dsp_clock(&self) -> Result<(u64, u64), Error> {
        unsafe {
            let mut dspclock = u64::default();
            let mut parentclock = u64::default();
            match ffi::FMOD_Channel_GetDSPClock(self.pointer, &mut dspclock, &mut parentclock) {
                ffi::FMOD_OK => Ok((dspclock, parentclock)),
                error => Err(err_fmod!("FMOD_Channel_GetDSPClock", error)),
            }
        }
    }
    pub fn set_delay(
        &self,
        dspclock_start: Option<u64>,
        dspclock_end: Option<u64>,
        stopchannels: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetDelay(
                self.pointer,
                dspclock_start.unwrap_or(0),
                dspclock_end.unwrap_or(0),
                from_bool!(stopchannels),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetDelay", error)),
            }
        }
    }
    pub fn get_delay(&self) -> Result<(u64, u64, bool), Error> {
        unsafe {
            let mut dspclock_start = u64::default();
            let mut dspclock_end = u64::default();
            let mut stopchannels = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Channel_GetDelay(
                self.pointer,
                &mut dspclock_start,
                &mut dspclock_end,
                &mut stopchannels,
            ) {
                ffi::FMOD_OK => Ok((dspclock_start, dspclock_end, to_bool!(stopchannels))),
                error => Err(err_fmod!("FMOD_Channel_GetDelay", error)),
            }
        }
    }
    pub fn add_fade_point(&self, dspclock: u64, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_AddFadePoint(self.pointer, dspclock, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_AddFadePoint", error)),
            }
        }
    }
    pub fn set_fade_point_ramp(&self, dspclock: u64, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetFadePointRamp(self.pointer, dspclock, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetFadePointRamp", error)),
            }
        }
    }
    pub fn remove_fade_points(&self, dspclock_start: u64, dspclock_end: u64) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_RemoveFadePoints(self.pointer, dspclock_start, dspclock_end) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_RemoveFadePoints", error)),
            }
        }
    }
    pub fn get_fade_points(&self) -> Result<(u32, u64, f32), Error> {
        unsafe {
            let mut numpoints = u32::default();
            let mut point_dspclock = u64::default();
            let mut point_volume = f32::default();
            match ffi::FMOD_Channel_GetFadePoints(
                self.pointer,
                &mut numpoints,
                &mut point_dspclock,
                &mut point_volume,
            ) {
                ffi::FMOD_OK => Ok((numpoints, point_dspclock, point_volume)),
                error => Err(err_fmod!("FMOD_Channel_GetFadePoints", error)),
            }
        }
    }
    pub fn get_dsp(&self, index: i32) -> Result<Dsp, Error> {
        unsafe {
            let mut dsp = null_mut();
            match ffi::FMOD_Channel_GetDSP(self.pointer, index, &mut dsp) {
                ffi::FMOD_OK => Ok(Dsp::from(dsp)),
                error => Err(err_fmod!("FMOD_Channel_GetDSP", error)),
            }
        }
    }
    pub fn add_dsp(&self, index: i32, dsp: Dsp) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_AddDSP(self.pointer, index, dsp.as_mut_ptr()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_AddDSP", error)),
            }
        }
    }
    pub fn remove_dsp(&self, dsp: Dsp) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_RemoveDSP(self.pointer, dsp.as_mut_ptr()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_RemoveDSP", error)),
            }
        }
    }
    pub fn get_num_ds_ps(&self) -> Result<i32, Error> {
        unsafe {
            let mut numdsps = i32::default();
            match ffi::FMOD_Channel_GetNumDSPs(self.pointer, &mut numdsps) {
                ffi::FMOD_OK => Ok(numdsps),
                error => Err(err_fmod!("FMOD_Channel_GetNumDSPs", error)),
            }
        }
    }
    pub fn set_dsp_index(&self, dsp: Dsp, index: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetDSPIndex(self.pointer, dsp.as_mut_ptr(), index) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetDSPIndex", error)),
            }
        }
    }
    pub fn get_dsp_index(&self, dsp: Dsp) -> Result<i32, Error> {
        unsafe {
            let mut index = i32::default();
            match ffi::FMOD_Channel_GetDSPIndex(self.pointer, dsp.as_mut_ptr(), &mut index) {
                ffi::FMOD_OK => Ok(index),
                error => Err(err_fmod!("FMOD_Channel_GetDSPIndex", error)),
            }
        }
    }
    pub fn set_3d_attributes(&self, pos: Option<Vector>, vel: Option<Vector>) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_Set3DAttributes(
                self.pointer,
                pos.map(|value| &value.into() as *const _).unwrap_or(null()),
                vel.map(|value| &value.into() as *const _).unwrap_or(null()),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Set3DAttributes", error)),
            }
        }
    }
    pub fn get_3d_attributes(&self) -> Result<(Vector, Vector), Error> {
        unsafe {
            let mut pos = ffi::FMOD_VECTOR::default();
            let mut vel = ffi::FMOD_VECTOR::default();
            match ffi::FMOD_Channel_Get3DAttributes(self.pointer, &mut pos, &mut vel) {
                ffi::FMOD_OK => Ok((Vector::try_from(pos)?, Vector::try_from(vel)?)),
                error => Err(err_fmod!("FMOD_Channel_Get3DAttributes", error)),
            }
        }
    }
    pub fn set_3d_min_max_distance(&self, mindistance: f32, maxdistance: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_Set3DMinMaxDistance(self.pointer, mindistance, maxdistance) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Set3DMinMaxDistance", error)),
            }
        }
    }
    pub fn get_3d_min_max_distance(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut mindistance = f32::default();
            let mut maxdistance = f32::default();
            match ffi::FMOD_Channel_Get3DMinMaxDistance(
                self.pointer,
                &mut mindistance,
                &mut maxdistance,
            ) {
                ffi::FMOD_OK => Ok((mindistance, maxdistance)),
                error => Err(err_fmod!("FMOD_Channel_Get3DMinMaxDistance", error)),
            }
        }
    }
    pub fn set_3d_cone_settings(
        &self,
        insideconeangle: f32,
        outsideconeangle: f32,
        outsidevolume: f32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_Set3DConeSettings(
                self.pointer,
                insideconeangle,
                outsideconeangle,
                outsidevolume,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Set3DConeSettings", error)),
            }
        }
    }
    pub fn get_3d_cone_settings(&self) -> Result<(f32, f32, f32), Error> {
        unsafe {
            let mut insideconeangle = f32::default();
            let mut outsideconeangle = f32::default();
            let mut outsidevolume = f32::default();
            match ffi::FMOD_Channel_Get3DConeSettings(
                self.pointer,
                &mut insideconeangle,
                &mut outsideconeangle,
                &mut outsidevolume,
            ) {
                ffi::FMOD_OK => Ok((insideconeangle, outsideconeangle, outsidevolume)),
                error => Err(err_fmod!("FMOD_Channel_Get3DConeSettings", error)),
            }
        }
    }
    pub fn set_3d_cone_orientation(&self, orientation: Vector) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_Set3DConeOrientation(self.pointer, &mut orientation.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Set3DConeOrientation", error)),
            }
        }
    }
    pub fn get_3d_cone_orientation(&self) -> Result<Vector, Error> {
        unsafe {
            let mut orientation = ffi::FMOD_VECTOR::default();
            match ffi::FMOD_Channel_Get3DConeOrientation(self.pointer, &mut orientation) {
                ffi::FMOD_OK => Ok(Vector::try_from(orientation)?),
                error => Err(err_fmod!("FMOD_Channel_Get3DConeOrientation", error)),
            }
        }
    }
    pub fn set_3d_custom_rolloff(&self, points: Vec<Vector>) -> Result<(), Error> {
        unsafe {
            let numpoints = points.len() as i32;
            match ffi::FMOD_Channel_Set3DCustomRolloff(
                self.pointer,
                vec_as_mut_ptr(points, |point| point.into()),
                numpoints,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Set3DCustomRolloff", error)),
            }
        }
    }
    pub fn get_3d_custom_rolloff(&self) -> Result<Vec<Vector>, Error> {
        unsafe {
            let mut points = null_mut();
            let mut numpoints = i32::default();
            match ffi::FMOD_Channel_Get3DCustomRolloff(self.pointer, &mut points, &mut numpoints) {
                ffi::FMOD_OK => Ok(to_vec!(points, numpoints, Vector::try_from)?),
                error => Err(err_fmod!("FMOD_Channel_Get3DCustomRolloff", error)),
            }
        }
    }
    pub fn set_3d_occlusion(
        &self,
        directocclusion: f32,
        reverbocclusion: f32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_Set3DOcclusion(self.pointer, directocclusion, reverbocclusion) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Set3DOcclusion", error)),
            }
        }
    }
    pub fn get_3d_occlusion(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut directocclusion = f32::default();
            let mut reverbocclusion = f32::default();
            match ffi::FMOD_Channel_Get3DOcclusion(
                self.pointer,
                &mut directocclusion,
                &mut reverbocclusion,
            ) {
                ffi::FMOD_OK => Ok((directocclusion, reverbocclusion)),
                error => Err(err_fmod!("FMOD_Channel_Get3DOcclusion", error)),
            }
        }
    }
    pub fn set_3d_spread(&self, angle: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_Set3DSpread(self.pointer, angle) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Set3DSpread", error)),
            }
        }
    }
    pub fn get_3d_spread(&self) -> Result<f32, Error> {
        unsafe {
            let mut angle = f32::default();
            match ffi::FMOD_Channel_Get3DSpread(self.pointer, &mut angle) {
                ffi::FMOD_OK => Ok(angle),
                error => Err(err_fmod!("FMOD_Channel_Get3DSpread", error)),
            }
        }
    }
    pub fn set_3d_level(&self, level: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_Set3DLevel(self.pointer, level) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Set3DLevel", error)),
            }
        }
    }
    pub fn get_3d_level(&self) -> Result<f32, Error> {
        unsafe {
            let mut level = f32::default();
            match ffi::FMOD_Channel_Get3DLevel(self.pointer, &mut level) {
                ffi::FMOD_OK => Ok(level),
                error => Err(err_fmod!("FMOD_Channel_Get3DLevel", error)),
            }
        }
    }
    pub fn set_3d_doppler_level(&self, level: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_Set3DDopplerLevel(self.pointer, level) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Set3DDopplerLevel", error)),
            }
        }
    }
    pub fn get_3d_doppler_level(&self) -> Result<f32, Error> {
        unsafe {
            let mut level = f32::default();
            match ffi::FMOD_Channel_Get3DDopplerLevel(self.pointer, &mut level) {
                ffi::FMOD_OK => Ok(level),
                error => Err(err_fmod!("FMOD_Channel_Get3DDopplerLevel", error)),
            }
        }
    }
    pub fn set_3d_distance_filter(
        &self,
        custom: bool,
        custom_level: f32,
        center_freq: Option<f32>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_Set3DDistanceFilter(
                self.pointer,
                from_bool!(custom),
                custom_level,
                center_freq.unwrap_or(0.0),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_Set3DDistanceFilter", error)),
            }
        }
    }
    pub fn get_3d_distance_filter(&self) -> Result<(bool, f32, f32), Error> {
        unsafe {
            let mut custom = ffi::FMOD_BOOL::default();
            let mut custom_level = f32::default();
            let mut center_freq = f32::default();
            match ffi::FMOD_Channel_Get3DDistanceFilter(
                self.pointer,
                &mut custom,
                &mut custom_level,
                &mut center_freq,
            ) {
                ffi::FMOD_OK => Ok((to_bool!(custom), custom_level, center_freq)),
                error => Err(err_fmod!("FMOD_Channel_Get3DDistanceFilter", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetUserData", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_Channel_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_Channel_GetUserData", error)),
            }
        }
    }
    pub fn set_frequency(&self, frequency: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetFrequency(self.pointer, frequency) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetFrequency", error)),
            }
        }
    }
    pub fn get_frequency(&self) -> Result<f32, Error> {
        unsafe {
            let mut frequency = f32::default();
            match ffi::FMOD_Channel_GetFrequency(self.pointer, &mut frequency) {
                ffi::FMOD_OK => Ok(frequency),
                error => Err(err_fmod!("FMOD_Channel_GetFrequency", error)),
            }
        }
    }
    pub fn set_priority(&self, priority: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetPriority(self.pointer, priority) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetPriority", error)),
            }
        }
    }
    pub fn get_priority(&self) -> Result<i32, Error> {
        unsafe {
            let mut priority = i32::default();
            match ffi::FMOD_Channel_GetPriority(self.pointer, &mut priority) {
                ffi::FMOD_OK => Ok(priority),
                error => Err(err_fmod!("FMOD_Channel_GetPriority", error)),
            }
        }
    }
    pub fn set_position(&self, position: u32, postype: ffi::FMOD_TIMEUNIT) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetPosition(self.pointer, position, postype) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetPosition", error)),
            }
        }
    }
    pub fn get_position(&self, postype: ffi::FMOD_TIMEUNIT) -> Result<u32, Error> {
        unsafe {
            let mut position = u32::default();
            match ffi::FMOD_Channel_GetPosition(self.pointer, &mut position, postype) {
                ffi::FMOD_OK => Ok(position),
                error => Err(err_fmod!("FMOD_Channel_GetPosition", error)),
            }
        }
    }
    pub fn set_channel_group(&self, channelgroup: ChannelGroup) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetChannelGroup(self.pointer, channelgroup.as_mut_ptr()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetChannelGroup", error)),
            }
        }
    }
    pub fn get_channel_group(&self) -> Result<ChannelGroup, Error> {
        unsafe {
            let mut channelgroup = null_mut();
            match ffi::FMOD_Channel_GetChannelGroup(self.pointer, &mut channelgroup) {
                ffi::FMOD_OK => Ok(ChannelGroup::from(channelgroup)),
                error => Err(err_fmod!("FMOD_Channel_GetChannelGroup", error)),
            }
        }
    }
    pub fn set_loop_count(&self, loopcount: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetLoopCount(self.pointer, loopcount) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetLoopCount", error)),
            }
        }
    }
    pub fn get_loop_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut loopcount = i32::default();
            match ffi::FMOD_Channel_GetLoopCount(self.pointer, &mut loopcount) {
                ffi::FMOD_OK => Ok(loopcount),
                error => Err(err_fmod!("FMOD_Channel_GetLoopCount", error)),
            }
        }
    }
    pub fn set_loop_points(
        &self,
        loopstart: u32,
        loopstarttype: ffi::FMOD_TIMEUNIT,
        loopend: u32,
        loopendtype: ffi::FMOD_TIMEUNIT,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Channel_SetLoopPoints(
                self.pointer,
                loopstart,
                loopstarttype,
                loopend,
                loopendtype,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Channel_SetLoopPoints", error)),
            }
        }
    }
    pub fn get_loop_points(
        &self,
        loopstarttype: ffi::FMOD_TIMEUNIT,
        loopendtype: ffi::FMOD_TIMEUNIT,
    ) -> Result<(u32, u32), Error> {
        unsafe {
            let mut loopstart = u32::default();
            let mut loopend = u32::default();
            match ffi::FMOD_Channel_GetLoopPoints(
                self.pointer,
                &mut loopstart,
                loopstarttype,
                &mut loopend,
                loopendtype,
            ) {
                ffi::FMOD_OK => Ok((loopstart, loopend)),
                error => Err(err_fmod!("FMOD_Channel_GetLoopPoints", error)),
            }
        }
    }
    pub fn is_virtual(&self) -> Result<bool, Error> {
        unsafe {
            let mut isvirtual = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Channel_IsVirtual(self.pointer, &mut isvirtual) {
                ffi::FMOD_OK => Ok(to_bool!(isvirtual)),
                error => Err(err_fmod!("FMOD_Channel_IsVirtual", error)),
            }
        }
    }
    pub fn get_current_sound(&self) -> Result<Sound, Error> {
        unsafe {
            let mut sound = null_mut();
            match ffi::FMOD_Channel_GetCurrentSound(self.pointer, &mut sound) {
                ffi::FMOD_OK => Ok(Sound::from(sound)),
                error => Err(err_fmod!("FMOD_Channel_GetCurrentSound", error)),
            }
        }
    }
    pub fn get_index(&self) -> Result<i32, Error> {
        unsafe {
            let mut index = i32::default();
            match ffi::FMOD_Channel_GetIndex(self.pointer, &mut index) {
                ffi::FMOD_OK => Ok(index),
                error => Err(err_fmod!("FMOD_Channel_GetIndex", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChannelControl {
    pointer: *mut ffi::FMOD_CHANNELCONTROL,
}

impl ChannelControl {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_CHANNELCONTROL) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_CHANNELCONTROL {
        self.pointer
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChannelGroup {
    pointer: *mut ffi::FMOD_CHANNELGROUP,
}

impl ChannelGroup {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_CHANNELGROUP) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_CHANNELGROUP {
        self.pointer
    }
    pub fn get_system_object(&self) -> Result<System, Error> {
        unsafe {
            let mut system = null_mut();
            match ffi::FMOD_ChannelGroup_GetSystemObject(self.pointer, &mut system) {
                ffi::FMOD_OK => Ok(System::from(system)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetSystemObject", error)),
            }
        }
    }
    pub fn stop(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Stop(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Stop", error)),
            }
        }
    }
    pub fn set_paused(&self, paused: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetPaused(self.pointer, from_bool!(paused)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetPaused", error)),
            }
        }
    }
    pub fn get_paused(&self) -> Result<bool, Error> {
        unsafe {
            let mut paused = ffi::FMOD_BOOL::default();
            match ffi::FMOD_ChannelGroup_GetPaused(self.pointer, &mut paused) {
                ffi::FMOD_OK => Ok(to_bool!(paused)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetPaused", error)),
            }
        }
    }
    pub fn set_volume(&self, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetVolume(self.pointer, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetVolume", error)),
            }
        }
    }
    pub fn get_volume(&self) -> Result<f32, Error> {
        unsafe {
            let mut volume = f32::default();
            match ffi::FMOD_ChannelGroup_GetVolume(self.pointer, &mut volume) {
                ffi::FMOD_OK => Ok(volume),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetVolume", error)),
            }
        }
    }
    pub fn set_volume_ramp(&self, ramp: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetVolumeRamp(self.pointer, from_bool!(ramp)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetVolumeRamp", error)),
            }
        }
    }
    pub fn get_volume_ramp(&self) -> Result<bool, Error> {
        unsafe {
            let mut ramp = ffi::FMOD_BOOL::default();
            match ffi::FMOD_ChannelGroup_GetVolumeRamp(self.pointer, &mut ramp) {
                ffi::FMOD_OK => Ok(to_bool!(ramp)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetVolumeRamp", error)),
            }
        }
    }
    pub fn get_audibility(&self) -> Result<f32, Error> {
        unsafe {
            let mut audibility = f32::default();
            match ffi::FMOD_ChannelGroup_GetAudibility(self.pointer, &mut audibility) {
                ffi::FMOD_OK => Ok(audibility),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetAudibility", error)),
            }
        }
    }
    pub fn set_pitch(&self, pitch: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetPitch(self.pointer, pitch) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetPitch", error)),
            }
        }
    }
    pub fn get_pitch(&self) -> Result<f32, Error> {
        unsafe {
            let mut pitch = f32::default();
            match ffi::FMOD_ChannelGroup_GetPitch(self.pointer, &mut pitch) {
                ffi::FMOD_OK => Ok(pitch),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetPitch", error)),
            }
        }
    }
    pub fn set_mute(&self, mute: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetMute(self.pointer, from_bool!(mute)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetMute", error)),
            }
        }
    }
    pub fn get_mute(&self) -> Result<bool, Error> {
        unsafe {
            let mut mute = ffi::FMOD_BOOL::default();
            match ffi::FMOD_ChannelGroup_GetMute(self.pointer, &mut mute) {
                ffi::FMOD_OK => Ok(to_bool!(mute)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetMute", error)),
            }
        }
    }
    pub fn set_reverb_properties(&self, instance: i32, wet: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetReverbProperties(self.pointer, instance, wet) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetReverbProperties", error)),
            }
        }
    }
    pub fn get_reverb_properties(&self, instance: i32) -> Result<f32, Error> {
        unsafe {
            let mut wet = f32::default();
            match ffi::FMOD_ChannelGroup_GetReverbProperties(self.pointer, instance, &mut wet) {
                ffi::FMOD_OK => Ok(wet),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetReverbProperties", error)),
            }
        }
    }
    pub fn set_low_pass_gain(&self, gain: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetLowPassGain(self.pointer, gain) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetLowPassGain", error)),
            }
        }
    }
    pub fn get_low_pass_gain(&self) -> Result<f32, Error> {
        unsafe {
            let mut gain = f32::default();
            match ffi::FMOD_ChannelGroup_GetLowPassGain(self.pointer, &mut gain) {
                ffi::FMOD_OK => Ok(gain),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetLowPassGain", error)),
            }
        }
    }
    pub fn set_mode(&self, mode: ffi::FMOD_MODE) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetMode(self.pointer, mode) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetMode", error)),
            }
        }
    }
    pub fn get_mode(&self) -> Result<ffi::FMOD_MODE, Error> {
        unsafe {
            let mut mode = ffi::FMOD_MODE::default();
            match ffi::FMOD_ChannelGroup_GetMode(self.pointer, &mut mode) {
                ffi::FMOD_OK => Ok(mode),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetMode", error)),
            }
        }
    }
    pub fn set_callback(&self, callback: ffi::FMOD_CHANNELCONTROL_CALLBACK) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetCallback(self.pointer, callback) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetCallback", error)),
            }
        }
    }
    pub fn is_playing(&self) -> Result<bool, Error> {
        unsafe {
            let mut isplaying = ffi::FMOD_BOOL::default();
            match ffi::FMOD_ChannelGroup_IsPlaying(self.pointer, &mut isplaying) {
                ffi::FMOD_OK => Ok(to_bool!(isplaying)),
                error => Err(err_fmod!("FMOD_ChannelGroup_IsPlaying", error)),
            }
        }
    }
    pub fn set_pan(&self, pan: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetPan(self.pointer, pan) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetPan", error)),
            }
        }
    }
    pub fn set_mix_levels_output(
        &self,
        frontleft: f32,
        frontright: f32,
        center: f32,
        lfe: f32,
        surroundleft: f32,
        surroundright: f32,
        backleft: f32,
        backright: f32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetMixLevelsOutput(
                self.pointer,
                frontleft,
                frontright,
                center,
                lfe,
                surroundleft,
                surroundright,
                backleft,
                backright,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetMixLevelsOutput", error)),
            }
        }
    }
    pub fn set_mix_levels_input(&self, levels: *mut f32, numlevels: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetMixLevelsInput(self.pointer, levels, numlevels) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetMixLevelsInput", error)),
            }
        }
    }
    pub fn set_mix_matrix(
        &self,
        matrix: Option<*mut f32>,
        outchannels: i32,
        inchannels: i32,
        inchannel_hop: Option<i32>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetMixMatrix(
                self.pointer,
                matrix.unwrap_or(null_mut()),
                outchannels,
                inchannels,
                inchannel_hop.unwrap_or(0),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetMixMatrix", error)),
            }
        }
    }
    pub fn get_mix_matrix(&self, inchannel_hop: i32) -> Result<(f32, i32, i32), Error> {
        unsafe {
            let mut matrix = f32::default();
            let mut outchannels = i32::default();
            let mut inchannels = i32::default();
            match ffi::FMOD_ChannelGroup_GetMixMatrix(
                self.pointer,
                &mut matrix,
                &mut outchannels,
                &mut inchannels,
                inchannel_hop,
            ) {
                ffi::FMOD_OK => Ok((matrix, outchannels, inchannels)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetMixMatrix", error)),
            }
        }
    }
    pub fn get_dsp_clock(&self) -> Result<(u64, u64), Error> {
        unsafe {
            let mut dspclock = u64::default();
            let mut parentclock = u64::default();
            match ffi::FMOD_ChannelGroup_GetDSPClock(self.pointer, &mut dspclock, &mut parentclock)
            {
                ffi::FMOD_OK => Ok((dspclock, parentclock)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetDSPClock", error)),
            }
        }
    }
    pub fn set_delay(
        &self,
        dspclock_start: Option<u64>,
        dspclock_end: Option<u64>,
        stopchannels: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetDelay(
                self.pointer,
                dspclock_start.unwrap_or(0),
                dspclock_end.unwrap_or(0),
                from_bool!(stopchannels),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetDelay", error)),
            }
        }
    }
    pub fn get_delay(&self) -> Result<(u64, u64, bool), Error> {
        unsafe {
            let mut dspclock_start = u64::default();
            let mut dspclock_end = u64::default();
            let mut stopchannels = ffi::FMOD_BOOL::default();
            match ffi::FMOD_ChannelGroup_GetDelay(
                self.pointer,
                &mut dspclock_start,
                &mut dspclock_end,
                &mut stopchannels,
            ) {
                ffi::FMOD_OK => Ok((dspclock_start, dspclock_end, to_bool!(stopchannels))),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetDelay", error)),
            }
        }
    }
    pub fn add_fade_point(&self, dspclock: u64, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_AddFadePoint(self.pointer, dspclock, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_AddFadePoint", error)),
            }
        }
    }
    pub fn set_fade_point_ramp(&self, dspclock: u64, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetFadePointRamp(self.pointer, dspclock, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetFadePointRamp", error)),
            }
        }
    }
    pub fn remove_fade_points(&self, dspclock_start: u64, dspclock_end: u64) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_RemoveFadePoints(
                self.pointer,
                dspclock_start,
                dspclock_end,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_RemoveFadePoints", error)),
            }
        }
    }
    pub fn get_fade_points(&self) -> Result<(u32, u64, f32), Error> {
        unsafe {
            let mut numpoints = u32::default();
            let mut point_dspclock = u64::default();
            let mut point_volume = f32::default();
            match ffi::FMOD_ChannelGroup_GetFadePoints(
                self.pointer,
                &mut numpoints,
                &mut point_dspclock,
                &mut point_volume,
            ) {
                ffi::FMOD_OK => Ok((numpoints, point_dspclock, point_volume)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetFadePoints", error)),
            }
        }
    }
    pub fn get_dsp(&self, index: i32) -> Result<Dsp, Error> {
        unsafe {
            let mut dsp = null_mut();
            match ffi::FMOD_ChannelGroup_GetDSP(self.pointer, index, &mut dsp) {
                ffi::FMOD_OK => Ok(Dsp::from(dsp)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetDSP", error)),
            }
        }
    }
    pub fn add_dsp(&self, index: i32, dsp: Dsp) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_AddDSP(self.pointer, index, dsp.as_mut_ptr()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_AddDSP", error)),
            }
        }
    }
    pub fn remove_dsp(&self, dsp: Dsp) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_RemoveDSP(self.pointer, dsp.as_mut_ptr()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_RemoveDSP", error)),
            }
        }
    }
    pub fn get_num_ds_ps(&self) -> Result<i32, Error> {
        unsafe {
            let mut numdsps = i32::default();
            match ffi::FMOD_ChannelGroup_GetNumDSPs(self.pointer, &mut numdsps) {
                ffi::FMOD_OK => Ok(numdsps),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetNumDSPs", error)),
            }
        }
    }
    pub fn set_dsp_index(&self, dsp: Dsp, index: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetDSPIndex(self.pointer, dsp.as_mut_ptr(), index) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetDSPIndex", error)),
            }
        }
    }
    pub fn get_dsp_index(&self, dsp: Dsp) -> Result<i32, Error> {
        unsafe {
            let mut index = i32::default();
            match ffi::FMOD_ChannelGroup_GetDSPIndex(self.pointer, dsp.as_mut_ptr(), &mut index) {
                ffi::FMOD_OK => Ok(index),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetDSPIndex", error)),
            }
        }
    }
    pub fn set_3d_attributes(&self, pos: Option<Vector>, vel: Option<Vector>) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Set3DAttributes(
                self.pointer,
                pos.map(|value| &value.into() as *const _).unwrap_or(null()),
                vel.map(|value| &value.into() as *const _).unwrap_or(null()),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Set3DAttributes", error)),
            }
        }
    }
    pub fn get_3d_attributes(&self) -> Result<(Vector, Vector), Error> {
        unsafe {
            let mut pos = ffi::FMOD_VECTOR::default();
            let mut vel = ffi::FMOD_VECTOR::default();
            match ffi::FMOD_ChannelGroup_Get3DAttributes(self.pointer, &mut pos, &mut vel) {
                ffi::FMOD_OK => Ok((Vector::try_from(pos)?, Vector::try_from(vel)?)),
                error => Err(err_fmod!("FMOD_ChannelGroup_Get3DAttributes", error)),
            }
        }
    }
    pub fn set_3d_min_max_distance(&self, mindistance: f32, maxdistance: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Set3DMinMaxDistance(self.pointer, mindistance, maxdistance)
            {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Set3DMinMaxDistance", error)),
            }
        }
    }
    pub fn get_3d_min_max_distance(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut mindistance = f32::default();
            let mut maxdistance = f32::default();
            match ffi::FMOD_ChannelGroup_Get3DMinMaxDistance(
                self.pointer,
                &mut mindistance,
                &mut maxdistance,
            ) {
                ffi::FMOD_OK => Ok((mindistance, maxdistance)),
                error => Err(err_fmod!("FMOD_ChannelGroup_Get3DMinMaxDistance", error)),
            }
        }
    }
    pub fn set_3d_cone_settings(
        &self,
        insideconeangle: f32,
        outsideconeangle: f32,
        outsidevolume: f32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Set3DConeSettings(
                self.pointer,
                insideconeangle,
                outsideconeangle,
                outsidevolume,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Set3DConeSettings", error)),
            }
        }
    }
    pub fn get_3d_cone_settings(&self) -> Result<(f32, f32, f32), Error> {
        unsafe {
            let mut insideconeangle = f32::default();
            let mut outsideconeangle = f32::default();
            let mut outsidevolume = f32::default();
            match ffi::FMOD_ChannelGroup_Get3DConeSettings(
                self.pointer,
                &mut insideconeangle,
                &mut outsideconeangle,
                &mut outsidevolume,
            ) {
                ffi::FMOD_OK => Ok((insideconeangle, outsideconeangle, outsidevolume)),
                error => Err(err_fmod!("FMOD_ChannelGroup_Get3DConeSettings", error)),
            }
        }
    }
    pub fn set_3d_cone_orientation(&self, orientation: Vector) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Set3DConeOrientation(self.pointer, &mut orientation.into())
            {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Set3DConeOrientation", error)),
            }
        }
    }
    pub fn get_3d_cone_orientation(&self) -> Result<Vector, Error> {
        unsafe {
            let mut orientation = ffi::FMOD_VECTOR::default();
            match ffi::FMOD_ChannelGroup_Get3DConeOrientation(self.pointer, &mut orientation) {
                ffi::FMOD_OK => Ok(Vector::try_from(orientation)?),
                error => Err(err_fmod!("FMOD_ChannelGroup_Get3DConeOrientation", error)),
            }
        }
    }
    pub fn set_3d_custom_rolloff(&self, points: Vec<Vector>) -> Result<(), Error> {
        unsafe {
            let numpoints = points.len() as i32;
            match ffi::FMOD_ChannelGroup_Set3DCustomRolloff(
                self.pointer,
                vec_as_mut_ptr(points, |point| point.into()),
                numpoints,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Set3DCustomRolloff", error)),
            }
        }
    }
    pub fn get_3d_custom_rolloff(&self) -> Result<Vec<Vector>, Error> {
        unsafe {
            let mut points = null_mut();
            let mut numpoints = i32::default();
            match ffi::FMOD_ChannelGroup_Get3DCustomRolloff(
                self.pointer,
                &mut points,
                &mut numpoints,
            ) {
                ffi::FMOD_OK => Ok(to_vec!(points, numpoints, Vector::try_from)?),
                error => Err(err_fmod!("FMOD_ChannelGroup_Get3DCustomRolloff", error)),
            }
        }
    }
    pub fn set_3d_occlusion(
        &self,
        directocclusion: f32,
        reverbocclusion: f32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Set3DOcclusion(
                self.pointer,
                directocclusion,
                reverbocclusion,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Set3DOcclusion", error)),
            }
        }
    }
    pub fn get_3d_occlusion(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut directocclusion = f32::default();
            let mut reverbocclusion = f32::default();
            match ffi::FMOD_ChannelGroup_Get3DOcclusion(
                self.pointer,
                &mut directocclusion,
                &mut reverbocclusion,
            ) {
                ffi::FMOD_OK => Ok((directocclusion, reverbocclusion)),
                error => Err(err_fmod!("FMOD_ChannelGroup_Get3DOcclusion", error)),
            }
        }
    }
    pub fn set_3d_spread(&self, angle: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Set3DSpread(self.pointer, angle) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Set3DSpread", error)),
            }
        }
    }
    pub fn get_3d_spread(&self) -> Result<f32, Error> {
        unsafe {
            let mut angle = f32::default();
            match ffi::FMOD_ChannelGroup_Get3DSpread(self.pointer, &mut angle) {
                ffi::FMOD_OK => Ok(angle),
                error => Err(err_fmod!("FMOD_ChannelGroup_Get3DSpread", error)),
            }
        }
    }
    pub fn set_3d_level(&self, level: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Set3DLevel(self.pointer, level) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Set3DLevel", error)),
            }
        }
    }
    pub fn get_3d_level(&self) -> Result<f32, Error> {
        unsafe {
            let mut level = f32::default();
            match ffi::FMOD_ChannelGroup_Get3DLevel(self.pointer, &mut level) {
                ffi::FMOD_OK => Ok(level),
                error => Err(err_fmod!("FMOD_ChannelGroup_Get3DLevel", error)),
            }
        }
    }
    pub fn set_3d_doppler_level(&self, level: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Set3DDopplerLevel(self.pointer, level) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Set3DDopplerLevel", error)),
            }
        }
    }
    pub fn get_3d_doppler_level(&self) -> Result<f32, Error> {
        unsafe {
            let mut level = f32::default();
            match ffi::FMOD_ChannelGroup_Get3DDopplerLevel(self.pointer, &mut level) {
                ffi::FMOD_OK => Ok(level),
                error => Err(err_fmod!("FMOD_ChannelGroup_Get3DDopplerLevel", error)),
            }
        }
    }
    pub fn set_3d_distance_filter(
        &self,
        custom: bool,
        custom_level: f32,
        center_freq: Option<f32>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Set3DDistanceFilter(
                self.pointer,
                from_bool!(custom),
                custom_level,
                center_freq.unwrap_or(0.0),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Set3DDistanceFilter", error)),
            }
        }
    }
    pub fn get_3d_distance_filter(&self) -> Result<(bool, f32, f32), Error> {
        unsafe {
            let mut custom = ffi::FMOD_BOOL::default();
            let mut custom_level = f32::default();
            let mut center_freq = f32::default();
            match ffi::FMOD_ChannelGroup_Get3DDistanceFilter(
                self.pointer,
                &mut custom,
                &mut custom_level,
                &mut center_freq,
            ) {
                ffi::FMOD_OK => Ok((to_bool!(custom), custom_level, center_freq)),
                error => Err(err_fmod!("FMOD_ChannelGroup_Get3DDistanceFilter", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_SetUserData", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_ChannelGroup_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetUserData", error)),
            }
        }
    }
    pub fn release(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_ChannelGroup_Release(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_ChannelGroup_Release", error)),
            }
        }
    }
    pub fn add_group(
        &self,
        group: ChannelGroup,
        propagatedspclock: bool,
    ) -> Result<DspConnection, Error> {
        unsafe {
            let mut connection = null_mut();
            match ffi::FMOD_ChannelGroup_AddGroup(
                self.pointer,
                group.as_mut_ptr(),
                from_bool!(propagatedspclock),
                &mut connection,
            ) {
                ffi::FMOD_OK => Ok(DspConnection::from(connection)),
                error => Err(err_fmod!("FMOD_ChannelGroup_AddGroup", error)),
            }
        }
    }
    pub fn get_num_groups(&self) -> Result<i32, Error> {
        unsafe {
            let mut numgroups = i32::default();
            match ffi::FMOD_ChannelGroup_GetNumGroups(self.pointer, &mut numgroups) {
                ffi::FMOD_OK => Ok(numgroups),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetNumGroups", error)),
            }
        }
    }
    pub fn get_group(&self, index: i32) -> Result<ChannelGroup, Error> {
        unsafe {
            let mut group = null_mut();
            match ffi::FMOD_ChannelGroup_GetGroup(self.pointer, index, &mut group) {
                ffi::FMOD_OK => Ok(ChannelGroup::from(group)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetGroup", error)),
            }
        }
    }
    pub fn get_parent_group(&self) -> Result<ChannelGroup, Error> {
        unsafe {
            let mut group = null_mut();
            match ffi::FMOD_ChannelGroup_GetParentGroup(self.pointer, &mut group) {
                ffi::FMOD_OK => Ok(ChannelGroup::from(group)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetParentGroup", error)),
            }
        }
    }
    pub fn get_name(&self, namelen: i32) -> Result<String, Error> {
        unsafe {
            let name = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            match ffi::FMOD_ChannelGroup_GetName(self.pointer, name, namelen) {
                ffi::FMOD_OK => Ok(CString::from_raw(name)
                    .into_string()
                    .map_err(Error::String)?),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetName", error)),
            }
        }
    }
    pub fn get_num_channels(&self) -> Result<i32, Error> {
        unsafe {
            let mut numchannels = i32::default();
            match ffi::FMOD_ChannelGroup_GetNumChannels(self.pointer, &mut numchannels) {
                ffi::FMOD_OK => Ok(numchannels),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetNumChannels", error)),
            }
        }
    }
    pub fn get_channel(&self, index: i32) -> Result<Channel, Error> {
        unsafe {
            let mut channel = null_mut();
            match ffi::FMOD_ChannelGroup_GetChannel(self.pointer, index, &mut channel) {
                ffi::FMOD_OK => Ok(Channel::from(channel)),
                error => Err(err_fmod!("FMOD_ChannelGroup_GetChannel", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dsp {
    pointer: *mut ffi::FMOD_DSP,
}

impl Dsp {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_DSP) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_DSP {
        self.pointer
    }
    pub fn release(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_Release(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_Release", error)),
            }
        }
    }
    pub fn get_system_object(&self) -> Result<System, Error> {
        unsafe {
            let mut system = null_mut();
            match ffi::FMOD_DSP_GetSystemObject(self.pointer, &mut system) {
                ffi::FMOD_OK => Ok(System::from(system)),
                error => Err(err_fmod!("FMOD_DSP_GetSystemObject", error)),
            }
        }
    }
    pub fn add_input(&self, input: Dsp, type_: DspConnectionType) -> Result<DspConnection, Error> {
        unsafe {
            let mut connection = null_mut();
            match ffi::FMOD_DSP_AddInput(
                self.pointer,
                input.as_mut_ptr(),
                &mut connection,
                type_.into(),
            ) {
                ffi::FMOD_OK => Ok(DspConnection::from(connection)),
                error => Err(err_fmod!("FMOD_DSP_AddInput", error)),
            }
        }
    }
    pub fn disconnect_from(
        &self,
        target: Option<Dsp>,
        connection: Option<DspConnection>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_DisconnectFrom(
                self.pointer,
                target.map(|value| value.as_mut_ptr()).unwrap_or(null_mut()),
                connection
                    .map(|value| value.as_mut_ptr())
                    .unwrap_or(null_mut()),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_DisconnectFrom", error)),
            }
        }
    }
    pub fn disconnect_all(&self, inputs: bool, outputs: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_DisconnectAll(self.pointer, from_bool!(inputs), from_bool!(outputs))
            {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_DisconnectAll", error)),
            }
        }
    }
    pub fn get_num_inputs(&self) -> Result<i32, Error> {
        unsafe {
            let mut numinputs = i32::default();
            match ffi::FMOD_DSP_GetNumInputs(self.pointer, &mut numinputs) {
                ffi::FMOD_OK => Ok(numinputs),
                error => Err(err_fmod!("FMOD_DSP_GetNumInputs", error)),
            }
        }
    }
    pub fn get_num_outputs(&self) -> Result<i32, Error> {
        unsafe {
            let mut numoutputs = i32::default();
            match ffi::FMOD_DSP_GetNumOutputs(self.pointer, &mut numoutputs) {
                ffi::FMOD_OK => Ok(numoutputs),
                error => Err(err_fmod!("FMOD_DSP_GetNumOutputs", error)),
            }
        }
    }
    pub fn get_input(&self, index: i32) -> Result<(Dsp, DspConnection), Error> {
        unsafe {
            let mut input = null_mut();
            let mut inputconnection = null_mut();
            match ffi::FMOD_DSP_GetInput(self.pointer, index, &mut input, &mut inputconnection) {
                ffi::FMOD_OK => Ok((Dsp::from(input), DspConnection::from(inputconnection))),
                error => Err(err_fmod!("FMOD_DSP_GetInput", error)),
            }
        }
    }
    pub fn get_output(&self, index: i32) -> Result<(Dsp, DspConnection), Error> {
        unsafe {
            let mut output = null_mut();
            let mut outputconnection = null_mut();
            match ffi::FMOD_DSP_GetOutput(self.pointer, index, &mut output, &mut outputconnection) {
                ffi::FMOD_OK => Ok((Dsp::from(output), DspConnection::from(outputconnection))),
                error => Err(err_fmod!("FMOD_DSP_GetOutput", error)),
            }
        }
    }
    pub fn set_active(&self, active: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_SetActive(self.pointer, from_bool!(active)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_SetActive", error)),
            }
        }
    }
    pub fn get_active(&self) -> Result<bool, Error> {
        unsafe {
            let mut active = ffi::FMOD_BOOL::default();
            match ffi::FMOD_DSP_GetActive(self.pointer, &mut active) {
                ffi::FMOD_OK => Ok(to_bool!(active)),
                error => Err(err_fmod!("FMOD_DSP_GetActive", error)),
            }
        }
    }
    pub fn set_bypass(&self, bypass: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_SetBypass(self.pointer, from_bool!(bypass)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_SetBypass", error)),
            }
        }
    }
    pub fn get_bypass(&self) -> Result<bool, Error> {
        unsafe {
            let mut bypass = ffi::FMOD_BOOL::default();
            match ffi::FMOD_DSP_GetBypass(self.pointer, &mut bypass) {
                ffi::FMOD_OK => Ok(to_bool!(bypass)),
                error => Err(err_fmod!("FMOD_DSP_GetBypass", error)),
            }
        }
    }
    pub fn set_wet_dry_mix(&self, prewet: f32, postwet: f32, dry: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_SetWetDryMix(self.pointer, prewet, postwet, dry) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_SetWetDryMix", error)),
            }
        }
    }
    pub fn get_wet_dry_mix(&self) -> Result<(f32, f32, f32), Error> {
        unsafe {
            let mut prewet = f32::default();
            let mut postwet = f32::default();
            let mut dry = f32::default();
            match ffi::FMOD_DSP_GetWetDryMix(self.pointer, &mut prewet, &mut postwet, &mut dry) {
                ffi::FMOD_OK => Ok((prewet, postwet, dry)),
                error => Err(err_fmod!("FMOD_DSP_GetWetDryMix", error)),
            }
        }
    }
    pub fn set_channel_format(
        &self,
        channelmask: ffi::FMOD_CHANNELMASK,
        numchannels: i32,
        source_speakermode: SpeakerMode,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_SetChannelFormat(
                self.pointer,
                channelmask,
                numchannels,
                source_speakermode.into(),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_SetChannelFormat", error)),
            }
        }
    }
    pub fn get_channel_format(&self) -> Result<(ffi::FMOD_CHANNELMASK, i32, SpeakerMode), Error> {
        unsafe {
            let mut channelmask = ffi::FMOD_CHANNELMASK::default();
            let mut numchannels = i32::default();
            let mut source_speakermode = ffi::FMOD_SPEAKERMODE::default();
            match ffi::FMOD_DSP_GetChannelFormat(
                self.pointer,
                &mut channelmask,
                &mut numchannels,
                &mut source_speakermode,
            ) {
                ffi::FMOD_OK => Ok((
                    channelmask,
                    numchannels,
                    SpeakerMode::from(source_speakermode)?,
                )),
                error => Err(err_fmod!("FMOD_DSP_GetChannelFormat", error)),
            }
        }
    }
    pub fn get_output_channel_format(
        &self,
        inmask: ffi::FMOD_CHANNELMASK,
        inchannels: i32,
        inspeakermode: SpeakerMode,
    ) -> Result<(ffi::FMOD_CHANNELMASK, i32, SpeakerMode), Error> {
        unsafe {
            let mut outmask = ffi::FMOD_CHANNELMASK::default();
            let mut outchannels = i32::default();
            let mut outspeakermode = ffi::FMOD_SPEAKERMODE::default();
            match ffi::FMOD_DSP_GetOutputChannelFormat(
                self.pointer,
                inmask,
                inchannels,
                inspeakermode.into(),
                &mut outmask,
                &mut outchannels,
                &mut outspeakermode,
            ) {
                ffi::FMOD_OK => Ok((outmask, outchannels, SpeakerMode::from(outspeakermode)?)),
                error => Err(err_fmod!("FMOD_DSP_GetOutputChannelFormat", error)),
            }
        }
    }
    pub fn reset(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_Reset(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_Reset", error)),
            }
        }
    }
    pub fn set_parameter_float(&self, index: i32, value: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_SetParameterFloat(self.pointer, index, value) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_SetParameterFloat", error)),
            }
        }
    }
    pub fn set_parameter_int(&self, index: i32, value: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_SetParameterInt(self.pointer, index, value) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_SetParameterInt", error)),
            }
        }
    }
    pub fn set_parameter_bool(&self, index: i32, value: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_SetParameterBool(self.pointer, index, from_bool!(value)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_SetParameterBool", error)),
            }
        }
    }
    pub fn set_parameter_data(
        &self,
        index: i32,
        data: *mut c_void,
        length: u32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_SetParameterData(self.pointer, index, data, length) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_SetParameterData", error)),
            }
        }
    }
    pub fn get_parameter_float(
        &self,
        index: i32,
        valuestrlen: i32,
    ) -> Result<(f32, String), Error> {
        unsafe {
            let mut value = f32::default();
            let valuestr = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            match ffi::FMOD_DSP_GetParameterFloat(
                self.pointer,
                index,
                &mut value,
                valuestr,
                valuestrlen,
            ) {
                ffi::FMOD_OK => Ok((
                    value,
                    CString::from_raw(valuestr)
                        .into_string()
                        .map_err(Error::String)?,
                )),
                error => Err(err_fmod!("FMOD_DSP_GetParameterFloat", error)),
            }
        }
    }
    pub fn get_parameter_int(&self, index: i32, valuestrlen: i32) -> Result<(i32, String), Error> {
        unsafe {
            let mut value = i32::default();
            let valuestr = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            match ffi::FMOD_DSP_GetParameterInt(
                self.pointer,
                index,
                &mut value,
                valuestr,
                valuestrlen,
            ) {
                ffi::FMOD_OK => Ok((
                    value,
                    CString::from_raw(valuestr)
                        .into_string()
                        .map_err(Error::String)?,
                )),
                error => Err(err_fmod!("FMOD_DSP_GetParameterInt", error)),
            }
        }
    }
    pub fn get_parameter_bool(
        &self,
        index: i32,
        valuestrlen: i32,
    ) -> Result<(bool, String), Error> {
        unsafe {
            let mut value = ffi::FMOD_BOOL::default();
            let valuestr = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            match ffi::FMOD_DSP_GetParameterBool(
                self.pointer,
                index,
                &mut value,
                valuestr,
                valuestrlen,
            ) {
                ffi::FMOD_OK => Ok((
                    to_bool!(value),
                    CString::from_raw(valuestr)
                        .into_string()
                        .map_err(Error::String)?,
                )),
                error => Err(err_fmod!("FMOD_DSP_GetParameterBool", error)),
            }
        }
    }
    pub fn get_parameter_data(
        &self,
        index: i32,
        valuestrlen: i32,
    ) -> Result<(*mut c_void, u32, String), Error> {
        unsafe {
            let mut data = null_mut();
            let mut length = u32::default();
            let valuestr = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            match ffi::FMOD_DSP_GetParameterData(
                self.pointer,
                index,
                &mut data,
                &mut length,
                valuestr,
                valuestrlen,
            ) {
                ffi::FMOD_OK => Ok((
                    data,
                    length,
                    CString::from_raw(valuestr)
                        .into_string()
                        .map_err(Error::String)?,
                )),
                error => Err(err_fmod!("FMOD_DSP_GetParameterData", error)),
            }
        }
    }
    pub fn get_num_parameters(&self) -> Result<i32, Error> {
        unsafe {
            let mut numparams = i32::default();
            match ffi::FMOD_DSP_GetNumParameters(self.pointer, &mut numparams) {
                ffi::FMOD_OK => Ok(numparams),
                error => Err(err_fmod!("FMOD_DSP_GetNumParameters", error)),
            }
        }
    }
    pub fn get_parameter_info(&self, index: i32) -> Result<DspParameterDesc, Error> {
        unsafe {
            let mut desc = null_mut();
            match ffi::FMOD_DSP_GetParameterInfo(self.pointer, index, &mut desc) {
                ffi::FMOD_OK => Ok(DspParameterDesc::try_from(*desc)?),
                error => Err(err_fmod!("FMOD_DSP_GetParameterInfo", error)),
            }
        }
    }
    pub fn get_data_parameter_index(&self, datatype: i32) -> Result<i32, Error> {
        unsafe {
            let mut index = i32::default();
            match ffi::FMOD_DSP_GetDataParameterIndex(self.pointer, datatype, &mut index) {
                ffi::FMOD_OK => Ok(index),
                error => Err(err_fmod!("FMOD_DSP_GetDataParameterIndex", error)),
            }
        }
    }
    pub fn show_config_dialog(&self, hwnd: *mut c_void, show: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_ShowConfigDialog(self.pointer, hwnd, from_bool!(show)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_ShowConfigDialog", error)),
            }
        }
    }
    pub fn get_info(&self) -> Result<(String, u32, i32, i32, i32), Error> {
        unsafe {
            let name = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut version = u32::default();
            let mut channels = i32::default();
            let mut configwidth = i32::default();
            let mut configheight = i32::default();
            match ffi::FMOD_DSP_GetInfo(
                self.pointer,
                name,
                &mut version,
                &mut channels,
                &mut configwidth,
                &mut configheight,
            ) {
                ffi::FMOD_OK => Ok((
                    CString::from_raw(name)
                        .into_string()
                        .map_err(Error::String)?,
                    version,
                    channels,
                    configwidth,
                    configheight,
                )),
                error => Err(err_fmod!("FMOD_DSP_GetInfo", error)),
            }
        }
    }
    pub fn get_type(&self) -> Result<DspType, Error> {
        unsafe {
            let mut type_ = ffi::FMOD_DSP_TYPE::default();
            match ffi::FMOD_DSP_GetType(self.pointer, &mut type_) {
                ffi::FMOD_OK => Ok(DspType::from(type_)?),
                error => Err(err_fmod!("FMOD_DSP_GetType", error)),
            }
        }
    }
    pub fn get_idle(&self) -> Result<bool, Error> {
        unsafe {
            let mut idle = ffi::FMOD_BOOL::default();
            match ffi::FMOD_DSP_GetIdle(self.pointer, &mut idle) {
                ffi::FMOD_OK => Ok(to_bool!(idle)),
                error => Err(err_fmod!("FMOD_DSP_GetIdle", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_SetUserData", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_DSP_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_DSP_GetUserData", error)),
            }
        }
    }
    pub fn set_metering_enabled(
        &self,
        input_enabled: bool,
        output_enabled: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSP_SetMeteringEnabled(
                self.pointer,
                from_bool!(input_enabled),
                from_bool!(output_enabled),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSP_SetMeteringEnabled", error)),
            }
        }
    }
    pub fn get_metering_enabled(&self) -> Result<(bool, bool), Error> {
        unsafe {
            let mut input_enabled = ffi::FMOD_BOOL::default();
            let mut output_enabled = ffi::FMOD_BOOL::default();
            match ffi::FMOD_DSP_GetMeteringEnabled(
                self.pointer,
                &mut input_enabled,
                &mut output_enabled,
            ) {
                ffi::FMOD_OK => Ok((to_bool!(input_enabled), to_bool!(output_enabled))),
                error => Err(err_fmod!("FMOD_DSP_GetMeteringEnabled", error)),
            }
        }
    }
    pub fn get_metering_info(&self) -> Result<(DspMeteringInfo, DspMeteringInfo), Error> {
        unsafe {
            let mut input_info = ffi::FMOD_DSP_METERING_INFO::default();
            let mut output_info = ffi::FMOD_DSP_METERING_INFO::default();
            match ffi::FMOD_DSP_GetMeteringInfo(self.pointer, &mut input_info, &mut output_info) {
                ffi::FMOD_OK => Ok((
                    DspMeteringInfo::try_from(input_info)?,
                    DspMeteringInfo::try_from(output_info)?,
                )),
                error => Err(err_fmod!("FMOD_DSP_GetMeteringInfo", error)),
            }
        }
    }
    pub fn get_cpu_usage(&self) -> Result<(u32, u32), Error> {
        unsafe {
            let mut exclusive = u32::default();
            let mut inclusive = u32::default();
            match ffi::FMOD_DSP_GetCPUUsage(self.pointer, &mut exclusive, &mut inclusive) {
                ffi::FMOD_OK => Ok((exclusive, inclusive)),
                error => Err(err_fmod!("FMOD_DSP_GetCPUUsage", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DspConnection {
    pointer: *mut ffi::FMOD_DSPCONNECTION,
}

impl DspConnection {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_DSPCONNECTION) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_DSPCONNECTION {
        self.pointer
    }
    pub fn get_input(&self) -> Result<Dsp, Error> {
        unsafe {
            let mut input = null_mut();
            match ffi::FMOD_DSPConnection_GetInput(self.pointer, &mut input) {
                ffi::FMOD_OK => Ok(Dsp::from(input)),
                error => Err(err_fmod!("FMOD_DSPConnection_GetInput", error)),
            }
        }
    }
    pub fn get_output(&self) -> Result<Dsp, Error> {
        unsafe {
            let mut output = null_mut();
            match ffi::FMOD_DSPConnection_GetOutput(self.pointer, &mut output) {
                ffi::FMOD_OK => Ok(Dsp::from(output)),
                error => Err(err_fmod!("FMOD_DSPConnection_GetOutput", error)),
            }
        }
    }
    pub fn set_mix(&self, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSPConnection_SetMix(self.pointer, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSPConnection_SetMix", error)),
            }
        }
    }
    pub fn get_mix(&self) -> Result<f32, Error> {
        unsafe {
            let mut volume = f32::default();
            match ffi::FMOD_DSPConnection_GetMix(self.pointer, &mut volume) {
                ffi::FMOD_OK => Ok(volume),
                error => Err(err_fmod!("FMOD_DSPConnection_GetMix", error)),
            }
        }
    }
    pub fn set_mix_matrix(
        &self,
        matrix: Option<*mut f32>,
        outchannels: i32,
        inchannels: i32,
        inchannel_hop: Option<i32>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSPConnection_SetMixMatrix(
                self.pointer,
                matrix.unwrap_or(null_mut()),
                outchannels,
                inchannels,
                inchannel_hop.unwrap_or(0),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSPConnection_SetMixMatrix", error)),
            }
        }
    }
    pub fn get_mix_matrix(&self, inchannel_hop: Option<i32>) -> Result<(f32, i32, i32), Error> {
        unsafe {
            let mut matrix = f32::default();
            let mut outchannels = i32::default();
            let mut inchannels = i32::default();
            match ffi::FMOD_DSPConnection_GetMixMatrix(
                self.pointer,
                &mut matrix,
                &mut outchannels,
                &mut inchannels,
                inchannel_hop.unwrap_or(0),
            ) {
                ffi::FMOD_OK => Ok((matrix, outchannels, inchannels)),
                error => Err(err_fmod!("FMOD_DSPConnection_GetMixMatrix", error)),
            }
        }
    }
    pub fn get_type(&self) -> Result<DspConnectionType, Error> {
        unsafe {
            let mut type_ = ffi::FMOD_DSPCONNECTION_TYPE::default();
            match ffi::FMOD_DSPConnection_GetType(self.pointer, &mut type_) {
                ffi::FMOD_OK => Ok(DspConnectionType::from(type_)?),
                error => Err(err_fmod!("FMOD_DSPConnection_GetType", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_DSPConnection_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_DSPConnection_SetUserData", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_DSPConnection_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_DSPConnection_GetUserData", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Geometry {
    pointer: *mut ffi::FMOD_GEOMETRY,
}

impl Geometry {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_GEOMETRY) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_GEOMETRY {
        self.pointer
    }
    pub fn release(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Geometry_Release(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Geometry_Release", error)),
            }
        }
    }
    pub fn add_polygon(
        &self,
        directocclusion: f32,
        reverbocclusion: f32,
        doublesided: bool,
        numvertices: i32,
        vertices: Vector,
    ) -> Result<i32, Error> {
        unsafe {
            let mut polygonindex = i32::default();
            match ffi::FMOD_Geometry_AddPolygon(
                self.pointer,
                directocclusion,
                reverbocclusion,
                from_bool!(doublesided),
                numvertices,
                &vertices.into(),
                &mut polygonindex,
            ) {
                ffi::FMOD_OK => Ok(polygonindex),
                error => Err(err_fmod!("FMOD_Geometry_AddPolygon", error)),
            }
        }
    }
    pub fn get_num_polygons(&self) -> Result<i32, Error> {
        unsafe {
            let mut numpolygons = i32::default();
            match ffi::FMOD_Geometry_GetNumPolygons(self.pointer, &mut numpolygons) {
                ffi::FMOD_OK => Ok(numpolygons),
                error => Err(err_fmod!("FMOD_Geometry_GetNumPolygons", error)),
            }
        }
    }
    pub fn get_max_polygons(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let mut maxpolygons = i32::default();
            let mut maxvertices = i32::default();
            match ffi::FMOD_Geometry_GetMaxPolygons(
                self.pointer,
                &mut maxpolygons,
                &mut maxvertices,
            ) {
                ffi::FMOD_OK => Ok((maxpolygons, maxvertices)),
                error => Err(err_fmod!("FMOD_Geometry_GetMaxPolygons", error)),
            }
        }
    }
    pub fn get_polygon_num_vertices(&self, index: i32) -> Result<i32, Error> {
        unsafe {
            let mut numvertices = i32::default();
            match ffi::FMOD_Geometry_GetPolygonNumVertices(self.pointer, index, &mut numvertices) {
                ffi::FMOD_OK => Ok(numvertices),
                error => Err(err_fmod!("FMOD_Geometry_GetPolygonNumVertices", error)),
            }
        }
    }
    pub fn set_polygon_vertex(
        &self,
        index: i32,
        vertexindex: i32,
        vertex: Vector,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Geometry_SetPolygonVertex(
                self.pointer,
                index,
                vertexindex,
                &vertex.into(),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Geometry_SetPolygonVertex", error)),
            }
        }
    }
    pub fn get_polygon_vertex(&self, index: i32, vertexindex: i32) -> Result<Vector, Error> {
        unsafe {
            let mut vertex = ffi::FMOD_VECTOR::default();
            match ffi::FMOD_Geometry_GetPolygonVertex(self.pointer, index, vertexindex, &mut vertex)
            {
                ffi::FMOD_OK => Ok(Vector::try_from(vertex)?),
                error => Err(err_fmod!("FMOD_Geometry_GetPolygonVertex", error)),
            }
        }
    }
    pub fn set_polygon_attributes(
        &self,
        index: i32,
        directocclusion: f32,
        reverbocclusion: f32,
        doublesided: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Geometry_SetPolygonAttributes(
                self.pointer,
                index,
                directocclusion,
                reverbocclusion,
                from_bool!(doublesided),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Geometry_SetPolygonAttributes", error)),
            }
        }
    }
    pub fn get_polygon_attributes(&self, index: i32) -> Result<(f32, f32, bool), Error> {
        unsafe {
            let mut directocclusion = f32::default();
            let mut reverbocclusion = f32::default();
            let mut doublesided = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Geometry_GetPolygonAttributes(
                self.pointer,
                index,
                &mut directocclusion,
                &mut reverbocclusion,
                &mut doublesided,
            ) {
                ffi::FMOD_OK => Ok((directocclusion, reverbocclusion, to_bool!(doublesided))),
                error => Err(err_fmod!("FMOD_Geometry_GetPolygonAttributes", error)),
            }
        }
    }
    pub fn set_active(&self, active: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Geometry_SetActive(self.pointer, from_bool!(active)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Geometry_SetActive", error)),
            }
        }
    }
    pub fn get_active(&self) -> Result<bool, Error> {
        unsafe {
            let mut active = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Geometry_GetActive(self.pointer, &mut active) {
                ffi::FMOD_OK => Ok(to_bool!(active)),
                error => Err(err_fmod!("FMOD_Geometry_GetActive", error)),
            }
        }
    }
    pub fn set_rotation(&self, forward: Option<Vector>, up: Option<Vector>) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Geometry_SetRotation(
                self.pointer,
                forward
                    .map(|value| &value.into() as *const _)
                    .unwrap_or(null()),
                up.map(|value| &value.into() as *const _).unwrap_or(null()),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Geometry_SetRotation", error)),
            }
        }
    }
    pub fn get_rotation(&self) -> Result<(Vector, Vector), Error> {
        unsafe {
            let mut forward = ffi::FMOD_VECTOR::default();
            let mut up = ffi::FMOD_VECTOR::default();
            match ffi::FMOD_Geometry_GetRotation(self.pointer, &mut forward, &mut up) {
                ffi::FMOD_OK => Ok((Vector::try_from(forward)?, Vector::try_from(up)?)),
                error => Err(err_fmod!("FMOD_Geometry_GetRotation", error)),
            }
        }
    }
    pub fn set_position(&self, position: Vector) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Geometry_SetPosition(self.pointer, &position.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Geometry_SetPosition", error)),
            }
        }
    }
    pub fn get_position(&self) -> Result<Vector, Error> {
        unsafe {
            let mut position = ffi::FMOD_VECTOR::default();
            match ffi::FMOD_Geometry_GetPosition(self.pointer, &mut position) {
                ffi::FMOD_OK => Ok(Vector::try_from(position)?),
                error => Err(err_fmod!("FMOD_Geometry_GetPosition", error)),
            }
        }
    }
    pub fn set_scale(&self, scale: Vector) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Geometry_SetScale(self.pointer, &scale.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Geometry_SetScale", error)),
            }
        }
    }
    pub fn get_scale(&self) -> Result<Vector, Error> {
        unsafe {
            let mut scale = ffi::FMOD_VECTOR::default();
            match ffi::FMOD_Geometry_GetScale(self.pointer, &mut scale) {
                ffi::FMOD_OK => Ok(Vector::try_from(scale)?),
                error => Err(err_fmod!("FMOD_Geometry_GetScale", error)),
            }
        }
    }
    pub fn save(&self) -> Result<(*mut c_void, i32), Error> {
        unsafe {
            let data = null_mut();
            let mut datasize = i32::default();
            match ffi::FMOD_Geometry_Save(self.pointer, data, &mut datasize) {
                ffi::FMOD_OK => Ok((data, datasize)),
                error => Err(err_fmod!("FMOD_Geometry_Save", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Geometry_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Geometry_SetUserData", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_Geometry_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_Geometry_GetUserData", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Polygon {
    pointer: *mut ffi::FMOD_POLYGON,
}

impl Polygon {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_POLYGON) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_POLYGON {
        self.pointer
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Reverb3d {
    pointer: *mut ffi::FMOD_REVERB3D,
}

impl Reverb3d {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_REVERB3D) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_REVERB3D {
        self.pointer
    }
    pub fn release(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Reverb3D_Release(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Reverb3D_Release", error)),
            }
        }
    }
    pub fn set_3d_attributes(
        &self,
        position: Option<Vector>,
        mindistance: f32,
        maxdistance: f32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Reverb3D_Set3DAttributes(
                self.pointer,
                position
                    .map(|value| &value.into() as *const _)
                    .unwrap_or(null()),
                mindistance,
                maxdistance,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Reverb3D_Set3DAttributes", error)),
            }
        }
    }
    pub fn get_3d_attributes(&self) -> Result<(Vector, f32, f32), Error> {
        unsafe {
            let mut position = ffi::FMOD_VECTOR::default();
            let mut mindistance = f32::default();
            let mut maxdistance = f32::default();
            match ffi::FMOD_Reverb3D_Get3DAttributes(
                self.pointer,
                &mut position,
                &mut mindistance,
                &mut maxdistance,
            ) {
                ffi::FMOD_OK => Ok((Vector::try_from(position)?, mindistance, maxdistance)),
                error => Err(err_fmod!("FMOD_Reverb3D_Get3DAttributes", error)),
            }
        }
    }
    pub fn set_properties(&self, properties: ReverbProperties) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Reverb3D_SetProperties(self.pointer, &properties.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Reverb3D_SetProperties", error)),
            }
        }
    }
    pub fn get_properties(&self) -> Result<ReverbProperties, Error> {
        unsafe {
            let mut properties = ffi::FMOD_REVERB_PROPERTIES::default();
            match ffi::FMOD_Reverb3D_GetProperties(self.pointer, &mut properties) {
                ffi::FMOD_OK => Ok(ReverbProperties::try_from(properties)?),
                error => Err(err_fmod!("FMOD_Reverb3D_GetProperties", error)),
            }
        }
    }
    pub fn set_active(&self, active: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Reverb3D_SetActive(self.pointer, from_bool!(active)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Reverb3D_SetActive", error)),
            }
        }
    }
    pub fn get_active(&self) -> Result<bool, Error> {
        unsafe {
            let mut active = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Reverb3D_GetActive(self.pointer, &mut active) {
                ffi::FMOD_OK => Ok(to_bool!(active)),
                error => Err(err_fmod!("FMOD_Reverb3D_GetActive", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Reverb3D_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Reverb3D_SetUserData", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_Reverb3D_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_Reverb3D_GetUserData", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sound {
    pointer: *mut ffi::FMOD_SOUND,
}

impl Sound {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_SOUND) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_SOUND {
        self.pointer
    }
    pub fn release(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_Release(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_Release", error)),
            }
        }
    }
    pub fn get_system_object(&self) -> Result<System, Error> {
        unsafe {
            let mut system = null_mut();
            match ffi::FMOD_Sound_GetSystemObject(self.pointer, &mut system) {
                ffi::FMOD_OK => Ok(System::from(system)),
                error => Err(err_fmod!("FMOD_Sound_GetSystemObject", error)),
            }
        }
    }
    pub fn lock(
        &self,
        offset: u32,
        length: u32,
    ) -> Result<(*mut c_void, *mut c_void, u32, u32), Error> {
        unsafe {
            let mut ptr_1 = null_mut();
            let mut ptr_2 = null_mut();
            let mut len_1 = u32::default();
            let mut len_2 = u32::default();
            match ffi::FMOD_Sound_Lock(
                self.pointer,
                offset,
                length,
                &mut ptr_1,
                &mut ptr_2,
                &mut len_1,
                &mut len_2,
            ) {
                ffi::FMOD_OK => Ok((ptr_1, ptr_2, len_1, len_2)),
                error => Err(err_fmod!("FMOD_Sound_Lock", error)),
            }
        }
    }
    pub fn unlock(
        &self,
        ptr_1: *mut c_void,
        ptr_2: *mut c_void,
        len_1: u32,
        len_2: u32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_Unlock(self.pointer, ptr_1, ptr_2, len_1, len_2) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_Unlock", error)),
            }
        }
    }
    pub fn set_defaults(&self, frequency: f32, priority: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_SetDefaults(self.pointer, frequency, priority) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_SetDefaults", error)),
            }
        }
    }
    pub fn get_defaults(&self) -> Result<(f32, i32), Error> {
        unsafe {
            let mut frequency = f32::default();
            let mut priority = i32::default();
            match ffi::FMOD_Sound_GetDefaults(self.pointer, &mut frequency, &mut priority) {
                ffi::FMOD_OK => Ok((frequency, priority)),
                error => Err(err_fmod!("FMOD_Sound_GetDefaults", error)),
            }
        }
    }
    pub fn set_3d_min_max_distance(&self, min: f32, max: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_Set3DMinMaxDistance(self.pointer, min, max) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_Set3DMinMaxDistance", error)),
            }
        }
    }
    pub fn get_3d_min_max_distance(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut min = f32::default();
            let mut max = f32::default();
            match ffi::FMOD_Sound_Get3DMinMaxDistance(self.pointer, &mut min, &mut max) {
                ffi::FMOD_OK => Ok((min, max)),
                error => Err(err_fmod!("FMOD_Sound_Get3DMinMaxDistance", error)),
            }
        }
    }
    pub fn set_3d_cone_settings(
        &self,
        insideconeangle: f32,
        outsideconeangle: f32,
        outsidevolume: f32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_Set3DConeSettings(
                self.pointer,
                insideconeangle,
                outsideconeangle,
                outsidevolume,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_Set3DConeSettings", error)),
            }
        }
    }
    pub fn get_3d_cone_settings(&self) -> Result<(f32, f32, f32), Error> {
        unsafe {
            let mut insideconeangle = f32::default();
            let mut outsideconeangle = f32::default();
            let mut outsidevolume = f32::default();
            match ffi::FMOD_Sound_Get3DConeSettings(
                self.pointer,
                &mut insideconeangle,
                &mut outsideconeangle,
                &mut outsidevolume,
            ) {
                ffi::FMOD_OK => Ok((insideconeangle, outsideconeangle, outsidevolume)),
                error => Err(err_fmod!("FMOD_Sound_Get3DConeSettings", error)),
            }
        }
    }
    pub fn set_3d_custom_rolloff(&self, points: Vec<Vector>) -> Result<(), Error> {
        unsafe {
            let numpoints = points.len() as i32;
            match ffi::FMOD_Sound_Set3DCustomRolloff(
                self.pointer,
                vec_as_mut_ptr(points, |point| point.into()),
                numpoints,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_Set3DCustomRolloff", error)),
            }
        }
    }
    pub fn get_3d_custom_rolloff(&self) -> Result<Vec<Vector>, Error> {
        unsafe {
            let mut points = null_mut();
            let mut numpoints = i32::default();
            match ffi::FMOD_Sound_Get3DCustomRolloff(self.pointer, &mut points, &mut numpoints) {
                ffi::FMOD_OK => Ok(to_vec!(points, numpoints, Vector::try_from)?),
                error => Err(err_fmod!("FMOD_Sound_Get3DCustomRolloff", error)),
            }
        }
    }
    pub fn get_sub_sound(&self, index: i32) -> Result<Sound, Error> {
        unsafe {
            let mut subsound = null_mut();
            match ffi::FMOD_Sound_GetSubSound(self.pointer, index, &mut subsound) {
                ffi::FMOD_OK => Ok(Sound::from(subsound)),
                error => Err(err_fmod!("FMOD_Sound_GetSubSound", error)),
            }
        }
    }
    pub fn get_sub_sound_parent(&self) -> Result<Sound, Error> {
        unsafe {
            let mut parentsound = null_mut();
            match ffi::FMOD_Sound_GetSubSoundParent(self.pointer, &mut parentsound) {
                ffi::FMOD_OK => Ok(Sound::from(parentsound)),
                error => Err(err_fmod!("FMOD_Sound_GetSubSoundParent", error)),
            }
        }
    }
    pub fn get_name(&self, namelen: i32) -> Result<String, Error> {
        unsafe {
            let name = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            match ffi::FMOD_Sound_GetName(self.pointer, name, namelen) {
                ffi::FMOD_OK => Ok(CString::from_raw(name)
                    .into_string()
                    .map_err(Error::String)?),
                error => Err(err_fmod!("FMOD_Sound_GetName", error)),
            }
        }
    }
    pub fn get_length(&self, lengthtype: ffi::FMOD_TIMEUNIT) -> Result<u32, Error> {
        unsafe {
            let mut length = u32::default();
            match ffi::FMOD_Sound_GetLength(self.pointer, &mut length, lengthtype) {
                ffi::FMOD_OK => Ok(length),
                error => Err(err_fmod!("FMOD_Sound_GetLength", error)),
            }
        }
    }
    pub fn get_format(&self) -> Result<(SoundType, SoundFormat, i32, i32), Error> {
        unsafe {
            let mut type_ = ffi::FMOD_SOUND_TYPE::default();
            let mut format = ffi::FMOD_SOUND_FORMAT::default();
            let mut channels = i32::default();
            let mut bits = i32::default();
            match ffi::FMOD_Sound_GetFormat(
                self.pointer,
                &mut type_,
                &mut format,
                &mut channels,
                &mut bits,
            ) {
                ffi::FMOD_OK => Ok((
                    SoundType::from(type_)?,
                    SoundFormat::from(format)?,
                    channels,
                    bits,
                )),
                error => Err(err_fmod!("FMOD_Sound_GetFormat", error)),
            }
        }
    }
    pub fn get_num_sub_sounds(&self) -> Result<i32, Error> {
        unsafe {
            let mut numsubsounds = i32::default();
            match ffi::FMOD_Sound_GetNumSubSounds(self.pointer, &mut numsubsounds) {
                ffi::FMOD_OK => Ok(numsubsounds),
                error => Err(err_fmod!("FMOD_Sound_GetNumSubSounds", error)),
            }
        }
    }
    pub fn get_num_tags(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let mut numtags = i32::default();
            let mut numtagsupdated = i32::default();
            match ffi::FMOD_Sound_GetNumTags(self.pointer, &mut numtags, &mut numtagsupdated) {
                ffi::FMOD_OK => Ok((numtags, numtagsupdated)),
                error => Err(err_fmod!("FMOD_Sound_GetNumTags", error)),
            }
        }
    }
    pub fn get_tag(&self, name: &str, index: Option<i32>) -> Result<Tag, Error> {
        unsafe {
            let mut tag = ffi::FMOD_TAG::default();
            match ffi::FMOD_Sound_GetTag(
                self.pointer,
                CString::new(name)?.as_ptr(),
                index.unwrap_or(0),
                &mut tag,
            ) {
                ffi::FMOD_OK => Ok(Tag::try_from(tag)?),
                error => Err(err_fmod!("FMOD_Sound_GetTag", error)),
            }
        }
    }
    pub fn get_open_state(&self) -> Result<(OpenState, u32, bool, bool), Error> {
        unsafe {
            let mut openstate = ffi::FMOD_OPENSTATE::default();
            let mut percentbuffered = u32::default();
            let mut starving = ffi::FMOD_BOOL::default();
            let mut diskbusy = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Sound_GetOpenState(
                self.pointer,
                &mut openstate,
                &mut percentbuffered,
                &mut starving,
                &mut diskbusy,
            ) {
                ffi::FMOD_OK => Ok((
                    OpenState::from(openstate)?,
                    percentbuffered,
                    to_bool!(starving),
                    to_bool!(diskbusy),
                )),
                error => Err(err_fmod!("FMOD_Sound_GetOpenState", error)),
            }
        }
    }
    pub fn read_data(&self, buffer: *mut c_void, length: u32) -> Result<u32, Error> {
        unsafe {
            let mut read = u32::default();
            match ffi::FMOD_Sound_ReadData(self.pointer, buffer, length, &mut read) {
                ffi::FMOD_OK => Ok(read),
                error => Err(err_fmod!("FMOD_Sound_ReadData", error)),
            }
        }
    }
    pub fn seek_data(&self, pcm: u32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_SeekData(self.pointer, pcm) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_SeekData", error)),
            }
        }
    }
    pub fn set_sound_group(&self, soundgroup: SoundGroup) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_SetSoundGroup(self.pointer, soundgroup.as_mut_ptr()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_SetSoundGroup", error)),
            }
        }
    }
    pub fn get_sound_group(&self) -> Result<SoundGroup, Error> {
        unsafe {
            let mut soundgroup = null_mut();
            match ffi::FMOD_Sound_GetSoundGroup(self.pointer, &mut soundgroup) {
                ffi::FMOD_OK => Ok(SoundGroup::from(soundgroup)),
                error => Err(err_fmod!("FMOD_Sound_GetSoundGroup", error)),
            }
        }
    }
    pub fn get_num_sync_points(&self) -> Result<i32, Error> {
        unsafe {
            let mut numsyncpoints = i32::default();
            match ffi::FMOD_Sound_GetNumSyncPoints(self.pointer, &mut numsyncpoints) {
                ffi::FMOD_OK => Ok(numsyncpoints),
                error => Err(err_fmod!("FMOD_Sound_GetNumSyncPoints", error)),
            }
        }
    }
    pub fn get_sync_point(&self, index: i32) -> Result<SyncPoint, Error> {
        unsafe {
            let mut point = null_mut();
            match ffi::FMOD_Sound_GetSyncPoint(self.pointer, index, &mut point) {
                ffi::FMOD_OK => Ok(SyncPoint::from(point)),
                error => Err(err_fmod!("FMOD_Sound_GetSyncPoint", error)),
            }
        }
    }
    pub fn get_sync_point_info(
        &self,
        point: SyncPoint,
        namelen: i32,
        offsettype: ffi::FMOD_TIMEUNIT,
    ) -> Result<(String, u32), Error> {
        unsafe {
            let name = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut offset = u32::default();
            match ffi::FMOD_Sound_GetSyncPointInfo(
                self.pointer,
                point.as_mut_ptr(),
                name,
                namelen,
                &mut offset,
                offsettype,
            ) {
                ffi::FMOD_OK => Ok((
                    CString::from_raw(name)
                        .into_string()
                        .map_err(Error::String)?,
                    offset,
                )),
                error => Err(err_fmod!("FMOD_Sound_GetSyncPointInfo", error)),
            }
        }
    }
    pub fn add_sync_point(
        &self,
        offset: u32,
        offsettype: ffi::FMOD_TIMEUNIT,
        name: Option<String>,
    ) -> Result<SyncPoint, Error> {
        unsafe {
            let mut point = null_mut();
            match ffi::FMOD_Sound_AddSyncPoint(
                self.pointer,
                offset,
                offsettype,
                name.map(|value| CString::new(value).map(|value| value.as_ptr()))
                    .unwrap_or(Ok(null_mut()))?,
                &mut point,
            ) {
                ffi::FMOD_OK => Ok(SyncPoint::from(point)),
                error => Err(err_fmod!("FMOD_Sound_AddSyncPoint", error)),
            }
        }
    }
    pub fn delete_sync_point(&self, point: SyncPoint) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_DeleteSyncPoint(self.pointer, point.as_mut_ptr()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_DeleteSyncPoint", error)),
            }
        }
    }
    pub fn set_mode(&self, mode: ffi::FMOD_MODE) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_SetMode(self.pointer, mode) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_SetMode", error)),
            }
        }
    }
    pub fn get_mode(&self) -> Result<ffi::FMOD_MODE, Error> {
        unsafe {
            let mut mode = ffi::FMOD_MODE::default();
            match ffi::FMOD_Sound_GetMode(self.pointer, &mut mode) {
                ffi::FMOD_OK => Ok(mode),
                error => Err(err_fmod!("FMOD_Sound_GetMode", error)),
            }
        }
    }
    pub fn set_loop_count(&self, loopcount: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_SetLoopCount(self.pointer, loopcount) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_SetLoopCount", error)),
            }
        }
    }
    pub fn get_loop_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut loopcount = i32::default();
            match ffi::FMOD_Sound_GetLoopCount(self.pointer, &mut loopcount) {
                ffi::FMOD_OK => Ok(loopcount),
                error => Err(err_fmod!("FMOD_Sound_GetLoopCount", error)),
            }
        }
    }
    pub fn set_loop_points(
        &self,
        loopstart: u32,
        loopstarttype: ffi::FMOD_TIMEUNIT,
        loopend: u32,
        loopendtype: ffi::FMOD_TIMEUNIT,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_SetLoopPoints(
                self.pointer,
                loopstart,
                loopstarttype,
                loopend,
                loopendtype,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_SetLoopPoints", error)),
            }
        }
    }
    pub fn get_loop_points(
        &self,
        loopstarttype: ffi::FMOD_TIMEUNIT,
        loopendtype: ffi::FMOD_TIMEUNIT,
    ) -> Result<(u32, u32), Error> {
        unsafe {
            let mut loopstart = u32::default();
            let mut loopend = u32::default();
            match ffi::FMOD_Sound_GetLoopPoints(
                self.pointer,
                &mut loopstart,
                loopstarttype,
                &mut loopend,
                loopendtype,
            ) {
                ffi::FMOD_OK => Ok((loopstart, loopend)),
                error => Err(err_fmod!("FMOD_Sound_GetLoopPoints", error)),
            }
        }
    }
    pub fn get_music_num_channels(&self) -> Result<i32, Error> {
        unsafe {
            let mut numchannels = i32::default();
            match ffi::FMOD_Sound_GetMusicNumChannels(self.pointer, &mut numchannels) {
                ffi::FMOD_OK => Ok(numchannels),
                error => Err(err_fmod!("FMOD_Sound_GetMusicNumChannels", error)),
            }
        }
    }
    pub fn set_music_channel_volume(&self, channel: i32, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_SetMusicChannelVolume(self.pointer, channel, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_SetMusicChannelVolume", error)),
            }
        }
    }
    pub fn get_music_channel_volume(&self, channel: i32) -> Result<f32, Error> {
        unsafe {
            let mut volume = f32::default();
            match ffi::FMOD_Sound_GetMusicChannelVolume(self.pointer, channel, &mut volume) {
                ffi::FMOD_OK => Ok(volume),
                error => Err(err_fmod!("FMOD_Sound_GetMusicChannelVolume", error)),
            }
        }
    }
    pub fn set_music_speed(&self, speed: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_SetMusicSpeed(self.pointer, speed) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_SetMusicSpeed", error)),
            }
        }
    }
    pub fn get_music_speed(&self) -> Result<f32, Error> {
        unsafe {
            let mut speed = f32::default();
            match ffi::FMOD_Sound_GetMusicSpeed(self.pointer, &mut speed) {
                ffi::FMOD_OK => Ok(speed),
                error => Err(err_fmod!("FMOD_Sound_GetMusicSpeed", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Sound_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Sound_SetUserData", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_Sound_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_Sound_GetUserData", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SoundGroup {
    pointer: *mut ffi::FMOD_SOUNDGROUP,
}

impl SoundGroup {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_SOUNDGROUP) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_SOUNDGROUP {
        self.pointer
    }
    pub fn release(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_SoundGroup_Release(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_SoundGroup_Release", error)),
            }
        }
    }
    pub fn get_system_object(&self) -> Result<System, Error> {
        unsafe {
            let mut system = null_mut();
            match ffi::FMOD_SoundGroup_GetSystemObject(self.pointer, &mut system) {
                ffi::FMOD_OK => Ok(System::from(system)),
                error => Err(err_fmod!("FMOD_SoundGroup_GetSystemObject", error)),
            }
        }
    }
    pub fn set_max_audible(&self, maxaudible: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_SoundGroup_SetMaxAudible(self.pointer, maxaudible) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_SoundGroup_SetMaxAudible", error)),
            }
        }
    }
    pub fn get_max_audible(&self) -> Result<i32, Error> {
        unsafe {
            let mut maxaudible = i32::default();
            match ffi::FMOD_SoundGroup_GetMaxAudible(self.pointer, &mut maxaudible) {
                ffi::FMOD_OK => Ok(maxaudible),
                error => Err(err_fmod!("FMOD_SoundGroup_GetMaxAudible", error)),
            }
        }
    }
    pub fn set_max_audible_behavior(&self, behavior: SoundGroupBehavior) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_SoundGroup_SetMaxAudibleBehavior(self.pointer, behavior.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_SoundGroup_SetMaxAudibleBehavior", error)),
            }
        }
    }
    pub fn get_max_audible_behavior(&self) -> Result<SoundGroupBehavior, Error> {
        unsafe {
            let mut behavior = ffi::FMOD_SOUNDGROUP_BEHAVIOR::default();
            match ffi::FMOD_SoundGroup_GetMaxAudibleBehavior(self.pointer, &mut behavior) {
                ffi::FMOD_OK => Ok(SoundGroupBehavior::from(behavior)?),
                error => Err(err_fmod!("FMOD_SoundGroup_GetMaxAudibleBehavior", error)),
            }
        }
    }
    pub fn set_mute_fade_speed(&self, speed: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_SoundGroup_SetMuteFadeSpeed(self.pointer, speed) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_SoundGroup_SetMuteFadeSpeed", error)),
            }
        }
    }
    pub fn get_mute_fade_speed(&self) -> Result<f32, Error> {
        unsafe {
            let mut speed = f32::default();
            match ffi::FMOD_SoundGroup_GetMuteFadeSpeed(self.pointer, &mut speed) {
                ffi::FMOD_OK => Ok(speed),
                error => Err(err_fmod!("FMOD_SoundGroup_GetMuteFadeSpeed", error)),
            }
        }
    }
    pub fn set_volume(&self, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_SoundGroup_SetVolume(self.pointer, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_SoundGroup_SetVolume", error)),
            }
        }
    }
    pub fn get_volume(&self) -> Result<f32, Error> {
        unsafe {
            let mut volume = f32::default();
            match ffi::FMOD_SoundGroup_GetVolume(self.pointer, &mut volume) {
                ffi::FMOD_OK => Ok(volume),
                error => Err(err_fmod!("FMOD_SoundGroup_GetVolume", error)),
            }
        }
    }
    pub fn stop(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_SoundGroup_Stop(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_SoundGroup_Stop", error)),
            }
        }
    }
    pub fn get_name(&self, namelen: i32) -> Result<String, Error> {
        unsafe {
            let name = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            match ffi::FMOD_SoundGroup_GetName(self.pointer, name, namelen) {
                ffi::FMOD_OK => Ok(CString::from_raw(name)
                    .into_string()
                    .map_err(Error::String)?),
                error => Err(err_fmod!("FMOD_SoundGroup_GetName", error)),
            }
        }
    }
    pub fn get_num_sounds(&self) -> Result<i32, Error> {
        unsafe {
            let mut numsounds = i32::default();
            match ffi::FMOD_SoundGroup_GetNumSounds(self.pointer, &mut numsounds) {
                ffi::FMOD_OK => Ok(numsounds),
                error => Err(err_fmod!("FMOD_SoundGroup_GetNumSounds", error)),
            }
        }
    }
    pub fn get_sound(&self, index: i32) -> Result<Sound, Error> {
        unsafe {
            let mut sound = null_mut();
            match ffi::FMOD_SoundGroup_GetSound(self.pointer, index, &mut sound) {
                ffi::FMOD_OK => Ok(Sound::from(sound)),
                error => Err(err_fmod!("FMOD_SoundGroup_GetSound", error)),
            }
        }
    }
    pub fn get_num_playing(&self) -> Result<i32, Error> {
        unsafe {
            let mut numplaying = i32::default();
            match ffi::FMOD_SoundGroup_GetNumPlaying(self.pointer, &mut numplaying) {
                ffi::FMOD_OK => Ok(numplaying),
                error => Err(err_fmod!("FMOD_SoundGroup_GetNumPlaying", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_SoundGroup_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_SoundGroup_SetUserData", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_SoundGroup_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_SoundGroup_GetUserData", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bank {
    pointer: *mut ffi::FMOD_STUDIO_BANK,
}

impl Bank {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_STUDIO_BANK) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_STUDIO_BANK {
        self.pointer
    }
    pub fn is_valid(&self) -> bool {
        unsafe { to_bool!(ffi::FMOD_Studio_Bank_IsValid(self.pointer)) }
    }
    pub fn get_id(&self) -> Result<Guid, Error> {
        unsafe {
            let mut id = ffi::FMOD_GUID::default();
            match ffi::FMOD_Studio_Bank_GetID(self.pointer, &mut id) {
                ffi::FMOD_OK => Ok(Guid::try_from(id)?),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetID", error)),
            }
        }
    }
    pub fn get_path(&self) -> Result<String, Error> {
        unsafe {
            let mut retrieved = i32::default();
            match ffi::FMOD_Studio_Bank_GetPath(self.pointer, null_mut(), 0, &mut retrieved) {
                ffi::FMOD_OK => {
                    let mut buf = vec![0u8; retrieved as usize];
                    match ffi::FMOD_Studio_Bank_GetPath(
                        self.pointer,
                        buf.as_mut_ptr() as *mut _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        ffi::FMOD_OK => Ok(CString::from_vec_with_nul_unchecked(buf)
                            .into_string()
                            .map_err(Error::String)?),
                        error => Err(err_fmod!("FMOD_Studio_Bank_GetPath", error)),
                    }
                }
                error => Err(err_fmod!("FMOD_Studio_Bank_GetPath", error)),
            }
        }
    }
    pub fn unload(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bank_Unload(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bank_Unload", error)),
            }
        }
    }
    pub fn load_sample_data(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bank_LoadSampleData(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bank_LoadSampleData", error)),
            }
        }
    }
    pub fn unload_sample_data(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bank_UnloadSampleData(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bank_UnloadSampleData", error)),
            }
        }
    }
    pub fn get_loading_state(&self) -> Result<LoadingState, Error> {
        unsafe {
            let mut state = ffi::FMOD_STUDIO_LOADING_STATE::default();
            match ffi::FMOD_Studio_Bank_GetLoadingState(self.pointer, &mut state) {
                ffi::FMOD_OK => Ok(LoadingState::from(state)?),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetLoadingState", error)),
            }
        }
    }
    pub fn get_sample_loading_state(&self) -> Result<LoadingState, Error> {
        unsafe {
            let mut state = ffi::FMOD_STUDIO_LOADING_STATE::default();
            match ffi::FMOD_Studio_Bank_GetSampleLoadingState(self.pointer, &mut state) {
                ffi::FMOD_OK => Ok(LoadingState::from(state)?),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetSampleLoadingState", error)),
            }
        }
    }
    pub fn get_string_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_Studio_Bank_GetStringCount(self.pointer, &mut count) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetStringCount", error)),
            }
        }
    }
    pub fn get_string_info(&self, index: i32, size: i32) -> Result<(Guid, String, i32), Error> {
        unsafe {
            let mut id = ffi::FMOD_GUID::default();
            let path = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut retrieved = i32::default();
            match ffi::FMOD_Studio_Bank_GetStringInfo(
                self.pointer,
                index,
                &mut id,
                path,
                size,
                &mut retrieved,
            ) {
                ffi::FMOD_OK => Ok((
                    Guid::try_from(id)?,
                    CString::from_raw(path)
                        .into_string()
                        .map_err(Error::String)?,
                    retrieved,
                )),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetStringInfo", error)),
            }
        }
    }
    pub fn get_event_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_Studio_Bank_GetEventCount(self.pointer, &mut count) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetEventCount", error)),
            }
        }
    }
    pub fn get_event_list(&self, capacity: i32) -> Result<(EventDescription, i32), Error> {
        unsafe {
            let mut array = null_mut();
            let mut count = i32::default();
            match ffi::FMOD_Studio_Bank_GetEventList(self.pointer, &mut array, capacity, &mut count)
            {
                ffi::FMOD_OK => Ok((EventDescription::from(array), count)),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetEventList", error)),
            }
        }
    }
    pub fn get_bus_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_Studio_Bank_GetBusCount(self.pointer, &mut count) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetBusCount", error)),
            }
        }
    }
    pub fn get_bus_list(&self, capacity: i32) -> Result<(Bus, i32), Error> {
        unsafe {
            let mut array = null_mut();
            let mut count = i32::default();
            match ffi::FMOD_Studio_Bank_GetBusList(self.pointer, &mut array, capacity, &mut count) {
                ffi::FMOD_OK => Ok((Bus::from(array), count)),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetBusList", error)),
            }
        }
    }
    pub fn get_vca_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_Studio_Bank_GetVCACount(self.pointer, &mut count) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetVCACount", error)),
            }
        }
    }
    pub fn get_vca_list(&self, capacity: i32) -> Result<(Vca, i32), Error> {
        unsafe {
            let mut array = null_mut();
            let mut count = i32::default();
            match ffi::FMOD_Studio_Bank_GetVCAList(self.pointer, &mut array, capacity, &mut count) {
                ffi::FMOD_OK => Ok((Vca::from(array), count)),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetVCAList", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_Studio_Bank_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_Studio_Bank_GetUserData", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bank_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bank_SetUserData", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bus {
    pointer: *mut ffi::FMOD_STUDIO_BUS,
}

impl Bus {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_STUDIO_BUS) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_STUDIO_BUS {
        self.pointer
    }
    pub fn is_valid(&self) -> bool {
        unsafe { to_bool!(ffi::FMOD_Studio_Bus_IsValid(self.pointer)) }
    }
    pub fn get_id(&self) -> Result<Guid, Error> {
        unsafe {
            let mut id = ffi::FMOD_GUID::default();
            match ffi::FMOD_Studio_Bus_GetID(self.pointer, &mut id) {
                ffi::FMOD_OK => Ok(Guid::try_from(id)?),
                error => Err(err_fmod!("FMOD_Studio_Bus_GetID", error)),
            }
        }
    }
    pub fn get_path(&self) -> Result<String, Error> {
        unsafe {
            let mut retrieved = i32::default();
            match ffi::FMOD_Studio_Bus_GetPath(self.pointer, null_mut(), 0, &mut retrieved) {
                ffi::FMOD_OK => {
                    let mut buf = vec![0u8; retrieved as usize];
                    match ffi::FMOD_Studio_Bus_GetPath(
                        self.pointer,
                        buf.as_mut_ptr() as *mut _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        ffi::FMOD_OK => Ok(CString::from_vec_with_nul_unchecked(buf)
                            .into_string()
                            .map_err(Error::String)?),
                        error => Err(err_fmod!("FMOD_Studio_Bus_GetPath", error)),
                    }
                }
                error => Err(err_fmod!("FMOD_Studio_Bus_GetPath", error)),
            }
        }
    }
    pub fn get_volume(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut volume = f32::default();
            let mut finalvolume = f32::default();
            match ffi::FMOD_Studio_Bus_GetVolume(self.pointer, &mut volume, &mut finalvolume) {
                ffi::FMOD_OK => Ok((volume, finalvolume)),
                error => Err(err_fmod!("FMOD_Studio_Bus_GetVolume", error)),
            }
        }
    }
    pub fn set_volume(&self, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bus_SetVolume(self.pointer, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bus_SetVolume", error)),
            }
        }
    }
    pub fn get_paused(&self) -> Result<bool, Error> {
        unsafe {
            let mut paused = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_Bus_GetPaused(self.pointer, &mut paused) {
                ffi::FMOD_OK => Ok(to_bool!(paused)),
                error => Err(err_fmod!("FMOD_Studio_Bus_GetPaused", error)),
            }
        }
    }
    pub fn set_paused(&self, paused: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bus_SetPaused(self.pointer, from_bool!(paused)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bus_SetPaused", error)),
            }
        }
    }
    pub fn get_mute(&self) -> Result<bool, Error> {
        unsafe {
            let mut mute = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_Bus_GetMute(self.pointer, &mut mute) {
                ffi::FMOD_OK => Ok(to_bool!(mute)),
                error => Err(err_fmod!("FMOD_Studio_Bus_GetMute", error)),
            }
        }
    }
    pub fn set_mute(&self, mute: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bus_SetMute(self.pointer, from_bool!(mute)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bus_SetMute", error)),
            }
        }
    }
    pub fn stop_all_events(&self, mode: StopMode) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bus_StopAllEvents(self.pointer, mode.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bus_StopAllEvents", error)),
            }
        }
    }
    pub fn get_port_index(&self) -> Result<u64, Error> {
        unsafe {
            let mut index = u64::default();
            match ffi::FMOD_Studio_Bus_GetPortIndex(self.pointer, &mut index) {
                ffi::FMOD_OK => Ok(index),
                error => Err(err_fmod!("FMOD_Studio_Bus_GetPortIndex", error)),
            }
        }
    }
    pub fn set_port_index(&self, index: u64) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bus_SetPortIndex(self.pointer, index) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bus_SetPortIndex", error)),
            }
        }
    }
    pub fn lock_channel_group(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bus_LockChannelGroup(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bus_LockChannelGroup", error)),
            }
        }
    }
    pub fn unlock_channel_group(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_Bus_UnlockChannelGroup(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_Bus_UnlockChannelGroup", error)),
            }
        }
    }
    pub fn get_channel_group(&self) -> Result<ChannelGroup, Error> {
        unsafe {
            let mut group = null_mut();
            match ffi::FMOD_Studio_Bus_GetChannelGroup(self.pointer, &mut group) {
                ffi::FMOD_OK => Ok(ChannelGroup::from(group)),
                error => Err(err_fmod!("FMOD_Studio_Bus_GetChannelGroup", error)),
            }
        }
    }
    pub fn get_cpu_usage(&self) -> Result<(u32, u32), Error> {
        unsafe {
            let mut exclusive = u32::default();
            let mut inclusive = u32::default();
            match ffi::FMOD_Studio_Bus_GetCPUUsage(self.pointer, &mut exclusive, &mut inclusive) {
                ffi::FMOD_OK => Ok((exclusive, inclusive)),
                error => Err(err_fmod!("FMOD_Studio_Bus_GetCPUUsage", error)),
            }
        }
    }
    pub fn get_memory_usage(&self) -> Result<MemoryUsage, Error> {
        unsafe {
            let mut memoryusage = ffi::FMOD_STUDIO_MEMORY_USAGE::default();
            match ffi::FMOD_Studio_Bus_GetMemoryUsage(self.pointer, &mut memoryusage) {
                ffi::FMOD_OK => Ok(MemoryUsage::try_from(memoryusage)?),
                error => Err(err_fmod!("FMOD_Studio_Bus_GetMemoryUsage", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CommandReplay {
    pointer: *mut ffi::FMOD_STUDIO_COMMANDREPLAY,
}

impl CommandReplay {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_STUDIO_COMMANDREPLAY) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_STUDIO_COMMANDREPLAY {
        self.pointer
    }
    pub fn is_valid(&self) -> bool {
        unsafe { to_bool!(ffi::FMOD_Studio_CommandReplay_IsValid(self.pointer)) }
    }
    pub fn get_system(&self) -> Result<Studio, Error> {
        unsafe {
            let mut system = null_mut();
            match ffi::FMOD_Studio_CommandReplay_GetSystem(self.pointer, &mut system) {
                ffi::FMOD_OK => Ok(Studio::from(system)),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_GetSystem", error)),
            }
        }
    }
    pub fn get_length(&self) -> Result<f32, Error> {
        unsafe {
            let mut length = f32::default();
            match ffi::FMOD_Studio_CommandReplay_GetLength(self.pointer, &mut length) {
                ffi::FMOD_OK => Ok(length),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_GetLength", error)),
            }
        }
    }
    pub fn get_command_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_Studio_CommandReplay_GetCommandCount(self.pointer, &mut count) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!(
                    "FMOD_Studio_CommandReplay_GetCommandCount",
                    error
                )),
            }
        }
    }
    pub fn get_command_info(&self, commandindex: i32) -> Result<CommandInfo, Error> {
        unsafe {
            let mut info = ffi::FMOD_STUDIO_COMMAND_INFO::default();
            match ffi::FMOD_Studio_CommandReplay_GetCommandInfo(
                self.pointer,
                commandindex,
                &mut info,
            ) {
                ffi::FMOD_OK => Ok(CommandInfo::try_from(info)?),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_GetCommandInfo", error)),
            }
        }
    }
    pub fn get_command_string(&self, commandindex: i32, length: i32) -> Result<String, Error> {
        unsafe {
            let buffer = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            match ffi::FMOD_Studio_CommandReplay_GetCommandString(
                self.pointer,
                commandindex,
                buffer,
                length,
            ) {
                ffi::FMOD_OK => Ok(CString::from_raw(buffer)
                    .into_string()
                    .map_err(Error::String)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_CommandReplay_GetCommandString",
                    error
                )),
            }
        }
    }
    pub fn get_command_at_time(&self, time: f32) -> Result<i32, Error> {
        unsafe {
            let mut commandindex = i32::default();
            match ffi::FMOD_Studio_CommandReplay_GetCommandAtTime(
                self.pointer,
                time,
                &mut commandindex,
            ) {
                ffi::FMOD_OK => Ok(commandindex),
                error => Err(err_fmod!(
                    "FMOD_Studio_CommandReplay_GetCommandAtTime",
                    error
                )),
            }
        }
    }
    pub fn set_bank_path(&self, bank_path: &str) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_SetBankPath(
                self.pointer,
                CString::new(bank_path)?.as_ptr(),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_SetBankPath", error)),
            }
        }
    }
    pub fn start(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_Start(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_Start", error)),
            }
        }
    }
    pub fn stop(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_Stop(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_Stop", error)),
            }
        }
    }
    pub fn seek_to_time(&self, time: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_SeekToTime(self.pointer, time) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_SeekToTime", error)),
            }
        }
    }
    pub fn seek_to_command(&self, commandindex: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_SeekToCommand(self.pointer, commandindex) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_SeekToCommand", error)),
            }
        }
    }
    pub fn get_paused(&self) -> Result<bool, Error> {
        unsafe {
            let mut paused = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_CommandReplay_GetPaused(self.pointer, &mut paused) {
                ffi::FMOD_OK => Ok(to_bool!(paused)),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_GetPaused", error)),
            }
        }
    }
    pub fn set_paused(&self, paused: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_SetPaused(self.pointer, from_bool!(paused)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_SetPaused", error)),
            }
        }
    }
    pub fn get_playback_state(&self) -> Result<PlaybackState, Error> {
        unsafe {
            let mut state = ffi::FMOD_STUDIO_PLAYBACK_STATE::default();
            match ffi::FMOD_Studio_CommandReplay_GetPlaybackState(self.pointer, &mut state) {
                ffi::FMOD_OK => Ok(PlaybackState::from(state)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_CommandReplay_GetPlaybackState",
                    error
                )),
            }
        }
    }
    pub fn get_current_command(&self) -> Result<(i32, f32), Error> {
        unsafe {
            let mut commandindex = i32::default();
            let mut currenttime = f32::default();
            match ffi::FMOD_Studio_CommandReplay_GetCurrentCommand(
                self.pointer,
                &mut commandindex,
                &mut currenttime,
            ) {
                ffi::FMOD_OK => Ok((commandindex, currenttime)),
                error => Err(err_fmod!(
                    "FMOD_Studio_CommandReplay_GetCurrentCommand",
                    error
                )),
            }
        }
    }
    pub fn release(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_Release(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_Release", error)),
            }
        }
    }
    pub fn set_frame_callback(
        &self,
        callback: ffi::FMOD_STUDIO_COMMANDREPLAY_FRAME_CALLBACK,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_SetFrameCallback(self.pointer, callback) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_CommandReplay_SetFrameCallback",
                    error
                )),
            }
        }
    }
    pub fn set_load_bank_callback(
        &self,
        callback: ffi::FMOD_STUDIO_COMMANDREPLAY_LOAD_BANK_CALLBACK,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_SetLoadBankCallback(self.pointer, callback) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_CommandReplay_SetLoadBankCallback",
                    error
                )),
            }
        }
    }
    pub fn set_create_instance_callback(
        &self,
        callback: ffi::FMOD_STUDIO_COMMANDREPLAY_CREATE_INSTANCE_CALLBACK,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_SetCreateInstanceCallback(self.pointer, callback) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_CommandReplay_SetCreateInstanceCallback",
                    error
                )),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_Studio_CommandReplay_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_GetUserData", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_CommandReplay_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_CommandReplay_SetUserData", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EventDescription {
    pointer: *mut ffi::FMOD_STUDIO_EVENTDESCRIPTION,
}

impl EventDescription {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_STUDIO_EVENTDESCRIPTION) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_STUDIO_EVENTDESCRIPTION {
        self.pointer
    }
    pub fn is_valid(&self) -> bool {
        unsafe { to_bool!(ffi::FMOD_Studio_EventDescription_IsValid(self.pointer)) }
    }
    pub fn get_id(&self) -> Result<Guid, Error> {
        unsafe {
            let mut id = ffi::FMOD_GUID::default();
            match ffi::FMOD_Studio_EventDescription_GetID(self.pointer, &mut id) {
                ffi::FMOD_OK => Ok(Guid::try_from(id)?),
                error => Err(err_fmod!("FMOD_Studio_EventDescription_GetID", error)),
            }
        }
    }
    pub fn get_path(&self) -> Result<String, Error> {
        unsafe {
            let mut retrieved = i32::default();
            match ffi::FMOD_Studio_EventDescription_GetPath(
                self.pointer,
                null_mut(),
                0,
                &mut retrieved,
            ) {
                ffi::FMOD_OK => {
                    let mut buf = vec![0u8; retrieved as usize];
                    match ffi::FMOD_Studio_EventDescription_GetPath(
                        self.pointer,
                        buf.as_mut_ptr() as *mut _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        ffi::FMOD_OK => Ok(CString::from_vec_with_nul_unchecked(buf)
                            .into_string()
                            .map_err(Error::String)?),
                        error => Err(err_fmod!("FMOD_Studio_EventDescription_GetPath", error)),
                    }
                }
                error => Err(err_fmod!("FMOD_Studio_EventDescription_GetPath", error)),
            }
        }
    }
    pub fn get_parameter_description_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_Studio_EventDescription_GetParameterDescriptionCount(
                self.pointer,
                &mut count,
            ) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetParameterDescriptionCount",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_description_by_index(
        &self,
        index: i32,
    ) -> Result<ParameterDescription, Error> {
        unsafe {
            let mut parameter = ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION::default();
            match ffi::FMOD_Studio_EventDescription_GetParameterDescriptionByIndex(
                self.pointer,
                index,
                &mut parameter,
            ) {
                ffi::FMOD_OK => Ok(ParameterDescription::try_from(parameter)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetParameterDescriptionByIndex",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_description_by_name(
        &self,
        name: &str,
    ) -> Result<ParameterDescription, Error> {
        unsafe {
            let mut parameter = ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION::default();
            match ffi::FMOD_Studio_EventDescription_GetParameterDescriptionByName(
                self.pointer,
                CString::new(name)?.as_ptr(),
                &mut parameter,
            ) {
                ffi::FMOD_OK => Ok(ParameterDescription::try_from(parameter)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetParameterDescriptionByName",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_description_by_id(
        &self,
        id: ParameterId,
    ) -> Result<ParameterDescription, Error> {
        unsafe {
            let mut parameter = ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION::default();
            match ffi::FMOD_Studio_EventDescription_GetParameterDescriptionByID(
                self.pointer,
                id.into(),
                &mut parameter,
            ) {
                ffi::FMOD_OK => Ok(ParameterDescription::try_from(parameter)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetParameterDescriptionByID",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_label_by_index(
        &self,
        index: i32,
        labelindex: i32,
        size: i32,
    ) -> Result<(String, i32), Error> {
        unsafe {
            let label = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut retrieved = i32::default();
            match ffi::FMOD_Studio_EventDescription_GetParameterLabelByIndex(
                self.pointer,
                index,
                labelindex,
                label,
                size,
                &mut retrieved,
            ) {
                ffi::FMOD_OK => Ok((
                    CString::from_raw(label)
                        .into_string()
                        .map_err(Error::String)?,
                    retrieved,
                )),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetParameterLabelByIndex",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_label_by_name(
        &self,
        name: &str,
        labelindex: i32,
        size: i32,
    ) -> Result<(String, i32), Error> {
        unsafe {
            let label = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut retrieved = i32::default();
            match ffi::FMOD_Studio_EventDescription_GetParameterLabelByName(
                self.pointer,
                CString::new(name)?.as_ptr(),
                labelindex,
                label,
                size,
                &mut retrieved,
            ) {
                ffi::FMOD_OK => Ok((
                    CString::from_raw(label)
                        .into_string()
                        .map_err(Error::String)?,
                    retrieved,
                )),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetParameterLabelByName",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_label_by_id(
        &self,
        id: ParameterId,
        labelindex: i32,
        size: i32,
    ) -> Result<(String, i32), Error> {
        unsafe {
            let label = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut retrieved = i32::default();
            match ffi::FMOD_Studio_EventDescription_GetParameterLabelByID(
                self.pointer,
                id.into(),
                labelindex,
                label,
                size,
                &mut retrieved,
            ) {
                ffi::FMOD_OK => Ok((
                    CString::from_raw(label)
                        .into_string()
                        .map_err(Error::String)?,
                    retrieved,
                )),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetParameterLabelByID",
                    error
                )),
            }
        }
    }
    pub fn get_user_property_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_Studio_EventDescription_GetUserPropertyCount(self.pointer, &mut count) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetUserPropertyCount",
                    error
                )),
            }
        }
    }
    pub fn get_user_property_by_index(&self, index: i32) -> Result<UserProperty, Error> {
        unsafe {
            let mut property = ffi::FMOD_STUDIO_USER_PROPERTY::default();
            match ffi::FMOD_Studio_EventDescription_GetUserPropertyByIndex(
                self.pointer,
                index,
                &mut property,
            ) {
                ffi::FMOD_OK => Ok(UserProperty::try_from(property)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetUserPropertyByIndex",
                    error
                )),
            }
        }
    }
    pub fn get_user_property(&self, name: &str) -> Result<UserProperty, Error> {
        unsafe {
            let mut property = ffi::FMOD_STUDIO_USER_PROPERTY::default();
            match ffi::FMOD_Studio_EventDescription_GetUserProperty(
                self.pointer,
                CString::new(name)?.as_ptr(),
                &mut property,
            ) {
                ffi::FMOD_OK => Ok(UserProperty::try_from(property)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetUserProperty",
                    error
                )),
            }
        }
    }
    pub fn get_length(&self) -> Result<i32, Error> {
        unsafe {
            let mut length = i32::default();
            match ffi::FMOD_Studio_EventDescription_GetLength(self.pointer, &mut length) {
                ffi::FMOD_OK => Ok(length),
                error => Err(err_fmod!("FMOD_Studio_EventDescription_GetLength", error)),
            }
        }
    }
    pub fn get_min_max_distance(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut min = f32::default();
            let mut max = f32::default();
            match ffi::FMOD_Studio_EventDescription_GetMinMaxDistance(
                self.pointer,
                &mut min,
                &mut max,
            ) {
                ffi::FMOD_OK => Ok((min, max)),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetMinMaxDistance",
                    error
                )),
            }
        }
    }
    pub fn get_sound_size(&self) -> Result<f32, Error> {
        unsafe {
            let mut size = f32::default();
            match ffi::FMOD_Studio_EventDescription_GetSoundSize(self.pointer, &mut size) {
                ffi::FMOD_OK => Ok(size),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetSoundSize",
                    error
                )),
            }
        }
    }
    pub fn is_snapshot(&self) -> Result<bool, Error> {
        unsafe {
            let mut snapshot = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_EventDescription_IsSnapshot(self.pointer, &mut snapshot) {
                ffi::FMOD_OK => Ok(to_bool!(snapshot)),
                error => Err(err_fmod!("FMOD_Studio_EventDescription_IsSnapshot", error)),
            }
        }
    }
    pub fn is_oneshot(&self) -> Result<bool, Error> {
        unsafe {
            let mut oneshot = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_EventDescription_IsOneshot(self.pointer, &mut oneshot) {
                ffi::FMOD_OK => Ok(to_bool!(oneshot)),
                error => Err(err_fmod!("FMOD_Studio_EventDescription_IsOneshot", error)),
            }
        }
    }
    pub fn is_stream(&self) -> Result<bool, Error> {
        unsafe {
            let mut is_stream = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_EventDescription_IsStream(self.pointer, &mut is_stream) {
                ffi::FMOD_OK => Ok(to_bool!(is_stream)),
                error => Err(err_fmod!("FMOD_Studio_EventDescription_IsStream", error)),
            }
        }
    }
    pub fn is_3d(&self) -> Result<bool, Error> {
        unsafe {
            let mut is_3_d = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_EventDescription_Is3D(self.pointer, &mut is_3_d) {
                ffi::FMOD_OK => Ok(to_bool!(is_3_d)),
                error => Err(err_fmod!("FMOD_Studio_EventDescription_Is3D", error)),
            }
        }
    }
    pub fn is_doppler_enabled(&self) -> Result<bool, Error> {
        unsafe {
            let mut doppler = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_EventDescription_IsDopplerEnabled(self.pointer, &mut doppler) {
                ffi::FMOD_OK => Ok(to_bool!(doppler)),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_IsDopplerEnabled",
                    error
                )),
            }
        }
    }
    pub fn has_sustain_point(&self) -> Result<bool, Error> {
        unsafe {
            let mut sustain_point = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_EventDescription_HasSustainPoint(
                self.pointer,
                &mut sustain_point,
            ) {
                ffi::FMOD_OK => Ok(to_bool!(sustain_point)),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_HasSustainPoint",
                    error
                )),
            }
        }
    }
    pub fn create_instance(&self) -> Result<EventInstance, Error> {
        unsafe {
            let mut instance = null_mut();
            match ffi::FMOD_Studio_EventDescription_CreateInstance(self.pointer, &mut instance) {
                ffi::FMOD_OK => Ok(EventInstance::from(instance)),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_CreateInstance",
                    error
                )),
            }
        }
    }
    pub fn get_instance_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_Studio_EventDescription_GetInstanceCount(self.pointer, &mut count) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetInstanceCount",
                    error
                )),
            }
        }
    }
    pub fn get_instance_list(&self, capacity: i32) -> Result<(EventInstance, i32), Error> {
        unsafe {
            let mut array = null_mut();
            let mut count = i32::default();
            match ffi::FMOD_Studio_EventDescription_GetInstanceList(
                self.pointer,
                &mut array,
                capacity,
                &mut count,
            ) {
                ffi::FMOD_OK => Ok((EventInstance::from(array), count)),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetInstanceList",
                    error
                )),
            }
        }
    }
    pub fn load_sample_data(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventDescription_LoadSampleData(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_LoadSampleData",
                    error
                )),
            }
        }
    }
    pub fn unload_sample_data(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventDescription_UnloadSampleData(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_UnloadSampleData",
                    error
                )),
            }
        }
    }
    pub fn get_sample_loading_state(&self) -> Result<LoadingState, Error> {
        unsafe {
            let mut state = ffi::FMOD_STUDIO_LOADING_STATE::default();
            match ffi::FMOD_Studio_EventDescription_GetSampleLoadingState(self.pointer, &mut state)
            {
                ffi::FMOD_OK => Ok(LoadingState::from(state)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_GetSampleLoadingState",
                    error
                )),
            }
        }
    }
    pub fn release_all_instances(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventDescription_ReleaseAllInstances(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventDescription_ReleaseAllInstances",
                    error
                )),
            }
        }
    }
    pub fn set_callback(
        &self,
        callback: ffi::FMOD_STUDIO_EVENT_CALLBACK,
        callbackmask: ffi::FMOD_STUDIO_EVENT_CALLBACK_TYPE,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventDescription_SetCallback(
                self.pointer,
                callback,
                callbackmask,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventDescription_SetCallback", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_Studio_EventDescription_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_Studio_EventDescription_GetUserData", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventDescription_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventDescription_SetUserData", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EventInstance {
    pointer: *mut ffi::FMOD_STUDIO_EVENTINSTANCE,
}

impl EventInstance {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_STUDIO_EVENTINSTANCE) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_STUDIO_EVENTINSTANCE {
        self.pointer
    }
    pub fn is_valid(&self) -> bool {
        unsafe { to_bool!(ffi::FMOD_Studio_EventInstance_IsValid(self.pointer)) }
    }
    pub fn get_description(&self) -> Result<EventDescription, Error> {
        unsafe {
            let mut description = null_mut();
            match ffi::FMOD_Studio_EventInstance_GetDescription(self.pointer, &mut description) {
                ffi::FMOD_OK => Ok(EventDescription::from(description)),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_GetDescription", error)),
            }
        }
    }
    pub fn get_volume(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut volume = f32::default();
            let mut finalvolume = f32::default();
            match ffi::FMOD_Studio_EventInstance_GetVolume(
                self.pointer,
                &mut volume,
                &mut finalvolume,
            ) {
                ffi::FMOD_OK => Ok((volume, finalvolume)),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_GetVolume", error)),
            }
        }
    }
    pub fn set_volume(&self, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetVolume(self.pointer, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_SetVolume", error)),
            }
        }
    }
    pub fn get_pitch(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut pitch = f32::default();
            let mut finalpitch = f32::default();
            match ffi::FMOD_Studio_EventInstance_GetPitch(self.pointer, &mut pitch, &mut finalpitch)
            {
                ffi::FMOD_OK => Ok((pitch, finalpitch)),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_GetPitch", error)),
            }
        }
    }
    pub fn set_pitch(&self, pitch: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetPitch(self.pointer, pitch) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_SetPitch", error)),
            }
        }
    }
    pub fn get_3d_attributes(&self) -> Result<Attributes3d, Error> {
        unsafe {
            let mut attributes = ffi::FMOD_3D_ATTRIBUTES::default();
            match ffi::FMOD_Studio_EventInstance_Get3DAttributes(self.pointer, &mut attributes) {
                ffi::FMOD_OK => Ok(Attributes3d::try_from(attributes)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_Get3DAttributes",
                    error
                )),
            }
        }
    }
    pub fn set_3d_attributes(&self, attributes: Attributes3d) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_Set3DAttributes(
                self.pointer,
                &mut attributes.into(),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_Set3DAttributes",
                    error
                )),
            }
        }
    }
    pub fn get_listener_mask(&self) -> Result<u32, Error> {
        unsafe {
            let mut mask = u32::default();
            match ffi::FMOD_Studio_EventInstance_GetListenerMask(self.pointer, &mut mask) {
                ffi::FMOD_OK => Ok(mask),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_GetListenerMask",
                    error
                )),
            }
        }
    }
    pub fn set_listener_mask(&self, mask: u32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetListenerMask(self.pointer, mask) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_SetListenerMask",
                    error
                )),
            }
        }
    }
    pub fn get_property(&self, index: EventProperty) -> Result<f32, Error> {
        unsafe {
            let mut value = f32::default();
            match ffi::FMOD_Studio_EventInstance_GetProperty(self.pointer, index.into(), &mut value)
            {
                ffi::FMOD_OK => Ok(value),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_GetProperty", error)),
            }
        }
    }
    pub fn set_property(&self, index: EventProperty, value: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetProperty(self.pointer, index.into(), value) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_SetProperty", error)),
            }
        }
    }
    pub fn get_reverb_level(&self, index: i32) -> Result<f32, Error> {
        unsafe {
            let mut level = f32::default();
            match ffi::FMOD_Studio_EventInstance_GetReverbLevel(self.pointer, index, &mut level) {
                ffi::FMOD_OK => Ok(level),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_GetReverbLevel", error)),
            }
        }
    }
    pub fn set_reverb_level(&self, index: i32, level: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetReverbLevel(self.pointer, index, level) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_SetReverbLevel", error)),
            }
        }
    }
    pub fn get_paused(&self) -> Result<bool, Error> {
        unsafe {
            let mut paused = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_EventInstance_GetPaused(self.pointer, &mut paused) {
                ffi::FMOD_OK => Ok(to_bool!(paused)),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_GetPaused", error)),
            }
        }
    }
    pub fn set_paused(&self, paused: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetPaused(self.pointer, from_bool!(paused)) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_SetPaused", error)),
            }
        }
    }
    pub fn start(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_Start(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_Start", error)),
            }
        }
    }
    pub fn stop(&self, mode: StopMode) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_Stop(self.pointer, mode.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_Stop", error)),
            }
        }
    }
    pub fn get_timeline_position(&self) -> Result<i32, Error> {
        unsafe {
            let mut position = i32::default();
            match ffi::FMOD_Studio_EventInstance_GetTimelinePosition(self.pointer, &mut position) {
                ffi::FMOD_OK => Ok(position),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_GetTimelinePosition",
                    error
                )),
            }
        }
    }
    pub fn set_timeline_position(&self, position: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetTimelinePosition(self.pointer, position) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_SetTimelinePosition",
                    error
                )),
            }
        }
    }
    pub fn get_playback_state(&self) -> Result<PlaybackState, Error> {
        unsafe {
            let mut state = ffi::FMOD_STUDIO_PLAYBACK_STATE::default();
            match ffi::FMOD_Studio_EventInstance_GetPlaybackState(self.pointer, &mut state) {
                ffi::FMOD_OK => Ok(PlaybackState::from(state)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_GetPlaybackState",
                    error
                )),
            }
        }
    }
    pub fn get_channel_group(&self) -> Result<ChannelGroup, Error> {
        unsafe {
            let mut group = null_mut();
            match ffi::FMOD_Studio_EventInstance_GetChannelGroup(self.pointer, &mut group) {
                ffi::FMOD_OK => Ok(ChannelGroup::from(group)),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_GetChannelGroup",
                    error
                )),
            }
        }
    }
    pub fn get_min_max_distance(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut min = f32::default();
            let mut max = f32::default();
            match ffi::FMOD_Studio_EventInstance_GetMinMaxDistance(self.pointer, &mut min, &mut max)
            {
                ffi::FMOD_OK => Ok((min, max)),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_GetMinMaxDistance",
                    error
                )),
            }
        }
    }
    pub fn release(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_Release(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_Release", error)),
            }
        }
    }
    pub fn is_virtual(&self) -> Result<bool, Error> {
        unsafe {
            let mut virtualstate = ffi::FMOD_BOOL::default();
            match ffi::FMOD_Studio_EventInstance_IsVirtual(self.pointer, &mut virtualstate) {
                ffi::FMOD_OK => Ok(to_bool!(virtualstate)),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_IsVirtual", error)),
            }
        }
    }
    pub fn get_parameter_by_name(&self, name: &str) -> Result<(f32, f32), Error> {
        unsafe {
            let mut value = f32::default();
            let mut finalvalue = f32::default();
            match ffi::FMOD_Studio_EventInstance_GetParameterByName(
                self.pointer,
                CString::new(name)?.as_ptr(),
                &mut value,
                &mut finalvalue,
            ) {
                ffi::FMOD_OK => Ok((value, finalvalue)),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_GetParameterByName",
                    error
                )),
            }
        }
    }
    pub fn set_parameter_by_name(
        &self,
        name: &str,
        value: f32,
        ignoreseekspeed: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetParameterByName(
                self.pointer,
                CString::new(name)?.as_ptr(),
                value,
                from_bool!(ignoreseekspeed),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_SetParameterByName",
                    error
                )),
            }
        }
    }
    pub fn set_parameter_by_name_with_label(
        &self,
        name: &str,
        label: &str,
        ignoreseekspeed: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetParameterByNameWithLabel(
                self.pointer,
                CString::new(name)?.as_ptr(),
                CString::new(label)?.as_ptr(),
                from_bool!(ignoreseekspeed),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_SetParameterByNameWithLabel",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_by_id(&self, id: ParameterId) -> Result<(f32, f32), Error> {
        unsafe {
            let mut value = f32::default();
            let mut finalvalue = f32::default();
            match ffi::FMOD_Studio_EventInstance_GetParameterByID(
                self.pointer,
                id.into(),
                &mut value,
                &mut finalvalue,
            ) {
                ffi::FMOD_OK => Ok((value, finalvalue)),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_GetParameterByID",
                    error
                )),
            }
        }
    }
    pub fn set_parameter_by_id(
        &self,
        id: ParameterId,
        value: f32,
        ignoreseekspeed: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetParameterByID(
                self.pointer,
                id.into(),
                value,
                from_bool!(ignoreseekspeed),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_SetParameterByID",
                    error
                )),
            }
        }
    }
    pub fn set_parameter_by_id_with_label(
        &self,
        id: ParameterId,
        label: &str,
        ignoreseekspeed: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetParameterByIDWithLabel(
                self.pointer,
                id.into(),
                CString::new(label)?.as_ptr(),
                from_bool!(ignoreseekspeed),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_SetParameterByIDWithLabel",
                    error
                )),
            }
        }
    }
    pub fn set_parameters_by_i_ds(
        &self,
        ids: ParameterId,
        values: *mut f32,
        count: i32,
        ignoreseekspeed: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetParametersByIDs(
                self.pointer,
                &ids.into(),
                values,
                count,
                from_bool!(ignoreseekspeed),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_EventInstance_SetParametersByIDs",
                    error
                )),
            }
        }
    }
    pub fn key_off(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_KeyOff(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_KeyOff", error)),
            }
        }
    }
    pub fn set_callback(
        &self,
        callback: ffi::FMOD_STUDIO_EVENT_CALLBACK,
        callbackmask: ffi::FMOD_STUDIO_EVENT_CALLBACK_TYPE,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetCallback(self.pointer, callback, callbackmask) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_SetCallback", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_Studio_EventInstance_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_GetUserData", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_EventInstance_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_SetUserData", error)),
            }
        }
    }
    pub fn get_cpu_usage(&self) -> Result<(u32, u32), Error> {
        unsafe {
            let mut exclusive = u32::default();
            let mut inclusive = u32::default();
            match ffi::FMOD_Studio_EventInstance_GetCPUUsage(
                self.pointer,
                &mut exclusive,
                &mut inclusive,
            ) {
                ffi::FMOD_OK => Ok((exclusive, inclusive)),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_GetCPUUsage", error)),
            }
        }
    }
    pub fn get_memory_usage(&self) -> Result<MemoryUsage, Error> {
        unsafe {
            let mut memoryusage = ffi::FMOD_STUDIO_MEMORY_USAGE::default();
            match ffi::FMOD_Studio_EventInstance_GetMemoryUsage(self.pointer, &mut memoryusage) {
                ffi::FMOD_OK => Ok(MemoryUsage::try_from(memoryusage)?),
                error => Err(err_fmod!("FMOD_Studio_EventInstance_GetMemoryUsage", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Studio {
    pointer: *mut ffi::FMOD_STUDIO_SYSTEM,
}

impl Studio {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_STUDIO_SYSTEM) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_STUDIO_SYSTEM {
        self.pointer
    }
    pub fn create() -> Result<Studio, Error> {
        unsafe {
            let mut system = null_mut();
            match ffi::FMOD_Studio_System_Create(&mut system, ffi::FMOD_VERSION) {
                ffi::FMOD_OK => Ok(Studio::from(system)),
                error => Err(err_fmod!("FMOD_Studio_System_Create", error)),
            }
        }
    }
    pub fn is_valid(&self) -> bool {
        unsafe { to_bool!(ffi::FMOD_Studio_System_IsValid(self.pointer)) }
    }
    pub fn set_advanced_settings(&self, settings: StudioAdvancedSettings) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetAdvancedSettings(self.pointer, &mut settings.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_SetAdvancedSettings", error)),
            }
        }
    }
    pub fn get_advanced_settings(&self) -> Result<StudioAdvancedSettings, Error> {
        unsafe {
            let mut settings = ffi::FMOD_STUDIO_ADVANCEDSETTINGS::default();
            match ffi::FMOD_Studio_System_GetAdvancedSettings(self.pointer, &mut settings) {
                ffi::FMOD_OK => Ok(StudioAdvancedSettings::try_from(settings)?),
                error => Err(err_fmod!("FMOD_Studio_System_GetAdvancedSettings", error)),
            }
        }
    }
    pub fn initialize(
        &self,
        maxchannels: i32,
        studioflags: ffi::FMOD_STUDIO_INITFLAGS,
        flags: ffi::FMOD_INITFLAGS,
        extradriverdata: Option<*mut c_void>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_Initialize(
                self.pointer,
                maxchannels,
                studioflags,
                flags,
                extradriverdata.unwrap_or(null_mut()),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_Initialize", error)),
            }
        }
    }
    pub fn release(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_Release(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_Release", error)),
            }
        }
    }
    pub fn update(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_Update(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_Update", error)),
            }
        }
    }
    pub fn get_core_system(&self) -> Result<System, Error> {
        unsafe {
            let mut coresystem = null_mut();
            match ffi::FMOD_Studio_System_GetCoreSystem(self.pointer, &mut coresystem) {
                ffi::FMOD_OK => Ok(System::from(coresystem)),
                error => Err(err_fmod!("FMOD_Studio_System_GetCoreSystem", error)),
            }
        }
    }
    pub fn get_event(&self, path_or_id: &str) -> Result<EventDescription, Error> {
        unsafe {
            let mut event = null_mut();
            match ffi::FMOD_Studio_System_GetEvent(
                self.pointer,
                CString::new(path_or_id)?.as_ptr(),
                &mut event,
            ) {
                ffi::FMOD_OK => Ok(EventDescription::from(event)),
                error => Err(err_fmod!("FMOD_Studio_System_GetEvent", error)),
            }
        }
    }
    pub fn get_bus(&self, path_or_id: &str) -> Result<Bus, Error> {
        unsafe {
            let mut bus = null_mut();
            match ffi::FMOD_Studio_System_GetBus(
                self.pointer,
                CString::new(path_or_id)?.as_ptr(),
                &mut bus,
            ) {
                ffi::FMOD_OK => Ok(Bus::from(bus)),
                error => Err(err_fmod!("FMOD_Studio_System_GetBus", error)),
            }
        }
    }
    pub fn get_vca(&self, path_or_id: &str) -> Result<Vca, Error> {
        unsafe {
            let mut vca = null_mut();
            match ffi::FMOD_Studio_System_GetVCA(
                self.pointer,
                CString::new(path_or_id)?.as_ptr(),
                &mut vca,
            ) {
                ffi::FMOD_OK => Ok(Vca::from(vca)),
                error => Err(err_fmod!("FMOD_Studio_System_GetVCA", error)),
            }
        }
    }
    pub fn get_bank(&self, path_or_id: &str) -> Result<Bank, Error> {
        unsafe {
            let mut bank = null_mut();
            match ffi::FMOD_Studio_System_GetBank(
                self.pointer,
                CString::new(path_or_id)?.as_ptr(),
                &mut bank,
            ) {
                ffi::FMOD_OK => Ok(Bank::from(bank)),
                error => Err(err_fmod!("FMOD_Studio_System_GetBank", error)),
            }
        }
    }
    pub fn get_event_by_id(&self, id: Guid) -> Result<EventDescription, Error> {
        unsafe {
            let mut event = null_mut();
            match ffi::FMOD_Studio_System_GetEventByID(self.pointer, &id.into(), &mut event) {
                ffi::FMOD_OK => Ok(EventDescription::from(event)),
                error => Err(err_fmod!("FMOD_Studio_System_GetEventByID", error)),
            }
        }
    }
    pub fn get_bus_by_id(&self, id: Guid) -> Result<Bus, Error> {
        unsafe {
            let mut bus = null_mut();
            match ffi::FMOD_Studio_System_GetBusByID(self.pointer, &id.into(), &mut bus) {
                ffi::FMOD_OK => Ok(Bus::from(bus)),
                error => Err(err_fmod!("FMOD_Studio_System_GetBusByID", error)),
            }
        }
    }
    pub fn get_vca_by_id(&self, id: Guid) -> Result<Vca, Error> {
        unsafe {
            let mut vca = null_mut();
            match ffi::FMOD_Studio_System_GetVCAByID(self.pointer, &id.into(), &mut vca) {
                ffi::FMOD_OK => Ok(Vca::from(vca)),
                error => Err(err_fmod!("FMOD_Studio_System_GetVCAByID", error)),
            }
        }
    }
    pub fn get_bank_by_id(&self, id: Guid) -> Result<Bank, Error> {
        unsafe {
            let mut bank = null_mut();
            match ffi::FMOD_Studio_System_GetBankByID(self.pointer, &id.into(), &mut bank) {
                ffi::FMOD_OK => Ok(Bank::from(bank)),
                error => Err(err_fmod!("FMOD_Studio_System_GetBankByID", error)),
            }
        }
    }
    pub fn get_sound_info(&self, key: &str) -> Result<SoundInfo, Error> {
        unsafe {
            let mut info = ffi::FMOD_STUDIO_SOUND_INFO::default();
            match ffi::FMOD_Studio_System_GetSoundInfo(
                self.pointer,
                CString::new(key)?.as_ptr(),
                &mut info,
            ) {
                ffi::FMOD_OK => Ok(SoundInfo::try_from(info)?),
                error => Err(err_fmod!("FMOD_Studio_System_GetSoundInfo", error)),
            }
        }
    }
    pub fn get_parameter_description_by_name(
        &self,
        name: &str,
    ) -> Result<ParameterDescription, Error> {
        unsafe {
            let mut parameter = ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION::default();
            match ffi::FMOD_Studio_System_GetParameterDescriptionByName(
                self.pointer,
                CString::new(name)?.as_ptr(),
                &mut parameter,
            ) {
                ffi::FMOD_OK => Ok(ParameterDescription::try_from(parameter)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_System_GetParameterDescriptionByName",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_description_by_id(
        &self,
        id: ParameterId,
    ) -> Result<ParameterDescription, Error> {
        unsafe {
            let mut parameter = ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION::default();
            match ffi::FMOD_Studio_System_GetParameterDescriptionByID(
                self.pointer,
                id.into(),
                &mut parameter,
            ) {
                ffi::FMOD_OK => Ok(ParameterDescription::try_from(parameter)?),
                error => Err(err_fmod!(
                    "FMOD_Studio_System_GetParameterDescriptionByID",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_label_by_name(
        &self,
        name: &str,
        labelindex: i32,
        size: i32,
    ) -> Result<(String, i32), Error> {
        unsafe {
            let label = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut retrieved = i32::default();
            match ffi::FMOD_Studio_System_GetParameterLabelByName(
                self.pointer,
                CString::new(name)?.as_ptr(),
                labelindex,
                label,
                size,
                &mut retrieved,
            ) {
                ffi::FMOD_OK => Ok((
                    CString::from_raw(label)
                        .into_string()
                        .map_err(Error::String)?,
                    retrieved,
                )),
                error => Err(err_fmod!(
                    "FMOD_Studio_System_GetParameterLabelByName",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_label_by_id(
        &self,
        id: ParameterId,
        labelindex: i32,
        size: i32,
    ) -> Result<(String, i32), Error> {
        unsafe {
            let label = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut retrieved = i32::default();
            match ffi::FMOD_Studio_System_GetParameterLabelByID(
                self.pointer,
                id.into(),
                labelindex,
                label,
                size,
                &mut retrieved,
            ) {
                ffi::FMOD_OK => Ok((
                    CString::from_raw(label)
                        .into_string()
                        .map_err(Error::String)?,
                    retrieved,
                )),
                error => Err(err_fmod!("FMOD_Studio_System_GetParameterLabelByID", error)),
            }
        }
    }
    pub fn get_parameter_by_id(&self, id: ParameterId) -> Result<(f32, f32), Error> {
        unsafe {
            let mut value = f32::default();
            let mut finalvalue = f32::default();
            match ffi::FMOD_Studio_System_GetParameterByID(
                self.pointer,
                id.into(),
                &mut value,
                &mut finalvalue,
            ) {
                ffi::FMOD_OK => Ok((value, finalvalue)),
                error => Err(err_fmod!("FMOD_Studio_System_GetParameterByID", error)),
            }
        }
    }
    pub fn set_parameter_by_id(
        &self,
        id: ParameterId,
        value: f32,
        ignoreseekspeed: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetParameterByID(
                self.pointer,
                id.into(),
                value,
                from_bool!(ignoreseekspeed),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_SetParameterByID", error)),
            }
        }
    }
    pub fn set_parameter_by_id_with_label(
        &self,
        id: ParameterId,
        label: &str,
        ignoreseekspeed: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetParameterByIDWithLabel(
                self.pointer,
                id.into(),
                CString::new(label)?.as_ptr(),
                from_bool!(ignoreseekspeed),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_System_SetParameterByIDWithLabel",
                    error
                )),
            }
        }
    }
    pub fn set_parameters_by_i_ds(
        &self,
        ids: ParameterId,
        values: *mut f32,
        count: i32,
        ignoreseekspeed: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetParametersByIDs(
                self.pointer,
                &ids.into(),
                values,
                count,
                from_bool!(ignoreseekspeed),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_SetParametersByIDs", error)),
            }
        }
    }
    pub fn get_parameter_by_name(&self, name: &str) -> Result<(f32, f32), Error> {
        unsafe {
            let mut value = f32::default();
            let mut finalvalue = f32::default();
            match ffi::FMOD_Studio_System_GetParameterByName(
                self.pointer,
                CString::new(name)?.as_ptr(),
                &mut value,
                &mut finalvalue,
            ) {
                ffi::FMOD_OK => Ok((value, finalvalue)),
                error => Err(err_fmod!("FMOD_Studio_System_GetParameterByName", error)),
            }
        }
    }
    pub fn set_parameter_by_name(
        &self,
        name: &str,
        value: f32,
        ignoreseekspeed: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetParameterByName(
                self.pointer,
                CString::new(name)?.as_ptr(),
                value,
                from_bool!(ignoreseekspeed),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_SetParameterByName", error)),
            }
        }
    }
    pub fn set_parameter_by_name_with_label(
        &self,
        name: &str,
        label: &str,
        ignoreseekspeed: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetParameterByNameWithLabel(
                self.pointer,
                CString::new(name)?.as_ptr(),
                CString::new(label)?.as_ptr(),
                from_bool!(ignoreseekspeed),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!(
                    "FMOD_Studio_System_SetParameterByNameWithLabel",
                    error
                )),
            }
        }
    }
    pub fn lookup_id(&self, path: &str) -> Result<Guid, Error> {
        unsafe {
            let mut id = ffi::FMOD_GUID::default();
            match ffi::FMOD_Studio_System_LookupID(
                self.pointer,
                CString::new(path)?.as_ptr(),
                &mut id,
            ) {
                ffi::FMOD_OK => Ok(Guid::try_from(id)?),
                error => Err(err_fmod!("FMOD_Studio_System_LookupID", error)),
            }
        }
    }
    pub fn lookup_path(&self, id: Guid) -> Result<String, Error> {
        unsafe {
            let mut retrieved = i32::default();
            let id = id.into();
            match ffi::FMOD_Studio_System_LookupPath(
                self.pointer,
                &id,
                null_mut(),
                0,
                &mut retrieved,
            ) {
                ffi::FMOD_OK => {
                    let mut buf = vec![0u8; retrieved as usize];
                    match ffi::FMOD_Studio_System_LookupPath(
                        self.pointer,
                        &id,
                        buf.as_mut_ptr() as *mut _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        ffi::FMOD_OK => Ok(CString::from_vec_with_nul_unchecked(buf)
                            .into_string()
                            .map_err(Error::String)?),
                        error => Err(err_fmod!("FMOD_Studio_System_LookupPath", error)),
                    }
                }
                error => Err(err_fmod!("FMOD_Studio_System_LookupPath", error)),
            }
        }
    }
    pub fn get_num_listeners(&self) -> Result<i32, Error> {
        unsafe {
            let mut numlisteners = i32::default();
            match ffi::FMOD_Studio_System_GetNumListeners(self.pointer, &mut numlisteners) {
                ffi::FMOD_OK => Ok(numlisteners),
                error => Err(err_fmod!("FMOD_Studio_System_GetNumListeners", error)),
            }
        }
    }
    pub fn set_num_listeners(&self, numlisteners: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetNumListeners(self.pointer, numlisteners) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_SetNumListeners", error)),
            }
        }
    }
    pub fn get_listener_attributes(&self, index: i32) -> Result<(Attributes3d, Vector), Error> {
        unsafe {
            let mut attributes = ffi::FMOD_3D_ATTRIBUTES::default();
            let mut attenuationposition = ffi::FMOD_VECTOR::default();
            match ffi::FMOD_Studio_System_GetListenerAttributes(
                self.pointer,
                index,
                &mut attributes,
                &mut attenuationposition,
            ) {
                ffi::FMOD_OK => Ok((
                    Attributes3d::try_from(attributes)?,
                    Vector::try_from(attenuationposition)?,
                )),
                error => Err(err_fmod!("FMOD_Studio_System_GetListenerAttributes", error)),
            }
        }
    }
    pub fn set_listener_attributes(
        &self,
        index: i32,
        attributes: Attributes3d,
        attenuationposition: Option<Vector>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetListenerAttributes(
                self.pointer,
                index,
                &attributes.into(),
                attenuationposition
                    .map(|value| &value.into() as *const _)
                    .unwrap_or(null()),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_SetListenerAttributes", error)),
            }
        }
    }
    pub fn get_listener_weight(&self, index: i32) -> Result<f32, Error> {
        unsafe {
            let mut weight = f32::default();
            match ffi::FMOD_Studio_System_GetListenerWeight(self.pointer, index, &mut weight) {
                ffi::FMOD_OK => Ok(weight),
                error => Err(err_fmod!("FMOD_Studio_System_GetListenerWeight", error)),
            }
        }
    }
    pub fn set_listener_weight(&self, index: i32, weight: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetListenerWeight(self.pointer, index, weight) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_SetListenerWeight", error)),
            }
        }
    }
    pub fn load_bank_file(
        &self,
        filename: &str,
        flags: ffi::FMOD_STUDIO_LOAD_BANK_FLAGS,
    ) -> Result<Bank, Error> {
        unsafe {
            let mut bank = null_mut();
            match ffi::FMOD_Studio_System_LoadBankFile(
                self.pointer,
                CString::new(filename)?.as_ptr(),
                flags,
                &mut bank,
            ) {
                ffi::FMOD_OK => Ok(Bank::from(bank)),
                error => Err(err_fmod!("FMOD_Studio_System_LoadBankFile", error)),
            }
        }
    }
    pub fn load_bank_memory(
        &self,
        buffer: &[u8],
        flags: ffi::FMOD_STUDIO_LOAD_BANK_FLAGS,
    ) -> Result<Bank, Error> {
        unsafe {
            let mut bank = null_mut();
            match ffi::FMOD_Studio_System_LoadBankMemory(
                self.pointer,
                buffer.as_ptr() as *const std::os::raw::c_char,
                buffer.len() as std::os::raw::c_int,
                LoadMemoryMode::Memory.into(),
                flags,
                &mut bank,
            ) {
                ffi::FMOD_OK => Ok(Bank::from(bank)),
                error => Err(err_fmod!("FMOD_Studio_System_LoadBankMemory", error)),
            }
        }
    }
    pub fn load_bank_custom(
        &self,
        info: BankInfo,
        flags: ffi::FMOD_STUDIO_LOAD_BANK_FLAGS,
    ) -> Result<Bank, Error> {
        unsafe {
            let mut bank = null_mut();
            match ffi::FMOD_Studio_System_LoadBankCustom(
                self.pointer,
                &info.into(),
                flags,
                &mut bank,
            ) {
                ffi::FMOD_OK => Ok(Bank::from(bank)),
                error => Err(err_fmod!("FMOD_Studio_System_LoadBankCustom", error)),
            }
        }
    }
    pub fn register_plugin(&self, description: DspDescription) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_RegisterPlugin(self.pointer, &description.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_RegisterPlugin", error)),
            }
        }
    }
    pub fn unregister_plugin(&self, name: &str) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_UnregisterPlugin(
                self.pointer,
                CString::new(name)?.as_ptr(),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_UnregisterPlugin", error)),
            }
        }
    }
    pub fn unload_all(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_UnloadAll(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_UnloadAll", error)),
            }
        }
    }
    pub fn flush_commands(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_FlushCommands(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_FlushCommands", error)),
            }
        }
    }
    pub fn flush_sample_loading(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_FlushSampleLoading(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_FlushSampleLoading", error)),
            }
        }
    }
    pub fn start_command_capture(
        &self,
        filename: &str,
        flags: ffi::FMOD_STUDIO_COMMANDCAPTURE_FLAGS,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_StartCommandCapture(
                self.pointer,
                CString::new(filename)?.as_ptr(),
                flags,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_StartCommandCapture", error)),
            }
        }
    }
    pub fn stop_command_capture(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_StopCommandCapture(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_StopCommandCapture", error)),
            }
        }
    }
    pub fn load_command_replay(
        &self,
        filename: &str,
        flags: ffi::FMOD_STUDIO_COMMANDREPLAY_FLAGS,
    ) -> Result<CommandReplay, Error> {
        unsafe {
            let mut replay = null_mut();
            match ffi::FMOD_Studio_System_LoadCommandReplay(
                self.pointer,
                CString::new(filename)?.as_ptr(),
                flags,
                &mut replay,
            ) {
                ffi::FMOD_OK => Ok(CommandReplay::from(replay)),
                error => Err(err_fmod!("FMOD_Studio_System_LoadCommandReplay", error)),
            }
        }
    }
    pub fn get_bank_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_Studio_System_GetBankCount(self.pointer, &mut count) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!("FMOD_Studio_System_GetBankCount", error)),
            }
        }
    }
    pub fn get_bank_list(&self, capacity: i32) -> Result<(Bank, i32), Error> {
        unsafe {
            let mut array = null_mut();
            let mut count = i32::default();
            match ffi::FMOD_Studio_System_GetBankList(
                self.pointer,
                &mut array,
                capacity,
                &mut count,
            ) {
                ffi::FMOD_OK => Ok((Bank::from(array), count)),
                error => Err(err_fmod!("FMOD_Studio_System_GetBankList", error)),
            }
        }
    }
    pub fn get_parameter_description_count(&self) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_Studio_System_GetParameterDescriptionCount(self.pointer, &mut count) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!(
                    "FMOD_Studio_System_GetParameterDescriptionCount",
                    error
                )),
            }
        }
    }
    pub fn get_parameter_description_list(
        &self,
        capacity: i32,
    ) -> Result<(ParameterDescription, i32), Error> {
        unsafe {
            let mut array = ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION::default();
            let mut count = i32::default();
            match ffi::FMOD_Studio_System_GetParameterDescriptionList(
                self.pointer,
                &mut array,
                capacity,
                &mut count,
            ) {
                ffi::FMOD_OK => Ok((ParameterDescription::try_from(array)?, count)),
                error => Err(err_fmod!(
                    "FMOD_Studio_System_GetParameterDescriptionList",
                    error
                )),
            }
        }
    }
    pub fn get_cpu_usage(&self) -> Result<(StudioCpuUsage, CpuUsage), Error> {
        unsafe {
            let mut usage = ffi::FMOD_STUDIO_CPU_USAGE::default();
            let mut usage_core = ffi::FMOD_CPU_USAGE::default();
            match ffi::FMOD_Studio_System_GetCPUUsage(self.pointer, &mut usage, &mut usage_core) {
                ffi::FMOD_OK => Ok((
                    StudioCpuUsage::try_from(usage)?,
                    CpuUsage::try_from(usage_core)?,
                )),
                error => Err(err_fmod!("FMOD_Studio_System_GetCPUUsage", error)),
            }
        }
    }
    pub fn get_buffer_usage(&self) -> Result<BufferUsage, Error> {
        unsafe {
            let mut usage = ffi::FMOD_STUDIO_BUFFER_USAGE::default();
            match ffi::FMOD_Studio_System_GetBufferUsage(self.pointer, &mut usage) {
                ffi::FMOD_OK => Ok(BufferUsage::try_from(usage)?),
                error => Err(err_fmod!("FMOD_Studio_System_GetBufferUsage", error)),
            }
        }
    }
    pub fn reset_buffer_usage(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_ResetBufferUsage(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_ResetBufferUsage", error)),
            }
        }
    }
    pub fn set_callback(
        &self,
        callback: ffi::FMOD_STUDIO_SYSTEM_CALLBACK,
        callbackmask: ffi::FMOD_STUDIO_SYSTEM_CALLBACK_TYPE,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetCallback(self.pointer, callback, callbackmask) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_SetCallback", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_System_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_System_SetUserData", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_Studio_System_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_Studio_System_GetUserData", error)),
            }
        }
    }
    pub fn get_memory_usage(&self) -> Result<MemoryUsage, Error> {
        unsafe {
            let mut memoryusage = ffi::FMOD_STUDIO_MEMORY_USAGE::default();
            match ffi::FMOD_Studio_System_GetMemoryUsage(self.pointer, &mut memoryusage) {
                ffi::FMOD_OK => Ok(MemoryUsage::try_from(memoryusage)?),
                error => Err(err_fmod!("FMOD_Studio_System_GetMemoryUsage", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vca {
    pointer: *mut ffi::FMOD_STUDIO_VCA,
}

impl Vca {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_STUDIO_VCA) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_STUDIO_VCA {
        self.pointer
    }
    pub fn is_valid(&self) -> bool {
        unsafe { to_bool!(ffi::FMOD_Studio_VCA_IsValid(self.pointer)) }
    }
    pub fn get_id(&self) -> Result<Guid, Error> {
        unsafe {
            let mut id = ffi::FMOD_GUID::default();
            match ffi::FMOD_Studio_VCA_GetID(self.pointer, &mut id) {
                ffi::FMOD_OK => Ok(Guid::try_from(id)?),
                error => Err(err_fmod!("FMOD_Studio_VCA_GetID", error)),
            }
        }
    }
    pub fn get_path(&self) -> Result<String, Error> {
        unsafe {
            let mut retrieved = i32::default();
            match ffi::FMOD_Studio_VCA_GetPath(self.pointer, null_mut(), 0, &mut retrieved) {
                ffi::FMOD_OK => {
                    let mut buf = vec![0u8; retrieved as usize];
                    match ffi::FMOD_Studio_VCA_GetPath(
                        self.pointer,
                        buf.as_mut_ptr() as *mut _,
                        retrieved,
                        &mut retrieved,
                    ) {
                        ffi::FMOD_OK => Ok(CString::from_vec_with_nul_unchecked(buf)
                            .into_string()
                            .map_err(Error::String)?),
                        error => Err(err_fmod!("FMOD_Studio_VCA_GetPath", error)),
                    }
                }
                error => Err(err_fmod!("FMOD_Studio_VCA_GetPath", error)),
            }
        }
    }
    pub fn get_volume(&self) -> Result<(f32, f32), Error> {
        unsafe {
            let mut volume = f32::default();
            let mut finalvolume = f32::default();
            match ffi::FMOD_Studio_VCA_GetVolume(self.pointer, &mut volume, &mut finalvolume) {
                ffi::FMOD_OK => Ok((volume, finalvolume)),
                error => Err(err_fmod!("FMOD_Studio_VCA_GetVolume", error)),
            }
        }
    }
    pub fn set_volume(&self, volume: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_Studio_VCA_SetVolume(self.pointer, volume) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_Studio_VCA_SetVolume", error)),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SyncPoint {
    pointer: *mut ffi::FMOD_SYNCPOINT,
}

impl SyncPoint {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_SYNCPOINT) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_SYNCPOINT {
        self.pointer
    }
}

#[derive(Debug, Clone, Copy)]
pub struct System {
    pointer: *mut ffi::FMOD_SYSTEM,
}

impl System {
    #[inline]
    pub fn from(pointer: *mut ffi::FMOD_SYSTEM) -> Self {
        Self { pointer }
    }
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::FMOD_SYSTEM {
        self.pointer
    }
    pub fn create() -> Result<System, Error> {
        unsafe {
            let mut system = null_mut();
            match ffi::FMOD_System_Create(&mut system, ffi::FMOD_VERSION) {
                ffi::FMOD_OK => Ok(System::from(system)),
                error => Err(err_fmod!("FMOD_System_Create", error)),
            }
        }
    }
    pub fn release(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_Release(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_Release", error)),
            }
        }
    }
    pub fn set_output(&self, output: OutputType) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetOutput(self.pointer, output.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetOutput", error)),
            }
        }
    }
    pub fn get_output(&self) -> Result<OutputType, Error> {
        unsafe {
            let mut output = ffi::FMOD_OUTPUTTYPE::default();
            match ffi::FMOD_System_GetOutput(self.pointer, &mut output) {
                ffi::FMOD_OK => Ok(OutputType::from(output)?),
                error => Err(err_fmod!("FMOD_System_GetOutput", error)),
            }
        }
    }
    pub fn get_num_drivers(&self) -> Result<i32, Error> {
        unsafe {
            let mut numdrivers = i32::default();
            match ffi::FMOD_System_GetNumDrivers(self.pointer, &mut numdrivers) {
                ffi::FMOD_OK => Ok(numdrivers),
                error => Err(err_fmod!("FMOD_System_GetNumDrivers", error)),
            }
        }
    }
    pub fn get_driver_info(
        &self,
        id: i32,
        namelen: i32,
    ) -> Result<(String, Guid, i32, SpeakerMode, i32), Error> {
        unsafe {
            let name = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut guid = ffi::FMOD_GUID::default();
            let mut systemrate = i32::default();
            let mut speakermode = ffi::FMOD_SPEAKERMODE::default();
            let mut speakermodechannels = i32::default();
            match ffi::FMOD_System_GetDriverInfo(
                self.pointer,
                id,
                name,
                namelen,
                &mut guid,
                &mut systemrate,
                &mut speakermode,
                &mut speakermodechannels,
            ) {
                ffi::FMOD_OK => Ok((
                    CString::from_raw(name)
                        .into_string()
                        .map_err(Error::String)?,
                    Guid::try_from(guid)?,
                    systemrate,
                    SpeakerMode::from(speakermode)?,
                    speakermodechannels,
                )),
                error => Err(err_fmod!("FMOD_System_GetDriverInfo", error)),
            }
        }
    }
    pub fn set_driver(&self, driver: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetDriver(self.pointer, driver) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetDriver", error)),
            }
        }
    }
    pub fn get_driver(&self) -> Result<i32, Error> {
        unsafe {
            let mut driver = i32::default();
            match ffi::FMOD_System_GetDriver(self.pointer, &mut driver) {
                ffi::FMOD_OK => Ok(driver),
                error => Err(err_fmod!("FMOD_System_GetDriver", error)),
            }
        }
    }
    pub fn set_software_channels(&self, numsoftwarechannels: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetSoftwareChannels(self.pointer, numsoftwarechannels) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetSoftwareChannels", error)),
            }
        }
    }
    pub fn get_software_channels(&self) -> Result<i32, Error> {
        unsafe {
            let mut numsoftwarechannels = i32::default();
            match ffi::FMOD_System_GetSoftwareChannels(self.pointer, &mut numsoftwarechannels) {
                ffi::FMOD_OK => Ok(numsoftwarechannels),
                error => Err(err_fmod!("FMOD_System_GetSoftwareChannels", error)),
            }
        }
    }
    pub fn set_software_format(
        &self,
        samplerate: Option<i32>,
        speakermode: Option<SpeakerMode>,
        numrawspeakers: Option<i32>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetSoftwareFormat(
                self.pointer,
                samplerate.unwrap_or(0),
                speakermode.map(|value| value.into()).unwrap_or(0),
                numrawspeakers.unwrap_or(0),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetSoftwareFormat", error)),
            }
        }
    }
    pub fn get_software_format(&self) -> Result<(i32, SpeakerMode, i32), Error> {
        unsafe {
            let mut samplerate = i32::default();
            let mut speakermode = ffi::FMOD_SPEAKERMODE::default();
            let mut numrawspeakers = i32::default();
            match ffi::FMOD_System_GetSoftwareFormat(
                self.pointer,
                &mut samplerate,
                &mut speakermode,
                &mut numrawspeakers,
            ) {
                ffi::FMOD_OK => Ok((samplerate, SpeakerMode::from(speakermode)?, numrawspeakers)),
                error => Err(err_fmod!("FMOD_System_GetSoftwareFormat", error)),
            }
        }
    }
    pub fn set_dsp_buffer_size(&self, bufferlength: u32, numbuffers: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetDSPBufferSize(self.pointer, bufferlength, numbuffers) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetDSPBufferSize", error)),
            }
        }
    }
    pub fn get_dsp_buffer_size(&self) -> Result<(u32, i32), Error> {
        unsafe {
            let mut bufferlength = u32::default();
            let mut numbuffers = i32::default();
            match ffi::FMOD_System_GetDSPBufferSize(
                self.pointer,
                &mut bufferlength,
                &mut numbuffers,
            ) {
                ffi::FMOD_OK => Ok((bufferlength, numbuffers)),
                error => Err(err_fmod!("FMOD_System_GetDSPBufferSize", error)),
            }
        }
    }
    pub fn set_file_system(
        &self,
        useropen: ffi::FMOD_FILE_OPEN_CALLBACK,
        userclose: ffi::FMOD_FILE_CLOSE_CALLBACK,
        userread: ffi::FMOD_FILE_READ_CALLBACK,
        userseek: ffi::FMOD_FILE_SEEK_CALLBACK,
        userasyncread: ffi::FMOD_FILE_ASYNCREAD_CALLBACK,
        userasynccancel: ffi::FMOD_FILE_ASYNCCANCEL_CALLBACK,
        blockalign: Option<i32>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetFileSystem(
                self.pointer,
                useropen,
                userclose,
                userread,
                userseek,
                userasyncread,
                userasynccancel,
                blockalign.unwrap_or(0),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetFileSystem", error)),
            }
        }
    }
    pub fn attach_file_system(
        &self,
        useropen: ffi::FMOD_FILE_OPEN_CALLBACK,
        userclose: ffi::FMOD_FILE_CLOSE_CALLBACK,
        userread: ffi::FMOD_FILE_READ_CALLBACK,
        userseek: ffi::FMOD_FILE_SEEK_CALLBACK,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_AttachFileSystem(
                self.pointer,
                useropen,
                userclose,
                userread,
                userseek,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_AttachFileSystem", error)),
            }
        }
    }
    pub fn set_advanced_settings(&self, settings: AdvancedSettings) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetAdvancedSettings(self.pointer, &mut settings.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetAdvancedSettings", error)),
            }
        }
    }
    pub fn get_advanced_settings(&self) -> Result<AdvancedSettings, Error> {
        unsafe {
            let mut settings = ffi::FMOD_ADVANCEDSETTINGS::default();
            match ffi::FMOD_System_GetAdvancedSettings(self.pointer, &mut settings) {
                ffi::FMOD_OK => Ok(AdvancedSettings::try_from(settings)?),
                error => Err(err_fmod!("FMOD_System_GetAdvancedSettings", error)),
            }
        }
    }
    pub fn set_callback(
        &self,
        callback: ffi::FMOD_SYSTEM_CALLBACK,
        callbackmask: ffi::FMOD_SYSTEM_CALLBACK_TYPE,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetCallback(self.pointer, callback, callbackmask) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetCallback", error)),
            }
        }
    }
    pub fn set_plugin_path(&self, path: &str) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetPluginPath(self.pointer, CString::new(path)?.as_ptr()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetPluginPath", error)),
            }
        }
    }
    pub fn load_plugin(&self, filename: &str, priority: Option<u32>) -> Result<u32, Error> {
        unsafe {
            let mut handle = u32::default();
            match ffi::FMOD_System_LoadPlugin(
                self.pointer,
                CString::new(filename)?.as_ptr(),
                &mut handle,
                priority.unwrap_or(0),
            ) {
                ffi::FMOD_OK => Ok(handle),
                error => Err(err_fmod!("FMOD_System_LoadPlugin", error)),
            }
        }
    }
    pub fn unload_plugin(&self, handle: u32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_UnloadPlugin(self.pointer, handle) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_UnloadPlugin", error)),
            }
        }
    }
    pub fn get_num_nested_plugins(&self, handle: u32) -> Result<i32, Error> {
        unsafe {
            let mut count = i32::default();
            match ffi::FMOD_System_GetNumNestedPlugins(self.pointer, handle, &mut count) {
                ffi::FMOD_OK => Ok(count),
                error => Err(err_fmod!("FMOD_System_GetNumNestedPlugins", error)),
            }
        }
    }
    pub fn get_nested_plugin(&self, handle: u32, index: i32) -> Result<u32, Error> {
        unsafe {
            let mut nestedhandle = u32::default();
            match ffi::FMOD_System_GetNestedPlugin(self.pointer, handle, index, &mut nestedhandle) {
                ffi::FMOD_OK => Ok(nestedhandle),
                error => Err(err_fmod!("FMOD_System_GetNestedPlugin", error)),
            }
        }
    }
    pub fn get_num_plugins(&self, plugintype: PluginType) -> Result<i32, Error> {
        unsafe {
            let mut numplugins = i32::default();
            match ffi::FMOD_System_GetNumPlugins(self.pointer, plugintype.into(), &mut numplugins) {
                ffi::FMOD_OK => Ok(numplugins),
                error => Err(err_fmod!("FMOD_System_GetNumPlugins", error)),
            }
        }
    }
    pub fn get_plugin_handle(&self, plugintype: PluginType, index: i32) -> Result<u32, Error> {
        unsafe {
            let mut handle = u32::default();
            match ffi::FMOD_System_GetPluginHandle(
                self.pointer,
                plugintype.into(),
                index,
                &mut handle,
            ) {
                ffi::FMOD_OK => Ok(handle),
                error => Err(err_fmod!("FMOD_System_GetPluginHandle", error)),
            }
        }
    }
    pub fn get_plugin_info(
        &self,
        handle: u32,
        namelen: i32,
    ) -> Result<(PluginType, String, u32), Error> {
        unsafe {
            let mut plugintype = ffi::FMOD_PLUGINTYPE::default();
            let name = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut version = u32::default();
            match ffi::FMOD_System_GetPluginInfo(
                self.pointer,
                handle,
                &mut plugintype,
                name,
                namelen,
                &mut version,
            ) {
                ffi::FMOD_OK => Ok((
                    PluginType::from(plugintype)?,
                    CString::from_raw(name)
                        .into_string()
                        .map_err(Error::String)?,
                    version,
                )),
                error => Err(err_fmod!("FMOD_System_GetPluginInfo", error)),
            }
        }
    }
    pub fn set_output_by_plugin(&self, handle: u32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetOutputByPlugin(self.pointer, handle) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetOutputByPlugin", error)),
            }
        }
    }
    pub fn get_output_by_plugin(&self) -> Result<u32, Error> {
        unsafe {
            let mut handle = u32::default();
            match ffi::FMOD_System_GetOutputByPlugin(self.pointer, &mut handle) {
                ffi::FMOD_OK => Ok(handle),
                error => Err(err_fmod!("FMOD_System_GetOutputByPlugin", error)),
            }
        }
    }
    pub fn create_dsp_by_plugin(&self, handle: u32) -> Result<Dsp, Error> {
        unsafe {
            let mut dsp = null_mut();
            match ffi::FMOD_System_CreateDSPByPlugin(self.pointer, handle, &mut dsp) {
                ffi::FMOD_OK => Ok(Dsp::from(dsp)),
                error => Err(err_fmod!("FMOD_System_CreateDSPByPlugin", error)),
            }
        }
    }
    pub fn get_dsp_info_by_plugin(&self, handle: u32) -> Result<DspDescription, Error> {
        unsafe {
            let mut description = null();
            match ffi::FMOD_System_GetDSPInfoByPlugin(self.pointer, handle, &mut description) {
                ffi::FMOD_OK => Ok(DspDescription::try_from(*description)?),
                error => Err(err_fmod!("FMOD_System_GetDSPInfoByPlugin", error)),
            }
        }
    }
    pub fn register_codec(
        &self,
        description: CodecDescription,
        priority: Option<u32>,
    ) -> Result<u32, Error> {
        unsafe {
            let mut handle = u32::default();
            match ffi::FMOD_System_RegisterCodec(
                self.pointer,
                &mut description.into(),
                &mut handle,
                priority.unwrap_or(0),
            ) {
                ffi::FMOD_OK => Ok(handle),
                error => Err(err_fmod!("FMOD_System_RegisterCodec", error)),
            }
        }
    }
    pub fn register_dsp(&self, description: DspDescription) -> Result<u32, Error> {
        unsafe {
            let mut handle = u32::default();
            match ffi::FMOD_System_RegisterDSP(self.pointer, &description.into(), &mut handle) {
                ffi::FMOD_OK => Ok(handle),
                error => Err(err_fmod!("FMOD_System_RegisterDSP", error)),
            }
        }
    }
    pub fn register_output(&self, description: OutputDescription) -> Result<u32, Error> {
        unsafe {
            let mut handle = u32::default();
            match ffi::FMOD_System_RegisterOutput(self.pointer, &description.into(), &mut handle) {
                ffi::FMOD_OK => Ok(handle),
                error => Err(err_fmod!("FMOD_System_RegisterOutput", error)),
            }
        }
    }
    pub fn init(
        &self,
        maxchannels: i32,
        flags: ffi::FMOD_INITFLAGS,
        extradriverdata: Option<*mut c_void>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_Init(
                self.pointer,
                maxchannels,
                flags,
                extradriverdata.unwrap_or(null_mut()),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_Init", error)),
            }
        }
    }
    pub fn close(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_Close(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_Close", error)),
            }
        }
    }
    pub fn update(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_Update(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_Update", error)),
            }
        }
    }
    pub fn set_speaker_position(
        &self,
        speaker: Speaker,
        x: f32,
        y: f32,
        active: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetSpeakerPosition(
                self.pointer,
                speaker.into(),
                x,
                y,
                from_bool!(active),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetSpeakerPosition", error)),
            }
        }
    }
    pub fn get_speaker_position(&self, speaker: Speaker) -> Result<(f32, f32, bool), Error> {
        unsafe {
            let mut x = f32::default();
            let mut y = f32::default();
            let mut active = ffi::FMOD_BOOL::default();
            match ffi::FMOD_System_GetSpeakerPosition(
                self.pointer,
                speaker.into(),
                &mut x,
                &mut y,
                &mut active,
            ) {
                ffi::FMOD_OK => Ok((x, y, to_bool!(active))),
                error => Err(err_fmod!("FMOD_System_GetSpeakerPosition", error)),
            }
        }
    }
    pub fn set_stream_buffer_size(
        &self,
        filebuffersize: u32,
        filebuffersizetype: ffi::FMOD_TIMEUNIT,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetStreamBufferSize(
                self.pointer,
                filebuffersize,
                filebuffersizetype,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetStreamBufferSize", error)),
            }
        }
    }
    pub fn get_stream_buffer_size(&self) -> Result<(u32, ffi::FMOD_TIMEUNIT), Error> {
        unsafe {
            let mut filebuffersize = u32::default();
            let mut filebuffersizetype = ffi::FMOD_TIMEUNIT::default();
            match ffi::FMOD_System_GetStreamBufferSize(
                self.pointer,
                &mut filebuffersize,
                &mut filebuffersizetype,
            ) {
                ffi::FMOD_OK => Ok((filebuffersize, filebuffersizetype)),
                error => Err(err_fmod!("FMOD_System_GetStreamBufferSize", error)),
            }
        }
    }
    pub fn set_3d_settings(
        &self,
        dopplerscale: f32,
        distancefactor: f32,
        rolloffscale: f32,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_Set3DSettings(
                self.pointer,
                dopplerscale,
                distancefactor,
                rolloffscale,
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_Set3DSettings", error)),
            }
        }
    }
    pub fn get_3d_settings(&self) -> Result<(f32, f32, f32), Error> {
        unsafe {
            let mut dopplerscale = f32::default();
            let mut distancefactor = f32::default();
            let mut rolloffscale = f32::default();
            match ffi::FMOD_System_Get3DSettings(
                self.pointer,
                &mut dopplerscale,
                &mut distancefactor,
                &mut rolloffscale,
            ) {
                ffi::FMOD_OK => Ok((dopplerscale, distancefactor, rolloffscale)),
                error => Err(err_fmod!("FMOD_System_Get3DSettings", error)),
            }
        }
    }
    pub fn set_3d_num_listeners(&self, numlisteners: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_Set3DNumListeners(self.pointer, numlisteners) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_Set3DNumListeners", error)),
            }
        }
    }
    pub fn get_3d_num_listeners(&self) -> Result<i32, Error> {
        unsafe {
            let mut numlisteners = i32::default();
            match ffi::FMOD_System_Get3DNumListeners(self.pointer, &mut numlisteners) {
                ffi::FMOD_OK => Ok(numlisteners),
                error => Err(err_fmod!("FMOD_System_Get3DNumListeners", error)),
            }
        }
    }
    pub fn set_3d_listener_attributes(
        &self,
        listener: i32,
        pos: Option<Vector>,
        vel: Option<Vector>,
        forward: Option<Vector>,
        up: Option<Vector>,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_Set3DListenerAttributes(
                self.pointer,
                listener,
                pos.map(|value| &value.into() as *const _).unwrap_or(null()),
                vel.map(|value| &value.into() as *const _).unwrap_or(null()),
                forward
                    .map(|value| &value.into() as *const _)
                    .unwrap_or(null()),
                up.map(|value| &value.into() as *const _).unwrap_or(null()),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_Set3DListenerAttributes", error)),
            }
        }
    }
    pub fn get_3d_listener_attributes(
        &self,
        listener: i32,
    ) -> Result<(Vector, Vector, Vector, Vector), Error> {
        unsafe {
            let mut pos = ffi::FMOD_VECTOR::default();
            let mut vel = ffi::FMOD_VECTOR::default();
            let mut forward = ffi::FMOD_VECTOR::default();
            let mut up = ffi::FMOD_VECTOR::default();
            match ffi::FMOD_System_Get3DListenerAttributes(
                self.pointer,
                listener,
                &mut pos,
                &mut vel,
                &mut forward,
                &mut up,
            ) {
                ffi::FMOD_OK => Ok((
                    Vector::try_from(pos)?,
                    Vector::try_from(vel)?,
                    Vector::try_from(forward)?,
                    Vector::try_from(up)?,
                )),
                error => Err(err_fmod!("FMOD_System_Get3DListenerAttributes", error)),
            }
        }
    }
    pub fn set_3d_rolloff_callback(
        &self,
        callback: ffi::FMOD_3D_ROLLOFF_CALLBACK,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_Set3DRolloffCallback(self.pointer, callback) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_Set3DRolloffCallback", error)),
            }
        }
    }
    pub fn mixer_suspend(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_MixerSuspend(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_MixerSuspend", error)),
            }
        }
    }
    pub fn mixer_resume(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_MixerResume(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_MixerResume", error)),
            }
        }
    }
    pub fn get_default_mix_matrix(
        &self,
        sourcespeakermode: SpeakerMode,
        targetspeakermode: SpeakerMode,
        matrixhop: Option<i32>,
    ) -> Result<f32, Error> {
        unsafe {
            let mut matrix = f32::default();
            match ffi::FMOD_System_GetDefaultMixMatrix(
                self.pointer,
                sourcespeakermode.into(),
                targetspeakermode.into(),
                &mut matrix,
                matrixhop.unwrap_or(0),
            ) {
                ffi::FMOD_OK => Ok(matrix),
                error => Err(err_fmod!("FMOD_System_GetDefaultMixMatrix", error)),
            }
        }
    }
    pub fn get_speaker_mode_channels(&self, mode: SpeakerMode) -> Result<i32, Error> {
        unsafe {
            let mut channels = i32::default();
            match ffi::FMOD_System_GetSpeakerModeChannels(self.pointer, mode.into(), &mut channels)
            {
                ffi::FMOD_OK => Ok(channels),
                error => Err(err_fmod!("FMOD_System_GetSpeakerModeChannels", error)),
            }
        }
    }
    pub fn get_version(&self) -> Result<u32, Error> {
        unsafe {
            let mut version = u32::default();
            match ffi::FMOD_System_GetVersion(self.pointer, &mut version) {
                ffi::FMOD_OK => Ok(version),
                error => Err(err_fmod!("FMOD_System_GetVersion", error)),
            }
        }
    }
    pub fn get_output_handle(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut handle = null_mut();
            match ffi::FMOD_System_GetOutputHandle(self.pointer, &mut handle) {
                ffi::FMOD_OK => Ok(handle),
                error => Err(err_fmod!("FMOD_System_GetOutputHandle", error)),
            }
        }
    }
    pub fn get_channels_playing(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let mut channels = i32::default();
            let mut realchannels = i32::default();
            match ffi::FMOD_System_GetChannelsPlaying(
                self.pointer,
                &mut channels,
                &mut realchannels,
            ) {
                ffi::FMOD_OK => Ok((channels, realchannels)),
                error => Err(err_fmod!("FMOD_System_GetChannelsPlaying", error)),
            }
        }
    }
    pub fn get_cpu_usage(&self) -> Result<CpuUsage, Error> {
        unsafe {
            let mut usage = ffi::FMOD_CPU_USAGE::default();
            match ffi::FMOD_System_GetCPUUsage(self.pointer, &mut usage) {
                ffi::FMOD_OK => Ok(CpuUsage::try_from(usage)?),
                error => Err(err_fmod!("FMOD_System_GetCPUUsage", error)),
            }
        }
    }
    pub fn get_file_usage(&self) -> Result<(i64, i64, i64), Error> {
        unsafe {
            let mut sample_bytes_read = i64::default();
            let mut stream_bytes_read = i64::default();
            let mut other_bytes_read = i64::default();
            match ffi::FMOD_System_GetFileUsage(
                self.pointer,
                &mut sample_bytes_read,
                &mut stream_bytes_read,
                &mut other_bytes_read,
            ) {
                ffi::FMOD_OK => Ok((sample_bytes_read, stream_bytes_read, other_bytes_read)),
                error => Err(err_fmod!("FMOD_System_GetFileUsage", error)),
            }
        }
    }
    pub fn create_sound(
        &self,
        name_or_data: &str,
        mode: ffi::FMOD_MODE,
        exinfo: Option<CreateSoundexInfo>,
    ) -> Result<Sound, Error> {
        unsafe {
            let mut sound = null_mut();
            match ffi::FMOD_System_CreateSound(
                self.pointer,
                CString::new(name_or_data)?.as_ptr(),
                mode,
                exinfo
                    .map(|value| &mut value.into() as *mut _)
                    .unwrap_or(null_mut()),
                &mut sound,
            ) {
                ffi::FMOD_OK => Ok(Sound::from(sound)),
                error => Err(err_fmod!("FMOD_System_CreateSound", error)),
            }
        }
    }
    pub fn create_stream(
        &self,
        name_or_data: &str,
        mode: ffi::FMOD_MODE,
        exinfo: Option<CreateSoundexInfo>,
    ) -> Result<Sound, Error> {
        unsafe {
            let mut sound = null_mut();
            match ffi::FMOD_System_CreateStream(
                self.pointer,
                CString::new(name_or_data)?.as_ptr(),
                mode,
                exinfo
                    .map(|value| &mut value.into() as *mut _)
                    .unwrap_or(null_mut()),
                &mut sound,
            ) {
                ffi::FMOD_OK => Ok(Sound::from(sound)),
                error => Err(err_fmod!("FMOD_System_CreateStream", error)),
            }
        }
    }
    pub fn create_dsp(&self, description: DspDescription) -> Result<Dsp, Error> {
        unsafe {
            let mut dsp = null_mut();
            match ffi::FMOD_System_CreateDSP(self.pointer, &description.into(), &mut dsp) {
                ffi::FMOD_OK => Ok(Dsp::from(dsp)),
                error => Err(err_fmod!("FMOD_System_CreateDSP", error)),
            }
        }
    }
    pub fn create_dsp_by_type(&self, type_: DspType) -> Result<Dsp, Error> {
        unsafe {
            let mut dsp = null_mut();
            match ffi::FMOD_System_CreateDSPByType(self.pointer, type_.into(), &mut dsp) {
                ffi::FMOD_OK => Ok(Dsp::from(dsp)),
                error => Err(err_fmod!("FMOD_System_CreateDSPByType", error)),
            }
        }
    }
    pub fn create_channel_group(&self, name: Option<String>) -> Result<ChannelGroup, Error> {
        unsafe {
            let mut channelgroup = null_mut();
            match ffi::FMOD_System_CreateChannelGroup(
                self.pointer,
                name.map(|value| CString::new(value).map(|value| value.as_ptr()))
                    .unwrap_or(Ok(null_mut()))?,
                &mut channelgroup,
            ) {
                ffi::FMOD_OK => Ok(ChannelGroup::from(channelgroup)),
                error => Err(err_fmod!("FMOD_System_CreateChannelGroup", error)),
            }
        }
    }
    pub fn create_sound_group(&self, name: &str) -> Result<SoundGroup, Error> {
        unsafe {
            let mut soundgroup = null_mut();
            match ffi::FMOD_System_CreateSoundGroup(
                self.pointer,
                CString::new(name)?.as_ptr(),
                &mut soundgroup,
            ) {
                ffi::FMOD_OK => Ok(SoundGroup::from(soundgroup)),
                error => Err(err_fmod!("FMOD_System_CreateSoundGroup", error)),
            }
        }
    }
    pub fn create_reverb_3d(&self) -> Result<Reverb3d, Error> {
        unsafe {
            let mut reverb = null_mut();
            match ffi::FMOD_System_CreateReverb3D(self.pointer, &mut reverb) {
                ffi::FMOD_OK => Ok(Reverb3d::from(reverb)),
                error => Err(err_fmod!("FMOD_System_CreateReverb3D", error)),
            }
        }
    }
    pub fn play_sound(
        &self,
        sound: Sound,
        channelgroup: Option<ChannelGroup>,
        paused: bool,
    ) -> Result<Channel, Error> {
        unsafe {
            let mut channel = null_mut();
            match ffi::FMOD_System_PlaySound(
                self.pointer,
                sound.as_mut_ptr(),
                channelgroup
                    .map(|value| value.as_mut_ptr())
                    .unwrap_or(null_mut()),
                from_bool!(paused),
                &mut channel,
            ) {
                ffi::FMOD_OK => Ok(Channel::from(channel)),
                error => Err(err_fmod!("FMOD_System_PlaySound", error)),
            }
        }
    }
    pub fn play_dsp(
        &self,
        dsp: Dsp,
        channelgroup: Option<ChannelGroup>,
        paused: bool,
    ) -> Result<Channel, Error> {
        unsafe {
            let mut channel = null_mut();
            match ffi::FMOD_System_PlayDSP(
                self.pointer,
                dsp.as_mut_ptr(),
                channelgroup
                    .map(|value| value.as_mut_ptr())
                    .unwrap_or(null_mut()),
                from_bool!(paused),
                &mut channel,
            ) {
                ffi::FMOD_OK => Ok(Channel::from(channel)),
                error => Err(err_fmod!("FMOD_System_PlayDSP", error)),
            }
        }
    }
    pub fn get_channel(&self, channelid: i32) -> Result<Channel, Error> {
        unsafe {
            let mut channel = null_mut();
            match ffi::FMOD_System_GetChannel(self.pointer, channelid, &mut channel) {
                ffi::FMOD_OK => Ok(Channel::from(channel)),
                error => Err(err_fmod!("FMOD_System_GetChannel", error)),
            }
        }
    }
    pub fn get_dsp_info_by_type(&self, type_: DspType) -> Result<DspDescription, Error> {
        unsafe {
            let mut description = null();
            match ffi::FMOD_System_GetDSPInfoByType(self.pointer, type_.into(), &mut description) {
                ffi::FMOD_OK => Ok(DspDescription::try_from(*description)?),
                error => Err(err_fmod!("FMOD_System_GetDSPInfoByType", error)),
            }
        }
    }
    pub fn get_master_channel_group(&self) -> Result<ChannelGroup, Error> {
        unsafe {
            let mut channelgroup = null_mut();
            match ffi::FMOD_System_GetMasterChannelGroup(self.pointer, &mut channelgroup) {
                ffi::FMOD_OK => Ok(ChannelGroup::from(channelgroup)),
                error => Err(err_fmod!("FMOD_System_GetMasterChannelGroup", error)),
            }
        }
    }
    pub fn get_master_sound_group(&self) -> Result<SoundGroup, Error> {
        unsafe {
            let mut soundgroup = null_mut();
            match ffi::FMOD_System_GetMasterSoundGroup(self.pointer, &mut soundgroup) {
                ffi::FMOD_OK => Ok(SoundGroup::from(soundgroup)),
                error => Err(err_fmod!("FMOD_System_GetMasterSoundGroup", error)),
            }
        }
    }
    pub fn attach_channel_group_to_port(
        &self,
        port_type: PortType,
        port_index: u64,
        channelgroup: ChannelGroup,
        pass_thru: bool,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_AttachChannelGroupToPort(
                self.pointer,
                port_type.into(),
                port_index,
                channelgroup.as_mut_ptr(),
                from_bool!(pass_thru),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_AttachChannelGroupToPort", error)),
            }
        }
    }
    pub fn detach_channel_group_from_port(&self, channelgroup: ChannelGroup) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_DetachChannelGroupFromPort(
                self.pointer,
                channelgroup.as_mut_ptr(),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_DetachChannelGroupFromPort", error)),
            }
        }
    }
    pub fn set_reverb_properties(
        &self,
        instance: i32,
        prop: ReverbProperties,
    ) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetReverbProperties(self.pointer, instance, &prop.into()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetReverbProperties", error)),
            }
        }
    }
    pub fn get_reverb_properties(&self, instance: i32) -> Result<ReverbProperties, Error> {
        unsafe {
            let mut prop = ffi::FMOD_REVERB_PROPERTIES::default();
            match ffi::FMOD_System_GetReverbProperties(self.pointer, instance, &mut prop) {
                ffi::FMOD_OK => Ok(ReverbProperties::try_from(prop)?),
                error => Err(err_fmod!("FMOD_System_GetReverbProperties", error)),
            }
        }
    }
    pub fn lock_dsp(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_LockDSP(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_LockDSP", error)),
            }
        }
    }
    pub fn unlock_dsp(&self) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_UnlockDSP(self.pointer) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_UnlockDSP", error)),
            }
        }
    }
    pub fn get_record_num_drivers(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let mut numdrivers = i32::default();
            let mut numconnected = i32::default();
            match ffi::FMOD_System_GetRecordNumDrivers(
                self.pointer,
                &mut numdrivers,
                &mut numconnected,
            ) {
                ffi::FMOD_OK => Ok((numdrivers, numconnected)),
                error => Err(err_fmod!("FMOD_System_GetRecordNumDrivers", error)),
            }
        }
    }
    pub fn get_record_driver_info(
        &self,
        id: i32,
        namelen: i32,
    ) -> Result<(String, Guid, i32, SpeakerMode, i32, ffi::FMOD_DRIVER_STATE), Error> {
        unsafe {
            let name = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            let mut guid = ffi::FMOD_GUID::default();
            let mut systemrate = i32::default();
            let mut speakermode = ffi::FMOD_SPEAKERMODE::default();
            let mut speakermodechannels = i32::default();
            let mut state = ffi::FMOD_DRIVER_STATE::default();
            match ffi::FMOD_System_GetRecordDriverInfo(
                self.pointer,
                id,
                name,
                namelen,
                &mut guid,
                &mut systemrate,
                &mut speakermode,
                &mut speakermodechannels,
                &mut state,
            ) {
                ffi::FMOD_OK => Ok((
                    CString::from_raw(name)
                        .into_string()
                        .map_err(Error::String)?,
                    Guid::try_from(guid)?,
                    systemrate,
                    SpeakerMode::from(speakermode)?,
                    speakermodechannels,
                    state,
                )),
                error => Err(err_fmod!("FMOD_System_GetRecordDriverInfo", error)),
            }
        }
    }
    pub fn get_record_position(&self, id: i32) -> Result<u32, Error> {
        unsafe {
            let mut position = u32::default();
            match ffi::FMOD_System_GetRecordPosition(self.pointer, id, &mut position) {
                ffi::FMOD_OK => Ok(position),
                error => Err(err_fmod!("FMOD_System_GetRecordPosition", error)),
            }
        }
    }
    pub fn record_start(&self, id: i32, sound: Sound, loop_: bool) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_RecordStart(
                self.pointer,
                id,
                sound.as_mut_ptr(),
                from_bool!(loop_),
            ) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_RecordStart", error)),
            }
        }
    }
    pub fn record_stop(&self, id: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_RecordStop(self.pointer, id) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_RecordStop", error)),
            }
        }
    }
    pub fn is_recording(&self, id: i32) -> Result<bool, Error> {
        unsafe {
            let mut recording = ffi::FMOD_BOOL::default();
            match ffi::FMOD_System_IsRecording(self.pointer, id, &mut recording) {
                ffi::FMOD_OK => Ok(to_bool!(recording)),
                error => Err(err_fmod!("FMOD_System_IsRecording", error)),
            }
        }
    }
    pub fn create_geometry(&self, maxpolygons: i32, maxvertices: i32) -> Result<Geometry, Error> {
        unsafe {
            let mut geometry = null_mut();
            match ffi::FMOD_System_CreateGeometry(
                self.pointer,
                maxpolygons,
                maxvertices,
                &mut geometry,
            ) {
                ffi::FMOD_OK => Ok(Geometry::from(geometry)),
                error => Err(err_fmod!("FMOD_System_CreateGeometry", error)),
            }
        }
    }
    pub fn set_geometry_settings(&self, maxworldsize: f32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetGeometrySettings(self.pointer, maxworldsize) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetGeometrySettings", error)),
            }
        }
    }
    pub fn get_geometry_settings(&self) -> Result<f32, Error> {
        unsafe {
            let mut maxworldsize = f32::default();
            match ffi::FMOD_System_GetGeometrySettings(self.pointer, &mut maxworldsize) {
                ffi::FMOD_OK => Ok(maxworldsize),
                error => Err(err_fmod!("FMOD_System_GetGeometrySettings", error)),
            }
        }
    }
    pub fn load_geometry(&self, data: *const c_void, datasize: i32) -> Result<Geometry, Error> {
        unsafe {
            let mut geometry = null_mut();
            match ffi::FMOD_System_LoadGeometry(self.pointer, data, datasize, &mut geometry) {
                ffi::FMOD_OK => Ok(Geometry::from(geometry)),
                error => Err(err_fmod!("FMOD_System_LoadGeometry", error)),
            }
        }
    }
    pub fn get_geometry_occlusion(
        &self,
        listener: Vector,
        source: Vector,
    ) -> Result<(f32, f32), Error> {
        unsafe {
            let mut direct = f32::default();
            let mut reverb = f32::default();
            match ffi::FMOD_System_GetGeometryOcclusion(
                self.pointer,
                &listener.into(),
                &source.into(),
                &mut direct,
                &mut reverb,
            ) {
                ffi::FMOD_OK => Ok((direct, reverb)),
                error => Err(err_fmod!("FMOD_System_GetGeometryOcclusion", error)),
            }
        }
    }
    pub fn set_network_proxy(&self, proxy: &str) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetNetworkProxy(self.pointer, CString::new(proxy)?.as_ptr()) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetNetworkProxy", error)),
            }
        }
    }
    pub fn get_network_proxy(&self, proxylen: i32) -> Result<String, Error> {
        unsafe {
            let proxy = CString::from_vec_unchecked(b"".to_vec()).into_raw();
            match ffi::FMOD_System_GetNetworkProxy(self.pointer, proxy, proxylen) {
                ffi::FMOD_OK => Ok(CString::from_raw(proxy)
                    .into_string()
                    .map_err(Error::String)?),
                error => Err(err_fmod!("FMOD_System_GetNetworkProxy", error)),
            }
        }
    }
    pub fn set_network_timeout(&self, timeout: i32) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetNetworkTimeout(self.pointer, timeout) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetNetworkTimeout", error)),
            }
        }
    }
    pub fn get_network_timeout(&self) -> Result<i32, Error> {
        unsafe {
            let mut timeout = i32::default();
            match ffi::FMOD_System_GetNetworkTimeout(self.pointer, &mut timeout) {
                ffi::FMOD_OK => Ok(timeout),
                error => Err(err_fmod!("FMOD_System_GetNetworkTimeout", error)),
            }
        }
    }
    pub fn set_user_data(&self, userdata: *mut c_void) -> Result<(), Error> {
        unsafe {
            match ffi::FMOD_System_SetUserData(self.pointer, userdata) {
                ffi::FMOD_OK => Ok(()),
                error => Err(err_fmod!("FMOD_System_SetUserData", error)),
            }
        }
    }
    pub fn get_user_data(&self) -> Result<*mut c_void, Error> {
        unsafe {
            let mut userdata = null_mut();
            match ffi::FMOD_System_GetUserData(self.pointer, &mut userdata) {
                ffi::FMOD_OK => Ok(userdata),
                error => Err(err_fmod!("FMOD_System_GetUserData", error)),
            }
        }
    }
}
