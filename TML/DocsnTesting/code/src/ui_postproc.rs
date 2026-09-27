use crate::{ui_preproc::UiLoad};

use std::sync::{OnceLock,Mutex};

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

pub fn get_payload(opcode: MergeOpcode) -> ULWithContentLabel{

    let mut tx_ul = UiLoad::default();

    with_rup(|rup|{
            match opcode.load_type{
        0 => {
            tx_ul.delta_map = rup.delta_map.clone();
        },
        _ => {}
    }
    });
    
    ULWithContentLabel{
        ui_load: tx_ul,
        content_label: opcode.load_type
    }
}
#[derive(Default,Clone)]
pub struct MergeOpcode{
    pub load_type: u8,
    pub indicator0: u64,
    pub indicator1: u64,
    pub indicator2: u64,
}
#[derive(Default,Clone)]
pub struct ULWithContentLabel{
    ui_load: UiLoad,
    content_label: u8
}
//How the merge-opcode works is that an initialized version of it is sent from the UI thread to this thread whose only purpose is to give information to said UI thread. The following is an explanation of how the merge-opcode works:
//
//The load_type is a u8 that indicates what type of data is being sent. The following indicators determine what scope of that data is sent, what I have in mind right now is that indicator0 determines the starting point of the data-
//interval and indicator1 the end point of the data-interval, what indicator 2 is to determine I'm not too sure of yet but it'll probably have something to do with the verbosity or type of data being sent. This is the starting-
//arithmetic or whatever, below I elaborate furhter on the developing specifics.
//
//load_type
//0 = delta_map
//
//For the first load-type, the three indicators do nothing and the entire BTreeMap is sent over by the TX