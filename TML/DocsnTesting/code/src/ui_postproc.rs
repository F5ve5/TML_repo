use crate::{biner::EventFinal, ui_preproc::UiLoad};

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

pub fn get_payload(opcode: MergeOpcode) -> UiPayload{

    let mut tx_up: UiPayload = UiPayload::I32(0);

    with_rup(|rup|{
        match opcode.load_type{
            0 => {
                UiPayload::delta_map(rup.length_map.clone())
            },
            1 => {
                UiPayload::I32(rup.initial_event_amount.clone())
            }
            _ => {
                UiPayload::null(0)
            }
        }
    });
}
#[derive(Default,Clone)]
pub struct MergeOpcode{
    pub load_type: u8,
    pub indicator0: u64,
    pub indicator1: u64,
    pub indicator2: u64,
}
#[repr(u8)]
pub enum UiPayload{
    I32(i32) = 0,
    events(Vec<EventFinal>) = 1,
    delta_map(BTreeMap<u64,i32>) = 2,
    length_map(BTreeMap<u64,i32>) = 3,
    null(u8) = 255
}
//How the merge-opcode works is that an initialized version of it is sent from the UI thread to this thread whose only purpose is to give information to said UI thread. The following is an explanation of how the merge-opcode works:
//
//The load_type is a u8 that indicates what type of data is being sent. The following indicators determine what scope of that data is sent, what I have in mind right now is that indicator0 determines the starting point of the data-
//interval and indicator1 the end point of the data-interval, what indicator 2 is to determine I'm not too sure of yet but it'll probably have something to do with the verbosity or type of data being sent. This is the starting-
//arithmetic or whatever, below I elaborate furhter on the developing specifics.
//
//load_type
//0 = delta_map
//1 = initial_event_amount
//For the first load-type, the three indicators do nothing and the entire BTreeMap is sent over by the TX