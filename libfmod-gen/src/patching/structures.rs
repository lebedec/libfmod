use crate::patching::dictionary::RENAMES;
use crate::Api;
use convert_case::{Case, Casing};
use quote::__private::TokenStream;

impl Api {
    pub fn patch_structures(&mut self) {
        self.structure_patches.insert("FMOD_DSP_PARAMETER_FFT".to_string(), quote! {
            impl TryFrom<Dsp> for DspParameterFft {
                type Error = Error;
                fn try_from(dsp: Dsp) -> Result<Self, Self::Error> {
                    match dsp.get_type() {
                        Ok(DspType::Fft) => {
                            let (ptr, _, _) = dsp.get_parameter_data(ffi::FMOD_DSP_FFT_SPECTRUMDATA, 0)?;
                            let fft = unsafe {
                                *(ptr as *const ffi::FMOD_DSP_PARAMETER_FFT)
                            };
                            DspParameterFft::try_from(fft)
                        },
                        _ => Err(Error::NotDspFft)
                    }
                }
            }
        });
        self.structure_patches.insert(
            "FMOD_CREATESOUNDEXINFO".to_string(),
            quote! {
               impl Default for CreateSoundexInfo {
                    fn default() -> Self {
                        Self::try_from(ffi::FMOD_CREATESOUNDEXINFO::default()).unwrap()
                    }
                }
            },
        );
        self.structure_patches.insert(
            "FMOD_GUID".to_string(),
            quote! {
               impl Guid {
                    pub fn from_ptr(value: *mut ffi::FMOD_GUID) -> Self {
                        let value = unsafe { *value };
                        Self {
                            data_1: value.Data1,
                            data_2: value.Data2,
                            data_3: value.Data3,
                            data_4: value.Data4,
                        }
                    }
                }
            },
        );
        self.structure_patches.insert(
            "FMOD_VECTOR".to_string(),
            quote! {
                impl Vector {
                    pub const fn new(x: f32, y: f32, z: f32) -> Self {
                        Vector { x, y, z }
                    }
                }
                impl From<[f32;3]> for Vector {
                    fn from(value: [f32;3]) -> Vector {
                        Vector {
                            x: value[0],
                            y: value[1],
                            z: value[2]
                        }
                    }
                }
                impl From<Vector> for [f32; 3] {
                    fn from(value: Vector) -> [f32; 3] {
                        [value.x, value.y, value.z]
                    }
                }
                impl From<(f32, f32, f32)> for Vector {
                    fn from(value: (f32, f32, f32)) -> Vector {
                        Vector {
                            x: value.0,
                            y: value.1,
                            z: value.2
                        }
                    }
                }
                impl From<Vector> for (f32, f32, f32) {
                    fn from(value: Vector) -> (f32, f32, f32) {
                        (value.x, value.y, value.z)
                    }
                }
            },
        );
    }

    pub fn patch_structure_derives(&mut self) {
        self.structure_derives
            .insert("FMOD_DSP_DESCRIPTION".to_string(), quote! { Clone });
        self.structure_derives.insert(
            "FMOD_VECTOR".to_string(),
            quote! { Debug, Clone, Copy, PartialEq },
        );
    }

    pub fn patch_structure_name(key: &str) -> String {
        let key = key.replace("FMOD_RESULT", "FMOD_FMODRESULT");
        let key = key.replace("FMOD_", "");
        let key = key.replace("STUDIO_SYSTEM", "STUDIOSYSTEM");
        let key = key.replace("STUDIO_ADVANCEDSETTINGS", "STUDIOADVANCEDSETTINGS");
        let key = key.replace("STUDIO_CPU_USAGE", "STUDIOCPUUSAGE");
        let key = key.replace("STUDIO_", "");
        let name = key.to_case(Case::Pascal);
        let name = match RENAMES.get(&name[..]) {
            None => name,
            Some(rename) => rename.to_string(),
        };
        name.to_string()
    }

    pub fn patch_ffi_structure_default(key: &str) -> Option<TokenStream> {
        let definition = match key {
            "FMOD_STUDIO_ADVANCEDSETTINGS" => quote! {
                impl Default for FMOD_STUDIO_ADVANCEDSETTINGS {
                    fn default() -> Self {
                        let mut value: Self = unsafe { std::mem::zeroed() };
                        value.cbsize = std::mem::size_of::<FMOD_STUDIO_ADVANCEDSETTINGS>() as _;
                        value
                    }
                }
            },
            "FMOD_ADVANCEDSETTINGS" => quote! {
                impl Default for FMOD_ADVANCEDSETTINGS {
                    fn default() -> Self {
                        let mut value: Self = unsafe { std::mem::zeroed() };
                        value.cbSize = std::mem::size_of::<FMOD_ADVANCEDSETTINGS>() as _;
                        value
                    }
                }
            },
            "FMOD_CREATESOUNDEXINFO" => quote! {
                impl Default for FMOD_CREATESOUNDEXINFO {
                    fn default() -> Self {
                        let mut value: Self = unsafe { std::mem::zeroed() };
                        value.cbsize = std::mem::size_of::<FMOD_CREATESOUNDEXINFO>() as _;
                        value
                    }
                }
            },
            _ => return None,
        };
        Some(definition)
    }
}
