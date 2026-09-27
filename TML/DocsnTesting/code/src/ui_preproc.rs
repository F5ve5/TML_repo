use std::{cell::{RefCell}, collections::{BTreeMap, HashMap}, hash::Hash};

use crate::{biner::EventFinal,misc::round_filetime_to_nearest_second,ui_postproc::with_rup};

#[derive(Default, Debug, Clone)]
pub struct UiLoad{
    pub delta_map: BTreeMap<u64,i32>,
    pub timeline: BTreeMap<u64,Vec<u32>>,
    pub events_by_prid: Vec<EventFinal>,
    pub pre_snapshot_events: HashMap<PidPlusOpcode,EventFinal>,
    pub snapshot_done: bool,
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
        lup.snapshot_done = false;
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

        let pse = lup.pre_snapshot_events.clone();
        //Necessary because I can't run iter() or into_iter() on lup

        for (_, e) in pse {
            match e.opcode {
                1 => update_delta_map(lup, &e.timestamp, true),
                2 => update_delta_map(lup, &e.timestamp, false),
                _ => {}
            }
        }

        lup.snapshot_done = true;
    });
}
#[inline]
pub fn process_etw(events: Vec<EventFinal>) {
    with_lup(|lup| {
        if !lup.snapshot_done {
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
            for e in events {
                match e.opcode {
                    1 => update_delta_map(lup,&e.timestamp, true),
                    2 => update_delta_map(lup,&e.timestamp, false),
                    _ => {}
                }
            }
        }
    });
}

#[inline]
pub fn update_delta_map(closure_lup: &mut UiLoad,t: &u64, positive: bool) {

    let rounded = round_filetime_to_nearest_second(t);
    let time = closure_lup.delta_map.entry(rounded).or_default();

    if positive {
        *time += 1;
    } else {
        *time -= 1;
    }
}

pub fn merge_to_postproc(){
    //The following is not possible because lup is somehow called twice at the same time:
    //with_rup(|rup|{
    //    *rup = with_lup(|lup|lup.clone());
    //})
    //Here's the fix:

    let snapshot = with_lup(|lup| lup.clone());

    with_rup(|rup| {
        *rup = snapshot;
    });
}