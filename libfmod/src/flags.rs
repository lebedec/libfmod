use crate::ffi;
use bitflags::bitflags;

bitflags! {

    pub struct StudioInit: ffi::FMOD_STUDIO_INITFLAGS {
        const NORMAL = ffi::FMOD_STUDIO_INIT_NORMAL;
        const LIVEUPDATE = ffi::FMOD_STUDIO_INIT_LIVEUPDATE;
        const ALLOW_MISSING_PLUGINS = ffi::FMOD_STUDIO_INIT_ALLOW_MISSING_PLUGINS;
        const SYNCHRONOUS_UPDATE = ffi::FMOD_STUDIO_INIT_SYNCHRONOUS_UPDATE;
        const DEFERRED_CALLBACKS = ffi::FMOD_STUDIO_INIT_DEFERRED_CALLBACKS;
        const LOAD_FROM_UPDATE = ffi::FMOD_STUDIO_INIT_LOAD_FROM_UPDATE;
        const MEMORY_TRACKING = ffi::FMOD_STUDIO_INIT_MEMORY_TRACKING;
    }

    pub struct Parameter: ffi::FMOD_STUDIO_PARAMETER_FLAGS {
        const READONLY = ffi::FMOD_STUDIO_PARAMETER_READONLY;
        const AUTOMATIC = ffi::FMOD_STUDIO_PARAMETER_AUTOMATIC;
        const GLOBAL = ffi::FMOD_STUDIO_PARAMETER_GLOBAL;
        const DISCRETE = ffi::FMOD_STUDIO_PARAMETER_DISCRETE;
        const LABELED = ffi::FMOD_STUDIO_PARAMETER_LABELED;
    }

    pub struct StudioSystemCallback: ffi::FMOD_STUDIO_SYSTEM_CALLBACK_TYPE {
        const PREUPDATE = ffi::FMOD_STUDIO_SYSTEM_CALLBACK_PREUPDATE;
        const POSTUPDATE = ffi::FMOD_STUDIO_SYSTEM_CALLBACK_POSTUPDATE;
        const BANK_UNLOAD = ffi::FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD;
        const LIVEUPDATE_CONNECTED = ffi::FMOD_STUDIO_SYSTEM_CALLBACK_LIVEUPDATE_CONNECTED;
        const LIVEUPDATE_DISCONNECTED = ffi::FMOD_STUDIO_SYSTEM_CALLBACK_LIVEUPDATE_DISCONNECTED;
        const ALL = ffi::FMOD_STUDIO_SYSTEM_CALLBACK_ALL;
    }

    pub struct EventCallback: ffi::FMOD_STUDIO_EVENT_CALLBACK_TYPE {
        const CREATED = ffi::FMOD_STUDIO_EVENT_CALLBACK_CREATED;
        const DESTROYED = ffi::FMOD_STUDIO_EVENT_CALLBACK_DESTROYED;
        const STARTING = ffi::FMOD_STUDIO_EVENT_CALLBACK_STARTING;
        const STARTED = ffi::FMOD_STUDIO_EVENT_CALLBACK_STARTED;
        const RESTARTED = ffi::FMOD_STUDIO_EVENT_CALLBACK_RESTARTED;
        const STOPPED = ffi::FMOD_STUDIO_EVENT_CALLBACK_STOPPED;
        const START_FAILED = ffi::FMOD_STUDIO_EVENT_CALLBACK_START_FAILED;
        const CREATE_PROGRAMMER_SOUND = ffi::FMOD_STUDIO_EVENT_CALLBACK_CREATE_PROGRAMMER_SOUND;
        const DESTROY_PROGRAMMER_SOUND = ffi::FMOD_STUDIO_EVENT_CALLBACK_DESTROY_PROGRAMMER_SOUND;
        const PLUGIN_CREATED = ffi::FMOD_STUDIO_EVENT_CALLBACK_PLUGIN_CREATED;
        const PLUGIN_DESTROYED = ffi::FMOD_STUDIO_EVENT_CALLBACK_PLUGIN_DESTROYED;
        const TIMELINE_MARKER = ffi::FMOD_STUDIO_EVENT_CALLBACK_TIMELINE_MARKER;
        const TIMELINE_BEAT = ffi::FMOD_STUDIO_EVENT_CALLBACK_TIMELINE_BEAT;
        const SOUND_PLAYED = ffi::FMOD_STUDIO_EVENT_CALLBACK_SOUND_PLAYED;
        const SOUND_STOPPED = ffi::FMOD_STUDIO_EVENT_CALLBACK_SOUND_STOPPED;
        const REAL_TO_VIRTUAL = ffi::FMOD_STUDIO_EVENT_CALLBACK_REAL_TO_VIRTUAL;
        const VIRTUAL_TO_REAL = ffi::FMOD_STUDIO_EVENT_CALLBACK_VIRTUAL_TO_REAL;
        const START_EVENT_COMMAND = ffi::FMOD_STUDIO_EVENT_CALLBACK_START_EVENT_COMMAND;
        const NESTED_TIMELINE_BEAT = ffi::FMOD_STUDIO_EVENT_CALLBACK_NESTED_TIMELINE_BEAT;
        const ALL = ffi::FMOD_STUDIO_EVENT_CALLBACK_ALL;
    }

    pub struct LoadBank: ffi::FMOD_STUDIO_LOAD_BANK_FLAGS {
        const NORMAL = ffi::FMOD_STUDIO_LOAD_BANK_NORMAL;
        const NONBLOCKING = ffi::FMOD_STUDIO_LOAD_BANK_NONBLOCKING;
        const DECOMPRESS_SAMPLES = ffi::FMOD_STUDIO_LOAD_BANK_DECOMPRESS_SAMPLES;
        const UNENCRYPTED = ffi::FMOD_STUDIO_LOAD_BANK_UNENCRYPTED;
    }

    pub struct CommandCapture: ffi::FMOD_STUDIO_COMMANDCAPTURE_FLAGS {
        const NORMAL = ffi::FMOD_STUDIO_COMMANDCAPTURE_NORMAL;
        const FILEFLUSH = ffi::FMOD_STUDIO_COMMANDCAPTURE_FILEFLUSH;
        const SKIP_INITIAL_STATE = ffi::FMOD_STUDIO_COMMANDCAPTURE_SKIP_INITIAL_STATE;
    }

    pub struct CommandReplay: ffi::FMOD_STUDIO_COMMANDREPLAY_FLAGS {
        const NORMAL = ffi::FMOD_STUDIO_COMMANDREPLAY_NORMAL;
        const SKIP_CLEANUP = ffi::FMOD_STUDIO_COMMANDREPLAY_SKIP_CLEANUP;
        const FAST_FORWARD = ffi::FMOD_STUDIO_COMMANDREPLAY_FAST_FORWARD;
        const SKIP_BANK_LOAD = ffi::FMOD_STUDIO_COMMANDREPLAY_SKIP_BANK_LOAD;
    }

    pub struct Debug: ffi::FMOD_DEBUG_FLAGS {
        const LEVEL_NONE = ffi::FMOD_DEBUG_LEVEL_NONE;
        const LEVEL_ERROR = ffi::FMOD_DEBUG_LEVEL_ERROR;
        const LEVEL_WARNING = ffi::FMOD_DEBUG_LEVEL_WARNING;
        const LEVEL_LOG = ffi::FMOD_DEBUG_LEVEL_LOG;
        const TYPE_MEMORY = ffi::FMOD_DEBUG_TYPE_MEMORY;
        const TYPE_FILE = ffi::FMOD_DEBUG_TYPE_FILE;
        const TYPE_CODEC = ffi::FMOD_DEBUG_TYPE_CODEC;
        const TYPE_TRACE = ffi::FMOD_DEBUG_TYPE_TRACE;
        const DISPLAY_TIMESTAMPS = ffi::FMOD_DEBUG_DISPLAY_TIMESTAMPS;
        const DISPLAY_LINENUMBERS = ffi::FMOD_DEBUG_DISPLAY_LINENUMBERS;
        const DISPLAY_THREAD = ffi::FMOD_DEBUG_DISPLAY_THREAD;
    }

    pub struct Memory: ffi::FMOD_MEMORY_TYPE {
        const NORMAL = ffi::FMOD_MEMORY_NORMAL;
        const STREAM_FILE = ffi::FMOD_MEMORY_STREAM_FILE;
        const STREAM_DECODE = ffi::FMOD_MEMORY_STREAM_DECODE;
        const SAMPLEDATA = ffi::FMOD_MEMORY_SAMPLEDATA;
        const DSP_BUFFER = ffi::FMOD_MEMORY_DSP_BUFFER;
        const PLUGIN = ffi::FMOD_MEMORY_PLUGIN;
        const PERSISTENT = ffi::FMOD_MEMORY_PERSISTENT;
        const ALL = ffi::FMOD_MEMORY_ALL;
    }

    pub struct Init: ffi::FMOD_INITFLAGS {
        const NORMAL = ffi::FMOD_INIT_NORMAL;
        const STREAM_FROM_UPDATE = ffi::FMOD_INIT_STREAM_FROM_UPDATE;
        const MIX_FROM_UPDATE = ffi::FMOD_INIT_MIX_FROM_UPDATE;
        const RIGHTHANDED_3D = ffi::FMOD_INIT_3D_RIGHTHANDED;
        const CLIP_OUTPUT = ffi::FMOD_INIT_CLIP_OUTPUT;
        const CHANNEL_LOWPASS = ffi::FMOD_INIT_CHANNEL_LOWPASS;
        const CHANNEL_DISTANCEFILTER = ffi::FMOD_INIT_CHANNEL_DISTANCEFILTER;
        const PROFILE_ENABLE = ffi::FMOD_INIT_PROFILE_ENABLE;
        const VOL0_BECOMES_VIRTUAL = ffi::FMOD_INIT_VOL0_BECOMES_VIRTUAL;
        const GEOMETRY_USECLOSEST = ffi::FMOD_INIT_GEOMETRY_USECLOSEST;
        const PREFER_DOLBY_DOWNMIX = ffi::FMOD_INIT_PREFER_DOLBY_DOWNMIX;
        const THREAD_UNSAFE = ffi::FMOD_INIT_THREAD_UNSAFE;
        const PROFILE_METER_ALL = ffi::FMOD_INIT_PROFILE_METER_ALL;
        const MEMORY_TRACKING = ffi::FMOD_INIT_MEMORY_TRACKING;
    }

    pub struct DriverState: ffi::FMOD_DRIVER_STATE {
        const CONNECTED = ffi::FMOD_DRIVER_STATE_CONNECTED;
        const DEFAULT = ffi::FMOD_DRIVER_STATE_DEFAULT;
    }

    pub struct TimeUnit: ffi::FMOD_TIMEUNIT {
        const MS = ffi::FMOD_TIMEUNIT_MS;
        const PCM = ffi::FMOD_TIMEUNIT_PCM;
        const PCMBYTES = ffi::FMOD_TIMEUNIT_PCMBYTES;
        const RAWBYTES = ffi::FMOD_TIMEUNIT_RAWBYTES;
        const PCMFRACTION = ffi::FMOD_TIMEUNIT_PCMFRACTION;
        const MODORDER = ffi::FMOD_TIMEUNIT_MODORDER;
        const MODROW = ffi::FMOD_TIMEUNIT_MODROW;
        const MODPATTERN = ffi::FMOD_TIMEUNIT_MODPATTERN;
    }

    pub struct SystemCallback: ffi::FMOD_SYSTEM_CALLBACK_TYPE {
        const DEVICELISTCHANGED = ffi::FMOD_SYSTEM_CALLBACK_DEVICELISTCHANGED;
        const DEVICELOST = ffi::FMOD_SYSTEM_CALLBACK_DEVICELOST;
        const MEMORYALLOCATIONFAILED = ffi::FMOD_SYSTEM_CALLBACK_MEMORYALLOCATIONFAILED;
        const THREADCREATED = ffi::FMOD_SYSTEM_CALLBACK_THREADCREATED;
        const BADDSPCONNECTION = ffi::FMOD_SYSTEM_CALLBACK_BADDSPCONNECTION;
        const PREMIX = ffi::FMOD_SYSTEM_CALLBACK_PREMIX;
        const POSTMIX = ffi::FMOD_SYSTEM_CALLBACK_POSTMIX;
        const ERROR = ffi::FMOD_SYSTEM_CALLBACK_ERROR;
        const MIDMIX = ffi::FMOD_SYSTEM_CALLBACK_MIDMIX;
        const THREADDESTROYED = ffi::FMOD_SYSTEM_CALLBACK_THREADDESTROYED;
        const PREUPDATE = ffi::FMOD_SYSTEM_CALLBACK_PREUPDATE;
        const POSTUPDATE = ffi::FMOD_SYSTEM_CALLBACK_POSTUPDATE;
        const RECORDLISTCHANGED = ffi::FMOD_SYSTEM_CALLBACK_RECORDLISTCHANGED;
        const BUFFEREDNOMIX = ffi::FMOD_SYSTEM_CALLBACK_BUFFEREDNOMIX;
        const DEVICEREINITIALIZE = ffi::FMOD_SYSTEM_CALLBACK_DEVICEREINITIALIZE;
        const OUTPUTUNDERRUN = ffi::FMOD_SYSTEM_CALLBACK_OUTPUTUNDERRUN;
        const RECORDPOSITIONCHANGED = ffi::FMOD_SYSTEM_CALLBACK_RECORDPOSITIONCHANGED;
        const ALL = ffi::FMOD_SYSTEM_CALLBACK_ALL;
    }

    pub struct Mode: ffi::FMOD_MODE {
        const DEFAULT = ffi::FMOD_DEFAULT;
        const LOOP_OFF = ffi::FMOD_LOOP_OFF;
        const LOOP_NORMAL = ffi::FMOD_LOOP_NORMAL;
        const LOOP_BIDI = ffi::FMOD_LOOP_BIDI;
        const FMOD_2D = ffi::FMOD_2D;
        const FMOD_3D = ffi::FMOD_3D;
        const CREATESTREAM = ffi::FMOD_CREATESTREAM;
        const CREATESAMPLE = ffi::FMOD_CREATESAMPLE;
        const CREATECOMPRESSEDSAMPLE = ffi::FMOD_CREATECOMPRESSEDSAMPLE;
        const OPENUSER = ffi::FMOD_OPENUSER;
        const OPENMEMORY = ffi::FMOD_OPENMEMORY;
        const OPENMEMORY_POINT = ffi::FMOD_OPENMEMORY_POINT;
        const OPENRAW = ffi::FMOD_OPENRAW;
        const OPENONLY = ffi::FMOD_OPENONLY;
        const ACCURATETIME = ffi::FMOD_ACCURATETIME;
        const MPEGSEARCH = ffi::FMOD_MPEGSEARCH;
        const NONBLOCKING = ffi::FMOD_NONBLOCKING;
        const UNIQUE = ffi::FMOD_UNIQUE;
        const HEADRELATIVE_3D = ffi::FMOD_3D_HEADRELATIVE;
        const WORLDRELATIVE_3D = ffi::FMOD_3D_WORLDRELATIVE;
        const INVERSEROLLOFF_3D = ffi::FMOD_3D_INVERSEROLLOFF;
        const LINEARROLLOFF_3D = ffi::FMOD_3D_LINEARROLLOFF;
        const LINEARSQUAREROLLOFF_3D = ffi::FMOD_3D_LINEARSQUAREROLLOFF;
        const INVERSETAPEREDROLLOFF_3D = ffi::FMOD_3D_INVERSETAPEREDROLLOFF;
        const CUSTOMROLLOFF_3D = ffi::FMOD_3D_CUSTOMROLLOFF;
        const IGNOREGEOMETRY_3D = ffi::FMOD_3D_IGNOREGEOMETRY;
        const IGNORETAGS = ffi::FMOD_IGNORETAGS;
        const LOWMEM = ffi::FMOD_LOWMEM;
        const VIRTUAL_PLAYFROMSTART = ffi::FMOD_VIRTUAL_PLAYFROMSTART;
    }

    pub struct ChannelMask: ffi::FMOD_CHANNELMASK {
        const FRONT_LEFT = ffi::FMOD_CHANNELMASK_FRONT_LEFT;
        const FRONT_RIGHT = ffi::FMOD_CHANNELMASK_FRONT_RIGHT;
        const FRONT_CENTER = ffi::FMOD_CHANNELMASK_FRONT_CENTER;
        const LOW_FREQUENCY = ffi::FMOD_CHANNELMASK_LOW_FREQUENCY;
        const SURROUND_LEFT = ffi::FMOD_CHANNELMASK_SURROUND_LEFT;
        const SURROUND_RIGHT = ffi::FMOD_CHANNELMASK_SURROUND_RIGHT;
        const BACK_LEFT = ffi::FMOD_CHANNELMASK_BACK_LEFT;
        const BACK_RIGHT = ffi::FMOD_CHANNELMASK_BACK_RIGHT;
        const BACK_CENTER = ffi::FMOD_CHANNELMASK_BACK_CENTER;
        const MONO = ffi::FMOD_CHANNELMASK_MONO;
        const STEREO = ffi::FMOD_CHANNELMASK_STEREO;
        const LRC = ffi::FMOD_CHANNELMASK_LRC;
        const QUAD = ffi::FMOD_CHANNELMASK_QUAD;
        const SURROUND = ffi::FMOD_CHANNELMASK_SURROUND;
        const MASK_5POINT1 = ffi::FMOD_CHANNELMASK_5POINT1;
        const MASK_5POINT1_REARS = ffi::FMOD_CHANNELMASK_5POINT1_REARS;
        const MASK_7POINT0 = ffi::FMOD_CHANNELMASK_7POINT0;
        const MASK_7POINT1 = ffi::FMOD_CHANNELMASK_7POINT1;
    }

    pub struct PortIndex: ffi::FMOD_PORT_INDEX {
        const NONE = ffi::FMOD_PORT_INDEX_NONE;
        const FLAG_VR_CONTROLLER = ffi::FMOD_PORT_INDEX_FLAG_VR_CONTROLLER;
    }

    pub struct ThreadPriority: ffi::FMOD_THREAD_PRIORITY {
        const PLATFORM_MIN = ffi::FMOD_THREAD_PRIORITY_PLATFORM_MIN;
        const PLATFORM_MAX = ffi::FMOD_THREAD_PRIORITY_PLATFORM_MAX;
        const DEFAULT = ffi::FMOD_THREAD_PRIORITY_DEFAULT;
        const LOW = ffi::FMOD_THREAD_PRIORITY_LOW;
        const MEDIUM = ffi::FMOD_THREAD_PRIORITY_MEDIUM;
        const HIGH = ffi::FMOD_THREAD_PRIORITY_HIGH;
        const VERY_HIGH = ffi::FMOD_THREAD_PRIORITY_VERY_HIGH;
        const EXTREME = ffi::FMOD_THREAD_PRIORITY_EXTREME;
        const CRITICAL = ffi::FMOD_THREAD_PRIORITY_CRITICAL;
        const MIXER = ffi::FMOD_THREAD_PRIORITY_MIXER;
        const FEEDER = ffi::FMOD_THREAD_PRIORITY_FEEDER;
        const STREAM = ffi::FMOD_THREAD_PRIORITY_STREAM;
        const FILE = ffi::FMOD_THREAD_PRIORITY_FILE;
        const NONBLOCKING = ffi::FMOD_THREAD_PRIORITY_NONBLOCKING;
        const RECORD = ffi::FMOD_THREAD_PRIORITY_RECORD;
        const GEOMETRY = ffi::FMOD_THREAD_PRIORITY_GEOMETRY;
        const PROFILER = ffi::FMOD_THREAD_PRIORITY_PROFILER;
        const STUDIO_UPDATE = ffi::FMOD_THREAD_PRIORITY_STUDIO_UPDATE;
        const STUDIO_LOAD_BANK = ffi::FMOD_THREAD_PRIORITY_STUDIO_LOAD_BANK;
        const STUDIO_LOAD_SAMPLE = ffi::FMOD_THREAD_PRIORITY_STUDIO_LOAD_SAMPLE;
        const CONVOLUTION1 = ffi::FMOD_THREAD_PRIORITY_CONVOLUTION1;
        const CONVOLUTION2 = ffi::FMOD_THREAD_PRIORITY_CONVOLUTION2;
    }

    pub struct ThreadStackSize: ffi::FMOD_THREAD_STACK_SIZE {
        const DEFAULT = ffi::FMOD_THREAD_STACK_SIZE_DEFAULT;
        const MIXER = ffi::FMOD_THREAD_STACK_SIZE_MIXER;
        const FEEDER = ffi::FMOD_THREAD_STACK_SIZE_FEEDER;
        const STREAM = ffi::FMOD_THREAD_STACK_SIZE_STREAM;
        const FILE = ffi::FMOD_THREAD_STACK_SIZE_FILE;
        const NONBLOCKING = ffi::FMOD_THREAD_STACK_SIZE_NONBLOCKING;
        const RECORD = ffi::FMOD_THREAD_STACK_SIZE_RECORD;
        const GEOMETRY = ffi::FMOD_THREAD_STACK_SIZE_GEOMETRY;
        const PROFILER = ffi::FMOD_THREAD_STACK_SIZE_PROFILER;
        const STUDIO_UPDATE = ffi::FMOD_THREAD_STACK_SIZE_STUDIO_UPDATE;
        const STUDIO_LOAD_BANK = ffi::FMOD_THREAD_STACK_SIZE_STUDIO_LOAD_BANK;
        const STUDIO_LOAD_SAMPLE = ffi::FMOD_THREAD_STACK_SIZE_STUDIO_LOAD_SAMPLE;
        const CONVOLUTION1 = ffi::FMOD_THREAD_STACK_SIZE_CONVOLUTION1;
        const CONVOLUTION2 = ffi::FMOD_THREAD_STACK_SIZE_CONVOLUTION2;
    }

    pub struct ThreadAffinity: ffi::FMOD_THREAD_AFFINITY {
        const GROUP_DEFAULT = ffi::FMOD_THREAD_AFFINITY_GROUP_DEFAULT;
        const GROUP_A = ffi::FMOD_THREAD_AFFINITY_GROUP_A;
        const GROUP_B = ffi::FMOD_THREAD_AFFINITY_GROUP_B;
        const GROUP_C = ffi::FMOD_THREAD_AFFINITY_GROUP_C;
        const MIXER = ffi::FMOD_THREAD_AFFINITY_MIXER;
        const FEEDER = ffi::FMOD_THREAD_AFFINITY_FEEDER;
        const STREAM = ffi::FMOD_THREAD_AFFINITY_STREAM;
        const FILE = ffi::FMOD_THREAD_AFFINITY_FILE;
        const NONBLOCKING = ffi::FMOD_THREAD_AFFINITY_NONBLOCKING;
        const RECORD = ffi::FMOD_THREAD_AFFINITY_RECORD;
        const GEOMETRY = ffi::FMOD_THREAD_AFFINITY_GEOMETRY;
        const PROFILER = ffi::FMOD_THREAD_AFFINITY_PROFILER;
        const STUDIO_UPDATE = ffi::FMOD_THREAD_AFFINITY_STUDIO_UPDATE;
        const STUDIO_LOAD_BANK = ffi::FMOD_THREAD_AFFINITY_STUDIO_LOAD_BANK;
        const STUDIO_LOAD_SAMPLE = ffi::FMOD_THREAD_AFFINITY_STUDIO_LOAD_SAMPLE;
        const CONVOLUTION1 = ffi::FMOD_THREAD_AFFINITY_CONVOLUTION1;
        const CONVOLUTION2 = ffi::FMOD_THREAD_AFFINITY_CONVOLUTION2;
        const CORE_ALL = ffi::FMOD_THREAD_AFFINITY_CORE_ALL;
        const CORE_0 = ffi::FMOD_THREAD_AFFINITY_CORE_0;
        const CORE_1 = ffi::FMOD_THREAD_AFFINITY_CORE_1;
        const CORE_2 = ffi::FMOD_THREAD_AFFINITY_CORE_2;
        const CORE_3 = ffi::FMOD_THREAD_AFFINITY_CORE_3;
        const CORE_4 = ffi::FMOD_THREAD_AFFINITY_CORE_4;
        const CORE_5 = ffi::FMOD_THREAD_AFFINITY_CORE_5;
        const CORE_6 = ffi::FMOD_THREAD_AFFINITY_CORE_6;
        const CORE_7 = ffi::FMOD_THREAD_AFFINITY_CORE_7;
        const CORE_8 = ffi::FMOD_THREAD_AFFINITY_CORE_8;
        const CORE_9 = ffi::FMOD_THREAD_AFFINITY_CORE_9;
        const CORE_10 = ffi::FMOD_THREAD_AFFINITY_CORE_10;
        const CORE_11 = ffi::FMOD_THREAD_AFFINITY_CORE_11;
        const CORE_12 = ffi::FMOD_THREAD_AFFINITY_CORE_12;
        const CORE_13 = ffi::FMOD_THREAD_AFFINITY_CORE_13;
        const CORE_14 = ffi::FMOD_THREAD_AFFINITY_CORE_14;
        const CORE_15 = ffi::FMOD_THREAD_AFFINITY_CORE_15;
    }

    pub struct CodecSeekMethod: ffi::FMOD_CODEC_SEEK_METHOD {
        const SET = ffi::FMOD_CODEC_SEEK_METHOD_SET;
        const CURRENT = ffi::FMOD_CODEC_SEEK_METHOD_CURRENT;
        const END = ffi::FMOD_CODEC_SEEK_METHOD_END;
    }

    pub struct OutputMethodMix: ffi::FMOD_OUTPUT_METHOD {
        const DIRECT = ffi::FMOD_OUTPUT_METHOD_MIX_DIRECT;
        const BUFFERED = ffi::FMOD_OUTPUT_METHOD_MIX_BUFFERED;
    }

}

