# libFMOD ![Crates.io](https://img.shields.io/crates/v/libfmod)

A library wrapper for integrating FMOD Engine in Rust applications. 
FFI wrapped in Rust code to make them safe, more idiomatic 
and abstract away uncomfortable manual C interface using.

#### Installation

A crate uses FMOD development libraries version number to simplify version match.
Backwards compatible changes and bug fixes in libFMOD encoded in patch part.
For example for FMOD Engine 2.02.06 you should use:

```toml
[dependencies]
libfmod = "2.2.601"
```

#### FMOD Development Libraries

FMOD development libraries can't be integrated and distributed as part of this crate. 
You should download and install it considering your current licensing option from:
https://www.fmod.com/download

**Windows**

You should manually provide FMOD development libraries for MSVC linker. 
For the first time you can just put following files to `.\target\debug\deps`
from default FMOD Engine installation folder `C:\Program Files (x86)\FMOD SoundSystem\FMOD Studio API Windows\`:

```bash
api\core\lib\x64\fmod.dll
api\core\lib\x64\fmod_vc.lib
api\studio\lib\x64\fmodstudio.dll
api\studio\lib\x64\fmodstudio_vc.lib
```

And then rename `fmod_vc.lib` and `fmodstudio_vc.lib` to `fmod.lib` and `fmodstudio.lib` accordingly.

#### Getting Started

The simplest way to get started is to initialize the FMOD system, load a sound, and play it.
Playing a sound does not block the application, all functions execute immediately, so we should poll for the sound to finish.

```rust
use libfmod::{Error, System};
use libfmod::ffi::{FMOD_DEFAULT, FMOD_INIT_NORMAL};

fn test_playing_sound() -> Result<(), Error> {
    let system = System::create()?;
    system.init(512, FMOD_INIT_NORMAL, None)?;
    let sound = system.create_sound("./data/heartbeat.ogg", FMOD_DEFAULT, None)?;
    let channel = system.play_sound(sound, None, false)?;
    while channel.is_playing()? {
        // do something else
    }
    system.release()
}
```

See more examples in [tests](tests) folder.

#### Contributing

This library is automatically generated by [libfmod-gen](https://github.com/lebedev-games/libfmod-gen) 
and can't be changed manually. All issues and pull requests must be created in repository of generator. 
