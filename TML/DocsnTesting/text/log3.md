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

EnableTraceEx2(
            session_handle,
            &SystemTraceControlGuid,
            EVENT_CONTROL_CODE_ENABLE_PROVIDER.0,
            TRACE_LEVEL_INFORMATION as u8,
            EVENT_TRACE_FLAG_PROCESS.0 as u64,
            0 as u64,
            0 as u32,
            None
        );