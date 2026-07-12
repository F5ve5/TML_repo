use windows::Win32::System::Diagnostics::Etw;

extern "system" fn my_callback(event_data: *mut EVENT_RECORD){
    println!("Event recieved!")
}

fn main(){
    let mut my_logfie = EVENT_TRACE_LOGFILEW::default();

    my_logfile.EventRecordCallback = Some(my_callback);

    my_logfile.ProcessTraceMode =
    PROCESS_TRACE_MODE_REAL_TIME |
    PROCESS_TRACE_MODE_EVENT_RECORD;

    let etw_session_name: Vec<16> = "ETWSESSIONNAME"
    .encode_utf16()
    .chain(std::iter::once(0))
    .collect();

    my_logfile.LoggerName = etw_session_name.as_ptr();
}
