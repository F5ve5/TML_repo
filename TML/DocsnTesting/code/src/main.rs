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
use std::{thread,sync::{OnceLock,mpsc}};

use crate::files::{bin_read_unread,bin_write,bin_establish};

pub static TX0: OnceLock<mpsc::Sender<Vec<u8>>> = OnceLock::new();
pub static TX2: OnceLock<mpsc::Sender<ui_preproc::UiLoad>> = OnceLock::new();

pub fn main(){

    let session_name_r: &str = "NT Kernel Logger";
    let session_name_c: Vec<u16> = misc::r_to_utf16_string(session_name_r);

    println!("1. StartTraceW");
    let session_handle = emitter::start_session(&session_name_c);
    println!("Handle: {:?}", session_handle);
    println!();

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

    println!("4. Other globals");
    ui_preproc::init_globals();
    ui_postproc::init_globals();
    println!("Successfully established ui globals");
    println!();

    println!("5. Spawning threads...");

    let (tx0, rx0) = std::sync::mpsc::channel::<Vec<u8>>();
    TX0.set(tx0).unwrap();
    thread::spawn(move || {
        consumer::trace_loop(consumer_handle);
    });

    thread::spawn(move || {
        loop{
            let bin_vec = rx0.recv().unwrap();
            
            let events = biner::unbin_events(&bin_vec);
            ui_preproc::process_etw(events);
            ui_preproc::merge_to_postproc();
        }
    });
    let (tx1, rx1) = std::sync::mpsc::channel::<ui_postproc::MergeOpcode>();
    let (tx2, rx2) = std::sync::mpsc::channel::<ui_postproc::ULWithContentLabel>();
    thread::spawn(move || {
        loop{
            match rx1.recv(){
                Ok(mo) => {tx2.send(ui_postproc::get_payload(mo)).unwrap();},
                Err(_) => {}
            }
        }
    });

    println!("6. NtQuerySystemInformation");
    println!("Waiting for etw to catch up...");
    thread::sleep(time::Duration::from_millis(1500));
    let snapshot_events = consumer::trace_snapshot();
    println!("Processing {} snapshot events...", snapshot_events.len());
    println!();
    ui_preproc::process_snapshot(snapshot_events);

    ui::start_egui(tx1,rx2);
}