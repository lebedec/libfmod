use crate::models::Api;
use convert_case::{Case, Casing};
use std::collections::HashMap;

lazy_static! {
    pub static ref RENAMES: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("Channelgroup", "ChannelGroup"),
            ("Dspconnection", "DspConnection"),
            ("Reverb3D", "Reverb3d"),
            ("Soundgroup", "SoundGroup"),
            ("Commandreplay", "CommandReplay"),
            ("Eventdescription", "EventDescription"),
            ("Eventinstance", "EventInstance"),
            ("Studiosystem", "Studio"),
            ("Allowfadeout", "AllowFadeout"),
            ("Instancetype", "InstanceType"),
            ("Eventdescription", "EventDescription"),
            ("Eventinstance", "EventInstance"),
            ("Parameterinstance", "ParameterInstance"),
            ("Commandreplay", "CommandReplay"),
            ("Commandcapture", "CommandCapture"),
            ("Channelmask", "ChannelMask"),
            ("Timeunit", "TimeUnit"),
            ("ChannelcontrolType", "ChannelControlType"),
            ("Channelgroup", "ChannelGroup"),
            ("Channelcontrol", "ChannelControl"),
            ("Soundgroup", "SoundGroup"),
            ("Outputtype", "OutputType"),
            ("Nosound", "NoSound"),
            ("Wavwriter", "WavWriter"),
            ("NosoundNrt", "NoSoundNrt"),
            ("WavwriterNrt", "WavWriterNrt"),
            ("Pulseaudio", "PulseAudio"),
            ("Coreaudio", "CoreAudio"),
            ("Audiotrack", "AudioTrack"),
            ("Opensl", "OpenSL"),
            ("Audioout", "AudioOut"),
            ("Webaudio", "WebAudio"),
            ("Nnaudio", "NnAudio"),
            ("Aaudio", "AAudio"),
            ("Speakermode", "SpeakerMode"),
            ("Channelorder", "ChannelOrder"),
            ("Waveformat", "WaveFormat"),
            ("Protools", "ProTools"),
            ("Allmono", "AllMono"),
            ("Allstereo", "AllStereo"),
            ("Plugintype", "PluginType"),
            ("Oggvorbis", "OggVorbis"),
            ("Audioqueue", "AudioQueue"),
            ("Pcmfloat", "PcmFloat"),
            ("Openstate", "OpenState"),
            ("Setposition", "SetPosition"),
            ("SoundgroupBehavior", "SoundGroupBehavior"),
            ("Steallowest", "StealLowest"),
            ("ChannelcontrolCallbackType", "ChannelControlCallbackType"),
            ("Virtualvoice", "VirtualVoice"),
            ("Syncpoint", "SyncPoint"),
            ("ChannelcontrolDspIndex", "ChannelControlDspIndex"),
            ("ErrorcallbackInstancetype", "ErrorCallbackInstancetype"),
            ("Dspconnection", "DspConnection"),
            ("StudioSystem", "Studio"),
            ("StudioEventdescription", "EventDescription"),
            ("StudioEventinstance", "EventInstance"),
            ("StudioParameterinstance", "ParameterInstance"),
            ("StudioBus", "Bus"),
            ("StudioVca", "Vca"),
            ("StudioBank", "Bank"),
            ("StudioCommandreplay", "CommandReplay"),
            ("Nointerp", "NoInterp"),
            ("DspconnectionType", "DspConnectionType"),
            ("Tagtype", "TagType"),
            ("Vorbiscomment", "VorbisComment"),
            ("Tagdatatype", "TagDataType"),
            ("DspLowpass", "DspLowPass"),
            ("DspHighpass", "DspHighPass"),
            ("Drylevel", "DryLevel"),
            ("Wetlevel", "WetLevel"),
            ("DspItlowpass", "DspItLowPass"),
            ("Overallgain", "OverallGain"),
            ("Fadetime", "FadeTime"),
            ("Maxamp", "MaxAmp"),
            ("Releasetime", "ReleaseTime"),
            ("Maximizergain", "MaximizerGain"),
            ("Lowshelf", "LowShelf"),
            ("Highshelf", "HighShelf"),
            ("Allpass", "AllPass"),
            ("DspPitchshift", "DspPitchShift"),
            ("Fftsize", "FftSize"),
            ("Maxchannels", "MaxChannels"),
            ("DspItecho", "DspItEcho"),
            ("Wetdrymix", "WetDryMix"),
            ("Leftdelay", "LeftDelay"),
            ("Rightdelay", "RightDelay"),
            ("Pandelay", "PanDelay"),
            ("Gainmakeup", "GainMakeup"),
            ("Usesidechain", "UseSidechain"),
            ("DspSfxreverb", "DspSfxReverb"),
            ("Decaytime", "DecayTime"),
            ("Earlydelay", "EarlyDelay"),
            ("Latedelay", "LateDelay"),
            ("Hfreference", "HfReference"),
            ("Hfdecayratio", "HfDecayRatio"),
            ("Lowshelffrequency", "LowShelfFrequency"),
            ("Lowshelfgain", "LowShelfGain"),
            ("Highcut", "HighCut"),
            ("Earlylatemix", "EarlyLateMix"),
            ("Wetlevel", "WetLevel"),
            ("Drylevel", "DryLevel"),
            ("DspLowpassSimple", "DspLowPassSimple"),
            ("Maxdelay", "MaxDelay"),
            ("Returnid", "ReturnId"),
            ("Linearsquared", "LinearSquared"),
            ("Inversetapered", "InverseTapered"),
            (
                "DspThreeEqCrossoverslopeType",
                "DspThreeEqCrossoverSlopeType",
            ),
            ("Lowgain", "LowGain"),
            ("Midgain", "MidGain"),
            ("Highgain", "HighGain"),
            ("Lowcrossover", "LowCrossover"),
            ("Highcrossover", "HightCorssover"),
            ("Crossoverslope", "CrossoverSlope"),
            ("Blackman", "BlackMan"),
            ("Blackmanharris", "BlackManHarris"),
            ("Windowsize", "WindowSize"),
            ("Windowtype", "WindowType"),
            ("Spectrumdata", "SpectrumData"),
            ("DspEnvelopefollower", "DspEnvelopeFollower"),
            ("Usesidechain", "UseSidechain"),
            ("DspChannelmixOutput", "DspChannelMixOutput"),
            ("Allquad", "AllQuad"),
            ("Alllfe", "AllLfe"),
            ("DspChannelmix", "DspChannelMix"),
            ("Outputgrouping", "OutputGrouping"),
            ("DspTransceiverSpeakermode", "DspTransceiverSpeakerMode"),
            ("Transmitspeakermode", "TransmitSpeakerMode"),
            ("DspObjectpan", "DspObjectPan"),
            ("Outputgain", "OutputGain"),
            ("3DAttributes", "Attributes3d"),
            ("3DAttributesMulti", "AttributesMulti3d"),
            ("Pluginlist", "PluginList"),
            ("Fmodresult", "FmodResult"),
            ("Studioadvancedsettings", "StudioAdvancedSettings"),
            ("Studiocpuusage", "StudioCpuUsage"),
            ("Asyncreadinfo", "AsyncReadInfo"),
            ("Advancedsettings", "AdvancedSettings"),
            ("Createsoundexinfo", "CreateSoundexInfo"),
            ("ErrorcallbackInfo", "ErrorCallbackInfo"),
            ("DspParameter3Dattributes", "DspParameterAttributes3d"),
            (
                "DspParameter3DattributesMulti",
                "DspParameterAttributes3dMulti",
            ),
            ("3DRolloffCallback", "RolloffCallback3d"),
            ("Channelpriority", "ChannelPriority"),
            ("Audioworklet", "AudioWorklet"),
            ("5Point1", "Mode5Point1"),
            ("7Point1", "Mode7Point1"),
            ("7Point1Point4", "Mode7Point1Point4"),
            ("12Db", "Slope12Db"),
            ("24Db", "Slope24Db"),
            ("48Db", "Slope48Db"),
        ])
    };
}

