use std::env;

fn main() {
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let logging = env::var("CARGO_FEATURE_LOGGING").is_ok();
    let [core, studio] = match (logging, target_family.as_str()) {
        (false, "windows") => ["fmod_vc", "fmodstudio_vc"],
        (true, "windows") => ["fmodL_vc", "fmodstudioL_vc"],
        (true, _) => ["fmodL", "fmodstudioL"],
        _ => ["fmod", "fmodstudio"]
    };
    println!("cargo:rustc-flags=-l {core}");
    println!("cargo:rustc-flags=-l {studio}");
}
