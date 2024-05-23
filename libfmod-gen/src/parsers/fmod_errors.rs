use crate::models::{Error, ErrorStringMapping};
use crate::repr::JsonConverter;
use pest::{error, Parser};

#[derive(Parser)]
#[grammar = "./grammars/fmod_errors.pest"]
struct FmodErrorsParser;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Header {
    pub mapping: ErrorStringMapping,
}

pub fn parse(source: &str) -> Result<Header, Error> {
    let declarations = FmodErrorsParser::parse(Rule::api, source)?
        .next()
        .ok_or(Error::FileMalformed)?;

    let arrays = vec![String::from("errors")];
    let converter = JsonConverter::new(arrays);

    let mut header = Header::default();
    for declaration in declarations.into_inner() {
        match declaration.as_rule() {
            Rule::ErrorStringMapping => header.mapping = converter.convert(declaration)?,
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
