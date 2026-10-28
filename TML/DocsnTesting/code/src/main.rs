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

use crate::{files::{bin_establish, bin_read_unread, bin_write}};

pub static TX0: OnceLock<mpsc::Sender<Vec<u8>>> = OnceLock::new();

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

    println!("4. Spawning threads...");

    let (tx1, rx1) = std::sync::mpsc::channel::<ui_postproc::UPOpcode>();
    let (tx20, rx2) = std::sync::mpsc::channel::<ui_postproc::UiPayload>();
    let tx21 = tx20.clone();
    let (tx0, rx0) = std::sync::mpsc::channel::<Vec<u8>>();
    TX0.set(tx0).unwrap();
    let (tx4, rx4) = std::sync::mpsc::channel::<ui_postproc::UPOpcode>();

    thread::spawn(move || {
        consumer::trace_loop(consumer_handle);
    });

    thread::spawn(move || {
        ui_preproc::init_globals(tx21);
        thread::sleep(Duration::from_millis(30));
        let snapshot_events = consumer::snapshot();
        ui_preproc::process_snapshot(snapshot_events);
        loop{
            let mut bin_vec = Vec::new();

            bin_vec.extend(rx0.recv().unwrap());
            
            let events = biner::unbin_events(&bin_vec);
            for e in events{
                ui_preproc::process_etw(e);
            }
            ui_preproc::mini_merge();

            while let Ok(upo) = rx4.try_recv(){
                ui_preproc::get_backend_payload(upo);
            }

            /////
            bin_write(&bin_vec).expect("bin_write failed");
        }
    });
    thread::spawn(move || {
        ui_postproc::init_globals(tx20);
        loop{
            let upo = rx1.recv().expect("Request RX failed");
            ui_postproc::handle_payload(upo,&tx4);
        }
    });

    println!("Successfully spawned threads!.. for now");
    println!();
    println!();

    ui::start_egui(tx1,rx2);
}