use crate::generators::ffi;
use crate::generators::lib::Signature;
use crate::models::{Argument, Function};
use crate::Api;

impl Signature {
    pub fn patch_function_signature(
        &mut self,
        owner: &str,
        function: &Function,
        argument: &Argument,
    ) -> bool {
        let pointer = ffi::describe_pointer(&argument.as_const, &argument.pointer);
        if self.arguments.is_empty()
            && argument.argument_type.is_user_type(owner)
            && pointer == "*mut"
        {
            self.arguments.push(quote! { &self });
            self.inputs.push(quote! { self.pointer });
            return true;
        }

        if function.name == "FMOD_Studio_System_Create" && argument.name == "headerversion" {
            self.inputs.push(quote! { ffi::FMOD_VERSION });
            return true;
        }

        if function.name == "FMOD_System_Create" && argument.name == "headerversion" {
            self.inputs.push(quote! { ffi::FMOD_VERSION });
            return true;
        }

        // FMOD_Sound_Set3DCustomRolloff
        if function.name == "FMOD_Sound_Set3DCustomRolloff" && argument.name == "numpoints" {
            self.targets
                .push(quote! { let numpoints = points.len() as i32; });
            self.inputs.push(quote! { numpoints });
            return true;
        }
        if function.name == "FMOD_Sound_Set3DCustomRolloff" && argument.name == "points" {
            self.arguments.push(quote! { points: Vec<Vector> });
            self.inputs
                .push(quote! { vec_as_mut_ptr(points, |point| point.into()) });
            return true;
        }
        if function.name == "FMOD_Sound_Get3DCustomRolloff" && argument.name == "numpoints" {
            self.targets
                .push(quote! { let mut numpoints = i32::default(); });
            self.inputs.push(quote! { &mut numpoints });
            return true;
        }
        if function.name == "FMOD_Sound_Get3DCustomRolloff" && argument.name == "points" {
            self.targets.push(quote! { let mut points = null_mut(); });
            self.inputs.push(quote! { &mut points });
            self.outputs
                .push(quote! { to_vec!(points, numpoints, Vector::try_from)? });
            self.return_types.push(quote! { Vec<Vector> });
            return true;
        }

        // FMOD_Channel_Set3DCustomRolloff
        if function.name == "FMOD_Channel_Set3DCustomRolloff" && argument.name == "numpoints" {
            self.targets
                .push(quote! { let numpoints = points.len() as i32; });
            self.inputs.push(quote! { numpoints });
            return true;
        }
        if function.name == "FMOD_Channel_Set3DCustomRolloff" && argument.name == "points" {
            self.arguments.push(quote! { points: Vec<Vector> });
            self.inputs
                .push(quote! { vec_as_mut_ptr(points, |point| point.into()) });
            return true;
        }
        if function.name == "FMOD_Channel_Get3DCustomRolloff" && argument.name == "numpoints" {
            self.targets
                .push(quote! { let mut numpoints = i32::default(); });
            self.inputs.push(quote! { &mut numpoints });
            return true;
        }
        if function.name == "FMOD_Channel_Get3DCustomRolloff" && argument.name == "points" {
            self.targets.push(quote! { let mut points = null_mut(); });
            self.inputs.push(quote! { &mut points });
            self.outputs
                .push(quote! { to_vec!(points, numpoints, Vector::try_from)? });
            self.return_types.push(quote! { Vec<Vector> });
            return true;
        }

        if function.name == "FMOD_ChannelGroup_Set3DCustomRolloff" && argument.name == "numpoints" {
            self.targets
                .push(quote! { let numpoints = points.len() as i32; });
            self.inputs.push(quote! { numpoints });
            return true;
        }
        if function.name == "FMOD_ChannelGroup_Set3DCustomRolloff" && argument.name == "points" {
            self.arguments.push(quote! { points: Vec<Vector> });
            self.inputs
                .push(quote! { vec_as_mut_ptr(points, |point| point.into()) });
            return true;
        }
        if function.name == "FMOD_ChannelGroup_Get3DCustomRolloff" && argument.name == "numpoints" {
            self.targets
                .push(quote! { let mut numpoints = i32::default(); });
            self.inputs.push(quote! { &mut numpoints });
            return true;
        }
        if function.name == "FMOD_ChannelGroup_Get3DCustomRolloff" && argument.name == "points" {
            self.targets.push(quote! { let mut points = null_mut(); });
            self.inputs.push(quote! { &mut points });
            self.outputs
                .push(quote! { to_vec!(points, numpoints, Vector::try_from)? });
            self.return_types.push(quote! { Vec<Vector> });
            return true;
        }

        if function.name == "FMOD_Studio_Bank_GetEventList" && argument.name == "count" {
            self.targets
                .push(quote! { let mut count = i32::default(); });
            self.inputs.push(quote! { &mut count });
            return true;
        }
        if function.name == "FMOD_Studio_Bank_GetEventList" && argument.name == "array" {
            self.targets
                .push(quote! { let mut array = vec![null_mut(); capacity as usize]; });
            self.inputs.push(quote! { array.as_mut_ptr() });
            self.outputs
                .push(quote! { array.into_iter().take(count as usize).map(EventDescription::from).collect() });
            self.return_types.push(quote! { Vec<EventDescription> });
            return true;
        }

        if function.name == "FMOD_Studio_Bank_GetBusList" && argument.name == "count" {
            self.targets
                .push(quote! { let mut count = i32::default(); });
            self.inputs.push(quote! { &mut count });
            return true;
        }
        if function.name == "FMOD_Studio_Bank_GetBusList" && argument.name == "array" {
            self.targets
                .push(quote! { let mut array = vec![null_mut(); capacity as usize]; });
            self.inputs.push(quote! { array.as_mut_ptr() });
            self.outputs
                .push(quote! { array.into_iter().take(count as usize).map(Bus::from).collect() });
            self.return_types.push(quote! { Vec<Bus> });
            return true;
        }

        if function.name == "FMOD_Studio_Bank_GetVCAList" && argument.name == "count" {
            self.targets
                .push(quote! { let mut count = i32::default(); });
            self.inputs.push(quote! { &mut count });
            return true;
        }
        if function.name == "FMOD_Studio_Bank_GetVCAList" && argument.name == "array" {
            self.targets
                .push(quote! { let mut array = vec![null_mut(); capacity as usize]; });
            self.inputs.push(quote! { array.as_mut_ptr() });
            self.outputs
                .push(quote! { array.into_iter().take(count as usize).map(Vca::from).collect() });
            self.return_types.push(quote! { Vec<Vca> });
            return true;
        }

        if function.name == "FMOD_Studio_EventDescription_GetInstanceList"
            && argument.name == "count"
        {
            self.targets
                .push(quote! { let mut count = i32::default(); });
            self.inputs.push(quote! { &mut count });
            return true;
        }
        if function.name == "FMOD_Studio_EventDescription_GetInstanceList"
            && argument.name == "array"
        {
            self.targets
                .push(quote! { let mut array = vec![null_mut(); capacity as usize]; });
            self.inputs.push(quote! { array.as_mut_ptr() });
            self.outputs.push(quote! { array.into_iter().take(count as usize).map(EventInstance::from).collect() });
            self.return_types.push(quote! { Vec<EventInstance> });
            return true;
        }

        if function.name == "FMOD_Studio_System_GetBankList" && argument.name == "count" {
            self.targets
                .push(quote! { let mut count = i32::default(); });
            self.inputs.push(quote! { &mut count });
            return true;
        }
        if function.name == "FMOD_Studio_System_GetBankList" && argument.name == "array" {
            self.targets
                .push(quote! { let mut array = vec![null_mut(); capacity as usize]; });
            self.inputs.push(quote! { array.as_mut_ptr() });
            self.outputs
                .push(quote! { array.into_iter().take(count as usize).map(Bank::from).collect() });
            self.return_types.push(quote! { Vec<Bank> });
            return true;
        }

        if function.name == "FMOD_Studio_System_GetParameterDescriptionList"
            && argument.name == "count"
        {
            self.targets
                .push(quote! { let mut count = i32::default(); });
            self.inputs.push(quote! { &mut count });
            return true;
        }
        if function.name == "FMOD_Studio_System_GetParameterDescriptionList"
            && argument.name == "array"
        {
            self.targets
                .push(quote! { let mut array = vec![ffi::FMOD_STUDIO_PARAMETER_DESCRIPTION::default(); capacity as usize]; });
            self.inputs.push(quote! { array.as_mut_ptr() });
            self.outputs
                .push(quote! { array.into_iter().take(count as usize).map(ParameterDescription::try_from).collect::<Result<_, Error>>()? });
            self.return_types.push(quote! { Vec<ParameterDescription> });
            return true;
        }

        return false;
    }
}

