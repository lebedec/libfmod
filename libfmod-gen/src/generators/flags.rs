use crate::models::{Api, Error, Flag, Flags};

use crate::patching::dictionary::RENAMES;
use convert_case::{Case, Casing};

fn format_flags_struct_ident(key: &str) -> String {
    let key = key.replace("FMOD_STUDIO_SYSTEM_CALLBACK", "STUDIO_SYSTEM_CALLBACK");
    let key = key.replace("FMOD_STUDIO_INIT", "STUDIO_INIT");
    let key = key.replace("FMOD_INIT", "INIT");
    let key = key.replace("FMOD_STUDIO_", "");
    let key = key.replace("FMOD_", "");
    let key = key.replace("FMOD", "MODE");
    let name = key.to_case(Case::Pascal);
    let name = match RENAMES.get(&name[..]) {
        None => name,
        Some(rename) => rename.to_string(),
    };
    format!("{name}")
}

fn get_shared_prefix(flags: &Vec<Flag>) -> String {
    let pattern: Vec<&str> = flags[0].name.split("_").collect();
    let mut words = pattern.len();
    for other in flags {
        let other: Vec<&str> = other.name.split("_").collect();
        for index in 0..words {
            if pattern[index] != other[index] {
                if index < words {
                    words = index;
                }
                break;
            }
        }
    }
    let words: Vec<String> = pattern
        .iter()
        .take(words)
        .map(ToString::to_string)
        .collect();
    words.join("_")
}

pub fn generate_flags(flags: &Flags) -> (String, String) {
    let prefix = get_shared_prefix(&flags.flags);
    let name = format_flags_struct_ident(&prefix);

    let mut variants: Vec<String> = vec![];

    let prefix_replace = format!("{prefix}_");
    for flag in &flags.flags {
        let mut name = flag.name.to_string();
        if name.starts_with("FMOD_CHANNELMASK_5POINT1") {
            name = name.replace("FMOD_CHANNELMASK_5POINT1", "FMOD_CHANNELMASK_MASK_5POINT1")
        }
        if name.starts_with("FMOD_CHANNELMASK_7POINT") {
            name = name.replace("FMOD_CHANNELMASK_7POINT", "FMOD_CHANNELMASK_MASK_7POINT")
        }

        name = name.replace(&prefix_replace, "");
        if name.starts_with("3D_") {
            name = name.replace("3D_", "");
            name += "_3D";
        }
        if name == "2D" {
            name = "FMOD_2D".to_string();
        }
        if name == "3D" {
            name = "FMOD_3D".to_string();
        }
        let ffi = &flag.name;
        variants.push(format!("        const {name} = ffi::{ffi};"));
    }

    let flags = &flags.name;
    let variants = variants.join("\n");
    let definition = format!(
        r#"
    pub struct {name}: ffi::{flags} {{
{variants}
    }}
    "#
    );
    let into = format!(
        r#"
impl Into<ffi::{flags}> for {name} {{
    fn into(self) -> ffi::{flags} {{
        self.bits
    }}
}}
    "#
    );
    (definition, into)
}

pub fn generate_flags_code(api: &Api) -> Result<String, Error> {
    // marcos content can't be formatted by Rust fmt
    // so, we use string building for flags generation
    let mut source = String::new();
    source += "\
use bitflags::bitflags;
use crate::ffi;

bitflags! {
    ";

    let mut into_traits = String::new();

    for flag in api.flags.iter() {
        let (code, into) = generate_flags(flag);
        source += &code;
        into_traits += &into;
    }

    source += "\n}\n";
    source += &into_traits;

    Ok(source)
}

pub fn generate_to_file(api: &Api) -> Result<String, Error> {
    generate_flags_code(api)
}
