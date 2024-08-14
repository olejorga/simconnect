#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::mem::transmute_copy;
use std::ptr;

#[derive(Debug)]
pub enum DispatchResult<'a> {
    Null,
    Exception(&'a SIMCONNECT_RECV_EXCEPTION),
    Open(&'a SIMCONNECT_RECV_OPEN),
    Quit(&'a SIMCONNECT_RECV_QUIT),
    Event(&'a SIMCONNECT_RECV_EVENT),
    EventObjectAddRemove(&'a SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE),
    EventFilename(&'a SIMCONNECT_RECV_EVENT_FILENAME),
    EventFrame(&'a SIMCONNECT_RECV_EVENT_FRAME),
    SimObjectData(&'a SIMCONNECT_RECV_SIMOBJECT_DATA),
    SimObjectDataByType(&'a SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE),
    WeatherObservation(&'a SIMCONNECT_RECV_WEATHER_OBSERVATION),
    CloudState(&'a SIMCONNECT_RECV_CLOUD_STATE),
    AssignedObjectId(&'a SIMCONNECT_RECV_ASSIGNED_OBJECT_ID),
    ReservedKey(&'a SIMCONNECT_RECV_RESERVED_KEY),
    CustomAction(&'a SIMCONNECT_RECV_CUSTOM_ACTION),
    SystemState(&'a SIMCONNECT_RECV_SYSTEM_STATE),
    ClientData(&'a SIMCONNECT_RECV_CLIENT_DATA),
    EventWeatherMode(&'a SIMCONNECT_RECV_EVENT_WEATHER_MODE),
    AirportList(&'a SIMCONNECT_RECV_AIRPORT_LIST),
    VorList(&'a SIMCONNECT_RECV_VOR_LIST),
    NdbList(&'a SIMCONNECT_RECV_NDB_LIST),
    WaypointList(&'a SIMCONNECT_RECV_WAYPOINT_LIST),
    EventMultiplayerServerStarted(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED),
    EventMultiplayerClientStarted(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED),
    EventMultiplayerSessionEnded(&'a SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED),
    EventRaceEnd(&'a SIMCONNECT_RECV_EVENT_RACE_END),
    EventRaceLap(&'a SIMCONNECT_RECV_EVENT_RACE_LAP),
}

#[derive(Debug)]
pub struct Client {
    sim_connect_handle: HANDLE,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            sim_connect_handle: std::ptr::null_mut(),
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, program_name: &str) -> bool {
        unsafe {
            let temp_1 = ptr::null_mut();
            let temp_2 = ptr::null_mut();

            let program_name = CString::new(program_name).unwrap();

            SimConnect_Open(
                &mut self.sim_connect_handle,
                program_name.as_ptr(),
                temp_1,
                0,
                temp_2,
                0,
            );

            !self.sim_connect_handle.is_null()
        }
    }

    pub fn close(&self) -> bool {
        unsafe { SimConnect_Close(self.sim_connect_handle) == 0 }
    }

    pub fn dispatch(&self) -> Result<DispatchResult, &str> {
        let mut data_buf: *mut SIMCONNECT_RECV = ptr::null_mut();

        let mut size_buf: DWORD = 32;
        let size_buf_pointer: *mut DWORD = &mut size_buf;

        unsafe {
            let result = SimConnect_GetNextDispatch(
                self.sim_connect_handle,
                &mut data_buf,
                size_buf_pointer,
            );
            if result != 0 {
                return Err("Failed getting data!");
            }

            return match (*data_buf).dwID as SIMCONNECT_RECV_ID {
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NULL => Ok(DispatchResult::Null),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => Ok(DispatchResult::Exception(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_EXCEPTION)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => Ok(DispatchResult::Open(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_OPEN)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_QUIT => Ok(DispatchResult::Quit(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_QUIT)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => Ok(DispatchResult::Event(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_EVENT)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_OBJECT_ADDREMOVE => {
                    Ok(DispatchResult::EventObjectAddRemove(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FILENAME => {
                    Ok(DispatchResult::EventFilename(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_FILENAME),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_FRAME => {
                    Ok(DispatchResult::EventFrame(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_FRAME),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA => {
                    Ok(DispatchResult::SimObjectData(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_SIMOBJECT_DATA),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SIMOBJECT_DATA_BYTYPE => {
                    Ok(DispatchResult::SimObjectDataByType(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WEATHER_OBSERVATION => {
                    Ok(DispatchResult::WeatherObservation(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_WEATHER_OBSERVATION),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLOUD_STATE => {
                    Ok(DispatchResult::CloudState(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_CLOUD_STATE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_ASSIGNED_OBJECT_ID => {
                    Ok(DispatchResult::AssignedObjectId(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_ASSIGNED_OBJECT_ID),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_RESERVED_KEY => {
                    Ok(DispatchResult::ReservedKey(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_RESERVED_KEY),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CUSTOM_ACTION => {
                    Ok(DispatchResult::CustomAction(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_CUSTOM_ACTION),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_SYSTEM_STATE => {
                    Ok(DispatchResult::SystemState(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_SYSTEM_STATE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_CLIENT_DATA => {
                    Ok(DispatchResult::ClientData(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_CLIENT_DATA),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_WEATHER_MODE => {
                    Ok(DispatchResult::EventWeatherMode(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_WEATHER_MODE),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_AIRPORT_LIST => {
                    Ok(DispatchResult::AirportList(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_AIRPORT_LIST),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_VOR_LIST => Ok(DispatchResult::VorList(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_VOR_LIST)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_NDB_LIST => Ok(DispatchResult::NdbList(
                    transmute_copy(&(data_buf as *const SIMCONNECT_RECV_NDB_LIST)),
                )),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_WAYPOINT_LIST => {
                    Ok(DispatchResult::WaypointList(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_WAYPOINT_LIST),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SERVER_STARTED => Ok(
                    DispatchResult::EventMultiplayerServerStarted(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED),
                    )),
                ),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_CLIENT_STARTED => Ok(
                    DispatchResult::EventMultiplayerClientStarted(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED),
                    )),
                ),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_MULTIPLAYER_SESSION_ENDED => Ok(
                    DispatchResult::EventMultiplayerSessionEnded(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED),
                    )),
                ),
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_END => {
                    Ok(DispatchResult::EventRaceEnd(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_RACE_END),
                    )))
                }
                SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT_RACE_LAP => {
                    Ok(DispatchResult::EventRaceLap(transmute_copy(
                        &(data_buf as *const SIMCONNECT_RECV_EVENT_RACE_LAP),
                    )))
                }
                _ => Err("Unhandled RECV_ID"),
            };
        }
    }

    pub fn call_dispatch(
        &self,
        dispatch_callback: DispatchProc,
        context: *mut std::os::raw::c_void,
    ) -> bool {
        unsafe { SimConnect_CallDispatch(self.sim_connect_handle, dispatch_callback, context) == 0 }
    }

    pub fn get_next_dispatch(
        &self,
        data_buffer: *mut *mut SIMCONNECT_RECV,
        data_buffer_size: *mut DWORD,
    ) -> bool {
        unsafe {
            SimConnect_GetNextDispatch(self.sim_connect_handle, data_buffer, data_buffer_size) == 0
        }
    }

    pub fn request_system_state(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        state: &str,
    ) -> bool {
        let state = CString::new(state).unwrap();

        unsafe {
            SimConnect_RequestSystemState(self.sim_connect_handle, request_id, state.as_ptr()) == 0
        }
    }

    pub fn map_client_event_to_sim_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        event_name: &str,
    ) -> bool {
        let event_name = CString::new(event_name).unwrap();

        unsafe {
            SimConnect_MapClientEventToSimEvent(
                self.sim_connect_handle,
                event_id,
                event_name.as_ptr(),
            ) == 0
        }
    }

    pub fn subscribe_to_system_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        event_name: &str,
    ) -> bool {
        let event_name = CString::new(event_name).unwrap();

        unsafe {
            SimConnect_SubscribeToSystemEvent(
                self.sim_connect_handle,
                event_id,
                event_name.as_ptr(),
            ) == 0
        }
    }

    pub fn unsubscribe_from_system_event(&self, event_id: SIMCONNECT_CLIENT_EVENT_ID) -> bool {
        unsafe { SimConnect_UnsubscribeFromSystemEvent(self.sim_connect_handle, event_id) == 0 }
    }

    pub fn request_data_on_sim_object(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        period: SIMCONNECT_CLIENT_DATA_PERIOD,
        flags: SIMCONNECT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestDataOnSimObject(
                self.sim_connect_handle,
                request_id,
                define_id,
                object_id,
                period,
                flags,
                origin,
                interval,
                limit,
            ) == 0
        }
    }

    pub fn request_data_on_sim_object_type(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        radius_in_meters: DWORD,
        object_type: SIMCONNECT_SIMOBJECT_TYPE,
    ) -> bool {
        unsafe {
            SimConnect_RequestDataOnSimObjectType(
                self.sim_connect_handle,
                request_id,
                define_id,
                radius_in_meters,
                object_type,
            ) == 0
        }
    }

    pub fn transmit_client_event(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        dw_data: DWORD,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        flags: SIMCONNECT_EVENT_FLAG,
    ) -> bool {
        unsafe {
            SimConnect_TransmitClientEvent(
                self.sim_connect_handle,
                object_id,
                event_id,
                dw_data,
                group_id,
                flags,
            ) == 0
        }
    }

    pub fn add_to_data_definition(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        datum_name: &str,
        units_name: &str,
        datum_type: SIMCONNECT_DATATYPE,
        datum_id: DWORD,
        epsilon: f32,
    ) -> bool {
        let datum_name = CString::new(datum_name).unwrap();
        let units_name = CString::new(units_name).unwrap();

        unsafe {
            SimConnect_AddToDataDefinition(
                self.sim_connect_handle,
                define_id,
                datum_name.as_ptr(),
                units_name.as_ptr(),
                datum_type,
                epsilon,
                datum_id,
            ) == 0
        }
    }

    pub unsafe fn set_data_on_sim_object(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        flags: SIMCONNECT_DATA_SET_FLAG,
        array_count: DWORD,
        size: DWORD,
        pntr: *mut ::std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_SetDataOnSimObject(
                self.sim_connect_handle,
                define_id,
                object_id,
                flags,
                array_count,
                size,
                pntr,
            ) == 0
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if !self.sim_connect_handle.is_null() {
            self.close();
        }
    }
}
