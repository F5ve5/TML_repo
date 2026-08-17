mod ffi0;
mod ffi1;
mod misc;
mod pref;
mod biner;
mod files;
mod cypher;

use std::{thread,sync::{OnceLock,mpsc}};

static TX0: OnceLock<mpsc::Sender<biner::EVRet>> = OnceLock::new();
pub fn main() {

    let (tx, rx) = std::sync::mpsc::channel::<biner::EVRet>();
    TX0.set(tx).unwrap();
    thread::spawn(move || {
    
    }
    );

    let session_name_r: &str = "NT Kernel Logger";
    let session_name_c: Vec<u16> = misc::r_to_utf16_string(session_name_r);

    let _session_handle = ffi1::start_session(&session_name_c);
    let consumer_handle = ffi0::open_trace(&session_name_c);
    println!("2. OpenTraceW");
    println!("Handle: {:?}", consumer_handle);
    println!();

    let be_res = files::bin_establish();
    match be_res{
        Ok(()) => (),
        Err(e) => println!("be: {:?}", e)
    };

    ffi0::trace_loop(consumer_handle);
    //ffi1::enable_provider(_session_handle);
}