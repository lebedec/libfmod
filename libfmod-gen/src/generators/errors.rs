use crate::models::{Api, Error};

pub fn generate_errors(api: &Api) -> Result<String, Error> {
    // match arms content can't be formatted by Rust fmt properly
    let mut source = String::new();
    source += "\
use crate::ffi;

pub fn map_fmod_error(result: ffi::FMOD_RESULT) -> &'static str {
    match result {
";
    for error in &api.errors.errors {
        source += &format!("\t\tffi::{} => \"{}\",\n", error.name, error.string);
    }
    source += "
        _ => \"Unknown error code\"
    }
}
    ";
    Ok(source)
}

pub fn generate_to_file(api: &Api) -> Result<String, Error> {
    generate_errors(api)
}
