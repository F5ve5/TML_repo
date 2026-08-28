mod consumer;
mod emitter;
mod misc;
mod pref;
mod biner;
mod files;
mod cypher;
mod eguiloop;

use std::{thread,sync::{OnceLock,mpsc}};

use crate::files::{bin_read_unread,bin_write,bin_establish};

pub static SESSION_TX: OnceLock<mpsc::Sender<Vec<u8>>> = OnceLock::new();

pub fn main(){
    let (tx0, rx0) = std::sync::mpsc::channel::<Vec<u8>>();
    SESSION_TX.set(tx0).unwrap();
    thread::spawn(move || {

        let session_name_r: &str = "NT Kernel Logger";
        let session_name_c: Vec<u16> = misc::r_to_utf16_string(session_name_r);

        let session_handle = emitter::start_session(&session_name_c);
        println!("1. StartTraceW");
        println!("Handle: {:?}", session_handle);
        println!();

        let consumer_handle = consumer::open_trace(&session_name_c);
        println!("2. OpenTraceW");
        println!("Handle: {:?}", consumer_handle);
        println!();

        println!("3. Bin file");
        let be_res = bin_establish();
        match be_res{
            Ok(()) => println!("Successfully established bin file"),
            Err(e) => println!("Could not establish bin file: {:?}", e)
        };
        println!();

        consumer::trace_loop(consumer_handle);
    });
    thread::spawn(move || {
        loop{
            match bin_write(&rx0.recv().unwrap()){
                Ok(_) => {},
                Err(e) => println!("bin_write failed: {}", e)
            };
        }
    });

    let (tx1, rx1) = std::sync::mpsc::channel::<Vec<biner::EventFinal>>();
    thread::spawn(move || {
        loop{
            thread::sleep(std::time::Duration::from_millis(3000));
            let bin_vec = bin_read_unread().unwrap();
            let event_vec = biner::unbin_events(&bin_vec);
            println!("events: {:?}", event_vec);
            tx1.send(event_vec).unwrap();
        }
    });
    eguiloop::start_egui(rx1);
}