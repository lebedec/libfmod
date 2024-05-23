use crate::models::{
    Callback, Constant, Enumeration, Error, Flags, OpaqueType, Preset, Structure, TypeAlias,
};
use crate::repr::JsonConverter;
use pest::{error, Parser};

#[derive(Parser)]
#[grammar = "./grammars/fmod_common.pest"]
struct FmodCommonParser;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Header {
    pub opaque_types: Vec<OpaqueType>,
    pub constants: Vec<Constant>,
    pub flags: Vec<Flags>,
    pub enumerations: Vec<Enumeration>,
    pub structures: Vec<Structure>,
    pub callbacks: Vec<Callback>,
    pub type_aliases: Vec<TypeAlias>,
    pub presets: Vec<Preset>,
}

pub fn parse(source: &str) -> Result<Header, Error> {
    let declarations = FmodCommonParser::parse(Rule::api, source)?
        .next()
        .ok_or(Error::FileMalformed)?;

    let arrays = vec![
        String::from("flags"),
        String::from("enumerators"),
        String::from("fields"),
        String::from("arguments"),
        String::from("values"),
    ];
    let converter = JsonConverter::new(arrays);

    let mut header = Header::default();
    for declaration in declarations.into_inner() {
        match declaration.as_rule() {
            Rule::OpaqueType => header.opaque_types.push(converter.convert(declaration)?),
            Rule::Constant => header.constants.push(converter.convert(declaration)?),
            Rule::Flags => header.flags.push(converter.convert(declaration)?),
            Rule::Enumeration => header.enumerations.push(converter.convert(declaration)?),
            Rule::Structure => {
                let structure: Structure = converter.convert(declaration)?;
                if let Some(index) = header
                    .opaque_types
                    .iter()
                    .position(|opaque_type| opaque_type.name == structure.name)
                {
                    header.opaque_types.remove(index);
                }
                header.structures.push(structure);
            }
            Rule::Callback => header.callbacks.push(converter.convert(declaration)?),
            Rule::TypeAlias => header.type_aliases.push(converter.convert(declaration)?),
            Rule::Preset => header.presets.push(converter.convert(declaration)?),
            _ => continue,
        }
    }

    Ok(header)
}

impl From<error::Error<Rule>> for Error {
    fn from(error: error::Error<Rule>) -> Self {
        Self::Pest(error.to_string())
    }
}
