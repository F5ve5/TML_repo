use std::{cell::{RefCell}, collections::{BTreeMap, HashMap}, hash::Hash};
use ui_postproc::UiPayload;


use crate::{biner::EventFinal, misc::round_filetime_to_nearest_second, ui_postproc::{self, UiPayload::length_map, with_rup}, ui_preproc};

#[derive(Default, Debug, Clone)]
pub struct UiLoad{
    //Option<> just so I can default the struct
    pub ui_tx: Option<std::sync::mpsc::Sender<UiPayload>>,


    pub lime_flagged: bool,

    
    pub initial_event_amount: i32,


    pub delta_map: BTreeMap<u64,i32>,
    pub delta_map_c: BTreeMap<u64,i32>,


    pub events_by_prid: Vec<EventFinal>,


    pub length_map: BTreeMap<u64,i32>,
    pub length_map_c: BTreeMap<u64,i32>,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct PidPlusOpcode{
    pid: Option<u32>,
    opcode: Option<u8>
}

thread_local! {
    static LOCAL_UI_PAYLOAD: RefCell<UiLoad> = RefCell::new(UiLoad::default());
}
#[inline]
fn with_lup<R>(f: impl FnOnce(&mut UiLoad) -> R) -> R {
    LOCAL_UI_PAYLOAD.with(|cell| {
        let mut lup = cell.borrow_mut();
        f(&mut *lup)
    })
}

pub fn init_globals(ui_tx:std::sync::mpsc::Sender<ui_postproc::UiPayload>){
    LOCAL_UI_PAYLOAD.with(|cell|{
        let mut lup = cell.borrow_mut();
        *lup = UiLoad::default();
        lup.initial_event_amount = -1;
        lup.ui_tx = Some(ui_tx);
        lup.lime_flagged = false;
    });
}

pub fn process_snapshot(events: Vec<EventFinal>) {
    with_lup(|lup| {
        let iea = events.len() as i32;

        lup.initial_event_amount = iea;
        *lup.length_map.entry(0).or_default() = iea;

        lup.ui_tx.as_ref().unwrap().send(UiPayload::iea(iea)).expect("TX30 failed");
    });
}
#[inline]
pub fn process_etw(e: EventFinal) {

    with_lup(|lup| {
        update_lup(lup,e);
    });
}

#[inline]
fn update_lup(closure_lup: &mut UiLoad,new_event:EventFinal){
    let mut delta: i32;
    if new_event.opcode == Some(1){
        delta = 1;
    }else if new_event.opcode == Some(2){
        delta = -1;
    }else{
        return
    }

    let mut current_entry = round_filetime_to_nearest_second(&new_event.timestamp.unwrap_or(0));
    *closure_lup.delta_map.entry(current_entry).or_default() += delta;
    *closure_lup.delta_map_c.entry(current_entry).or_default() += delta;

    //Update the length map as far in time as the delta map has progressed
    if let Some(last_event) = closure_lup.delta_map.last_key_value(){

        //Continue length map progression depending on behind value
        //There is always the default (iea) at the absolute beginning of the length map which is removed before merging to ui
        let x = closure_lup.length_map.range(..current_entry).next_back();
        let lm_tip = x.unwrap().1.clone();

        while current_entry <= *last_event.0{

            *closure_lup.length_map.entry(current_entry).or_insert(lm_tip) += delta;
            *closure_lup.length_map_c.entry(current_entry).or_insert(lm_tip) += delta;


            current_entry += 10_000_000;
        }
    }
}

pub fn get_backend_payload(upo: ui_postproc::UPOpcode){
    let up = match upo.xI{
        3 => {
            ui_postproc::UiPayload::delta_map(with_lup(|lup|lup.delta_map.clone()))
        }
        4 => {
            ui_postproc::UiPayload::length_map(with_lup(|lup|{
                let mut lm_clone = lup.length_map.clone();
                lm_clone.pop_first();
                return lm_clone
            }))
        }
        _ => {
            ui_postproc::UiPayload::null(())
        }
    };
    with_lup(|lup|{
        lup.ui_tx.as_ref().unwrap().send(up).expect("Preprocessing TX failed");
    });
}

pub fn mini_merge(){
    with_lup(|lup|{
        if  !lup.lime_flagged && !lup.length_map_c.is_empty(){

            let mut lm_clone = lup.length_map.clone();
            lm_clone.pop_first();
            
            lup.ui_tx.as_ref().unwrap().send(ui_postproc::UiPayload::length_map(
                lm_clone
            )).expect("Preprocessing TX failed");

            lup.lime_flagged = true;
        }
    });

    let lup_mini_clone = with_lup(|lup|{
        UiLoad{
            delta_map_c: lup.delta_map_c.clone(),
            length_map_c: lup.length_map_c.clone(),
            ..Default::default()
        }
    });

    with_rup(|rup|{
        rup.delta_map_c = lup_mini_clone.delta_map_c;
        rup.length_map_c = lup_mini_clone.length_map_c;
    });
}