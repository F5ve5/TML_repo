use windows::Win32::System::Diagnostics::Etw;

///

#[repr(C)]

///

let x = 10;

let a = &x;
let b = &x;

///

pub fn build_logfilew(name: &str){
    
    let mut my_logfie = EVENT_TRACE_LOGFILEW::default();

    let etw_session_name: Vec<u16> = "ETWSESSIONNAME"
    .encode_utf16()
    .chain(std::iter::once(0))
    .collect();

    my_logfile.LoggerName = etw_session_name.as_ptr();

    my_logfile.EventRecordCallback = Some(my_callback);

    my_logfile.ProcessTraceMode =
    PROCESS_TRACE_MODE_REAL_TIME |
    PROCESS_TRACE_MODE_EVENT_RECORD;
}

///