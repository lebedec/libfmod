use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use quote::__private::{Ident, LexError, Literal, TokenStream};
use quote::quote;

use crate::models::Type::FundamentalType;
use crate::models::{
    Api, Argument, Callback, Constant, Enumeration, Error, ErrorStringMapping, Field, Flags,
    Function, OpaqueType, Pointer, Preset, Structure, Type, TypeAlias, Union,
};

impl From<rustfmt_wrapper::Error> for Error {
    fn from(error: rustfmt_wrapper::Error) -> Self {
        Error::Fmt(format!("{:?}", error))
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::ParseInt(error.to_string())
    }
}

impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Self {
        Error::ParseFloat(error.to_string())
    }
}

impl From<LexError> for Error {
    fn from(error: LexError) -> Self {
        Error::LexError(error.to_string())
    }
}

pub fn generate_opaque_type(value: &OpaqueType) -> TokenStream {
    let name = format_ident!("{}", value.name);

    quote! {
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct #name {
            _unused: [u8; 0]
        }
    }
}

pub fn generate_constant(constant: &Constant) -> Result<TokenStream, Error> {
    let name = format_ident!("{}", &constant.name);
    let value = &constant.value;

    let tokens = if value.len() == "0xFFFFFFFFFFFFFFFF".len() && value.starts_with("0x") {
        let value = TokenStream::from_str(value)?;
        quote! {
            pub const #name: c_ulonglong = #value;
        }
    } else if value.len() == "0xaaaabbcc".len() && value.starts_with("0x") {
        let value = TokenStream::from_str(value)?;
        quote! {
            pub const #name: c_uint = #value;
        }
    } else {
        let value = Literal::u32_unsuffixed(value.parse()?);
        quote! {
            pub const #name: c_uint = #value;
        }
    };

    Ok(tokens)
}

