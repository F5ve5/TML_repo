use windows::Win32::System::Diagnostics::Etw::*;
use std::mem::size_of;

pub fn start_live_session(session_name: &[u16]) -> PROCESSTRACE_HANDLE {

    let etp_size = size_of::<EVENT_TRACE_PROPERTIES>;

    let buffer_size = etp_size + (session_name.len() * 2);

    let buffer = vec![0u8,buffer_size];

    let props = buffer.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES;

    
}