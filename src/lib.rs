use std::ptr::null_mut;
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
    Channelpriority,
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
            EventProperty::Channelpriority => ffi::FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY,
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
            ffi::FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY => Ok(EventProperty::Channelpriority),
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
    Ustaining,
    Opped,
    Rting,
    Opping,
}
impl From<PlaybackState> for ffi::FMOD_STUDIO_PLAYBACK_STATE {
    fn from(value: PlaybackState) -> ffi::FMOD_STUDIO_PLAYBACK_STATE {
        match value {
            PlaybackState::Playing => ffi::FMOD_STUDIO_PLAYBACK_PLAYING,
            PlaybackState::Ustaining => ffi::FMOD_STUDIO_PLAYBACK_SUSTAINING,
            PlaybackState::Opped => ffi::FMOD_STUDIO_PLAYBACK_STOPPED,
            PlaybackState::Rting => ffi::FMOD_STUDIO_PLAYBACK_STARTING,
            PlaybackState::Opping => ffi::FMOD_STUDIO_PLAYBACK_STOPPING,
        }
    }
}
impl PlaybackState {
    pub fn from(value: ffi::FMOD_STUDIO_PLAYBACK_STATE) -> Result<PlaybackState, Error> {
        match value {
            ffi::FMOD_STUDIO_PLAYBACK_PLAYING => Ok(PlaybackState::Playing),
            ffi::FMOD_STUDIO_PLAYBACK_SUSTAINING => Ok(PlaybackState::Ustaining),
            ffi::FMOD_STUDIO_PLAYBACK_STOPPED => Ok(PlaybackState::Opped),
            ffi::FMOD_STUDIO_PLAYBACK_STARTING => Ok(PlaybackState::Rting),
            ffi::FMOD_STUDIO_PLAYBACK_STOPPING => Ok(PlaybackState::Opping),
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
    _5Point1,
    _7Point1,
    _7Point1Point4,
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
            SpeakerMode::_5Point1 => ffi::FMOD_SPEAKERMODE_5POINT1,
            SpeakerMode::_7Point1 => ffi::FMOD_SPEAKERMODE_7POINT1,
            SpeakerMode::_7Point1Point4 => ffi::FMOD_SPEAKERMODE_7POINT1POINT4,
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
            ffi::FMOD_SPEAKERMODE_5POINT1 => Ok(SpeakerMode::_5Point1),
            ffi::FMOD_SPEAKERMODE_7POINT1 => Ok(SpeakerMode::_7Point1),
            ffi::FMOD_SPEAKERMODE_7POINT1POINT4 => Ok(SpeakerMode::_7Point1Point4),
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
    _3Dattributes,
    Sidechain,
    Fft,
    _3DattributesMulti,
    AttenuationRange,
}
impl From<DspParameterDataType> for ffi::FMOD_DSP_PARAMETER_DATA_TYPE {
    fn from(value: DspParameterDataType) -> ffi::FMOD_DSP_PARAMETER_DATA_TYPE {
        match value {
            DspParameterDataType::User => ffi::FMOD_DSP_PARAMETER_DATA_TYPE_USER,
            DspParameterDataType::OverallGain => ffi::FMOD_DSP_PARAMETER_DATA_TYPE_OVERALLGAIN,
            DspParameterDataType::_3Dattributes => ffi::FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES,
            DspParameterDataType::Sidechain => ffi::FMOD_DSP_PARAMETER_DATA_TYPE_SIDECHAIN,
            DspParameterDataType::Fft => ffi::FMOD_DSP_PARAMETER_DATA_TYPE_FFT,
            DspParameterDataType::_3DattributesMulti => {
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
                Ok(DspParameterDataType::_3Dattributes)
            }
            ffi::FMOD_DSP_PARAMETER_DATA_TYPE_SIDECHAIN => Ok(DspParameterDataType::Sidechain),
            ffi::FMOD_DSP_PARAMETER_DATA_TYPE_FFT => Ok(DspParameterDataType::Fft),
            ffi::FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES_MULTI => {
                Ok(DspParameterDataType::_3DattributesMulti)
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
    _2DStereoPosition,
    _2DDirection,
    _2DExtent,
    _2DRotation,
    _2DLfeLevel,
    _2DStereoMode,
    _2DStereoSeparation,
    _2DStereoAxis,
    EnabledSpeakers,
    _3DPosition,
    _3DRolloff,
    _3DMinDistance,
    _3DMaxDistance,
    _3DExtentMode,
    _3DSoundSize,
    _3DMinExtent,
    _3DPanBlend,
    LfeUpmixEnabled,
    OverallGain,
    SurroundSpeakerMode,
    _2DHeightBlend,
    AttenuationRange,
    OverrideRange,
}
impl From<DspPan> for ffi::FMOD_DSP_PAN {
    fn from(value: DspPan) -> ffi::FMOD_DSP_PAN {
        match value {
            DspPan::Mode => ffi::FMOD_DSP_PAN_MODE,
            DspPan::_2DStereoPosition => ffi::FMOD_DSP_PAN_2D_STEREO_POSITION,
            DspPan::_2DDirection => ffi::FMOD_DSP_PAN_2D_DIRECTION,
            DspPan::_2DExtent => ffi::FMOD_DSP_PAN_2D_EXTENT,
            DspPan::_2DRotation => ffi::FMOD_DSP_PAN_2D_ROTATION,
            DspPan::_2DLfeLevel => ffi::FMOD_DSP_PAN_2D_LFE_LEVEL,
            DspPan::_2DStereoMode => ffi::FMOD_DSP_PAN_2D_STEREO_MODE,
            DspPan::_2DStereoSeparation => ffi::FMOD_DSP_PAN_2D_STEREO_SEPARATION,
            DspPan::_2DStereoAxis => ffi::FMOD_DSP_PAN_2D_STEREO_AXIS,
            DspPan::EnabledSpeakers => ffi::FMOD_DSP_PAN_ENABLED_SPEAKERS,
            DspPan::_3DPosition => ffi::FMOD_DSP_PAN_3D_POSITION,
            DspPan::_3DRolloff => ffi::FMOD_DSP_PAN_3D_ROLLOFF,
            DspPan::_3DMinDistance => ffi::FMOD_DSP_PAN_3D_MIN_DISTANCE,
            DspPan::_3DMaxDistance => ffi::FMOD_DSP_PAN_3D_MAX_DISTANCE,
            DspPan::_3DExtentMode => ffi::FMOD_DSP_PAN_3D_EXTENT_MODE,
            DspPan::_3DSoundSize => ffi::FMOD_DSP_PAN_3D_SOUND_SIZE,
            DspPan::_3DMinExtent => ffi::FMOD_DSP_PAN_3D_MIN_EXTENT,
            DspPan::_3DPanBlend => ffi::FMOD_DSP_PAN_3D_PAN_BLEND,
            DspPan::LfeUpmixEnabled => ffi::FMOD_DSP_PAN_LFE_UPMIX_ENABLED,
            DspPan::OverallGain => ffi::FMOD_DSP_PAN_OVERALL_GAIN,
            DspPan::SurroundSpeakerMode => ffi::FMOD_DSP_PAN_SURROUND_SPEAKER_MODE,
            DspPan::_2DHeightBlend => ffi::FMOD_DSP_PAN_2D_HEIGHT_BLEND,
            DspPan::AttenuationRange => ffi::FMOD_DSP_PAN_ATTENUATION_RANGE,
            DspPan::OverrideRange => ffi::FMOD_DSP_PAN_OVERRIDE_RANGE,
        }
    }
}
impl DspPan {
    pub fn from(value: ffi::FMOD_DSP_PAN) -> Result<DspPan, Error> {
        match value {
            ffi::FMOD_DSP_PAN_MODE => Ok(DspPan::Mode),
            ffi::FMOD_DSP_PAN_2D_STEREO_POSITION => Ok(DspPan::_2DStereoPosition),
            ffi::FMOD_DSP_PAN_2D_DIRECTION => Ok(DspPan::_2DDirection),
            ffi::FMOD_DSP_PAN_2D_EXTENT => Ok(DspPan::_2DExtent),
            ffi::FMOD_DSP_PAN_2D_ROTATION => Ok(DspPan::_2DRotation),
            ffi::FMOD_DSP_PAN_2D_LFE_LEVEL => Ok(DspPan::_2DLfeLevel),
            ffi::FMOD_DSP_PAN_2D_STEREO_MODE => Ok(DspPan::_2DStereoMode),
            ffi::FMOD_DSP_PAN_2D_STEREO_SEPARATION => Ok(DspPan::_2DStereoSeparation),
            ffi::FMOD_DSP_PAN_2D_STEREO_AXIS => Ok(DspPan::_2DStereoAxis),
            ffi::FMOD_DSP_PAN_ENABLED_SPEAKERS => Ok(DspPan::EnabledSpeakers),
            ffi::FMOD_DSP_PAN_3D_POSITION => Ok(DspPan::_3DPosition),
            ffi::FMOD_DSP_PAN_3D_ROLLOFF => Ok(DspPan::_3DRolloff),
            ffi::FMOD_DSP_PAN_3D_MIN_DISTANCE => Ok(DspPan::_3DMinDistance),
            ffi::FMOD_DSP_PAN_3D_MAX_DISTANCE => Ok(DspPan::_3DMaxDistance),
            ffi::FMOD_DSP_PAN_3D_EXTENT_MODE => Ok(DspPan::_3DExtentMode),
            ffi::FMOD_DSP_PAN_3D_SOUND_SIZE => Ok(DspPan::_3DSoundSize),
            ffi::FMOD_DSP_PAN_3D_MIN_EXTENT => Ok(DspPan::_3DMinExtent),
            ffi::FMOD_DSP_PAN_3D_PAN_BLEND => Ok(DspPan::_3DPanBlend),
            ffi::FMOD_DSP_PAN_LFE_UPMIX_ENABLED => Ok(DspPan::LfeUpmixEnabled),
            ffi::FMOD_DSP_PAN_OVERALL_GAIN => Ok(DspPan::OverallGain),
            ffi::FMOD_DSP_PAN_SURROUND_SPEAKER_MODE => Ok(DspPan::SurroundSpeakerMode),
            ffi::FMOD_DSP_PAN_2D_HEIGHT_BLEND => Ok(DspPan::_2DHeightBlend),
            ffi::FMOD_DSP_PAN_ATTENUATION_RANGE => Ok(DspPan::AttenuationRange),
            ffi::FMOD_DSP_PAN_OVERRIDE_RANGE => Ok(DspPan::OverrideRange),
            _ => Err(err_enum!("FMOD_DSP_PAN", value)),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DspThreeEqCrossoverSlopeType {
    _12Db,
    _24Db,
    _48Db,
}
impl From<DspThreeEqCrossoverSlopeType> for ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_TYPE {
    fn from(value: DspThreeEqCrossoverSlopeType) -> ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_TYPE {
        match value {
            DspThreeEqCrossoverSlopeType::_12Db => ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_12DB,
            DspThreeEqCrossoverSlopeType::_24Db => ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_24DB,
            DspThreeEqCrossoverSlopeType::_48Db => ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_48DB,
        }
    }
}
impl DspThreeEqCrossoverSlopeType {
    pub fn from(
        value: ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_TYPE,
    ) -> Result<DspThreeEqCrossoverSlopeType, Error> {
        match value {
            ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_12DB => Ok(DspThreeEqCrossoverSlopeType::_12Db),
            ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_24DB => Ok(DspThreeEqCrossoverSlopeType::_24Db),
            ffi::FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_48DB => Ok(DspThreeEqCrossoverSlopeType::_48Db),
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
    _3DPosition,
    _3DRolloff,
    _3DMinDistance,
    _3DMaxDistance,
    _3DExtentMode,
    _3DSoundSize,
    _3DMinExtent,
    OverallGain,
    OutputGain,
    AttenuationRange,
    OverrideRange,
}
impl From<DspObjectPan> for ffi::FMOD_DSP_OBJECTPAN {
    fn from(value: DspObjectPan) -> ffi::FMOD_DSP_OBJECTPAN {
        match value {
            DspObjectPan::_3DPosition => ffi::FMOD_DSP_OBJECTPAN_3D_POSITION,
            DspObjectPan::_3DRolloff => ffi::FMOD_DSP_OBJECTPAN_3D_ROLLOFF,
            DspObjectPan::_3DMinDistance => ffi::FMOD_DSP_OBJECTPAN_3D_MIN_DISTANCE,
            DspObjectPan::_3DMaxDistance => ffi::FMOD_DSP_OBJECTPAN_3D_MAX_DISTANCE,
            DspObjectPan::_3DExtentMode => ffi::FMOD_DSP_OBJECTPAN_3D_EXTENT_MODE,
            DspObjectPan::_3DSoundSize => ffi::FMOD_DSP_OBJECTPAN_3D_SOUND_SIZE,
            DspObjectPan::_3DMinExtent => ffi::FMOD_DSP_OBJECTPAN_3D_MIN_EXTENT,
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
            ffi::FMOD_DSP_OBJECTPAN_3D_POSITION => Ok(DspObjectPan::_3DPosition),
            ffi::FMOD_DSP_OBJECTPAN_3D_ROLLOFF => Ok(DspObjectPan::_3DRolloff),
            ffi::FMOD_DSP_OBJECTPAN_3D_MIN_DISTANCE => Ok(DspObjectPan::_3DMinDistance),
            ffi::FMOD_DSP_OBJECTPAN_3D_MAX_DISTANCE => Ok(DspObjectPan::_3DMaxDistance),
            ffi::FMOD_DSP_OBJECTPAN_3D_EXTENT_MODE => Ok(DspObjectPan::_3DExtentMode),
            ffi::FMOD_DSP_OBJECTPAN_3D_SOUND_SIZE => Ok(DspObjectPan::_3DSoundSize),
            ffi::FMOD_DSP_OBJECTPAN_3D_MIN_EXTENT => Ok(DspObjectPan::_3DMinExtent),
            ffi::FMOD_DSP_OBJECTPAN_OVERALL_GAIN => Ok(DspObjectPan::OverallGain),
            ffi::FMOD_DSP_OBJECTPAN_OUTPUTGAIN => Ok(DspObjectPan::OutputGain),
            ffi::FMOD_DSP_OBJECTPAN_ATTENUATION_RANGE => Ok(DspObjectPan::AttenuationRange),
            ffi::FMOD_DSP_OBJECTPAN_OVERRIDE_RANGE => Ok(DspObjectPan::OverrideRange),
            _ => Err(err_enum!("FMOD_DSP_OBJECTPAN", value)),
        }
    }
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
            Err(err_fmod!("FMOD_Studio_System_Create", result))
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
            Err(err_fmod!("FMOD_System_Create", result))
        }
    }
}
