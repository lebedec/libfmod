use std::collections::{BTreeMap, HashSet};
use std::ops::AddAssign;
use std::str::FromStr;

use convert_case::{Case, Casing};
use quote::__private::{Ident, TokenStream};

use crate::ffi;
use crate::ffi::describe_pointer;
use crate::models::Type::{FundamentalType, UserType};
use crate::models::{
    Api, Argument, Enumeration, Error, Field, Function, Modifier, Pointer, Structure, Type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub structure: Structure,
    pub constructor: Function,
    pub methods: Vec<Function>,
}

#[derive(Debug, Default)]
pub struct Lib {
    pub structs: Vec<Struct>,
}

fn extract_struct_key(name: &str) -> String {
    match name.rfind('_') {
        Some(index) => name[..index].to_uppercase(),
        None => name.to_string(),
    }
}

fn format_variant(enumeration: &str, name: &str) -> Ident {
    let name = Api::patch_enumerator(name);
    let enumeration_words: Vec<&str> = enumeration.split("_").collect();
    let variant_words: Vec<&str> = name.split("_").collect();
    // enumeration:
    // ["FMOD", "STUDIO", "PLAYBACK", "STATE"]
    // variants:
    // ["FMOD", "STUDIO", "PLAYBACK", "SUSTAINING"]
    // ["FMOD", "STUDIO", "PLAYBACK", "STOPPED"]
    // ...
    let key = variant_words
        .into_iter()
        .enumerate()
        .skip_while(|(index, word)| enumeration_words.get(*index) == Some(word))
        .map(|(_, word)| word)
        .collect::<Vec<&str>>()
        .join("_");

    let key = key.to_case(Case::UpperCamel);
    let name = Api::patch_variant_name(&key);
    format_ident!("{}", name)
}

fn extract_method_name(name: &str) -> String {
    match name.rfind('_') {
        Some(index) => name[index..]
            .to_string()
            .to_case(Case::Snake)
            .replace("3_d", "3d"),
        None => name.to_string(),
    }
}

fn format_struct_ident(key: &str) -> Ident {
    let name = Api::patch_structure_name(key);
    format_ident!("{}", name)
}

pub fn format_argument_ident(name: &str) -> Ident {
    let name = name.replace("3D", "-3d-");
    let name = name.to_case(Case::Snake);
    let name = Api::patch_ident(&name);
    format_ident!("{}", name)
}