impl Api {
    pub fn patch_functions(&mut self) {
        self.function_patches.insert(
            "FMOD_System_CreateStream".to_string(),
            quote! {
                pub fn create_stream(
                    &self,
                    name_or_data: &str,
                    mode: impl Into<ffi::FMOD_MODE>,
                    exinfo: Option<CreateSoundexInfo>,
                ) -> Result<Sound, Error> {
                    unsafe {
                        let mut sound = null_mut();
                        match ffi::FMOD_System_CreateStream(
                            self.pointer,
                            CString::new(name_or_data)?.as_ptr(),
                            mode.into(),
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
                pub fn create_stream_from(
                    &self,
                    data: &[u8],
                    mode: impl Into<ffi::FMOD_MODE>,
                    exinfo: CreateSoundexInfo,
                ) -> Result<Sound, Error> {
                    unsafe {
                        let mut sound = null_mut();
                        match ffi::FMOD_System_CreateStream(
                            self.pointer,
                            data.as_ptr() as *const _,
                            mode.into(),
                            &mut exinfo.into() as *mut _,
                            &mut sound,
                        ) {
                            ffi::FMOD_OK => Ok(Sound::from(sound)),
                            error => Err(err_fmod!("FMOD_System_CreateStream", error)),
                        }
                    }
                }
            },
        );
        self.function_patches.insert(
            "FMOD_System_CreateSound".to_string(),
            quote! {
                pub fn create_sound(
                    &self,
                    name_or_data: &str,
                    mode: impl Into<ffi::FMOD_MODE>,
                    exinfo: Option<CreateSoundexInfo>,
                ) -> Result<Sound, Error> {
                    unsafe {
                        let mut sound = null_mut();
                        match ffi::FMOD_System_CreateSound(
                            self.pointer,
                            CString::new(name_or_data)?.as_ptr(),
                            mode.into(),
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
                pub fn create_sound_from(
                    &self,
                    data: &[u8],
                    mode: impl Into<ffi::FMOD_MODE>,
                    exinfo: CreateSoundexInfo,
                ) -> Result<Sound, Error> {
                    unsafe {
                        let mut sound = null_mut();
                        match ffi::FMOD_System_CreateSound(
                            self.pointer,
                            data.as_ptr() as *const _,
                            mode.into(),
                            &mut exinfo.into() as *mut _,
                            &mut sound,
                        ) {
                            ffi::FMOD_OK => Ok(Sound::from(sound)),
                            error => Err(err_fmod!("FMOD_System_CreateSound", error)),
                        }
                    }
                }
            },
        );
        self.function_patches.insert(
            "FMOD_Studio_System_LoadBankMemory".to_string(),
            quote! {
                pub fn load_bank_memory(
                    &self,
                    buffer: &[u8],
                    flags: impl Into<ffi::FMOD_STUDIO_LOAD_BANK_FLAGS>,
                ) -> Result<Bank, Error> {
                    unsafe {
                        let mut bank = null_mut();
                        match ffi::FMOD_Studio_System_LoadBankMemory(
                            self.pointer,
                            buffer.as_ptr() as *const std::os::raw::c_char,
                            buffer.len() as std::os::raw::c_int,
                            LoadMemoryMode::Memory.into(),
                            flags.into(),
                            &mut bank,
                        ) {
                            ffi::FMOD_OK => Ok(Bank::from(bank)),
                            error => Err(err_fmod!("FMOD_Studio_System_LoadBankMemory", error)),
                        }
                    }
                }
            },
        );
        self.function_patches.insert(
            "FMOD_Studio_Bank_GetPath".to_string(),
            quote! {
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
                                    &mut retrieved
                                ) {
                                    ffi::FMOD_OK => Ok(
                                        CString::from_vec_with_nul_unchecked(buf)
                                            .into_string()
                                            .map_err(Error::String)?
                                    ),
                                    error => Err(err_fmod!("FMOD_Studio_Bank_GetPath", error)),
                                }
                            }
                            error => {
                                Err(err_fmod!("FMOD_Studio_Bank_GetPath", error))
                            }
                        }
                    }
                }
            }
        );
        self.function_patches.insert("FMOD_Studio_VCA_GetPath".to_string(), quote! {
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
        });
        self.function_patches.insert("FMOD_Studio_Bus_GetPath".to_string(), quote! {
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
        });
        self.function_patches.insert("FMOD_Studio_System_LookupPath".to_string(), quote! {
            pub fn lookup_path(&self, id: Guid) -> Result<String, Error> {
                unsafe {
                    let mut retrieved = i32::default();
                    let id = id.into();
                    match ffi::FMOD_Studio_System_LookupPath(self.pointer, &id, null_mut(), 0, &mut retrieved) {
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
        });
        self.function_patches.insert("FMOD_Studio_EventDescription_GetPath".to_string(), quote! {
            pub fn get_path(&self) -> Result<String, Error> {
                unsafe {
                    let mut retrieved = i32::default();
                    match ffi::FMOD_Studio_EventDescription_GetPath(self.pointer, null_mut(), 0, &mut retrieved) {
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
        });
        self.function_patches.insert(
            "FMOD_Studio_System_IsValid".to_string(),
            quote! {
                pub fn is_valid(&self) -> bool {
                    unsafe {
                        to_bool!(ffi::FMOD_Studio_System_IsValid(self.pointer))
                    }
                }
            },
        );
        self.function_patches.insert(
            "FMOD_Studio_EventDescription_IsValid".to_string(),
            quote! {
                pub fn is_valid(&self) -> bool {
                    unsafe {
                        to_bool!(ffi::FMOD_Studio_EventDescription_IsValid(self.pointer))
                    }
                }
            },
        );
        self.function_patches.insert(
            "FMOD_Studio_EventInstance_IsValid".to_string(),
            quote! {
                pub fn is_valid(&self) -> bool {
                    unsafe {
                        to_bool!(ffi::FMOD_Studio_EventInstance_IsValid(self.pointer))
                    }
                }
            },
        );
        self.function_patches.insert(
            "FMOD_Studio_Bus_IsValid".to_string(),
            quote! {
                pub fn is_valid(&self) -> bool {
                    unsafe {
                        to_bool!(ffi::FMOD_Studio_Bus_IsValid(self.pointer))
                    }
                }
            },
        );
        self.function_patches.insert(
            "FMOD_Studio_VCA_IsValid".to_string(),
            quote! {
                pub fn is_valid(&self) -> bool {
                    unsafe {
                        to_bool!(ffi::FMOD_Studio_VCA_IsValid(self.pointer))
                    }
                }
            },
        );
        self.function_patches.insert(
            "FMOD_Studio_Bank_IsValid".to_string(),
            quote! {
                pub fn is_valid(&self) -> bool {
                    unsafe {
                        to_bool!(ffi::FMOD_Studio_Bank_IsValid(self.pointer))
                    }
                }
            },
        );
        self.function_patches.insert(
            "FMOD_Studio_CommandReplay_IsValid".to_string(),
            quote! {
                pub fn is_valid(&self) -> bool {
                    unsafe {
                        to_bool!(ffi::FMOD_Studio_CommandReplay_IsValid(self.pointer))
                    }
                }
            },
        );
    }
}