pub const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "try", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield",
];

pub const ENUMERATOR_RENAMES: &[(&str, &str)] = &[
    ("FMOD_STUDIO_LOAD_MEMORY", "FMOD_STUDIO_LOAD_MEMORY_MEMORY"),
    (
        "FMOD_STUDIO_LOAD_MEMORY_POINT",
        "FMOD_STUDIO_LOAD_MEMORY_MEMORY_POINT",
    ),
];

impl Api {
    pub fn patch_ident(ident: &str) -> String {
        if KEYWORDS.contains(&&*ident.to_lowercase()) {
            format!("{}_", ident)
        } else {
            ident.to_string()
        }
    }

    pub fn patch_enumerator(name: &str) -> String {
        match ENUMERATOR_RENAMES.iter().find(|pair| pair.0 == name) {
            None => name.to_string(),
            Some(pair) => pair.1.to_string(),
        }
    }

    pub fn patch_variant_name(key: &str) -> String {
        let key = if key.starts_with("3D") {
            format!("{}3d", &key[2..]).to_case(Case::UpperCamel)
        } else {
            key.to_string()
        };

        let key = if key.starts_with("2D") {
            format!("{}2d", &key[2..]).to_case(Case::UpperCamel)
        } else {
            key.to_string()
        };

        let name = key;
        let name = match RENAMES.get(&name[..]) {
            None => name,
            Some(rename) => rename.to_string(),
        };
        name
    }
}
