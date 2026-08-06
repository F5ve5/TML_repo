use windows::Win32::System::Diagnostics::Etw::EVENT_PROPERTY_INFO;

pub fn r_to_utf16_string(s: &str) -> Vec<u16> {
    s.encode_utf16()
    .chain(std::iter::once(0))
    .collect()
}

pub unsafe fn utf16_to_r_string(s_ptr: *const u16) -> String {
    let mut len = 0;
    while *s_ptr.add(len) != 0 {
        len += 1;
    }

    let slice = std::slice::from_raw_parts(s_ptr, len);
    String::from_utf16_lossy(slice)
}

pub fn decode_nonstruct(prop: &EVENT_PROPERTY_INFO, buf: &[u8]) {
    let out_type = unsafe { prop.Anonymous1.nonStructType.OutType };

    match out_type {
        TDH_OUTTYPE_NULL => {
            println!("NULL");
        }

        TDH_OUTTYPE_STRING => {
            // UTF-16 LE string
            let wide = unsafe {
                std::slice::from_raw_parts(
                    buf.as_ptr() as *const u16,
                    buf.len() / 2,
                )
            };
            let s = String::from_utf16_lossy(wide);
            println!("String: {}", s);
        }

        TDH_OUTTYPE_ANSISTRING => {
            // UTF-8 / ANSI
            let s = String::from_utf8_lossy(buf);
            println!("AnsiString: {}", s);
        }

        TDH_OUTTYPE_INT8 => {
            println!("i8: {}", buf[0] as i8);
        }

        TDH_OUTTYPE_UINT8 => {
            println!("u8: {}", buf[0]);
        }

        TDH_OUTTYPE_INT16 => {
            let v = i16::from_le_bytes(buf.try_into().unwrap());
            println!("i16: {}", v);
        }

        TDH_OUTTYPE_UINT16 => {
            let v = u16::from_le_bytes(buf.try_into().unwrap());
            println!("u16: {}", v);
        }

        TDH_OUTTYPE_INT32 => {
            let v = i32::from_le_bytes(buf.try_into().unwrap());
            println!("i32: {}", v);
        }

        TDH_OUTTYPE_UINT32 => {
            let v = u32::from_le_bytes(buf.try_into().unwrap());
            println!("u32: {}", v);
        }

        TDH_OUTTYPE_INT64 => {
            let v = i64::from_le_bytes(buf.try_into().unwrap());
            println!("i64: {}", v);
        }

        TDH_OUTTYPE_UINT64 => {
            let v = u64::from_le_bytes(buf.try_into().unwrap());
            println!("u64: {}", v);
        }

        TDH_OUTTYPE_HEXINT32 => {
            let v = u32::from_le_bytes(buf.try_into().unwrap());
            println!("Hex32: 0x{:X}", v);
        }

        TDH_OUTTYPE_HEXINT64 => {
            let v = u64::from_le_bytes(buf.try_into().unwrap());
            println!("Hex64: 0x{:X}", v);
        }

        TDH_OUTTYPE_GUID => {
            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            struct GUID {
                Data1: u32,
                Data2: u16,
                Data3: u16,
                Data4: [u8; 8],
            }

            let guid = unsafe { *(buf.as_ptr() as *const GUID) };
            println!("GUID: {:?}", guid);
        }

        TDH_OUTTYPE_FILETIME => {
            let ft = u64::from_le_bytes(buf.try_into().unwrap());
            println!("FILETIME raw: {}", ft);
        }

        TDH_OUTTYPE_SID => {
            println!("SID (raw bytes): {:02X?}", buf);
        }

        TDH_OUTTYPE_BINARY => {
            println!("Binary: {:02X?}", buf);
        }

        _ => {
            println!("Unsupported OutType: {}", out_type);
        }
    }
}