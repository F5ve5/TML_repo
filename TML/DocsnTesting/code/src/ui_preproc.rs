use std::{cell::{RefCell}, collections::{BTreeMap, HashMap}, hash::Hash};

use crate::{biner::EventFinal,misc::round_filetime_to_nearest_second,ui_postproc::with_rup};

#[derive(Default, Debug, Clone)]
pub struct UiLoad{
    pub startup_tx: Option<std::sync::mpsc::Sender<i64>>,
    pub currently_active_processes: Vec<PidPlusOpcode>,


    pub pre_snapshot_events: HashMap<PidPlusOpcode,EventFinal>,
    pub initial_event_amount: i64,


    pub delta_map: BTreeMap<u64,i32>,
    pub events_by_prid: Vec<EventFinal>,


    pub length_map: BTreeMap<u64,i32>,
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

pub fn init_globals(startup_tx:std::sync::mpsc::Sender<i64>) {
    LOCAL_UI_PAYLOAD.with(|cell|{
        let mut lup = cell.borrow_mut();
        *lup = UiLoad::default();
        lup.initial_event_amount = -1;
        *lup.length_map.entry(1).or_default() = 0;
        lup.startup_tx = Some(startup_tx);
    });
}

pub fn process_snapshot(events: Vec<EventFinal>) {
    with_lup(|lup| {
        for e in events {
            lup.pre_snapshot_events.insert(
                PidPlusOpcode {
                    pid: e.process_id,
                    opcode: e.opcode
                },
                e,
            );
        }

        lup.initial_event_amount = lup.pre_snapshot_events.len() as i64;

        let pse: Vec<EventFinal> = lup.pre_snapshot_events.values().cloned().collect();

        for e in pse{
            update_lup(lup,e);
        }
    });
}
#[inline]
pub fn process_etw(e: EventFinal) {

    with_lup(|lup| {
        if lup.initial_event_amount == -1 {
            lup.pre_snapshot_events.insert(
                PidPlusOpcode {
                    pid: e.process_id,
                    opcode: e.opcode
                },
                e,
            );
        } else {
            update_lup(lup,e);
        }
    });
}

#[inline]
fn update_lup(closure_lup: &mut UiLoad,new_event:EventFinal){
    let mut current_entry = round_filetime_to_nearest_second(&new_event.timestamp.unwrap_or(0));
    if new_event.opcode == Some(1){
        *closure_lup.delta_map.entry(current_entry).or_default() += 1;
    }else if new_event.opcode == Some(2){
        *closure_lup.delta_map.entry(current_entry).or_default() += -1;
    }

    if let Some(last_event) = closure_lup.delta_map.last_key_value(){
        let last_lm_entry_value = closure_lup.length_map.last_key_value().unwrap().1.clone();
        while current_entry <= *last_event.0{
            if new_event.opcode == Some(1){
                *closure_lup.length_map.entry(current_entry).or_insert(last_lm_entry_value) += 1;
            }else if new_event.opcode == Some(2){
                *closure_lup.length_map.entry(current_entry).or_insert(last_lm_entry_value) -= 1;
            }

            current_entry += 10_000_000;
        }
    }
}

pub fn merge_to_postproc(){
    let lup = with_lup(|lup| lup.clone());

    with_rup(|rup|{
        *rup = lup;
    });

    with_lup(|lup| {
        if let Some(tx) = lup.startup_tx.take(){
            let iea = lup.initial_event_amount;
            tx.send(iea).unwrap();
        }
    });
}