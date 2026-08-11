use crate::{files::{bin_read_unread, bin_write}, misc, pref::WANTED_EVENT_HEADER_INFO};

use windows::Win32::System::Diagnostics::Etw::*;
use core::slice;
use std::{io::{Error, ErrorKind}, mem::size_of_val};

pub fn bin_property(index_b: u8, epi_flags: i32, epi_type: EVENT_PROPERTY_INFO_0, property_buffer_b: &[u8]){

    let mut bin_vec: Vec<u8> = Vec::new();

    let flags_b = epi_flags.to_le_bytes();
    let type_union_b = unsafe{
    slice::from_raw_parts(&epi_type as *const EVENT_PROPERTY_INFO_0 as *const u8, 8)
    }; //u16 + u16 + u32; union
    let p_buf_size_b = (property_buffer_b.len() as u32).to_le_bytes();
    
    bin_vec.push(index_b);
    bin_vec.extend_from_slice(&flags_b);
    bin_vec.extend_from_slice(&type_union_b);
    bin_vec.extend_from_slice(&p_buf_size_b);
    bin_vec.extend_from_slice(property_buffer_b);

    let bw_res = bin_write(&bin_vec);
    match bw_res{
        Ok(()) => (),
        Err(e) => print!("bin_write failed in bin_property: {:?}", e)
    };
}

pub fn  unbin_property() -> misc::UPRet{

    let bin_vec = bin_read_unread();
    match bin_vec{
        Ok(_) => {},
        Err(e) => println!("bin_read_unread failed in unbin_property: {:?}", e)
    };
}

pub fn bin_header(event_header: &EVENT_HEADER){
    
let mut bin_vec: Vec<u8> = Vec::new();
    
if WANTED_EVENT_HEADER_INFO[0]{
    let data_b = event_header.Size.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(1);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[1]{
    let data_b = event_header.HeaderType.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(2);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[2]{
    let data_b = event_header.Flags.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(3);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[3]{
    let data_b = event_header.EventProperty.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(4);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[4]{
    let data_b = event_header.ThreadId.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(5);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[5]{
    let data_b = event_header.ProcessId.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(6);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[6]{
    let data_b = event_header.TimeStamp.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(7);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[7]{
    let data_b = event_header.ProviderId.to_u128().to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[8]{
    let data_b = event_header.EventDescriptor.Id.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(9);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[9]{
    let data_b = event_header.EventDescriptor.Version.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(10);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[10]{
    let data_b = event_header.EventDescriptor.Channel.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(11);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[11]{
    let data_b = event_header.EventDescriptor.Level.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(12);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[12]{
    let data_b = event_header.EventDescriptor.Opcode.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(13);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[13]{
    let data_b = event_header.EventDescriptor.Task.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(14);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[14]{
    let data_b = event_header.EventDescriptor.Keyword.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(15);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[15]{
    let data_b = event_header.ActivityId.to_u128().to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(16);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[16]{
    bin_vec.push(17);
    unsafe{
    if (event_header.Flags as u32) & EVENT_HEADER_FLAG_PRIVATE_SESSION != 0{
        let data_b = event_header.Anonymous.ProcessorTime.to_le_bytes();
        let data_size = size_of_val(&event_header.Anonymous.ProcessorTime.to_le_bytes()) as u16;
        let data_size_b = data_size.to_le_bytes();

        bin_vec.extend_from_slice(&data_size_b);
        bin_vec.extend_from_slice(&data_b);
    }else{
        let data_b0 = event_header.Anonymous.Anonymous.KernelTime.to_le_bytes();

        let data_b1 = event_header.Anonymous.Anonymous.UserTime.to_le_bytes();

        let data_size  = (size_of_val(&event_header.Anonymous.Anonymous.KernelTime.to_le_bytes()) +
        size_of_val(&event_header.Anonymous.Anonymous.UserTime.to_le_bytes())) as u16;
        let data_size_b = data_size.to_le_bytes();

        bin_vec.extend_from_slice(&data_size_b);
        bin_vec.extend_from_slice(&data_b0);
        bin_vec.extend_from_slice(&data_b1);
    }
    }

    let bw_res = bin_write(&bin_vec);
    match bw_res{
        Ok(()) => (),
        Err(e) => print!("bin_write failed in bin_header: {:?}", e)
    };

}
}