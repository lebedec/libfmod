use crate::Api;
use quote::__private::TokenStream;

impl Api {
    pub fn patch_rust_struct_field_definition(
        &self,
        structure: &str,
        field: &str,
    ) -> Option<TokenStream> {
        let expression = match (structure, field) {
            ("FMOD_DSP_PARAMETER_FFT", "spectrum") => quote! { pub spectrum: Vec<Vec<f32>> },
            ("FMOD_CREATESOUNDEXINFO", "dlsname") => quote! { pub dlsname: Option<String> },
            ("FMOD_CREATESOUNDEXINFO", "fsbguid") => quote! { pub fsbguid: Option<Guid> },
            ("FMOD_CREATESOUNDEXINFO", "encryptionkey") => {
                quote! { pub encryptionkey: Option<String> }
            }
            ("FMOD_CREATESOUNDEXINFO", "initialsoundgroup") => {
                quote! { pub initialsoundgroup: Option<SoundGroup> }
            }
            ("FMOD_CREATESOUNDEXINFO", "inclusionlist") => {
                quote! { pub inclusionlist: Option<Vec<i32>> }
            }
            //
            ("FMOD_CREATESOUNDEXINFO", "inclusionlistnum") => quote! {},
            ("FMOD_ADVANCEDSETTINGS", "cbSize") => quote! {},
            ("FMOD_STUDIO_ADVANCEDSETTINGS", "cbsize") => quote! {},
            ("FMOD_CREATESOUNDEXINFO", "cbsize") => quote! {},
            ("FMOD_DSP_DESCRIPTION", "numparameters") => quote! {},
            ("FMOD_DSP_PARAMETER_FFT", "numchannels") => quote! {},
            _ => return None,
        };
        Some(expression)
    }

    /*
        fn is_convertable(structure: &Structure, field: &Field) -> bool {
        match (&structure.name[..], &field.name[..]) {
            ("FMOD_ADVANCEDSETTINGS", "cbSize") => false,
            ("FMOD_STUDIO_ADVANCEDSETTINGS", "cbsize") => false,
            ("FMOD_CREATESOUNDEXINFO", "cbsize") => false,
            ("FMOD_DSP_DESCRIPTION", "numparameters") => false,
            ("FMOD_DSP_PARAMETER_FFT", "numchannels") => false,
            _ => true,
        }
    }
         */

    pub fn patch_field_try_from(&self, structure: &str, field: &str) -> Option<TokenStream> {
        let expression = match (structure, field) {
            ("FMOD_ADVANCEDSETTINGS", "cbSize") => quote! {},
            ("FMOD_STUDIO_ADVANCEDSETTINGS", "cbsize") => quote! {},
            ("FMOD_CREATESOUNDEXINFO", "cbsize") => quote! {},
            ("FMOD_DSP_DESCRIPTION", "numparameters") => quote! {},
            ("FMOD_DSP_PARAMETER_FFT", "numchannels") => quote! {},
            //
            ("FMOD_CREATESOUNDEXINFO", "inclusionlist") => quote! {
                ptr_opt!(value.inclusionlist, to_vec!(value.inclusionlist, value.inclusionlistnum))
            },
            ("FMOD_CREATESOUNDEXINFO", "inclusionlistnum") => quote! {},
            ("FMOD_CREATESOUNDEXINFO", "dlsname") => quote! {
                ptr_opt!(value.dlsname, to_string!(value.dlsname)?)
            },
            ("FMOD_CREATESOUNDEXINFO", "encryptionkey") => quote! {
                ptr_opt!(value.encryptionkey, to_string!(value.encryptionkey)?)
            },
            ("FMOD_CREATESOUNDEXINFO", "initialsoundgroup") => quote! {
                ptr_opt!(value.initialsoundgroup, SoundGroup::from(value.initialsoundgroup))
            },
            ("FMOD_CREATESOUNDEXINFO", "fsbguid") => quote! {
                ptr_opt!(value.fsbguid, Guid::from_ptr(value.fsbguid))
            },
            ("FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI", "relative") => {
                quote! { attr3d_array8(value.relative.map(Attributes3d::try_from).into_iter().collect::<Result<Vec<Attributes3d>, Error>>()?) }
            }
            ("FMOD_ADVANCEDSETTINGS", "ASIOChannelList") => {
                quote! { to_vec!(value.ASIOChannelList, value.ASIONumChannels, |ptr| to_string!(ptr))? }
            }
            ("FMOD_ADVANCEDSETTINGS", "ASIOSpeakerList") => {
                quote! { to_vec!(value.ASIOSpeakerList, value.ASIONumChannels, Speaker::from)? }
            }
            ("FMOD_OUTPUT_OBJECT3DINFO", "buffer") => {
                quote! { to_vec!(value.buffer, value.bufferlength) }
            }
            ("FMOD_DSP_BUFFER_ARRAY", "buffernumchannels") => {
                quote! { to_vec!(value.buffernumchannels, value.numbuffers) }
            }
            ("FMOD_DSP_BUFFER_ARRAY", "bufferchannelmask") => {
                quote! { to_vec!(value.bufferchannelmask, value.numbuffers) }
            }
            ("FMOD_DSP_BUFFER_ARRAY", "buffers") => {
                quote! { to_vec!(value.buffers, value.numbuffers, |ptr| Ok(*ptr))? }
            }
            ("FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR", "pointparamvalues") => {
                quote! { to_vec!(value.pointparamvalues, value.numpoints) }
            }
            ("FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR", "pointpositions") => {
                quote! { to_vec!(value.pointpositions, value.numpoints) }
            }
            ("FMOD_DSP_PARAMETER_DESC_INT", "valuenames") => {
                quote! { vec![] } // TODO
            }
            ("FMOD_DSP_PARAMETER_DESC_BOOL", "valuenames") => {
                quote! { vec![] } // TODO
            }
            ("FMOD_DSP_PARAMETER_FFT", "spectrum") => {
                quote! { to_vec!(value.spectrum.as_ptr(), value.numchannels, |ptr| Ok(to_vec!(ptr, value.length)))? }
            }
            ("FMOD_DSP_DESCRIPTION", "paramdesc") => {
                quote! { to_vec!(*value.paramdesc, value.numparameters, DspParameterDesc::try_from)? }
            }
            ("FMOD_DSP_STATE", "sidechaindata") => {
                quote! { to_vec!(value.sidechaindata, value.sidechainchannels) }
            }
            _ => return None,
        };
        Some(expression)
    }