impl Into<ffi::FMOD_STUDIO_INITFLAGS> for StudioInit {
    fn into(self) -> ffi::FMOD_STUDIO_INITFLAGS {
        self.bits
    }
}

impl Into<ffi::FMOD_STUDIO_PARAMETER_FLAGS> for Parameter {
    fn into(self) -> ffi::FMOD_STUDIO_PARAMETER_FLAGS {
        self.bits
    }
}

impl Into<ffi::FMOD_STUDIO_SYSTEM_CALLBACK_TYPE> for StudioSystemCallback {
    fn into(self) -> ffi::FMOD_STUDIO_SYSTEM_CALLBACK_TYPE {
        self.bits
    }
}

impl Into<ffi::FMOD_STUDIO_EVENT_CALLBACK_TYPE> for EventCallback {
    fn into(self) -> ffi::FMOD_STUDIO_EVENT_CALLBACK_TYPE {
        self.bits
    }
}

impl Into<ffi::FMOD_STUDIO_LOAD_BANK_FLAGS> for LoadBank {
    fn into(self) -> ffi::FMOD_STUDIO_LOAD_BANK_FLAGS {
        self.bits
    }
}

impl Into<ffi::FMOD_STUDIO_COMMANDCAPTURE_FLAGS> for CommandCapture {
    fn into(self) -> ffi::FMOD_STUDIO_COMMANDCAPTURE_FLAGS {
        self.bits
    }
}

