use quote::__private::TokenStream;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum Error {
    FileMalformed,
    Pest(String),
    Serde(String),
    Fmt(String),
    ParseInt(String),
    ParseFloat(String),
    LexError(String),
    Io(String),
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Pointer {
    NormalPointer(String),
    DoublePointer(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Type {
    FundamentalType(String),
    UserType(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Argument {
    pub as_const: Option<String>,
    pub argument_type: Type,
    pub pointer: Option<Pointer>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpaqueType {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Constant {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Flag {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Flags {
    pub flags_type: Type,
    pub name: String,
    pub flags: Vec<Flag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Enumerator {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Enumeration {
    pub name: String,
    pub enumerators: Vec<Enumerator>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field {
    pub as_const: Option<String>,
    pub as_array: Option<String>,
    pub field_type: Type,
    pub pointer: Option<Pointer>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Union {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Structure {
    pub name: String,
    pub fields: Vec<Field>,
    pub union: Option<Union>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Callback {
    pub return_type: Type,
    pub pointer: Option<Pointer>,
    pub name: String,
    pub arguments: Vec<Argument>,
    pub varargs: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeAlias {
    pub base_type: Type,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorString {
    pub name: String,
    pub string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ErrorStringMapping {
    pub errors: Vec<ErrorString>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Preset {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Modifier {
    None,
    Out,
    Opt,
}

#[derive(Debug, Default)]
pub struct Api {
    pub opaque_types: Vec<OpaqueType>,
    pub constants: Vec<Constant>,
    pub flags: Vec<Flags>,
    pub enumerations: Vec<Enumeration>,
    pub structures: Vec<Structure>,
    pub callbacks: Vec<Callback>,
    pub type_aliases: Vec<TypeAlias>,
    pub functions: Vec<(String, Vec<Function>)>,
    pub presets: Vec<Preset>,
    pub errors: ErrorStringMapping,
    pub modifiers: HashMap<String, Modifier>,
    pub structure_patches: HashMap<String, TokenStream>,
    pub structure_derives: HashMap<String, TokenStream>,
    pub function_patches: HashMap<String, TokenStream>,
}
