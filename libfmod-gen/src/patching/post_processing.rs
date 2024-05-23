use crate::models::{Api, Modifier, OpaqueType};

impl Api {
    pub fn apply_postprocessing(&mut self) {
        if !self
            .opaque_types
            .iter()
            .any(|opaque_type| opaque_type.name == "FMOD_STUDIO_SYSTEM")
        {
            self.opaque_types.push(OpaqueType {
                name: "FMOD_STUDIO_SYSTEM".into(),
            });
        }
        let not_specified_output = &[
            "FMOD_Studio_CommandReplay_GetSystem+system",
            "FMOD_Studio_CommandReplay_GetCommandString+buffer",
            "FMOD_Studio_CommandReplay_GetPaused+paused",
            "FMOD_Studio_CommandReplay_GetUserData+userdata",
            "FMOD_Studio_EventDescription_Is3D+is3D",
            "FMOD_Studio_System_GetCoreSystem+coresystem",
            "FMOD_System_GetNumNestedPlugins+count",
        ];
        for key in not_specified_output {
            self.modifiers.insert(key.to_string(), Modifier::Out);
        }
        let not_output = &[
            "FMOD_System_Set3DNumListeners+numlisteners",
            "FMOD_Channel_GetMixMatrix+inchannel_hop",
            "FMOD_ChannelGroup_GetMixMatrix+inchannel_hop",
        ];
        for key in not_output {
            self.modifiers.remove(&key.to_string());
        }
    }
}
