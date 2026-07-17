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

classic session start for thread + process events (also dont call enabletrace): ----------------

    (*props).Wnode.BufferSize = buffer_size as u32;
    (*props).Wnode.Flags = WNODE_FLAG_TRACED_GUID;

//Classic flags
    (*props).EnableFlags =     EVENT_TRACE_FLAG_PROCESS |
    EVENT_TRACE_FLAG_THREAD;
    (*props).Wnode.Guid = SystemTraceControlGuid;

    (*props).LogFileMode = EVENT_TRACE_REAL_TIME_MODE;

    (*props).LoggerNameOffset = etp_size as u32;

and the modern one: ---------------------

(*props).Wnode.BufferSize = buffer_size as u32;
(*props).Wnode.Flags = WNODE_FLAG_TRACED_GUID;

(*props).LogFileMode = EVENT_TRACE_REAL_TIME_MODE;

(*props).LoggerNameOffset = etp_size as u32;

OGSTARTTRACE and heres the original comments ver of the whole buffer assigning phase: ----------------------

pub fn start_session(session_name: &[u16]) -> CONTROLTRACE_HANDLE {

    let etp_size = size_of::<EVENT_TRACE_PROPERTIES>();

    let buffer_size = etp_size + (session_name.len() * 2);

//Optimally, if one of my variables has to be translated to different types throughout the script, it's better to just not play smart about it and simply convert them at those lines
    let mut buffer = vec![0u8; buffer_size];

    let props = buffer.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES;

//Weirdly, I have to declare the handle beforehand and pass it to the function later instead of the function retungin a handle
    let mut session_handle: CONTROLTRACE_HANDLE = CONTROLTRACE_HANDLE::default();

//This reference only exists because the function for whatever reason won't let me pass &handle directly
    let session_handle_ref: *mut CONTROLTRACE_HANDLE = &mut session_handle;

unsafe{
//Cool little line. When commenting I am always referring to the code below btw
//
//So I am creating a pointer to the point in the buffer where the session name will be placed I do that by first referring to the buffer as type u8 which is one byte and that is necessary
//because afterwards I add ".add(size_of::<EVENT_TRACE_PROPERTIES>())" where .add() happens to add the amount of whatever comes afterwards in the type of whatever came beforehand which sort of
//makes sense.. and "add" means move forward by that much in memory AND size_of returns the size of the following variable in bytes
//
//After having moved forward that much in memory, I convert that point in memory to a mutable u16 pointer which makes sense because the string that I am pointing towards is in format UTF16 aka
//WCHARformat
//
//Additionally, the two blocks of code below are also required to be in the unsafe scope not because of using a potentially invalid pointer but because they are manipulating memory without Rusts'
//ordinary precaution. Like Rust can't verify whether the location I am pointing to is safe or if writing to it'd overwrite existing data
    let string_ptr = (props as *mut u8)
    .add(size_of::<EVENT_TRACE_PROPERTIES>())
    as *mut u16;

//Apparently the simplest way to implement any type of vector or array into memory because you have to assign every single byte by themselves. Basically does the same as a for loop which writes
//in each character into each of slot of memory but this function was literally made for this so I guess I'll let it take the spotlight
    std::ptr::copy_nonoverlapping(
    session_name.as_ptr(),
    string_ptr,
    session_name.len(),
    );
let stw_msg = StartTraceW( session_handle_ref, PCWSTR(session_name.as_ptr()), props);
   
    print!("1:");
    println!("Message from starttrace: {:?}", stw_msg );
    println!("Session handle from starttrace: {:?}", session_handle);
    println!("Guid: {:?}", (*props).Wnode.Guid);
println!("Flags: {:?}", (*props).Wnode.Flags);
println!("Mode: {:?}", (*props).LogFileMode);
println!("Enable: {:?}", (*props).EnableFlags);
    println!();
    }

return session_handle;
}

OGOPENTRACE Here's the original opentrace too since I wanna remove the comments: ------------------------------------

pub fn open_trace(session_name: &[u16]) -> PROCESSTRACE_HANDLE {
    
    let mut logfile = EVENT_TRACE_LOGFILEW::default();

//PWSTR = Pointer to Wide character STRing - and yes a pointer to it is equivalent to a pointer to the first letter of the string
    logfile.LoggerName = PWSTR(session_name.as_ptr() as *mut u16);

    logfile.Anonymous2.EventRecordCallback = Some(gimme_eventdata);

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

.NETGUID heres the proper guid and all for enabling the .net provider, actually works too -----------------------------------------

EnableTraceEx2(
    session_handle,
    &provider,
    EVENT_CONTROL_CODE_ENABLE_PROVIDER.0,
    TRACE_LEVEL_VERBOSE as u8,
    u64::MAX,
    0,
    0,
    None
);