pub fn map_c_type(c_type: &Type) -> TokenStream {
    let name = match c_type {
        FundamentalType(name) => {
            let name = name.replace("unsigned", "u").replace(" ", "");
            format_ident!("c_{}", name)
        }
        Type::UserType(name) => format_ident!("{}", name),
    };
    quote! { #name }
}

pub fn describe_pointer<'a>(as_const: &'a Option<String>, pointer: &'a Option<Pointer>) -> &'a str {
    let description = match (as_const, pointer) {
        (None, None) => "",
        (None, Some(Pointer::NormalPointer(_))) => "*mut",
        (None, Some(Pointer::DoublePointer(_))) => "*mut *mut",
        (Some(_), Some(Pointer::NormalPointer(_))) => "*const",
        (Some(_), Some(Pointer::DoublePointer(_))) => "*const *const",
        (Some(_), None) => "",
    };
    description
}

pub fn format_rust_type(
    c_type: &Type,
    as_const: &Option<String>,
    pointer: &Option<Pointer>,
    as_array: &Option<TokenStream>,
) -> TokenStream {
    let name = map_c_type(c_type);
    let pointer = describe_pointer(as_const, pointer);
    let pointer = TokenStream::from_str(pointer).expect("not implemented yet");
    let rust_type = quote! { #pointer #name };
    match as_array {
        Some(dimension) => quote! { [#rust_type; #dimension as usize] },
        None => rust_type,
    }
}

pub fn generate_type_alias(type_alias: &TypeAlias) -> TokenStream {
    let name = format_ident!("{}", type_alias.name);
    let base = format_rust_type(&type_alias.base_type, &None, &None, &None);

    quote! {
        pub type #name = #base;
    }
}

pub fn generate_enumeration(enumeration: &Enumeration) -> Result<TokenStream, Error> {
    let name = format_ident!("{}", enumeration.name);
    let mut value: i32 = -1;
    let mut enumerators = vec![];
    for enumerator in &enumeration.enumerators {
        let label = format_ident!("{}", &enumerator.name);
        let value = match &enumerator.value {
            None => {
                value += 1;
                value
            }
            Some(repr) => {
                value = repr.parse()?;
                value
            }
        };
        let literal = Literal::i32_unsuffixed(value);
        enumerators.push(quote! {
            pub const #label: #name = #literal;
        });
    }
    Ok(quote! {
        pub type #name = c_int;
        #(#enumerators)*
    })
}

pub fn format_rust_ident(name: &str) -> Ident {
    let name = Api::patch_ident(&name);
    format_ident!("{}", name)
}

pub fn generate_argument(argument: &Argument) -> TokenStream {
    let name = format_rust_ident(&argument.name);
    let argument_type = format_rust_type(
        &argument.argument_type,
        &argument.as_const,
        &argument.pointer,
        &None,
    );
    quote! {
        #name: #argument_type
    }
}

impl Type {
    pub fn is_void(&self) -> bool {
        self == &FundamentalType("void".into())
    }
}

impl Callback {
    pub fn returns(&self) -> Option<TokenStream> {
        if !(self.return_type.is_void() && self.pointer.is_none()) {
            let return_type = format_rust_type(&self.return_type, &None, &self.pointer, &None);
            Some(return_type)
        } else {
            None
        }
    }
}

pub fn generate_callback(callback: &Callback) -> TokenStream {
    let name = format_ident!("{}", callback.name);
    let arguments = callback.arguments.iter().map(generate_argument);
    let varargs = if callback.varargs.is_some() {
        Some(quote! {, ...})
    } else {
        None
    };
    let return_type = if let Some(return_type) = callback.returns() {
        Some(quote! { -> #return_type })
    } else {
        None
    };

    quote! {
        pub type #name = Option<
            unsafe extern "C" fn(#(#arguments),* #varargs) #return_type
        >;
    }
}

pub fn generate_flags(flags: &Flags) -> Result<TokenStream, Error> {
    let name = format_ident!("{}", flags.name);
    let base_type = map_c_type(&flags.flags_type);
    let mut values = vec![];
    for flag in &flags.flags {
        let value = TokenStream::from_str(&flag.value)?;
        let flag = format_ident!("{}", flag.name);
        values.push(quote! {
            pub const #flag: #name = #value;
        })
    }
    Ok(quote! {
        pub type #name = #base_type;
        #(#values)*
    })
}

impl Field {
    pub fn array(&self) -> Option<TokenStream> {
        match &self.as_array {
            None => None,
            Some(repr) => Some(
                TokenStream::from_str(&repr[1..repr.len() - 1]).expect("unexpected array repr"),
            ),
        }
    }
}

pub fn generate_field(field: &Field) -> TokenStream {
    let name = format_rust_ident(&field.name);
    let field_type = format_rust_type(
        &field.field_type,
        &field.as_const,
        &field.pointer,
        &field.array(),
    );
    quote! {
        pub #name: #field_type
    }
}

pub fn generate_structure_default(structure: &Structure) -> TokenStream {
    let name = format_ident!("{}", structure.name);
    match Api::patch_ffi_structure_default(&structure.name) {
        Some(definition) => definition,
        None => quote! {
            impl Default for #name {
                fn default() -> Self {
                    unsafe { std::mem::zeroed() }
                }
            }
        },
    }
}

pub fn generate_structure_union(name: &Ident, union: &Union) -> TokenStream {
    let fields = union.fields.iter().map(generate_field);
    quote! {
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub union #name {
            #(#fields),*
        }
    }
}

pub fn generate_structure(structure: &Structure) -> TokenStream {
    let name = format_ident!("{}", structure.name);
    let fields = structure.fields.iter().map(generate_field);
    let default = generate_structure_default(&structure);
    match &structure.union {
        None => {
            quote! {
                #[repr(C)]
                #[derive(Debug, Copy, Clone)]
                pub struct #name {
                    #(#fields),*
                }
                #default
            }
        }
        Some(union) => {
            let union_name = format_ident!("{}_UNION", structure.name);
            let union = generate_structure_union(&union_name, union);
            quote! {
                #[repr(C)]
                #[derive(Copy, Clone)]
                pub struct #name {
                    #(#fields),*,
                    pub union: #union_name
                }
                #default
                #union
            }
        }
    }
}

pub fn generate_function(function: &Function) -> TokenStream {
    let name = format_ident!("{}", function.name);
    let arguments = function.arguments.iter().map(generate_argument);
    let return_type = map_c_type(&function.return_type);
    quote! {
        pub fn #name(#(#arguments),*) -> #return_type;
    }
}

pub fn generate_extern(_link: &String, api: &Vec<Function>) -> TokenStream {
    let functions = api.iter().map(generate_function);
    quote! {
        extern "C" {
            #(#functions)*
        }
    }
}

pub fn generate_preset(structure: &Structure, preset: &Preset) -> Result<TokenStream, Error> {
    let name = format_ident!("{}", preset.name);
    let mut fields: Vec<TokenStream> = vec![];
    for (index, value) in preset.values.iter().enumerate() {
        let value = if value.ends_with("f") {
            &value[0..value.len() - 1]
        } else {
            &value[..]
        };
        let value: f32 = value.parse()?;
        let field = format_rust_ident(&structure.fields[index].name);
        let value = Literal::f32_unsuffixed(value);
        fields.push(quote! {
            #field: #value
        });
    }
    let structure = format_ident!("{}", structure.name);

    Ok(quote! {
        pub const #name: #structure = #structure {
            #(#fields),*
        };
    })
}

pub fn generate_errors_mapping_code(mapping: &ErrorStringMapping) -> TokenStream {
    let mut cases = vec![];
    for error in &mapping.errors {
        let result = format_ident!("{}", error.name);
        let error = &error.string;
        cases.push(quote! {
            #result => #error,
        });
    }
    quote! {
        pub fn map_fmod_error(result: FMOD_RESULT) -> &'static str {
            match result {
                #(#cases)*
                _ => "Unknown error code"
            }
        }
    }
}

pub fn generate_ffi_code(api: &Api) -> Result<TokenStream, Error> {
    let opaque_types: Vec<TokenStream> =
        api.opaque_types.iter().map(generate_opaque_type).collect();

    let mut constants = vec![];
    for constant in &api.constants {
        constants.push(generate_constant(constant)?);
    }

    let type_aliases: Vec<TokenStream> = api.type_aliases.iter().map(generate_type_alias).collect();

    let mut enumerations = vec![];
    for enumeration in &api.enumerations {
        enumerations.push(generate_enumeration(enumeration)?);
    }

    let callbacks: Vec<TokenStream> = api.callbacks.iter().map(generate_callback).collect();

    let mut flags = vec![];
    for flag in &api.flags {
        flags.push(generate_flags(flag)?);
    }

    let mut structures = vec![];
    for structure in &api.structures {
        structures.push(generate_structure(structure));
    }

    let mut libraries = vec![];
    for (link, functions) in &api.functions {
        libraries.push(generate_extern(link, functions));
    }

    let mut presets = vec![];
    if let Some(structure) = api
        .structures
        .iter()
        .find(|structure| structure.name == "FMOD_REVERB_PROPERTIES")
    {
        for preset in &api.presets {
            presets.push(generate_preset(structure, preset)?);
        }
    }

    let errors = if api.errors.errors.is_empty() {
        None
    } else {
        Some(generate_errors_mapping_code(&api.errors))
    };

    Ok(quote! {
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]
        #![allow(unused_parens)]
        use std::os::raw::{c_char, c_float, c_int, c_longlong, c_short, c_uchar, c_uint, c_ulonglong, c_ushort, c_void};

        #(#opaque_types)*
        #(#type_aliases)*
        #(#constants)*
        #(#enumerations)*
        #(#flags)*
        #(#structures)*
        #(#presets)*
        #(#callbacks)*
        #(#libraries)*
        #errors
    })
}

pub fn generate(api: &Api) -> Result<String, Error> {
    let code = generate_ffi_code(api)?;
    rustfmt_wrapper::rustfmt(code).map_err(Error::from)
}
