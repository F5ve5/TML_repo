use windows::Win32::System::Diagnostics::Etw::*;
use windows::Win32::Foundation::ERROR_SUCCESS;
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
    (*props).Wnode.Guid = SystemTraceControlGuid;
    (*props).Wnode.Flags = WNODE_FLAG_TRACED_GUID;
    (*props).Wnode.ClientContext = 2;
  
    (*props).LogFileMode = EVENT_TRACE_REAL_TIME_MODE;
    (*props).LoggerNameOffset = etp_size as u32;
    
    (*props).EnableFlags = EVENT_TRACE_FLAG_PROCESS;

    let string_ptr = (props as *mut u8)
    .add(size_of::<EVENT_TRACE_PROPERTIES>())
    as *mut u16;
    std::ptr::copy_nonoverlapping(
    session_name.as_ptr(),
    string_ptr,
    session_name.len(),
    );

    let stw_status = StartTraceW( &mut session_handle, PCWSTR(session_name.as_ptr()), props);
    if stw_status.0 != ERROR_SUCCESS.0{
        println!("StartTraceW failed: {:?}", stw_status);
    }
    }

    return session_handle;
}

pub fn _enable_provider(session_handle: CONTROLTRACE_HANDLE){

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