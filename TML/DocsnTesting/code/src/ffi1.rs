use windows::Win32::System::Diagnostics::Etw::*;
use windows::core::PCWSTR;
use std::mem::size_of;

pub fn start_live_session(session_name: &[u16]) -> CONTROLTRACE_HANDLE {

    let etp_size = size_of::<EVENT_TRACE_PROPERTIES>();

    let buffer_size = etp_size + (session_name.len() * 2);

//Optimally, if one of my variables has to be translated to different types throughout the script, it's better to just not play smart about it and simply convert them at those lines
    let mut buffer = vec![0u8; buffer_size];

    let props = buffer.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES;

//Weirdly, I have to declare the handle beforehand in contrast to the function returning it as a callback like it did in OpenTraceW
    let mut handle: CONTROLTRACE_HANDLE = CONTROLTRACE_HANDLE::default();

//This reference only exists because the function for whatever reason won't let me pass &handle directly
    let handle_r: *mut CONTROLTRACE_HANDLE = &mut handle;

unsafe{
    (*props).Wnode.BufferSize = buffer_size as u32;
//Interestingly, the purpose of the below flag is solely for determining that the buffer is supposed to be treated by windows as a "normal etw buffer".
//Quite a PROLIFIC flag indeed (Love that word)
//Other flags in this field mostly signal to windows that the buffer is to be used for other things similar to etw, to my understanding at least.
    (*props).Wnode.Flags = WNODE_FLAG_TRACED_GUID;
//And this flag is for saying that it's data is to be streamed live rather than fed into a file or "circular" or whatever the other options were
    (*props).LogFileMode = EVENT_TRACE_REAL_TIME_MODE;

    (*props).LoggerNameOffset = etp_size as u32;
 
//Cool little line. When commenting I am always referring to the code below btw
//
//So I am creating a pointer to the point in the buffer where the session name will be placed I do that by first referring to the buffer as type u8 which is one byte and that is necessary
//because afterwards I add ".add(size_of::<EVENT_TRACE_PROPERTIES>())" where .add() happens to add the amount of whatever comes afterwards in the type of whatever came beforehand which sort of
//makes sense.. and "add" means move forward by that much in memory AND size_of returns the size of the following variable in bytes
//
//After having moved forward that much in memory, I convert that point in memory to a mutable u16 pointer which makes sense because the string that I am pointing towards is in format UTF16 aka
//WIDE format
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

    let stw_message = StartTraceW( handle_r, PCWSTR(session_name.as_ptr()), props);
   
    println!("message: {:?}", stw_message );
    println!("handle: {:?}", handle);
}

std::thread::sleep(std::time::Duration::from_secs(30));

return handle;
}