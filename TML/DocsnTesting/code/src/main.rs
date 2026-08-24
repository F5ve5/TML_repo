mod consumer;
mod emitter;
mod misc;
mod pref;
mod biner;
mod files;
mod cypher;
mod eguiloop;

use std::{thread,sync::{OnceLock,mpsc}};

pub static TX0_SET_UP_TRACE: OnceLock<mpsc::Sender<biner::UERet>> = OnceLock::new();
pub fn main() {

    let (tx0, rx0) = std::sync::mpsc::channel::<biner::UERet>();

    TX0_SET_UP_TRACE.set(tx0).unwrap();

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

        let be_res = files::bin_establish();
        match be_res{
            Ok(()) => (),
            Err(e) => println!("be: {:?}", e)
        };

        consumer::trace_loop(consumer_handle);
    }
    );

    eguiloop::start_egui();
}