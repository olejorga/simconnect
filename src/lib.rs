#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::mem::transmute_copy;
use std::ptr;
pub struct Name(pub String);
pub struct Unit(pub String);

pub struct Variable {
    pub name: Name,
    pub unit: Unit,
    pub value: f64,
}

pub struct Event {
    pub name: Name,
    pub unit: Unit,
    pub value: f64,
}

pub enum Message<'a> {
    Open,
    Quit,
    Exception(String),
    Variable(&'a Variable),
}

pub struct Client {
    client: HANDLE,
    variables: Vec<Variable>
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: std::ptr::null_mut(),
            variables: Vec::new()
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, name: &str) -> Result<(), ()> {
        unsafe {
            let client_name = CString::new(name).unwrap();

            let result = SimConnect_Open(
                &mut self.client,
                client_name.as_ptr(),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                0,
            );

            if result == 0 && !self.client.is_null() {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    pub fn close(&self) -> Result<(), ()> {
        unsafe {
            let result = SimConnect_Close(self.client);

            if result == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    pub fn receive(&mut self) -> Option<Message> {
        let mut buffer: *mut SIMCONNECT_RECV = ptr::null_mut();
        let mut buffer_size: DWORD = 32;
        let buffer_size_ptr: *mut DWORD = &mut buffer_size;

        unsafe {
            let result = SimConnect_GetNextDispatch(
                self.client,
                &mut buffer,
                buffer_size_ptr,
            );
                        
            if result != 0 {
                return None;
            }

            return match (*buffer).dwID as SIMCONNECT_RECV_ID {
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => {
                    // let data: &SIMCONNECT_RECV_OPEN = transmute_copy(&(buffer as *const SIMCONNECT_RECV_OPEN));
                    Some(Message::Open)
                },
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => {
                    // let data: &SIMCONNECT_RECV_QUIT = transmute_copy(&(buffer as *const SIMCONNECT_RECV_QUIT));
                    Some(Message::Quit)
                },
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => {
                    let data: &SIMCONNECT_RECV_EXCEPTION = transmute_copy(&(buffer as *const SIMCONNECT_RECV_EXCEPTION));
                    let code = data.dwException;

                    Some(Message::Exception(code.to_string()))
                },
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                    let data: &SIMCONNECT_RECV_SIMOBJECT_DATA = transmute_copy(&(buffer as *const SIMCONNECT_RECV_SIMOBJECT_DATA));
                    let value_ptr = std::ptr::addr_of!(data.dwData) as *const f64;
                    let value = std::ptr::read_unaligned(value_ptr);
                    let id = data.dwDefineID;
                    let variable = &mut self.variables[id as usize];

                    variable.value = value;

                    Some(Message::Variable(&*variable))
                },
                _ => None
            };
        }
    }

    pub fn observe(&mut self, variable: Variable) -> Result<(), ()> {
        let name = CString::new(variable.name.0.clone()).unwrap();
        let unit = CString::new(variable.unit.0.clone()).unwrap();

        self.variables.push(variable);

        let id = self.variables.len() - 1;

        unsafe {
            let mut result: i64 = 0;

            result += SimConnect_AddToDataDefinition(
                self.client,
                id.try_into().unwrap(),
                name.as_ptr(),
                unit.as_ptr(),
                SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
                0.0,
                0,
            ) as i64;

            result += SimConnect_RequestDataOnSimObject(
                self.client,
                0,
                id.try_into().unwrap(),
                0,
                SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME,
                0,
                0,
                0,
                0,
            ) as i64;

            if result == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    pub fn transmit(&self, event: &Event) -> Result<(), ()> {
        let name = CString::new(event.name.0.clone()).unwrap();

        unsafe {
            let mut result: i64 = 0;

            result += SimConnect_MapClientEventToSimEvent(
                self.client,
                0,
                name.as_ptr(),
            ) as i64;

            result += SimConnect_TransmitClientEvent(
                self.client,
                0,
                0,
                event.value as u32,
                0,
                0,
            ) as i64;

            if result == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    // pub fn set(&self, variable: Variable) -> Result<(), ()> {
    //     unsafe {
    //         let result = SimConnect_SetDataOnSimObject(
    //             self.client,
    //             define_id,
    //             object_id,
    //             flags,
    //             array_count,
    //             size,
    //             pntr,
    //         );

    //         if result == 0 {
    //             Ok(())
    //         } else {
    //             Err(())
    //         }
    //     }
    // }
}
