use windows::Win32::Graphics::Dxgi::DXGI_OUTDUPL_POINTER_SHAPE_TYPE;

use crate::{biner::EventFinal, misc::round_filetime_to_nearest_second, ui_preproc::UiLoad};

use std::{{collections::BTreeMap},sync::{OnceLock,Mutex}};

pub static READY_UI_PAYLOAD: OnceLock<Mutex<UiLoad>> = OnceLock::new();
pub fn with_rup<R>(f: impl FnOnce(&mut UiLoad) -> R) -> R {
    let mutex = READY_UI_PAYLOAD.get().unwrap();
    let mut guard = mutex.lock().unwrap();
    f(&mut *guard)
}
 
pub fn init_globals(ui_tx:std::sync::mpsc::Sender<UiPayload>){
    READY_UI_PAYLOAD
        .set(Mutex::new(UiLoad::default()))
        .expect("READY_UI_PAYLOAD already initialized");
    
    with_rup(|rup|rup.ui_tx = Some(ui_tx))
}

pub fn handle_payload(opcode: UPOpcode,preproc_tx: &std::sync::mpsc::Sender<UPOpcode>){

    with_rup(|rup|{
        let up = match opcode.xI{
            3 => {
                preproc_tx.send(opcode).expect("Postprocessing TX failed");
                UiPayload::loading(3)
            },
            4 => {
                preproc_tx.send(opcode).expect("Postprocessing TX failed");
                UiPayload::loading(4)
            }
            5 => {
                let lmc = with_rup(|rup|rup.length_map_c.clone());
                with_rup(|rup|rup.length_map_c.clear());
                UiPayload::length_map_c(lmc)
            }
            6 => {
                let dmc = with_rup(|rup|rup.delta_map_c.clone());
                with_rup(|rup|rup.delta_map_c.clear());
                UiPayload::delta_map_c(dmc)
            }
            _ => {
                UiPayload::null(())
            }
        };
        rup.ui_tx.as_ref().unwrap().send(up).expect("Postprocessing TX failed");
    });
}

#[derive(Default,Clone)]
pub struct UPOpcode{
    pub xI: u8,
    pub xII: u64,
    pub xIII: u64,
}
#[derive(Clone)]
#[repr(u8)]
pub enum UiPayload{
    loading(u8) = 0,
    iea(i64) = 1,
    events(Vec<EventFinal>) = 2,
    delta_map(BTreeMap<u64,i32>) = 3,
    length_map(BTreeMap<u64,i32>) = 4,
    delta_map_c(BTreeMap<u64,i32>) = 5,
    length_map_c(BTreeMap<u64,i32>) = 6,
    null(()) = 255,
}
//As you can't actually cast the enum variant as u8 which makes the whole #[repr(u8)] completely useless here(?)
impl UiPayload {
    pub fn tag(&self) -> u8 {
        match self {
            Self::loading(_) => 0,
            Self::iea(_) => 1,
            Self::events(_) => 2,
            Self::delta_map(_) => 3,
            Self::length_map(_) => 4,
            Self::delta_map_c(_) => 5,
            Self::length_map_c(_) => 6,
            Self::null(_) => 255,
        }
    }
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