use std::{cell::{RefCell}, collections::{BTreeMap, HashMap}, hash::Hash};

use crate::{biner::EventFinal,misc::round_filetime_to_nearest_second,ui_postproc::with_rup};

#[derive(Default, Debug, Clone)]
pub struct UiLoad{
    pub delta_map: BTreeMap<u64,i32>,
    pub length_map: BTreeMap<u64,i32>,
    pub events_by_prid: Vec<EventFinal>,
    pub pre_snapshot_events: HashMap<PidPlusOpcode,EventFinal>,
    pub initial_event_amount: i32,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct PidPlusOpcode{
    pid: u32,
    opcode: u8
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

pub fn init_globals() {
    LOCAL_UI_PAYLOAD.with(|cell| {
        let mut lup = cell.borrow_mut();
        *lup = UiLoad::default();
        lup.initial_event_amount = -1;
    });
}

pub fn process_snapshot(events: Vec<EventFinal>) {
    with_lup(|lup| {
        for e in events {
            lup.pre_snapshot_events.insert(
                PidPlusOpcode {
                    pid: e.process_id,
                    opcode: e.opcode,
                },
                e,
            );
        }

        let pse_vec: Vec<EventFinal> = lup.pre_snapshot_events.values().cloned().collect();
        //Necessary because I can't run iter() or into_iter() on lup

        lup.initial_event_amount = pse_vec.len() as i32;

        update_lup(pse_vec);
    });
}
#[inline]
pub fn process_etw(events: Vec<EventFinal>) {

    with_lup(|lup| {
        if lup.initial_event_amount == -1 {
            for e in events {
                lup.pre_snapshot_events.insert(
                    PidPlusOpcode {
                        pid: e.process_id,
                        opcode: e.opcode,
                    },
                    e,
                );
            }
        } else {
            update_lup(events);
        }
    });
}

#[inline]
fn update_lup(new_events: Vec<EventFinal>){

    let mut new_events_rounded: BTreeMap<u64,i32> = BTreeMap::new();
    for new_event in new_events{
        let rounded_ne = round_filetime_to_nearest_second(&new_event.timestamp);
        let rounded_ne_plus_one = round_filetime_to_nearest_second(&new_event.timestamp) + 10_000_000;
        if new_event.opcode == 1{
            *new_events_rounded.entry(rounded_ne).or_default() += 1;
            with_lup(|lup|{*lup.delta_map.entry(rounded_ne).or_default() += 1});


        }else if new_event.opcode == 2{
            *new_events_rounded.entry(rounded_ne_plus_one).or_default() -= 1;
            with_lup(|lup|{*lup.delta_map.entry(rounded_ne_plus_one).or_default() -= 1});
        }else{

        }
    }
    let mut first_ner = new_events_rounded.first_key_value().unwrap().0.clone();
    let last_ner = new_events_rounded.last_key_value().unwrap().0;
    let mut lm_iteration_value: i32 = 0;
    while first_ner >= *last_ner{
        match new_events_rounded.get_key_value(&first_ner){
            Some(ne_pair) => lm_iteration_value += ne_pair.1,
            None => {}
        }

        with_lup(|lup|*lup.length_map.entry(first_ner).or_default() += lm_iteration_value);

        first_ner += 10_000_000;
    }

}

pub fn merge_to_postproc(){
    //The following is not possible because lup is somehow called twice at the same time:
    //with_rup(|rup|{
    //    *rup = with_lup(|lup|lup.clone());
    //})
    //Here's the fix:

    let lup = with_lup(|lup| lup.clone());

    with_rup(|rup| {
        *rup = lup;
    });
}