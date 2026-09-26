use std::{cell::{OnceCell, RefCell}, collections::{BTreeMap, HashMap, HashSet, btree_set}, hash::Hash, ops::Sub, sync::{Mutex, OnceLock}};

use crate::{biner::EventFinal,misc::round_filetime_to_nearest_second};

thread_local! {
    static LOCAL_UI_PAYLOAD: RefCell<UiLoad> = RefCell::new(UiLoad::default());
}

#[derive(Default, Debug)]
pub struct UiLoad {
    pub delta_map: BTreeMap<u64,i32>,
    pub timeline: BTreeMap<u64,Vec<u32>>,
    pub events_by_prid: Vec<EventFinal>,
    pub pre_snapshot_events: HashMap<PidPlusOpcode,EventFinal>,
    pub snapshot_done: bool,
}

pub fn init_globals() {
    LOCAL_UI_PAYLOAD.with(|cell| {
        let mut lup = cell.borrow_mut();
        *lup = UiLoad::default();
        lup.snapshot_done = false;
    });
}

#[inline]
fn get_mut_lup<R>(f: impl FnOnce(&mut UiLoad) -> R) -> R {
    LOCAL_UI_PAYLOAD.with(|cell| {
        let mut lup = cell.borrow_mut();
        f(&mut *lup)
    })
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct PidPlusOpcode{
    pid: u32,
    opcode: u8
}

pub fn process_snapshot(events: Vec<EventFinal>){
    let lup = get_mut_lup();

    for e in events{
        lup.pre_snapshot_events.insert(PidPlusOpcode{
            pid: e.process_id,
            opcode: e.opcode
        },
        e);
    }

    for (_,e) in lup.pre_snapshot_events.iter(){
        if e.opcode == 1{
            update_delta_map(&e.timestamp, true);
        }else if e.opcode == 2{
            update_delta_map(&e.timestamp, false);
        }
    }
    lup.snapshot_done = true;
}
#[inline]
pub fn process_etw(events: Vec<EventFinal>){
    let lup = get_mut_lup();

    if !lup.snapshot_done{
        for e in events{
            lup.pre_snapshot_events.insert(PidPlusOpcode{
                pid: e.process_id,
                opcode: e.opcode,
            },
            e);
        }
    }else{
        for e in events{
            if e.opcode == 1{
                update_delta_map(&e.timestamp, true);
            }else if e.opcode == 2{
                update_delta_map(&e.timestamp, false);
        }
        }
    }
}



fn make_prid(e:EventFinal) -> u32{
    let lup = get_mut_lup(f)

    lup.events_by_prid.push(e);

    (lup.events_by_prid.len() - 1) as u32
}



fn update_delta_map(t: &u64, positive: bool){
    let lup = get_mut_lup();

    let time = lup.delta_map.entry(round_filetime_to_nearest_second(t)).or_default();

    if positive {
        *time += 1;
    }else{
        *time -= 1;
    }
}