use windows::Win32::System::Diagnostics::Etw::*;
use windows::core::PCWSTR;
use std::mem::size_of;

pub fn start_session(session_name: &[u16]) -> CONTROLTRACE_HANDLE {

    let etp_size = size_of::<EVENT_TRACE_PROPERTIES>();

    let buffer_size = etp_size + (session_name.len() * 2);

    let mut buffer = vec![0u8; buffer_size];

    let props = buffer.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES;

    let mut session_handle: CONTROLTRACE_HANDLE = CONTROLTRACE_HANDLE::default();
    
unsafe{
    (*props).Wnode.BufferSize = buffer_size as u32;
    (*props).Wnode.Flags = WNODE_FLAG_TRACED_GUID;

    (*props).LogFileMode = EVENT_TRACE_REAL_TIME_MODE |
    EVENT_TRACE_SYSTEM_LOGGER_MODE;

    (*props).LoggerNameOffset = etp_size as u32;
 
    let string_ptr = (props as *mut u8)
    .add(size_of::<EVENT_TRACE_PROPERTIES>())
    as *mut u16;

    std::ptr::copy_nonoverlapping(
    session_name.as_ptr(),
    string_ptr,
    session_name.len(),
    );

    let stw_msg = StartTraceW( &mut session_handle, PCWSTR(session_name.as_ptr()), props);
   
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

pub fn enable_trace(session_handle: CONTROLTRACE_HANDLE){
    unsafe{
        let etx_msg = EnableTraceEx2(
    session_handle,
    &SystemProcessProviderGuid,
    EVENT_CONTROL_CODE_ENABLE_PROVIDER.0 as u32,
    TRACE_LEVEL_INFORMATION as u8,
    0,
    0,
    0,
    None,
        );
            println!("3:");
            println!("Message from enableprovider: {:?}", etx_msg);
            println!();
    }   
}