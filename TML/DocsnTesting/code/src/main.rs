mod ffi0;
mod misc;

pub fn main() {
    let session_name_r: &str = "ETWSESSIONNAME";
    let session_name_c: Vec<u16> = misc::r_to_cstring(session_name_r);

    let my_trace_handle = ffi0::get_trace_handle(&session_name_c);
}