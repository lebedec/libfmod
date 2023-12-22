use std::env;

fn main() {
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let core = match &*target_family {
        "windows" => "fmod_vc",
        "unix" => "fmod",
        _ => "libfmod",
    };
    let studio = match &*target_family {
        "windows" => "fmodstudio_vc",
        "unix" => "fmodstudio",
        _ => "libfmodstudio",
    };
    println!("cargo:rustc-flags=-l {core}");
    println!("cargo:rustc-flags=-l {studio}");
}
