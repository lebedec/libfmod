# libFMOD

A library wrapper for integrating FMOD Engine in Rust applications. 
FFI wrapped in Rust code to make them safe, more idiomatic 
and abstract away uncomfortable manual C interface using.

#### FMOD Development Libraries

FMOD development libraries can't be integrated and distributed as part of this crate. 
You should download and install it considering your current licensing option from:
https://www.fmod.com/download

#### Usage Example (wip)

Adaptation of official simple event example for FMOD v2.02.03 (all banks included in FMOD Studio application):

```rust
fn example() -> Result<(), Error> {
    let studio = Studio::create()?;
    let system = studio.get_core_system()?;
    
    system.set_software_format(0, 6, 0)?;
    studio.initialize(1024, 0, 0, null_mut())?;
    
    let master = studio.load_bank_file("./assets/Master.bank", 0)?;
    let strings = studio.load_bank_file("./assets/Master.strings.bank", 0)?;
    let sfx = studio.load_bank_file("./assets/SFX.bank", 0)?;

    let looping_ambience_description = studio.get_event("event:/Ambience/Country")?;
    let looping_ambience = looping_ambience_description.create_instance()?;
    looping_ambience.start()?;

    while studio.update().is_ok() {}

    studio.release()?;
}


```

#### Contributing

This library is automatically generated by [libfmod-gen](https://github.com/lebedev-games/libfmod-gen) 
and can't be changed manually. All issues and pull requests must be created in repository of generator. 