pub fn format_rust_type(
    c_type: &Type,
    as_const: &Option<String>,
    pointer: &Option<Pointer>,
    as_array: &Option<TokenStream>,
    api: &Api,
) -> TokenStream {
    let ptr = describe_pointer(as_const, pointer);
    let tokens = match c_type {
        FundamentalType(name) => match (ptr, &name[..]) {
            ("*const", "char") => quote! { String },
            ("*const *const", "char") => quote! { Vec<String> },
            ("*mut", "char") => quote! { String },
            ("*mut", "void") => quote! { *mut c_void },
            ("*mut", "int") => quote! { Vec<i32> },
            ("*mut", "float") => quote! { Vec<f32> },
            ("*mut *mut", "float") => quote! { Vec<f32> },
            ("*mut *mut", "char") => quote! { Vec<String> },
            ("", "unsigned char") => quote! { u8 },
            ("", "char") => quote! { c_char },
            ("", "int") => quote! { i32 },
            ("", "unsigned int") => quote! { u32 },
            ("", "short") => quote! { i16 },
            ("", "unsigned short") => quote! { u16 },
            ("", "long long") => quote! { i64 },
            ("", "long") => quote! { i64 },
            ("", "unsigned long long") => quote! { u64 },
            ("", "unsigned long") => quote! { u64 },
            ("", "float") => quote! { f32 },
            _ => {
                let name = format_ident!("{}", name);
                quote! { Box<#name> }
            }
        },
        UserType(name) => match (ptr, api.describe_user_type(name)) {
            ("*mut", UserTypeDesc::OpaqueType) => {
                let name = format_struct_ident(name);
                quote! { #name }
            }
            ("*mut", UserTypeDesc::Structure) => {
                let name = format_struct_ident(name);
                quote! { #name }
            }
            ("*mut *mut", UserTypeDesc::Structure) => {
                let name = format_struct_ident(name);
                quote! { Vec<#name> }
            }
            ("*mut", UserTypeDesc::Flags) => {
                let name = format_ident!("{}", name);
                quote! { Vec<ffi::#name> }
            }
            ("*mut", UserTypeDesc::Enumeration) => {
                let name = format_struct_ident(name);
                quote! { Vec<#name> }
            }
            ("", UserTypeDesc::Structure) => {
                let name = format_struct_ident(name);
                quote! { #name }
            }
            ("", UserTypeDesc::Enumeration) => {
                let name = format_struct_ident(name);
                quote! { #name }
            }
            ("", _) => {
                let name = format_ident!("{}", name);
                quote! { ffi::#name }
            }
            _ => quote! { err },
        },
    };
    match as_array {
        None => tokens,
        Some(dimension) => {
            quote! {
                [#tokens; #dimension as usize]
            }
        }
    }
}

pub fn generate_enumeration(enumeration: &Enumeration) -> TokenStream {
    let name = format_struct_ident(&enumeration.name);

    let mut variants = vec![];
    let mut enumerator_arms = vec![];
    let mut variant_arms = vec![];

    for enumerator in &enumeration.enumerators {
        if enumerator.name.ends_with("FORCEINT") {
            continue;
        }
        let variant = format_variant(&enumeration.name, &enumerator.name);
        let enumerator = format_ident!("{}", enumerator.name);
        enumerator_arms.push(quote! {#name::#variant => ffi::#enumerator});
        variant_arms.push(quote! {ffi::#enumerator => Ok(#name::#variant)});
        variants.push(variant);
    }

    let enumeration_name = &enumeration.name;
    let enumeration = format_ident!("{}", enumeration_name);

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum #name {
            #(#variants),*
        }

        impl From<#name> for ffi::#enumeration {
            fn from(value: #name) -> ffi::#enumeration {
                match value {
                    #(#enumerator_arms),*
                }
            }
        }

        impl #name {
            pub fn from(value: ffi::#enumeration) -> Result<#name, Error> {
                match value {
                    #(#variant_arms),*,
                    _ => Err(err_enum!(#enumeration_name, value)),
                }
            }
        }
    }
}

pub fn generate_field(structure: &Structure, field: &Field, api: &Api) -> TokenStream {
    match api.patch_rust_struct_field_definition(&structure.name[..], &field.name[..]) {
        Some(definition) => return definition,
        _ => {}
    };

    let name = format_argument_ident(&field.name);
    let as_array = match &field.as_array {
        None => None,
        Some(dimension) => {
            let token = &dimension[1..dimension.len() - 1];
            let dimension = match api.describe_user_type(token) {
                UserTypeDesc::Constant => {
                    let name = format_ident!("{}", token);
                    quote! { ffi::#name }
                }
                _ => TokenStream::from_str(token).expect("not implemented yet"),
            };
            Some(dimension)
        }
    };
    let field_type = format_rust_type(
        &field.field_type,
        &field.as_const,
        &field.pointer,
        &as_array,
        &api,
    );
    quote! {
        pub #name: #field_type
    }
}

pub fn generate_field_from(structure: &str, field: &Field, api: &Api) -> TokenStream {
    let name = format_argument_ident(&field.name);
    let value_name = ffi::format_rust_ident(&field.name);
    let ptr = describe_pointer(&field.as_const, &field.pointer);

    let getter = match api.patch_field_try_from(structure, &field.name[..]) {
        Some(expression) => {
            if expression.is_empty() {
                return expression;
            } else {
                expression
            }
        }
        _ => match &field.field_type {
            FundamentalType(name) => match (ptr, &name[..]) {
                ("*const", "char") => quote! { to_string!(value.#value_name)? },
                ("*mut", "char") => quote! { to_string!(value.#value_name)? },
                _ => quote! { value.#value_name },
            },
            UserType(name) => match (ptr, api.describe_user_type(name)) {
                ("*mut", UserTypeDesc::OpaqueType) => {
                    let name = format_struct_ident(name);
                    quote! { #name::from(value.#value_name) }
                }
                ("*mut", UserTypeDesc::Structure) => {
                    let name = format_struct_ident(name);
                    quote! { #name::try_from(*value.#value_name)? }
                }
                ("", UserTypeDesc::Structure) => {
                    let name = format_struct_ident(name);
                    quote! { #name::try_from(value.#value_name)? }
                }
                ("", UserTypeDesc::Enumeration) => {
                    let name = format_struct_ident(name);
                    quote! { #name::from(value.#value_name)? }
                }
                _ => quote! { value.#value_name },
            },
        },
    };

    quote! {#name: #getter}
}

pub fn generate_into_field(structure: &str, field: &Field, api: &Api) -> TokenStream {
    let name = ffi::format_rust_ident(&field.name);
    let self_name = format_argument_ident(&field.name);
    let ptr = describe_pointer(&field.as_const, &field.pointer);

    let getter = match api.patch_field_into(structure, &field.name[..]) {
        Some(expression) => expression,
        _ => match &field.field_type {
            FundamentalType(name) => match (ptr, &name[..]) {
                ("*const", "char") => quote! { move_string_to_c!(self.#self_name) },
                ("*mut", "char") => quote! { move_string_to_c!(self.#self_name) as *mut _ },
                _ => quote! { self.#self_name },
            },
            UserType(name) => match (ptr, api.describe_user_type(name)) {
                ("*mut", UserTypeDesc::OpaqueType) => {
                    quote! { self.#self_name.as_mut_ptr() }
                }
                ("*mut", UserTypeDesc::Structure) => {
                    quote! { &mut self.#self_name.into() }
                }
                ("", UserTypeDesc::Structure) => {
                    quote! { self.#self_name.into() }
                }
                ("", UserTypeDesc::Enumeration) => {
                    quote! { self.#self_name.into() }
                }
                _ => quote! { self.#self_name },
            },
        },
    };

    quote! {#name: #getter}
}

pub fn generate_presets(structure: &Structure, api: &Api) -> TokenStream {
    let mut presets = vec![];
    if structure.name == "FMOD_REVERB_PROPERTIES" {
        for preset in &api.presets {
            let ident = format_ident!("{}", preset.name);
            let preset = preset.name.replace("FMOD_PRESET_", "").to_lowercase();
            let preset = format_ident!("{}", preset);
            let preset = quote! {
                #[inline]
                pub fn #preset() -> Self {
                    Self::try_from(ffi::#ident).unwrap()
                }
            };
            presets.push(preset);
        }
    }
    let name = format_struct_ident(&structure.name);
    if presets.is_empty() {
        quote! {}
    } else {
        quote! {
            impl #name {
                #(#presets)*
            }
        }
    }
}

pub fn generate_structure_into(structure: &Structure, api: &Api) -> TokenStream {
    let ident = format_ident!("{}", structure.name);
    let name = format_struct_ident(&structure.name);
    let conversion = structure
        .fields
        .iter()
        .map(|field| generate_into_field(&structure.name, field, api));
    let union = if structure.union.is_some() {
        Some(quote! { ,union: self.union })
    } else {
        None
    };
    quote! {
        impl Into<ffi::#ident> for #name {
            fn into(self) -> ffi::#ident {
                ffi::#ident {
                    #(#conversion),*
                    #union
                }
            }
        }
    }
}

pub fn generate_structure_try_from(structure: &Structure, api: &Api) -> TokenStream {
    let ident = format_ident!("{}", structure.name);
    let name = format_struct_ident(&structure.name);
    let conversion = structure
        .fields
        .iter()
        .map(|field| generate_field_from(&structure.name, field, api))
        .filter(|definition| !definition.is_empty());
    let union = if structure.union.is_some() {
        Some(quote! { ,union: value.union })
    } else {
        None
    };
    quote! {
        impl TryFrom<ffi::#ident> for #name {
            type Error = Error;

            fn try_from(value: ffi::#ident) -> Result<Self, Self::Error> {
                unsafe {
                    Ok(#name {
                        #(#conversion),*
                        #union
                    })
                }
            }
        }
    }
}

pub fn generate_structure(structure: &Structure, api: &Api) -> TokenStream {
    let name = format_struct_ident(&structure.name);
    let mut fields: Vec<TokenStream> = structure
        .fields
        .iter()
        .map(|field| generate_field(structure, field, api))
        .filter(|definition| !definition.is_empty())
        .collect();

    let mut derive = match api.structure_derives.get(&structure.name) {
        None => quote! { Debug, Clone },
        Some(drive) => drive.clone(),
    };
    if structure.union.is_some() {
        let name = format_ident!("{}_UNION", structure.name);
        fields.push(quote! {
            pub union: ffi::#name
        });
        derive = quote! { Clone };
    }
    let presets = generate_presets(structure, api);
    let into = generate_structure_into(structure, api);
    let try_from = generate_structure_try_from(structure, api);
    let conversions = api.structure_patches.get(&structure.name);
    quote! {
        #[derive(#derive)]
        pub struct #name {
            #(#fields),*
        }
        #presets
        #try_from
        #conversions
        #into
    }
}

struct OutArgument {
    pub target: TokenStream,
    pub source: TokenStream,
    pub output: TokenStream,
    pub retype: TokenStream,
}

struct InArgument {
    pub param: TokenStream,
    pub input: TokenStream,
}

pub fn quote_tuple(items: &Vec<TokenStream>) -> TokenStream {
    match items.len() {
        0 => quote! { () },
        1 => {
            let item = &items[0];
            quote! { #item }
        }
        _ => quote! { (#(#items),*) },
    }
}

fn map_optional(argument: &Argument, api: &Api) -> InArgument {
    let pointer = ffi::describe_pointer(&argument.as_const, &argument.pointer);
    let name = format_argument_ident(&argument.name);
    match &argument.argument_type {
        FundamentalType(type_name) => match &format!("{}:{}", pointer, type_name)[..] {
            ":int" => InArgument {
                param: quote! { #name: Option<i32> },
                input: quote! { #name.unwrap_or(0) },
            },
            ":float" => InArgument {
                param: quote! { #name: Option<f32> },
                input: quote! { #name.unwrap_or(0.0) },
            },
            ":unsigned long long" => InArgument {
                param: quote! { #name: Option<u64> },
                input: quote! { #name.unwrap_or(0) },
            },
            ":unsigned int" => InArgument {
                param: quote! { #name: Option<u32> },
                input: quote! { #name.unwrap_or(0) },
            },
            "*mut:float" => InArgument {
                param: quote! { #name: Option<*mut f32> },
                input: quote! { #name.unwrap_or(null_mut()) },
            },
            "*const:char" => InArgument {
                param: quote! { #name: Option<String> },
                input: quote! { #name.map(|value| CString::new(value).map(|value| value.as_ptr())).unwrap_or(Ok(null_mut()))? },
            },
            "*mut:void" => InArgument {
                param: quote! { #name: Option<*mut c_void> },
                input: quote! { #name.unwrap_or(null_mut()) },
            },
            argument_type => {
                unimplemented!("opt {}", argument_type)
            }
        },
        UserType(user_type) => {
            let tp = format_struct_ident(&user_type);
            let ident = format_ident!("{}", user_type);
            match (pointer, api.describe_user_type(&user_type)) {
                ("*mut", UserTypeDesc::Structure) => InArgument {
                    param: quote! { #name: Option<#tp> },
                    input: quote! { #name.map(|value| &mut value.into() as *mut _).unwrap_or(null_mut()) },
                },
                ("*mut", UserTypeDesc::OpaqueType) => InArgument {
                    param: quote! { #name: Option<#tp> },
                    input: quote! { #name.map(|value| value.as_mut_ptr()).unwrap_or(null_mut()) },
                },
                ("*const", UserTypeDesc::Structure) => InArgument {
                    param: quote! { #name: Option<#tp> },
                    input: quote! { #name.map(#tp::into).as_ref().map(from_ref).unwrap_or_else(null) },
                },
                ("", UserTypeDesc::Enumeration) => InArgument {
                    param: quote! { #name: Option<#tp> },
                    input: quote! { #name.map(|value| value.into()).unwrap_or(0) },
                },
                ("", UserTypeDesc::Callback) => InArgument {
                    param: quote! { #name: ffi::#ident },
                    input: quote! { #name },
                },
                user_type => unimplemented!("opt {:?}", user_type),
            }
        }
    }
}

fn map_input(argument: &Argument, api: &Api) -> InArgument {
    let pointer = ffi::describe_pointer(&argument.as_const, &argument.pointer);
    let argument_type = &argument.argument_type;
    let argument = format_argument_ident(&argument.name);
    match argument_type {
        FundamentalType(type_name) => match &format!("{}:{}", pointer, type_name)[..] {
            ":float" => InArgument {
                param: quote! { #argument: f32 },
                input: quote! { #argument },
            },
            ":int" => InArgument {
                param: quote! { #argument: i32 },
                input: quote! { #argument },
            },
            ":unsigned int" => InArgument {
                param: quote! { #argument: u32 },
                input: quote! { #argument },
            },
            ":unsigned long long" => InArgument {
                param: quote! { #argument: u64 },
                input: quote! { #argument },
            },
            "*const:char" => InArgument {
                param: quote! { #argument: &str },
                input: quote! { CString::new(#argument)?.as_ptr() },
            },
            "*mut:void" => InArgument {
                param: quote! { #argument: *mut c_void },
                input: quote! { #argument },
            },
            "*const:void" => InArgument {
                param: quote! { #argument: *const c_void },
                input: quote! { #argument },
            },
            "*mut:float" => InArgument {
                param: quote! { #argument: *mut f32 },
                input: quote! { #argument },
            },
            _ => unimplemented!(),
        },
        UserType(type_name) => {
            let rust_type = format_struct_ident(&type_name);
            let ident = format_ident!("{}", type_name);
            match (pointer, api.describe_user_type(&type_name)) {
                ("*mut", UserTypeDesc::OpaqueType) => InArgument {
                    param: quote! { #argument: #rust_type },
                    input: quote! { #argument.as_mut_ptr() },
                },
                ("*const", UserTypeDesc::Structure) => InArgument {
                    param: quote! { #argument: #rust_type },
                    input: quote! { &#argument.into() },
                },
                ("*mut", UserTypeDesc::Structure) => InArgument {
                    param: quote! { #argument: #rust_type },
                    input: quote! { &mut #argument.into() },
                },
                ("", UserTypeDesc::Structure) => InArgument {
                    param: quote! { #argument: #rust_type },
                    input: quote! { #argument.into() },
                },
                ("", UserTypeDesc::Flags) => InArgument {
                    param: quote! { #argument: impl Into<ffi::#ident> },
                    input: quote! { #argument.into() },
                },
                ("", UserTypeDesc::Enumeration) => InArgument {
                    param: quote! { #argument: #rust_type },
                    input: quote! { #argument.into() },
                },
                ("", UserTypeDesc::Callback) => InArgument {
                    param: quote! { #argument: ffi::#ident },
                    input: quote! { #argument },
                },
                ("", UserTypeDesc::TypeAlias) => match &type_name[..] {
                    "FMOD_BOOL" => InArgument {
                        param: quote! { #argument: bool },
                        input: quote! { from_bool!(#argument) },
                    },
                    "FMOD_PORT_INDEX" => InArgument {
                        param: quote! { #argument: u64 },
                        input: quote! { #argument },
                    },
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            }
        }
    }
}

fn map_output(argument: &Argument, _function: &Function, api: &Api) -> OutArgument {
    let pointer = ffi::describe_pointer(&argument.as_const, &argument.pointer);
    let arg = format_argument_ident(&argument.name);

    match &argument.argument_type {
        FundamentalType(type_name) => match &format!("{}:{}", pointer, type_name)[..] {
            "*mut:char" => OutArgument {
                target: quote! { let #arg = CString::from_vec_unchecked(b"".to_vec()).into_raw(); },
                source: quote! { #arg },
                output: quote! { CString::from_raw(#arg).into_string().map_err(Error::String)? },
                retype: quote! { String },
            },
            "*mut:float" => OutArgument {
                target: quote! { let mut #arg = f32::default(); },
                source: quote! { &mut #arg },
                output: quote! { #arg },
                retype: quote! { f32 },
            },
            "*mut:unsigned long long" => OutArgument {
                target: quote! { let mut #arg = u64::default(); },
                source: quote! { &mut #arg },
                output: quote! { #arg },
                retype: quote! { u64 },
            },
            "*mut:long long" => OutArgument {
                target: quote! { let mut #arg = i64::default(); },
                source: quote! { &mut #arg },
                output: quote! { #arg },
                retype: quote! { i64 },
            },
            "*mut:unsigned int" => OutArgument {
                target: quote! { let mut #arg = u32::default(); },
                source: quote! { &mut #arg },
                output: quote! { #arg },
                retype: quote! { u32 },
            },
            "*mut:int" => OutArgument {
                target: quote! { let mut #arg = i32::default(); },
                source: quote! { &mut #arg },
                output: quote! { #arg },
                retype: quote! { i32 },
            },
            "*mut *mut:void" => OutArgument {
                target: quote! { let mut #arg = null_mut(); },
                source: quote! { &mut #arg },
                output: quote! { #arg },
                retype: quote! { *mut c_void },
            },
            "*mut:void" => OutArgument {
                target: quote! { let #arg = null_mut(); },
                source: quote! { #arg },
                output: quote! { #arg },
                retype: quote! { *mut c_void },
            },
            _ => unimplemented!(),
        },
        UserType(user_type) => {
            let type_name = format_struct_ident(&user_type);
            let ident = format_ident!("{}", user_type);

            match (pointer, api.describe_user_type(&user_type)) {
                ("*mut", UserTypeDesc::TypeAlias) => match &user_type[..] {
                    "FMOD_BOOL" => OutArgument {
                        target: quote! { let mut #arg = ffi::FMOD_BOOL::default(); },
                        source: quote! { &mut #arg },
                        output: quote! { to_bool!(#arg) },
                        retype: quote! { bool },
                    },
                    "FMOD_PORT_INDEX" => OutArgument {
                        target: quote! { let mut #arg = u64::default(); },
                        source: quote! { &mut #arg },
                        output: quote! { #arg },
                        retype: quote! { u64 },
                    },
                    _ => unimplemented!(),
                },
                ("*mut *mut", UserTypeDesc::OpaqueType) => OutArgument {
                    target: quote! { let mut #arg = null_mut(); },
                    source: quote! { &mut #arg },
                    output: quote! { #type_name::from(#arg) },
                    retype: quote! { #type_name },
                },
                ("*mut", UserTypeDesc::Flags) => OutArgument {
                    target: quote! { let mut #arg = ffi::#ident::default(); },
                    source: quote! { &mut #arg },
                    output: quote! { #arg },
                    retype: quote! { ffi::#ident },
                },
                ("*mut", UserTypeDesc::Structure) => OutArgument {
                    target: quote! { let mut #arg = ffi::#ident::default(); },
                    source: quote! { &mut #arg },
                    output: quote! { #type_name::try_from(#arg)? },
                    retype: quote! { #type_name },
                },
                ("*mut *mut", UserTypeDesc::Structure) => OutArgument {
                    target: quote! { let mut #arg = null_mut(); },
                    source: quote! { &mut #arg },
                    output: quote! { #type_name::try_from(*#arg)? },
                    retype: quote! { #type_name },
                },
                ("*const *const", UserTypeDesc::Structure) => OutArgument {
                    target: quote! { let mut #arg = null(); },
                    source: quote! { &mut #arg },
                    output: quote! { #type_name::try_from(*#arg)? },
                    retype: quote! { #type_name },
                },
                ("*mut", UserTypeDesc::Enumeration) => OutArgument {
                    target: quote! { let mut #arg = ffi::#ident::default(); },
                    source: quote! { &mut #arg },
                    output: quote! { #type_name::from(#arg)? },
                    retype: quote! { #type_name },
                },
                _ => unimplemented!(),
            }
        }
    }
}

pub struct Signature {
    pub arguments: Vec<TokenStream>,
    pub inputs: Vec<TokenStream>,
    pub targets: Vec<TokenStream>,
    pub outputs: Vec<TokenStream>,
    pub return_types: Vec<TokenStream>,
}

impl Signature {
    pub fn new() -> Self {
        Self {
            arguments: vec![],
            inputs: vec![],
            targets: vec![],
            outputs: vec![],
            return_types: vec![],
        }
    }

    pub fn define(
        self,
    ) -> (
        Vec<TokenStream>,
        Vec<TokenStream>,
        Vec<TokenStream>,
        TokenStream,
        TokenStream,
    ) {
        (
            self.arguments,
            self.inputs,
            self.targets,
            quote_tuple(&self.outputs),
            quote_tuple(&self.return_types),
        )
    }
}

impl AddAssign<InArgument> for Signature {
    fn add_assign(&mut self, argument: InArgument) {
        self.arguments.push(argument.param);
        self.inputs.push(argument.input);
    }
}

impl AddAssign<OutArgument> for Signature {
    fn add_assign(&mut self, argument: OutArgument) {
        self.targets.push(argument.target);
        self.inputs.push(argument.source);
        self.outputs.push(argument.output);
        self.return_types.push(argument.retype);
    }
}

pub fn generate_method(owner: &str, function: &Function, api: &Api) -> TokenStream {
    let mut signature = Signature::new();

    if let Some(overriding) = api.function_patches.get(&function.name) {
        return overriding.clone();
    }

    for argument in &function.arguments {
        if !signature.patch_function_signature(owner, function, argument) {
            match api.get_modifier(&function.name, &argument.name) {
                Modifier::None => signature += map_input(argument, api),
                Modifier::Opt => signature += map_optional(argument, api),
                Modifier::Out => signature += map_output(argument, function, api),
            }
        }
    }

    let (arguments, inputs, out, output, returns) = signature.define();
    let method_name = extract_method_name(&function.name);
    let method = format_ident!("{}", method_name);
    let function_name = &function.name;
    let function = format_ident!("{}", function_name);

    quote! {
        pub fn #method( #(#arguments),* ) -> Result<#returns, Error> {
            unsafe {
                #(#out)*
                match ffi::#function( #(#inputs),* ) {
                    ffi::FMOD_OK => Ok(#output),
                    error => Err(err_fmod!(#function_name, error)),
                }
            }
        }
    }
}

pub fn generate_opaque_type(key: &String, methods: &Vec<&Function>, api: &Api) -> TokenStream {
    let name = format_struct_ident(key);
    let opaque_type = format_ident!("{}", key);

    let methods: Vec<TokenStream> = methods
        .iter()
        .map(|method| generate_method(key, method, api))
        .collect();

    quote! {
        #[derive(Debug, Clone, Copy)]
        pub struct #name {
            pointer: *mut ffi::#opaque_type,
        }

        unsafe impl Send for #name {}

        unsafe impl Sync for #name {}

        impl #name {
            #[inline]
            pub fn from(pointer: *mut ffi::#opaque_type) -> Self {
                Self { pointer }
            }
            #[inline]
            pub fn as_mut_ptr(&self) -> *mut ffi::#opaque_type {
                self.pointer
            }
            #(#methods)*
        }
    }
}

#[derive(Debug)]
enum UserTypeDesc {
    OpaqueType,
    Structure,
    Enumeration,
    Flags,
    Constant,
    TypeAlias,
    Callback,
    Unknown,
}

impl Api {
    pub fn is_structure(&self, key: &str) -> bool {
        self.structures
            .iter()
            .any(|structure| &structure.name == key)
    }

    pub fn is_opaque_type(&self, key: &str) -> bool {
        self.opaque_types
            .iter()
            .any(|opaque_type| &opaque_type.name == key)
    }

    pub fn is_enumeration(&self, key: &str) -> bool {
        self.enumerations
            .iter()
            .any(|enumeration| &enumeration.name == key)
    }

    pub fn is_flags(&self, key: &str) -> bool {
        self.flags.iter().any(|flags| &flags.name == key)
    }

    pub fn is_constant(&self, key: &str) -> bool {
        self.constants.iter().any(|constant| &constant.name == key)
    }

    pub fn is_type_alias(&self, key: &str) -> bool {
        self.type_aliases
            .iter()
            .any(|type_alias| &type_alias.name == key)
    }

    pub fn is_callback(&self, key: &str) -> bool {
        self.callbacks.iter().any(|callback| &callback.name == key)
    }

    fn describe_user_type(&self, key: &str) -> UserTypeDesc {
        if self.is_structure(key) {
            UserTypeDesc::Structure
        } else if self.is_enumeration(key) {
            UserTypeDesc::Enumeration
        } else if self.is_flags(key) {
            UserTypeDesc::Flags
        } else if self.is_opaque_type(key) {
            UserTypeDesc::OpaqueType
        } else if self.is_constant(key) {
            UserTypeDesc::Constant
        } else if self.is_type_alias(key) {
            UserTypeDesc::TypeAlias
        } else if self.is_callback(key) {
            UserTypeDesc::Callback
        } else {
            UserTypeDesc::Unknown
        }
    }

    pub fn get_modifier(&self, function: &str, argument: &str) -> Modifier {
        let key = format!("{}+{}", function, argument);
        match self.modifiers.get(&key) {
            None => Modifier::None,
            Some(modifier) => modifier.clone(),
        }
    }
}

impl Type {
    pub fn is_user_type(&self, name: &str) -> bool {
        match self {
            FundamentalType(_) => false,
            UserType(user_type) => user_type == name,
        }
    }
}

pub fn generate_lib_code(api: &Api) -> Result<TokenStream, Error> {
    let functions: Vec<&Function> = api
        .functions
        .iter()
        .flat_map(|(_, functions)| functions)
        .collect();

    let opaque_types = api
        .opaque_types
        .iter()
        .map(|opaque_type| opaque_type.name.clone());
    let opaque_types: HashSet<String> = HashSet::from_iter(opaque_types);

    let mut types: BTreeMap<String, Vec<&Function>> = BTreeMap::new();
    for ot in &opaque_types {
        types.insert(ot.clone(), vec![]);
    }
    for function in &functions {
        let key = extract_struct_key(&function.name);
        if opaque_types.contains(&key) {
            match types.get_mut(&key) {
                Some(methods) => methods.push(function),
                None => {
                    types.insert(key, vec![function]);
                }
            }
        } else {
            println!("Global function: {}", function.name);
        }
    }

    let types: Vec<TokenStream> = types
        .iter()
        .map(|(key, methods)| generate_opaque_type(key, methods, api))
        .collect();

    let enumerations: Vec<TokenStream> =
        api.enumerations.iter().map(generate_enumeration).collect();

    let mut structures: Vec<TokenStream> = vec![];
    for structure in &api.structures {
        structures.push(generate_structure(structure, api));
    }

    Ok(quote! {
        #![allow(unused_unsafe)]
        use std::os::raw::{c_char};
        use std::ffi::{c_void, CStr, CString, IntoStringError, NulError};
        use std::fmt::{Display, Formatter};
        use std::mem::size_of;
        use std::ptr::{null, null_mut};
        use std::slice;
        pub mod errors;
        pub mod ffi;
        #[cfg(feature = "flags")]
        mod flags;
        #[cfg(feature = "flags")]
        pub use flags::*;

        #[derive(Debug)]
        pub enum Error {
            Fmod {
                function: String,
                code: i32,
                message: String,
            },
            EnumBindgen {
                enumeration: String,
                value: String
            },
            String(IntoStringError),
            StringNul(NulError),
            NotDspFft
        }

        impl Display for Error {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    Error::Fmod {
                        function,
                        code,
                        message,
                    } => {
                        write!(f, "{}: {} ({})", function, message, code)
                    }
                    Error::EnumBindgen { enumeration, value } => {
                        write!(f, "FMOD returns unexpected value {} for {} enum", value, enumeration)
                    }
                    Error::String(_) => {
                        write!(f, "invalid UTF-8 when converting C string")
                    }
                    Error::StringNul(_) => {
                        write!(f, "nul byte was found in the middle, C strings can't contain it")
                    }
                    Error::NotDspFft => {
                        write!(f, "trying get FFT from DSP which not FFT")
                    }
                }
            }
        }

        impl std::error::Error for Error {}

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
                    message: errors::map_fmod_error($code).to_string(),
                }
            };
        }

        macro_rules! move_string_to_c {
            ($ value : expr) => {
                CString::new($value).unwrap_or(CString::from(c"err!")).into_raw()
            }
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

        macro_rules! ptr_opt {
            ($ ptr : expr , $ value : expr) => {
                if $ptr.is_null() {
                    None
                } else {
                    Some($value)
                }
            };
        }

        macro_rules! opt_ptr {
            ($ opt : expr , $ map : expr) => {
                $opt.map($map).unwrap_or(null_mut())
            };
        }

        macro_rules! to_vec {
            ($ ptr : expr , $ length : expr, $ closure : expr) => {
                 if $length == 0 { Ok(vec![]) } else { slice::from_raw_parts($ptr, $length as usize).to_vec().into_iter().map($closure).collect::<Result<Vec<_>, Error>>() }
            };
            ($ ptr : expr , $ length : expr) => {
                if $length == 0 { vec![] } else { slice::from_raw_parts($ptr, $length as usize).to_vec() }
            };
        }

        macro_rules! to_bool {
            ($ value: expr ) => {
                match $value {
                    1 => true,
                    _ => false
                }
            }
        }
        macro_rules! from_bool {
            ($ value: expr ) => {
                match $value {
                    true => 1,
                    _ => 0
                }
            }
        }

        pub fn attr3d_array8(values: Vec<Attributes3d>) -> [Attributes3d; ffi::FMOD_MAX_LISTENERS as usize] {
            values.try_into().expect("slice with incorrect length")
        }

        pub fn vec_as_mut_ptr<T, O, F>(values: Vec<T>, map: F) -> *mut O
        where
            F: FnMut(T) -> O,
        {
            let mut values = values.into_iter().map(map).collect::<Vec<O>>();
            Box::into_raw(values.into_boxed_slice()) as *mut _
        }

        const fn from_ref<T: ?Sized>(value: &T) -> *const T {
            value
        }

        #(#enumerations)*
        #(#structures)*
        #(#types)*
    })
}

pub fn generate(api: &Api) -> Result<String, Error> {
    generate_lib_code(api).map(|code| code.to_string())
}
