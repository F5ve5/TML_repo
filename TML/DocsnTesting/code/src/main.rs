mod ffi0;
mod ffi1;
mod misc;

pub fn main() {
    let session_name_r: &str = "ETWSESSIONNAME";
    let session_name_c: Vec<u16> = misc::r_to_cstring(session_name_r);

    let session_handle0 = ffi1::start_live_session(&session_name_c);
    let consumer_handle = ffi0::open_trace(&session_name_c);
}