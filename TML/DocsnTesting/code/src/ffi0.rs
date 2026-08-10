use crate::{biner, misc, pref};

use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS};
use windows::Win32::System::Diagnostics::Etw::*;
use windows::core::PWSTR;
use windows::core::GUID;
use misc::PropertyValue;

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

extern "system" fn gimme_eventdata(er: *mut EVENT_RECORD){
    unsafe{

    let mut tei_buf_size: u32 = 0;

    let status0 = TdhGetEventInformation(
        er, 
        None, 
        None, 
        &mut tei_buf_size
    );
    if(status0 != ERROR_INSUFFICIENT_BUFFER.0){
        println!("First TdhGetEventInformation failed: {:?}", status0);
        return;
    }

    let mut tei_buf = vec![0u8; tei_buf_size as usize];
    let tei = tei_buf.as_mut_ptr() as *mut TRACE_EVENT_INFO;

    let status1 = TdhGetEventInformation(
        er, 
        None, 
        Some(tei), 
        &mut tei_buf_size
    );
    if(status1 != ERROR_SUCCESS.0){
        println!("Second TdhGetEventInformation failed: {:?}", status1);
        return;
    }

    biner::bin_header(&(*er).EventHeader);

    let props_tei = std::slice::from_raw_parts(
        (*tei).EventPropertyInfoArray.as_ptr(),
        (*tei).PropertyCount as usize,
    );
    for (i, prop_tei) in props_tei.iter().enumerate() {
        
        let prop_name_ptr = (tei as *const u8).add(prop_tei.NameOffset as usize);

        if(pref::WANTED_PROPS[i]){

        let prop_desc = PROPERTY_DATA_DESCRIPTOR {
            PropertyName: prop_name_ptr as u64,
            ArrayIndex: u32::MAX,
            Reserved: 0,
        };

        let mut prop_buf_size = 0;

        let status2 = TdhGetPropertySize(
            er, 
            None, 
            &[prop_desc], 
            &mut prop_buf_size
        );
        if status2 != ERROR_SUCCESS.0 {
            println!("TdhGetPropertySize failed: {:?}", status2);
            return;
        }

        let mut prop_buf = vec![0u8; prop_buf_size as usize];

        let status3 = TdhGetProperty(
            er,
            None,
            &[prop_desc],
            &mut prop_buf,
        );
        if status3 != ERROR_SUCCESS.0 {
            println!("TdhGetProperty failed: {:?}", status3);
            return;
        }

        //println!("Property #{} {} Decoded: {:?}", i, utf16_to_r_string(prop_name_ptr as *const u16), decode_property(&prop_buf, prop_tei.Anonymous1.nonStructType.InType));
        biner::bin_property(i as u8, prop_tei.Flags.0, prop_tei.Anonymous1, &prop_buf);
        }else{
        //println!("Property #{} {} Not Wanted", i, utf16_to_r_string(prop_name_ptr as *const u16));
        };
    }
    }
}

pub unsafe fn decode_property(
    buf_ptr: &[u8],
    intype: u16,
) -> misc::PropertyValue {
    match intype {
        x if x == TDH_INTYPE_UINT8.0 as u16 => {
            misc::PropertyValue::U8(*(buf_ptr.as_ptr() as *const u8))
        }

        x if x == TDH_INTYPE_UINT16.0 as u16 => {
            misc::PropertyValue::U16(*(buf_ptr.as_ptr() as *const u16))
        }

        x if x == TDH_INTYPE_UINT32.0 as u16 => {
            misc::PropertyValue::U32(*(buf_ptr.as_ptr() as *const u32))
        }

        x if x == TDH_INTYPE_UINT64.0 as u16 => {
            misc::PropertyValue::U64(*(buf_ptr.as_ptr() as *const u64))
        }

        x if x == TDH_INTYPE_INT32.0 as u16 => {
            misc::PropertyValue::I32(*(buf_ptr.as_ptr() as *const i32))
        }

        x if x == TDH_INTYPE_INT64.0 as u16 => {
            misc::PropertyValue::I64(*(buf_ptr.as_ptr() as *const i64))
        }

        x if x == TDH_INTYPE_BOOLEAN.0 as u16 => {
            misc::PropertyValue::Bool(*(buf_ptr.as_ptr() as *const u32) != 0)
        }

        x if x == TDH_INTYPE_UNICODESTRING.0 as u16 => {
            misc::PropertyValue::String(misc::utf16_to_r_string(buf_ptr.as_ptr() as *const u16))
        }

        x if x == TDH_INTYPE_GUID.0 as u16 => {
            misc::PropertyValue::Guid(*(buf_ptr.as_ptr() as *const GUID))
        }

        _ => misc::PropertyValue::Bytes(buf_ptr.to_vec()),
    }
}