impl Into<ffi::FMOD_STUDIO_COMMANDREPLAY_FLAGS> for CommandReplay {
    fn into(self) -> ffi::FMOD_STUDIO_COMMANDREPLAY_FLAGS {
        self.bits
    }
}

impl Into<ffi::FMOD_DEBUG_FLAGS> for Debug {
    fn into(self) -> ffi::FMOD_DEBUG_FLAGS {
        self.bits
    }
}

impl Into<ffi::FMOD_MEMORY_TYPE> for Memory {
    fn into(self) -> ffi::FMOD_MEMORY_TYPE {
        self.bits
    }
}

impl Into<ffi::FMOD_INITFLAGS> for Init {
    fn into(self) -> ffi::FMOD_INITFLAGS {
        self.bits
    }
}

impl Into<ffi::FMOD_DRIVER_STATE> for DriverState {
    fn into(self) -> ffi::FMOD_DRIVER_STATE {
        self.bits
    }
}

impl Into<ffi::FMOD_TIMEUNIT> for TimeUnit {
    fn into(self) -> ffi::FMOD_TIMEUNIT {
        self.bits
    }
}

impl Into<ffi::FMOD_SYSTEM_CALLBACK_TYPE> for SystemCallback {
    fn into(self) -> ffi::FMOD_SYSTEM_CALLBACK_TYPE {
        self.bits
    }
}

