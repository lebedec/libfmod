use std::env;

fn main() {
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let core = match target_family.as_str() {
        "windows" => "fmod_vc",
        "unix" => "fmod",
        _ => "libfmod",
    };
    let studio = match target_family.as_str() {
        "windows" => "fmodstudio_vc",
        "unix" => "fmodstudio",
        _ => "libfmodstudio",
    };
    let mut core = core.to_string();
    let mut studio = studio.to_string();
    if env::var("CARGO_FEATURE_DEBUG").is_ok() {
        core += "L";
        studio += "L";
    }
    println!("cargo:rustc-flags=-l {core}");
    println!("cargo:rustc-flags=-l {studio}");
}
