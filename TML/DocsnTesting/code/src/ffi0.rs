use std::ptr::null;

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

    let mut ed_buffer_size: u32 = 0;

    let status0 = TdhGetEventInformation(_event_data, None, None, &mut ed_buffer_size);

    let mut ed_buffer = vec![0u8; ed_buffer_size as usize];
    let ed = ed_buffer.as_mut_ptr() as *mut TRACE_EVENT_INFO;

    let status1 = TdhGetEventInformation(_event_data, None, Some(ed), &mut ed_buffer_size);

    println!("{:?}", status0);
    println!("{:?}", status1);

    let props = std::slice::from_raw_parts(
        (*ed).EventPropertyInfoArray.as_ptr(),
        (*ed).TopLevelPropertyCount as usize,
    );

    for prop in props {
        println!("{:?}", prop.NameOffset);
    }
    }

    
}
