use windows::Win32::System::Diagnostics::Etw::*;
use windows::core::PWSTR;

pub fn open_trace(session_name: &[u16]) -> PROCESSTRACE_HANDLE {
    
    let mut logfile = EVENT_TRACE_LOGFILEW::default();

    logfile.LoggerName = PWSTR(session_name.as_ptr() as *mut u16);

    logfile.Anonymous2.EventRecordCallback = Some(gimme_eventdata);

    logfile.Anonymous1.ProcessTraceMode =
    PROCESS_TRACE_MODE_REAL_TIME |
    PROCESS_TRACE_MODE_EVENT_RECORD;

    return unsafe {OpenTraceW(&mut logfile)};
}

pub fn trace_loop(consumer_handle: PROCESSTRACE_HANDLE) {
    unsafe {
        let result = ProcessTrace(
            &[consumer_handle],
            None,
            None
        );

        println!("processtrace message: {:?}", result);
        println!();
    }
}

extern "system" fn gimme_eventdata(_event_data: *mut EVENT_RECORD){

    unsafe{
        println!(
    "Opcode: {}, Version: {}, Task: {}, ID: {}, payload size: {}",
    (*_event_data).EventHeader.EventDescriptor.Opcode,
    (*_event_data).EventHeader.EventDescriptor.Version,
    (*_event_data).EventHeader.EventDescriptor.Task,
    (*_event_data).EventHeader.EventDescriptor.Id,
    (*_event_data).UserDataLength
    );
    }
}
