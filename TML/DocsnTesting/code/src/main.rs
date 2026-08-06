mod ffi0;
mod ffi1;
mod misc;
mod events;

use std::sync::OnceLock;

pub fn main() {

    let (tx, rx) = std::sync::mpsc::channel::<events::pr_event>();

    static TX: OnceLock<std::sync::mpsc::Sender<events::pr_event>> = OnceLock::new();

    std::thread::spawn(move || {
    
    }
    );

    let session_name_r: &str = "NT Kernel Logger";
    let session_name_c: Vec<u16> = misc::r_to_utf16_string(session_name_r);

    let session_handle = ffi1::start_session(&session_name_c);
    let consumer_handle = ffi0::open_trace(&session_name_c);
    println!("2:");
    println!("Consumer handle from opentrace: {:?}", consumer_handle);
    println!();
    ffi0::trace_loop(consumer_handle);
    ffi1::enable_provider(session_handle);
}