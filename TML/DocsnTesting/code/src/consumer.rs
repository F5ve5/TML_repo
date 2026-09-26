use crate::biner::EventFinal;
use crate::{TX0, biner, cypher, pref};

use windows::Win32::Foundation::{
    ERROR_INSUFFICIENT_BUFFER,
    ERROR_SUCCESS,
};

use windows::Win32::System::Diagnostics::Etw::*;

use ntapi::ntexapi::{
    NtQuerySystemInformation,
    SYSTEM_PROCESS_INFORMATION,
    SystemProcessInformation
};

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

#[inline]
extern "system" fn gimme_eventdata(er: *mut EVENT_RECORD){
    let mut bin_vec: Vec<u8> = Vec::new();
    bin_vec.extend_from_slice(&[0u8; 5]);
    unsafe{

    ////
    let mut tei_buf_size: u32 = 0;

    let status0 = TdhGetEventInformation(
        er, 
        None, 
        None, 
        &mut tei_buf_size
    );
    if status0 != ERROR_INSUFFICIENT_BUFFER.0{
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
    if status1 != ERROR_SUCCESS.0{
        println!("Second TdhGetEventInformation failed: {:?}", status1);
        return;
    }
    //

    bin_vec.extend_from_slice(&biner::bin_event_header(&(*er).EventHeader));

    ////
    let props_tei = std::slice::from_raw_parts(
        (*tei).EventPropertyInfoArray.as_ptr(),
        (*tei).PropertyCount as usize,
    );
    //
    
    for prop_tei in props_tei.iter() {
        
        ////
        let prop_name_ptr = ((tei as *const u8).add(prop_tei.NameOffset as usize)) as *const u16;
        //
        
        if pref::WANTED_PROPS[(cypher::get_property_name_index(prop_name_ptr)) as usize]{

            ////
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
            //
        
            let pni = cypher::get_property_name_index(prop_name_ptr) as u8;

            let type_union_raw: &[u8] = std::slice::from_raw_parts(&prop_tei.Anonymous1 as *const _ as *const u8, 8);
            let pti = cypher::get_property_type_index( type_union_raw, prop_tei.Flags.0 as u32);

            bin_vec.extend_from_slice(&biner::bin_event_property(pni, pti, &prop_buf));

            //println!("Decoded property #{} {}", i, misc::utf16_to_r_string(prop_name_ptr as *const 
        }else{           
            //println!("Unwanted property #{} {}", i, misc::utf16_to_r_string(prop_name_ptr as *const u16));
        };
    }
    }

    bin_vec[0] = 255;
    let bv_len_from_255 = bin_vec.len() as u32;
    bin_vec[1..5].copy_from_slice(&bv_len_from_255.to_le_bytes());

    TX0.get().unwrap().send(bin_vec).unwrap();
}

pub fn trace_snapshot() -> Vec<EventFinal> {
    use std::ptr::null_mut;

    unsafe {
        let mut needed = 0u32;
        let _status0 = NtQuerySystemInformation(
            SystemProcessInformation,
            null_mut(),
            0,
            &mut needed,
        );

        let mut buffer = vec![0u8; needed as usize];

        let status1 = NtQuerySystemInformation(
            SystemProcessInformation,
            buffer.as_mut_ptr() as *mut _,
            needed,
            &mut needed,
        );
        if status1 < 0 {
            panic!("NtQuerySystemInformation failed: {:?}", status1);
        }

        let mut events: Vec<EventFinal> = Vec::new();
        let mut offset = 0usize;

        loop {
            let spi = &*(buffer.as_ptr().add(offset) as *const SYSTEM_PROCESS_INFORMATION);

            let pid = spi.UniqueProcessId as usize as u32;
            let ppid = spi.InheritedFromUniqueProcessId as usize as u32;

            // FILETIME timestamp (100ns ticks since 1601)
            let i64_timestamp = spi.CreateTime.QuadPart();
            let timestamp = u64::try_from(*i64_timestamp).unwrap();

            let image_name = if spi.ImageName.Buffer.is_null() {
                "<System>".to_string()
            } else {
                let chars = std::slice::from_raw_parts(
                    spi.ImageName.Buffer,
                    (spi.ImageName.Length / 2) as usize,
                );
                String::from_utf16_lossy(chars)
            };

            let event = EventFinal {
                other_header_values: Vec::new(),
                timestamp,
                opcode: 1, // snapshot = synthetic ProcessStart
                process_id: pid,
                properties: vec![
                    // ParentId
                    biner::EventPropertyFinal {
                        name_index: 2,
                        type_index: 2,
                        value: ppid.to_le_bytes().to_vec(),
                    },

                    // ImageFileName
                    biner::EventPropertyFinal {
                        name_index: 8,
                        type_index: 18,
                        value: image_name.into_bytes(),
                    },
                ],
            };

            events.push(event);

            if spi.NextEntryOffset == 0 {
                break;
            }

            offset += spi.NextEntryOffset as usize;
        }

        events
    }
}