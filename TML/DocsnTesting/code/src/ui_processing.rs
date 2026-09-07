use std::collections::BTreeMap;

use crate::{UiPayload, biner::EventFinal};

pub static process_persona: 

#[inline]
pub fn process_ui(events: Vec<EventFinal>) -> UiPayload{
    let pid_starts: BTreeMap<u64,u32> = BTreeMap::new();
    let pid_ends: BTreeMap<u64, u32> = BTreeMap::new();
    for (_,e) in events.iter().enumerate(){
        match e.opcode {
            1 => {
                pid_starts
                    .entry(e.timestamp)
                    .or_default()
                    .push(e.process_id);
            },
            2 => {
                pid_ends
                    .entry(e.timestamp)
                    .or_default()
                    .push(e.process_id);
            }
        }
    }

    let pid_actives: BTreeMap<u64,Vec<u32>>{

    }
    UiPayload {}
}
#[inline]
fn mark_start(e:&EventFinal){

}
#[inline]
fn mark_end(e:&EventFinal){

}

pub struct UiPayload{

}