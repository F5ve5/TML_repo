use crate::{cypher, files::{bin_read_unread, bin_write}, misc, pref::{WANTED_EVENT_HEADER_INFO, WANTED_PROPS}};

use windows::Win32::System::Diagnostics::Etw::*;
use std::mem::size_of_val;

#[inline]
pub fn unbin_events() -> EVRet{

    let bin_vec = match bin_read_unread() {
        Ok(v) => v,
        Err(e) => {
            println!("bin_read_unread failed in unbin_events: {:?}", e);
            return EVRet {
                headers: Vec::new(),
                properties: Vec::new(),
            };
        }
    };

    let mut ehf_vec: Vec<EventHeaderFinal> = Vec::new();
    let mut epf_vec: Vec<EventPropertyFinal> = Vec::new();

    let mut currentbyte: usize = 0;

    let mut header_value_amount: u8;
    let mut property_value_amount: u8;

    let mut header_value_index: u8;
    let mut header_value_size: u32;
    let mut header_value: &[u8];

    let mut property_name_index: u8;
    let mut property_value_type: u8;
    let mut property_valuebuffer_size: u32;
    let mut property_value_buffer: &[u8];

    header_value_amount = bin_vec[currentbyte];
        currentbyte += 1;
    property_value_amount = bin_vec[currentbyte];
        currentbyte += 1;

    while header_value_amount >= 1{
        header_value_amount -= 1;

        header_value_index = bin_vec[currentbyte];
            currentbyte += 1;
        header_value_size = u32::from_le_bytes([bin_vec[currentbyte], bin_vec[currentbyte + 1], bin_vec[currentbyte + 2], bin_vec[currentbyte + 3]]);
            currentbyte += 4;
        header_value = &bin_vec[currentbyte..(currentbyte + header_value_size as usize)];
            currentbyte += header_value_size as usize;

        ehf_vec.push(EventHeaderFinal{
            index: header_value_index,
            value: header_value.to_vec()
        });
    }
    while property_value_amount >= 1{
        property_value_amount-= 1;

        property_name_index = bin_vec[currentbyte];
            currentbyte += 1;
        property_value_type = bin_vec[currentbyte];
            currentbyte += 1;
        property_valuebuffer_size = u32::from_le_bytes([bin_vec[currentbyte], bin_vec[currentbyte + 1], bin_vec[currentbyte + 2], bin_vec[currentbyte + 3]]);
            currentbyte += 4;
        property_value_buffer = &bin_vec[currentbyte..(currentbyte + property_valuebuffer_size as usize)];
            currentbyte += property_valuebuffer_size as usize;

        epf_vec.push(EventPropertyFinal{
            name_index: property_name_index,
            type_index: property_value_type,
            value: property_value_buffer.to_vec()
        });
    }

    return EVRet{
        headers: ehf_vec,
        properties: epf_vec
    };
}
#[derive(Debug)]
pub struct EventHeaderFinal{
    index: u8,
    value: Vec<u8>
}
#[derive(Debug)]
pub struct EventPropertyFinal{
    name_index: u8,
    type_index: u8,
    value: Vec<u8>
}
#[derive(Debug)]
pub struct EVRet{
    headers: Vec<EventHeaderFinal>,
    properties: Vec<EventPropertyFinal>
}

#[inline]
pub fn bin_event_property(name_index_b: u8, type_index_b: u8, property_buffer_b: &[u8]){

    let mut bin_vec: Vec<u8> = Vec::new();
    let p_buf_size_b = (property_buffer_b.len() as u32).to_le_bytes();
    
    bin_vec.push(name_index_b);
    bin_vec.push(type_index_b);
    bin_vec.extend_from_slice(&p_buf_size_b);
    bin_vec.extend_from_slice(property_buffer_b);

    let bw_res = bin_write(&bin_vec);
    match bw_res{
        Ok(()) => (),
        Err(e) => print!("bin_write failed in bin_event_property: {:?}", e)
    };
}

#[inline]
pub fn bin_event_header(event_header: &EVENT_HEADER){
    
let mut bin_vec: Vec<u8> = Vec::new();

let header_value_amount_b = WANTED_EVENT_HEADER_INFO.iter().filter(|&&x| x).count();
let property_value_amount_b = WANTED_PROPS.iter().filter(|&&x| x).count();

bin_vec.push(header_value_amount_b as u8);
bin_vec.push(property_value_amount_b as u8);
    
if WANTED_EVENT_HEADER_INFO[0]{
    let data_b = event_header.Size.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(0u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[1]{
    let data_b = event_header.HeaderType.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(1u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[2]{
    let data_b = event_header.Flags.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(2u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[3]{
    let data_b = event_header.EventProperty.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(3u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[4]{
    let data_b = event_header.ThreadId.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(4u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[5]{
    let data_b = event_header.ProcessId.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(5u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[6]{
    let data_b = event_header.TimeStamp.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(6u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[7]{
    let data_b = event_header.ProviderId.to_u128().to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(7u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[8]{
    let data_b = event_header.EventDescriptor.Id.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(8u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[9]{
    let data_b = event_header.EventDescriptor.Version.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(9u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[10]{
    let data_b = event_header.EventDescriptor.Channel.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(10u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[11]{
    let data_b = event_header.EventDescriptor.Level.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(11u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[12]{
    let data_b = event_header.EventDescriptor.Opcode.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(12u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[13]{
    let data_b = event_header.EventDescriptor.Task.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(13u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[14]{
    let data_b = event_header.EventDescriptor.Keyword.to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(14u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[15]{
    let data_b = event_header.ActivityId.to_u128().to_le_bytes();
    let data_size = size_of_val(&data_b) as u32;
    let data_size_b = data_size.to_le_bytes();
    bin_vec.push(15u8);
    bin_vec.extend_from_slice(&data_size_b);
    bin_vec.extend_from_slice(&data_b);
}

if WANTED_EVENT_HEADER_INFO[16]{
    bin_vec.push(16u8);
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
        Err(e) => print!("bin_write failed in bin_event_header: {:?}", e)
    };

}
}