impl Into<ffi::FMOD_MODE> for Mode {
    fn into(self) -> ffi::FMOD_MODE {
        self.bits
    }
}

impl Into<ffi::FMOD_CHANNELMASK> for ChannelMask {
    fn into(self) -> ffi::FMOD_CHANNELMASK {
        self.bits
    }
}

impl Into<ffi::FMOD_PORT_INDEX> for PortIndex {
    fn into(self) -> ffi::FMOD_PORT_INDEX {
        self.bits
    }
}

impl Into<ffi::FMOD_THREAD_PRIORITY> for ThreadPriority {
    fn into(self) -> ffi::FMOD_THREAD_PRIORITY {
        self.bits
    }
}

impl Into<ffi::FMOD_THREAD_STACK_SIZE> for ThreadStackSize {
    fn into(self) -> ffi::FMOD_THREAD_STACK_SIZE {
        self.bits
    }
}

impl Into<ffi::FMOD_THREAD_AFFINITY> for ThreadAffinity {
    fn into(self) -> ffi::FMOD_THREAD_AFFINITY {
        self.bits
    }
}

impl Into<ffi::FMOD_CODEC_SEEK_METHOD> for CodecSeekMethod {
    fn into(self) -> ffi::FMOD_CODEC_SEEK_METHOD {
        self.bits
    }
}

impl Into<ffi::FMOD_OUTPUT_METHOD> for OutputMethodMix {
    fn into(self) -> ffi::FMOD_OUTPUT_METHOD {
        self.bits
    }
}
