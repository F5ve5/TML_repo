mod ffi0;
mod ffi1;
mod ffi2;
mod misc;

pub fn main() {
    let session_name_r: &str = "ETWWW";
    let session_name_c: Vec<u16> = misc::r_to_cstring(session_name_r);

    let session_handle = ffi1::start_session(&session_name_c);
    let consumer_handle = ffi0::open_trace(&session_name_c);
    ffi1::enable_provider(session_handle);
    ffi0::process_trace(consumer_handle);
}