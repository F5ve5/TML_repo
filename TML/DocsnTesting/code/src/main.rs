mod ffi0;
mod ffi1;
mod misc;

pub fn main() {
    let session_name_r: &str = "NT Kernel Logger";
    let session_name_c: Vec<u16> = misc::r_to_cstring(session_name_r);

    let session_handle = ffi1::start_session(&session_name_c);
    let consumer_handle = ffi0::open_trace(&session_name_c);
    println!("2:");
    println!("Consumer handle from opentrace: {:?}", consumer_handle);
    println!();
    ffi0::trace_loop(consumer_handle);
    ffi1::enable_provider(session_handle);
}