use std::{cell::{RefCell}, collections::{BTreeMap, HashMap}, hash::Hash};
use ui_postproc::UiPayload;


use crate::{biner::EventFinal, misc::round_filetime_to_nearest_second, ui_postproc::{self, UiPayload::length_map, with_rup}, ui_preproc};

#[derive(Default, Debug, Clone)]
pub struct UiLoad{
    //Just so I can default the struct
    pub ui_tx: Option<std::sync::mpsc::Sender<UiPayload>>,

    
    pub initial_event_amount: i64,


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
        *lup.length_map.entry(0).or_default() = 0;
        lup.ui_tx = Some(ui_tx);
    });
}

pub fn process_snapshot(events: Vec<EventFinal>) {
    with_lup(|lup| {
        let iea = events.len() as i64;

        lup.initial_event_amount = iea;
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
    let mut current_entry = round_filetime_to_nearest_second(&new_event.timestamp.unwrap_or(0));
    if new_event.opcode == Some(1){
        *closure_lup.delta_map.entry(current_entry).or_default() += 1;
        *closure_lup.delta_map_c.entry(current_entry).or_default() += -1;
    }else if new_event.opcode == Some(2){
        *closure_lup.delta_map.entry(current_entry).or_default() += -1;
        *closure_lup.delta_map_c.entry(current_entry).or_default() += -1;
    }

    if let Some(last_event) = closure_lup.delta_map.last_key_value(){
        let x = closure_lup.length_map.range(..current_entry).next_back();
        let lm_tip = *x.unwrap().1;

        while current_entry <= *last_event.0{
            if new_event.opcode == Some(1){
                *closure_lup.length_map.entry(current_entry).or_insert(lm_tip) += 1;
                *closure_lup.length_map_c.entry(current_entry).or_insert(lm_tip) += 1;
            }else if new_event.opcode == Some(2){
                *closure_lup.length_map.entry(current_entry).or_insert(lm_tip) -= 1;
                *closure_lup.length_map_c.entry(current_entry).or_insert(lm_tip) -= 1;
            }

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
            ui_postproc::UiPayload::length_map(with_lup(|lup|lup.length_map.clone()))
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
        if !lup.length_map_c.is_empty() && lup.length_map.contains_key(&0){
            lup.length_map.pop_first();
            let lm = lup.length_map.clone();
            lup.ui_tx.as_ref().unwrap().send(UiPayload::length_map(lm)).expect("Preprocessing TX failed");
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