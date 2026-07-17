use windows::Win32::System::Diagnostics::Etw::*;
use windows::core::PCWSTR;
use std::mem::size_of;
use windows::core::GUID;

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
    (*props).Wnode.BufferSize = buffer_size as u32;
//Interestingly, the purpose of the below flag is solely for determining that the buffer is supposed to be treated by windows as a "normal etw buffer".
//Quite a PROLIFIC flag indeed (Love that word)
//Other flags in this field mostly signal to windows that the buffer is to be used for other things similar to etw, to my understanding at least.
    (*props).Wnode.Flags = WNODE_FLAG_TRACED_GUID;
//And this flag is for saying that it's data is to be streamed live rather than fed into a file or "circular" or whatever the other options were
    (*props).LogFileMode = EVENT_TRACE_REAL_TIME_MODE;
//This field definitely seems weird at first because like why doesn't windows know the size of the struct it's reading? The answer is that it's once again because windows is old, or more
//precisely because the size of the struct and the things within it has changed over time while etw and what it expects to receive has not.
    (*props).LoggerNameOffset = etp_size as u32;
 
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
    println!();
}

return session_handle;
}

pub fn enable_session_provider(session_handle: CONTROLTRACE_HANDLE){
    unsafe{
        let etx_msg = EnableTraceEx2(
            session_handle,
            &SystemTraceControlGuid,
            EVENT_CONTROL_CODE_ENABLE_PROVIDER.0,
            TRACE_LEVEL_INFORMATION as u8,
            EVENT_TRACE_FLAG_PROCESS.0 as u64,
            0 as u64,
            0 as u32,
            None
        );
            println!("3:");
            println!("Message from enableprovider: {:?}", etx_msg);
            println!();
    }
}  