mod consumer;
mod emitter;
mod misc;
mod pref;
mod biner;
mod files;
mod cypher;
mod ui_preproc;
mod ui_postproc;
mod ui;

use core::time;
use std::{sync::{OnceLock,mpsc}, thread, time::Duration};

use crate::{files::{bin_establish, bin_read_unread, bin_write}, ui_postproc::UiPayload::events};

pub static TX0: OnceLock<mpsc::Sender<Vec<u8>>> = OnceLock::new();
pub static TX2: OnceLock<mpsc::Sender<ui_preproc::UiLoad>> = OnceLock::new();

pub fn main(){

    let session_name_r: &str = "NT Kernel Logger";
    let session_name_c: Vec<u16> = misc::r_to_utf16_string(session_name_r);

    println!("1. StartTraceW");
    let session_handle = emitter::start_session(&session_name_c);
    println!("Handle: {:?}", session_handle);
    println!();

    let (tx0, rx0) = std::sync::mpsc::channel::<Vec<u8>>();
    TX0.set(tx0).unwrap();
    println!("2. OpenTraceW");
    let consumer_handle = consumer::open_trace(&session_name_c);
    println!("Handle: {:?}", consumer_handle);
    println!();

    println!("3. Bin file");
    match bin_establish(){
        Ok(()) => println!("Successfully established bin file"),
        Err(e) => println!("Could not establish bin file: {:?}", e)
    };
    println!();

    let (tx3, rx3) = std::sync::mpsc::channel::<i64>();

    println!("5. Spawning threads...");

    thread::spawn(move || {
        consumer::trace_loop(consumer_handle);
    });

    thread::spawn(move || {
        ui_preproc::init_globals(tx3);
        thread::sleep(Duration::from_millis(30));
        let snapshot_events = consumer::snapshot();
        ui_preproc::process_snapshot(snapshot_events);
        loop{
            let mut bin_vec = Vec::new();

            bin_vec.extend(rx0.recv().unwrap());
            
            let eventss = biner::unbin_events(&bin_vec);
            for e in eventss{
                ui_preproc::process_etw(e);
            }
            ui_preproc::merge_to_postproc();
        }
    });
    let (tx1, rx1) = std::sync::mpsc::channel::<Vec<ui_postproc::UPOpcode>>();
    let (tx2, rx2) = std::sync::mpsc::channel::<ui_postproc::UiPayload>();
    thread::spawn(move || {
        ui_postproc::init_globals();
        loop{
            let upos = rx1.recv().unwrap();
            for upo in upos{
                tx2.send(ui_postproc::get_payload(upo)).unwrap();
            }
        }
    });

    ui::start_egui(tx1,rx2,rx3);
}