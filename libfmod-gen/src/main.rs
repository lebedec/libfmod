#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate quote;

extern crate proc_macro;

#[macro_use]
extern crate pest_derive;

use crate::generators::{ffi, flags, lib};
use crate::models::{Api, Error};
use crate::parsers::{
    fmod, fmod_codec, fmod_common, fmod_docs, fmod_dsp, fmod_dsp_effects, fmod_errors, fmod_output,
    fmod_studio, fmod_studio_common,
};
use std::path::Path;
use std::{env, fs};

mod generators;
mod models;
mod parsers;
mod patching;
mod repr;

fn generate_lib_fmod(source: &str, destination: &str) -> Result<(), Error> {
    let source = Path::new(source);
    if !source.join("api/studio/inc/fmod_studio.h").exists() {
        return Err(Error::Io(
            "FMOD headers not found, make sure input is FMOD SDK \
            directory with api, doc, plugin folders"
                .to_string(),
        ));
    }
    let mut api = Api::default();
    let data = fs::read_to_string(source.join("api/studio/inc/fmod_studio.h"))?;
    let header = fmod_studio::parse(&data)?;
    let link = "fmodstudio".into();
    api.functions.push((link, header.functions.clone()));
    let data = fs::read_to_string(source.join("api/studio/inc/fmod_studio_common.h"))?;
    let header = fmod_studio_common::parse(&data)?;
    api.opaque_types.extend(header.opaque_types);
    api.constants.extend(header.constants);
    api.enumerations.extend(header.enumerations);
    api.callbacks.extend(header.callbacks);
    api.flags.extend(header.flags);
    api.structures.extend(header.structures);

    let data = fs::read_to_string(source.join("api/core/inc/fmod.h"))?;
    let header = fmod::parse(&data)?;
    let link = "fmod".into();
    api.functions.push((link, header.functions.clone()));

    let data = fs::read_to_string(source.join("api/core/inc/fmod_common.h"))?;
    let header = fmod_common::parse(&data)?;
    api.opaque_types.extend(header.opaque_types);
    api.type_aliases.extend(header.type_aliases);
    api.constants.extend(header.constants);
    api.enumerations.extend(header.enumerations);
    api.callbacks.extend(header.callbacks);
    api.flags.extend(header.flags);
    api.structures.extend(header.structures);
    api.presets.extend(header.presets);

    let data = fs::read_to_string(source.join("api/core/inc/fmod_codec.h"))?;
    let header = fmod_codec::parse(&data)?;
    api.opaque_types.extend(header.opaque_types);
    api.constants.extend(header.constants);
    api.callbacks.extend(header.callbacks);
    api.flags.extend(header.flags);
    api.structures.extend(header.structures);

    let data = fs::read_to_string(source.join("api/core/inc/fmod_output.h"))?;
    let header = fmod_output::parse(&data)?;
    api.opaque_types.extend(header.opaque_types);
    api.constants.extend(header.constants);
    api.callbacks.extend(header.callbacks);
    api.flags.extend(header.flags);
    api.structures.extend(header.structures);

    let data = fs::read_to_string(source.join("api/core/inc/fmod_dsp.h"))?;
    let header = fmod_dsp::parse(&data)?;
    api.opaque_types.extend(header.opaque_types);
    api.constants.extend(header.constants);
    api.enumerations.extend(header.enumerations);
    api.callbacks.extend(header.callbacks);
    api.flags.extend(header.flags);
    api.structures.extend(header.structures);

    let data = fs::read_to_string(source.join("api/core/inc/fmod_dsp_effects.h"))?;
    let header = fmod_dsp_effects::parse(&data)?;
    api.constants.extend(header.constants);
    api.enumerations.extend(header.enumerations);
    api.structures.extend(header.structures);

    let data = fs::read_to_string(source.join("api/core/inc/fmod_errors.h"))?;
    let header = fmod_errors::parse(&data)?;
    api.errors = header.mapping.clone();

    api.modifiers = fmod_docs::parse_parameter_modifiers(&[
        source.join("doc/FMOD API User Manual/core-api-system.html"),
        source.join("doc/FMOD API User Manual/core-api-soundgroup.html"),
        source.join("doc/FMOD API User Manual/core-api-sound.html"),
        source.join("doc/FMOD API User Manual/core-api-reverb3d.html"),
        source.join("doc/FMOD API User Manual/core-api-geometry.html"),
        source.join("doc/FMOD API User Manual/core-api-dspconnection.html"),
        source.join("doc/FMOD API User Manual/core-api-dsp.html"),
        source.join("doc/FMOD API User Manual/core-api-channelgroup.html"),
        source.join("doc/FMOD API User Manual/core-api-channelcontrol.html"),
        source.join("doc/FMOD API User Manual/core-api-channel.html"),
        source.join("doc/FMOD API User Manual/core-api-common.html"),
        source.join("doc/FMOD API User Manual/plugin-api-codec.html"),
        source.join("doc/FMOD API User Manual/plugin-api-dsp.html"),
        source.join("doc/FMOD API User Manual/plugin-api-output.html"),
        source.join("doc/FMOD API User Manual/studio-api-bank.html"),
        source.join("doc/FMOD API User Manual/studio-api-bus.html"),
        source.join("doc/FMOD API User Manual/studio-api-commandreplay.html"),
        source.join("doc/FMOD API User Manual/studio-api-common.html"),
        source.join("doc/FMOD API User Manual/studio-api-eventdescription.html"),
        source.join("doc/FMOD API User Manual/studio-api-eventinstance.html"),
        source.join("doc/FMOD API User Manual/studio-api-system.html"),
        source.join("doc/FMOD API User Manual/studio-api-vca.html"),
    ])?;

    println!("FMOD API");
    println!("Opaque Types: {}", api.opaque_types.len());
    println!("Type Aliases: {}", api.type_aliases.len());
    println!(
        "Structures: {} (Fields: {})",
        api.structures.len(),
        api.structures
            .iter()
            .flat_map(|structure| &structure.fields)
            .count()
    );
    println!("Constants: {}", api.constants.len());
    println!(
        "Flags: {} (Options: {})",
        api.flags.len(),
        api.flags.iter().flat_map(|flags| &flags.flags).count()
    );
    println!(
        "Enumerations: {} (Variants: {})",
        api.enumerations.len(),
        api.enumerations
            .iter()
            .flat_map(|enumeration| &enumeration.enumerators)
            .count()
    );
    println!("Callbacks: {}", api.callbacks.len());
    println!(
        "Functions: {} (Arguments: {})",
        api.functions
            .iter()
            .flat_map(|(_, functions)| functions)
            .count(),
        api.functions
            .iter()
            .flat_map(|(_, functions)| functions)
            .flat_map(|function| &function.arguments)
            .count()
    );
    println!("Parameter Modifiers: {}", api.modifiers.len());
    println!("Errors: {}", api.errors.errors.len());

    api.patch_all();

    let destination = Path::new(destination);
    if !destination.join("src/ffi.rs").exists() {
        return Err(Error::Io(
            "src not found, make sure output is libfmod project directory".to_string(),
        ));
    }
    let code = ffi::generate(&api)?;
    fs::write(destination.join("src/ffi.rs"), code)?;
    let code = lib::generate(&api)?;
    fs::write(destination.join("src/lib.rs"), code)?;
    let code = flags::generate_to_file(&api)?;
    fs::write(destination.join("src/flags.rs"), code)?;

    Ok(())
}

const FMOD_SDK_PATH: &str = "C:\\Program Files (x86)\\FMOD SoundSystem\\FMOD Studio API Windows";
const OUTPUT_DIR: &str = "../libfmod";

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = match args.get(1) {
        None => FMOD_SDK_PATH,
        Some(source) => source,
    };
    let destination = match args.get(2) {
        None => OUTPUT_DIR,
        Some(destination) => destination,
    };
    println!("source {} {}", source, destination);
    if let Err(error) = generate_lib_fmod(&source, &destination) {
        println!("Unable to generate libfmod, {:?}", error);
    }
}
