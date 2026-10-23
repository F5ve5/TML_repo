use windows::Win32::Graphics::Dxgi::DXGI_OUTDUPL_POINTER_SHAPE_TYPE;

use crate::{biner::EventFinal, misc::round_filetime_to_nearest_second, ui_preproc::UiLoad};

use std::{{collections::BTreeMap},sync::{OnceLock,Mutex}};

pub static READY_UI_PAYLOAD: OnceLock<Mutex<UiLoad>> = OnceLock::new();
pub fn with_rup<R>(f: impl FnOnce(&mut UiLoad) -> R) -> R {
    let mutex = READY_UI_PAYLOAD.get().unwrap();
    let mut guard = mutex.lock().unwrap();
    f(&mut *guard)
}

pub fn init_globals(){
    READY_UI_PAYLOAD
        .set(Mutex::new(UiLoad::default()))
        .expect("READY_UI_PAYLOAD already initialized");
}

pub fn get_payload(opcode: UPOpcode) -> UiPayload{

    with_rup(|rup|{
        match opcode.xI{
            1 => {
                if opcode.xII == 0 && opcode.xIII == 0{
                    UiPayload::delta_map(rup.delta_map.clone())
                }else{
                    UiPayload::delta_map(
                        rup.delta_map
                            .range(opcode.xII..opcode.xIII)
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect()
                    )
                }
            },
            2 => {
                if opcode.xII == 0 && opcode.xIII == 0{
                    UiPayload::length_map(rup.length_map.clone())
                }else{
                    UiPayload::length_map(
                        rup.length_map
                            .range(opcode.xII..opcode.xIII)
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect()
                    )
                }
            }
            _ => {
                UiPayload::null(())
            }
        }
    });
    UiPayload::null(())
}

#[derive(Default,Clone)]
pub struct UPOpcode{
    pub xI: u8,
    pub xII: u64,
    pub xIII: u64,
}
#[repr(u8)]
pub enum UiPayload{
    invalid(u8) = 0,
    I64(i64) = 1,
    events(Vec<EventFinal>) = 2,
    delta_map(BTreeMap<u64,i32>) = 3,
    length_map(BTreeMap<u64,i32>) = 4,
    null(()) = 255,
}
//How the ui-payload-opcode works is that an initialized version of it is sent from the UI thread to this thread whose only purpose is to give information to said UI thread. The following is an explanation of how the merge-opcode works:
//
//The load_type is a u8 that indicates what type of data is being sent. The following indicators determine what scope of that data is sent, what I have in mind right now is that indicator0 determines the starting point of the data-
//interval and indicator1 the end point of the data-interval, what indicator 2 is to determine I'm not too sure of yet but it'll probably have something to do with the verbosity or type of data being sent. This is the starting-
//arithmetic or whatever, below I elaborate furhter on the developing specifics.
//
//load_type
//1 = delta_map
//2 = initial_event_amount
//For the first load-type, the three indicators do nothing and the entire BTreeMap is sent over by the TX