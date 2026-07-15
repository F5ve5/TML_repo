use windows::Win32::System::Diagnostics::Etw::*;
use windows::core::PWSTR;

pub fn open_trace(session_name: &[u16]) -> PROCESSTRACE_HANDLE {
    
    let mut logfile = EVENT_TRACE_LOGFILEW::default();

//PWSTR = Pointer to Wide character STRing
    logfile.LoggerName = PWSTR(session_name.as_ptr() as *mut u16);

    logfile.Anonymous2.EventRecordCallback = Some(my_callback);

    logfile.Anonymous1.ProcessTraceMode =
    PROCESS_TRACE_MODE_REAL_TIME |
    PROCESS_TRACE_MODE_EVENT_RECORD;

//Interesting thing here, the function expects a "*mut EVEBT_TRACE_LOGFILEW" but I'm giving it "&mut EVENT_TRACE_LOGFILEW", basically because a mutable reference is guaranteed to be safe,
//Rust can safely convert it into a raw pointer without having to worry about it leading nowhere.
//
//Further, though the ampersand and the star might seem to have a very similar use as per one meaning a reference and the other a pointer, they have very different uses in "let" lines;
//let x: u32 = 5;
//let &y = x;
//let *z = x;
//Here the ampersand will create a reference to x, while z will become where the unsigned integer 5 points in memory. So one works up, one works down.
    return unsafe {OpenTraceW(&mut logfile)};
}

pub fn process_trace(session_handle: PROCESSTRACE_HANDLE) {
    unsafe {
        let result = ProcessTrace(
            &[session_handle],
            None,
            None
        );

        println!("processtrace message: {:?}", result);
    }
}

extern "system" fn my_callback(_event_data: *mut EVENT_RECORD){
    println!("Event recieved in callback function!")
}
