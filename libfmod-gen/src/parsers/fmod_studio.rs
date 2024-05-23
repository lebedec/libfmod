use crate::models::{Error, Function};
use crate::repr::JsonConverter;
use pest::{error, Parser};

#[derive(Parser)]
#[grammar = "./grammars/fmod_studio.pest"]
struct FmodStudioParser;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Header {
    pub functions: Vec<Function>,
}

pub fn parse(source: &str) -> Result<Header, Error> {
    let declarations = FmodStudioParser::parse(Rule::api, source)?
        .next()
        .ok_or(Error::FileMalformed)?;

    let arrays = vec![String::from("arguments")];
    let converter = JsonConverter::new(arrays);

    let mut header = Header::default();
    for declaration in declarations.into_inner() {
        match declaration.as_rule() {
            Rule::Function => header.functions.push(converter.convert(declaration)?),
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