    pub fn patch_field_into(&self, structure: &str, field: &str) -> Option<TokenStream> {
        let expression = match (structure, field) {
            ("FMOD_CREATESOUNDEXINFO", "inclusionlist") => {
                quote! { opt_ptr!(self.inclusionlist.clone(), |v| v.as_slice().as_ptr()as *mut _) }
            }
            ("FMOD_CREATESOUNDEXINFO", "inclusionlistnum") => {
                quote! { self.inclusionlist.map(|v| v.len()).unwrap_or(0) as _ }
            }
            ("FMOD_CREATESOUNDEXINFO", "dlsname") => {
                quote! { opt_ptr!(self.dlsname.map(|v| CString::new(v).unwrap()), |v| v.as_ptr()) }
            }
            ("FMOD_CREATESOUNDEXINFO", "encryptionkey") => {
                quote! { opt_ptr!(self.encryptionkey.map(|v| CString::new(v).unwrap()), |v| v.as_ptr()) }
            }
            ("FMOD_CREATESOUNDEXINFO", "initialsoundgroup") => {
                quote! { opt_ptr!(self.initialsoundgroup, |v| v.as_mut_ptr()) }
            }
            ("FMOD_CREATESOUNDEXINFO", "fsbguid") => {
                quote! { opt_ptr!(self.fsbguid, |v| &mut v.into() as *mut _) }
            }
            ("FMOD_CREATESOUNDEXINFO", "cbsize") => {
                quote! { size_of::<ffi::FMOD_CREATESOUNDEXINFO>() as i32 }
            }
            ("FMOD_ADVANCEDSETTINGS", "cbSize") => {
                quote! { size_of::<ffi::FMOD_ADVANCEDSETTINGS>() as i32 }
            }
            ("FMOD_STUDIO_ADVANCEDSETTINGS", "cbsize") => {
                quote! { size_of::<ffi::FMOD_STUDIO_ADVANCEDSETTINGS>() as i32 }
            }
            ("FMOD_DSP_DESCRIPTION", "numparameters") => {
                quote! { self.paramdesc.len() as i32 }
            }
            ("FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI", "relative") => {
                quote! { self.relative.map(Attributes3d::into) }
            }
            ("FMOD_OUTPUT_OBJECT3DINFO", "buffer") => {
                quote! { self.buffer.as_ptr() as *mut _ }
            }
            ("FMOD_ADVANCEDSETTINGS", "ASIOChannelList") => {
                quote! { self.asio_channel_list.into_iter().map(|val| val.as_ptr()).collect::<Vec<_>>().as_mut_ptr().cast() }
            }
            ("FMOD_ADVANCEDSETTINGS", "ASIOSpeakerList") => {
                quote! { self.asio_speaker_list.into_iter().map(|val| val.into()).collect::<Vec<_>>().as_mut_ptr() }
            }
            ("FMOD_DSP_BUFFER_ARRAY", "buffernumchannels") => {
                quote! { self.buffernumchannels.as_ptr() as *mut _ }
            }
            ("FMOD_DSP_BUFFER_ARRAY", "bufferchannelmask") => {
                quote! { self.bufferchannelmask.as_ptr() as *mut _ }
            }
            ("FMOD_DSP_BUFFER_ARRAY", "buffers") => {
                quote! { self.buffers.as_ptr() as *mut _ }
            }
            ("FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR", "pointparamvalues") => {
                quote! { self.pointparamvalues.as_ptr() as *mut _ }
            }
            ("FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR", "pointpositions") => {
                quote! { self.pointpositions.as_ptr() as *mut _ }
            }
            ("FMOD_DSP_PARAMETER_DESC_INT", "valuenames") => {
                quote! { self.valuenames.as_ptr() as *mut _ }
            }
            ("FMOD_DSP_PARAMETER_DESC_BOOL", "valuenames") => {
                quote! { self.valuenames.as_ptr() as *mut _ }
            }
            ("FMOD_DSP_DESCRIPTION", "paramdesc") => {
                quote! { &mut vec_as_mut_ptr(self.paramdesc, |param| param.into()) }
            }
            ("FMOD_DSP_STATE", "sidechaindata") => {
                quote! { self.sidechaindata.as_ptr() as *mut _ }
            }
            ("FMOD_DSP_PARAMETER_FFT", "numchannels") => {
                quote! { self.spectrum.len() as i32 }
            }
            ("FMOD_DSP_PARAMETER_FFT", "spectrum") => {
                quote! { [null_mut(); 32] }
            }
            _ => return None,
        };
        Some(expression)
    }
}
