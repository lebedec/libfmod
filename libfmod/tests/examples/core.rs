use std::os::raw::{c_char, c_float, c_int};
use std::ptr::null_mut;

use libfmod::ffi::{
    FMOD_DSP_PARAMETER_DESC_FLOAT, FMOD_DSP_PARAMETER_DESC_UNION, FMOD_DSP_STATE, FMOD_INIT_NORMAL,
    FMOD_LOOP_NORMAL, FMOD_OK, FMOD_RESULT,
};
use libfmod::{DspDescription, DspParameterDesc, DspParameterType, Error, System};

#[test]
fn test_dsp_custom() -> Result<(), Error> {
    let system = System::create()?;
    system.init(32, FMOD_INIT_NORMAL, None)?;

    let sound = system.create_sound("./tests/data/Assets/1.ogg", FMOD_LOOP_NORMAL, None)?;
    system.play_sound(sound, None, false)?;

    let volume_desc = DspParameterDesc {
        type_: DspParameterType::Float,
        name: name16("volume"),
        label: name16("%"),
        description: "linear volume in percent".to_string(),
        union: FMOD_DSP_PARAMETER_DESC_UNION {
            floatdesc: FMOD_DSP_PARAMETER_DESC_FLOAT {
                min: 0.0,
                max: 1.0,
                defaultval: 1.0,
                mapping: Default::default(),
            },
        },
    };

    struct MyDspData {
        volume: f32,
    }

    unsafe extern "C" fn create_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
        let data = Box::new(MyDspData { volume: 1.0 });
        (*dsp_state).plugindata = Box::into_raw(data) as *mut MyDspData as *mut _;
        FMOD_OK
    }

    unsafe extern "C" fn set_parameter_float_callback(
        dsp_state: *mut FMOD_DSP_STATE,
        _index: c_int,
        value: c_float,
    ) -> FMOD_RESULT {
        let data = (*dsp_state).plugindata as *mut MyDspData;
        (*data).volume = value;
        FMOD_OK
    }

    unsafe extern "C" fn get_parameter_float_callback(
        dsp_state: *mut FMOD_DSP_STATE,
        _index: c_int,
        value: *mut c_float,
        _valuestr: *mut c_char,
    ) -> FMOD_RESULT {
        let data = (*dsp_state).plugindata as *mut MyDspData;
        value.write((*data).volume);
        FMOD_OK
    }

    let dspdesc = DspDescription {
        pluginsdkversion: 0,
        name: name32("My first DSP unit"),
        version: 0x00010000,
        numinputbuffers: 1,
        numoutputbuffers: 1,
        create: Some(create_callback),
        release: None,
        reset: None,
        read: None,
        process: None,
        setposition: None,
        paramdesc: vec![volume_desc],
        setparameterfloat: Some(set_parameter_float_callback),
        setparameterint: None,
        setparameterbool: None,
        setparameterdata: None,
        getparameterfloat: Some(get_parameter_float_callback),
        getparameterint: None,
        getparameterbool: None,
        getparameterdata: None,
        shouldiprocess: None,
        userdata: null_mut(),
        sys_register: None,
        sys_deregister: None,
        sys_mix: None,
    };

    let mydsp = system.create_dsp(dspdesc)?;
    let mastergroup = system.get_master_channel_group()?;
    mastergroup.add_dsp(0, mydsp)?;

    for step in 0..5 {
        match step {
            1 => {
                mydsp.set_bypass(true)?;
            }
            2 => {
                mydsp.set_bypass(false)?;
            }
            3 => {
                mydsp.set_parameter_float(0, 0.25)?;
            }
            4 => {
                let (value, _) = mydsp.get_parameter_float(0, 0)?;
                println!("volume: {}", value);
            }
            _ => {}
        }
    }
    let info = mydsp.get_parameter_info(0)?;
    println!("default: {}", unsafe { info.union.floatdesc.defaultval });

    system.release()
}

fn name16(name: &str) -> [i8; 16] {
    let mut output = [0; 16];
    for (i, ch) in name.as_bytes().iter().enumerate() {
        output[i] = *ch as i8;
    }
    output
}

fn name32(name: &str) -> [i8; 32] {
    let mut output = [0; 32];
    for (i, ch) in name.as_bytes().iter().enumerate() {
        output[i] = *ch as i8;
    }
    output
}
