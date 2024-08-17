use simconnect::{
    SimConnect_AddToDataDefinition, SimConnect_GetNextDispatch, SimConnect_Open,
    SimConnect_RequestDataOnSimObject, DWORD, HANDLE,
    SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME,
    SIMCONNECT_RECV, SIMCONNECT_RECV_ID, SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA,
    SIMCONNECT_RECV_SIMOBJECT_DATA,
};

use std::{ffi::CString, mem::transmute_copy, ptr, thread, time::Duration};

struct Variable {
    name: &'static str,
    unit: &'static str,
}

#[derive(Debug)]
struct Values {
    speed: f64,
    altitude: f64,
    longitude: f64,
    latitude: f64
}

const VARIABLES: [Variable; 4] = [
    Variable {
        name: "AIRSPEED INDICATED",
        unit: "Knots",
    },
    Variable {
        name: "PLANE ALTITUDE",
        unit: "Feet",
    },
    Variable {
        name: "PLANE LATITUDE",
        unit: "Degrees",
    },
    Variable {
        name: "PLANE LONGITUDE",
        unit: "Degrees",
    },
];

fn main() {
    let mut client: HANDLE = ptr::null_mut();
    let name: CString = CString::new("DEMO").unwrap();

    unsafe {
        if SimConnect_Open(
            &mut client,
            name.as_ptr(),
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            0,
        ) != 0
        {
            panic!("FAILED TO OPEN");
        }
    }

    for (index, variable) in VARIABLES.iter().enumerate() {
        let name: CString = CString::new(variable.name).unwrap();
        let unit: CString = CString::new(variable.unit).unwrap();

        unsafe {
            if SimConnect_AddToDataDefinition(
                client,
                0,
                name.as_ptr(),
                unit.as_ptr(),
                SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
                0.0,
                index as u32,
            ) != 0
            {
                panic!("FAILED TO ADD DATA DEFINITION");
            }
        }
    }

    unsafe {
        if SimConnect_RequestDataOnSimObject(
            client,
            0,
            0,
            0,
            SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME,
            0,
            0,
            0,
            0,
        ) != 0
        {
            panic!("FAILED TO REQUEST DATA ON SIM OBJECT");
        }
    }

    loop {
        let mut buffer: *mut SIMCONNECT_RECV = ptr::null_mut();
        let mut buffer_size: DWORD = 32;
        let buffer_size_ptr: *mut DWORD = &mut buffer_size;

        unsafe {
            if SimConnect_GetNextDispatch(client, &mut buffer, buffer_size_ptr) != 0 {
                continue;
            }

            match (*buffer).dwID as SIMCONNECT_RECV_ID {
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                    let data: &SIMCONNECT_RECV_SIMOBJECT_DATA =
                        transmute_copy(&(buffer as *const SIMCONNECT_RECV_SIMOBJECT_DATA));
                    let values_ptr = std::ptr::addr_of!(data.dwData) as *const Values;
                    let values = std::ptr::read_unaligned(values_ptr);

                    println!("{:?}", values);
                }
                _ => continue,
            }

            thread::sleep(Duration::from_millis(1));
        }
    }
}
