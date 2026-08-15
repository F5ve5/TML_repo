use windows::Win32::System::Diagnostics::Etw::*;

#[derive(Debug)]
pub enum DecodedValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Utf16(String),
    Ansi(String),
    Guid(u128),
    FileTime(u64),
    SystemTime(u64),
    Hex(Vec<u8>),
    Binary(Vec<u8>),
}

pub fn decode_serialized_property(
    union_bytes: &[u8],
    flags: u32,
    buf: &[u8],
) -> DecodedValue {

    assert_eq!(union_bytes.len(), 8);

    // EVENT_PROPERTY_INFO.Flags
    const PROPERTY_STRUCT: u32 = 0x1;
    const PROPERTY_PARAM_LENGTH: u32 = 0x2;
    const PROPERTY_PARAM_COUNT: u32 = 0x4;
    const PROPERTY_WBEM_XML_FRAGMENT: u32 = 0x8;
    const PROPERTY_PARAM_FIXED_LENGTH: u32 = 0x10;
    const PROPERTY_PARAM_FIXED_COUNT: u32 = 0x20;
    const PROPERTY_HAS_CUSTOM_SCHEMA: u32 = 0x40;
    const PROPERTY_HAS_TAGS: u32 = 0x80;

    // ---------------------------------------------------------
    // 1. Determine which member of EVENT_PROPERTY_INFO_0
    //    the serialized 8 bytes represent.
    // ---------------------------------------------------------

    let (intype, out_type) =
        if flags & PROPERTY_STRUCT != 0 {

            // structType:
            //
            // [ StructStartIndex:u16 ]
            // [ NumOfStructMembers:u16 ]
            // [ padding:u32 ]

            // A struct does not have an InType/OutType of its own.
            // Its members have to be decoded individually.
            return DecodedValue::Binary(buf.to_vec());

        } else if flags & PROPERTY_HAS_CUSTOM_SCHEMA != 0 {

            // customSchemaType:
            //
            // [ InType:u16 ]
            // [ OutType:u16 ]
            // [ CustomSchemaOffset:u32 ]

            let intype = u16::from_le_bytes([
                union_bytes[0],
                union_bytes[1],
            ]);

            let out_type = u16::from_le_bytes([
                union_bytes[2],
                union_bytes[3],
            ]);

            (intype, out_type)

        } else {

            // nonStructType:
            //
            // [ InType:u16 ]
            // [ OutType:u16 ]
            // [ MapNameOffset:u32 ]

            let intype = u16::from_le_bytes([
                union_bytes[0],
                union_bytes[1],
            ]);

            let out_type = u16::from_le_bytes([
                union_bytes[2],
                union_bytes[3],
            ]);

            (intype, out_type)
        };

    // ---------------------------------------------------------
    // 2. Decode according to InType
    // ---------------------------------------------------------

    match intype {

        x if x == TDH_INTYPE_UINT8.0 as u16 => {
            DecodedValue::U8(buf[0])
        }

        x if x == TDH_INTYPE_UINT16.0 as u16 => {
            DecodedValue::U16(
                u16::from_le_bytes(
                    buf.try_into().unwrap()
                )
            )
        }

        x if x == TDH_INTYPE_UINT32.0 as u16 => {
            DecodedValue::U32(
                u32::from_le_bytes(
                    buf.try_into().unwrap()
                )
            )
        }

        x if x == TDH_INTYPE_UINT64.0 as u16 => {
            DecodedValue::U64(
                u64::from_le_bytes(
                    buf.try_into().unwrap()
                )
            )
        }

        x if x == TDH_INTYPE_INT8.0 as u16 => {
            DecodedValue::I8(buf[0] as i8)
        }

        x if x == TDH_INTYPE_INT16.0 as u16 => {
            DecodedValue::I16(
                i16::from_le_bytes(
                    buf.try_into().unwrap()
                )
            )
        }

        x if x == TDH_INTYPE_INT32.0 as u16 => {
            DecodedValue::I32(
                i32::from_le_bytes(
                    buf.try_into().unwrap()
                )
            )
        }

        x if x == TDH_INTYPE_INT64.0 as u16 => {
            DecodedValue::I64(
                i64::from_le_bytes(
                    buf.try_into().unwrap()
                )
            )
        }

        x if x == TDH_INTYPE_FLOAT.0 as u16 => {
            DecodedValue::F32(
                f32::from_le_bytes(
                    buf.try_into().unwrap()
                )
            )
        }

        x if x == TDH_INTYPE_DOUBLE.0 as u16 => {
            DecodedValue::F64(
                f64::from_le_bytes(
                    buf.try_into().unwrap()
                )
            )
        }

        x if x == TDH_INTYPE_UNICODESTRING.0 as u16 => {

            let mut utf16 = Vec::new();
            let mut i = 0;

            while i + 1 < buf.len() {

                let code_unit =
                    u16::from_le_bytes([
                        buf[i],
                        buf[i + 1],
                    ]);

                if code_unit == 0 {
                    break;
                }

                utf16.push(code_unit);
                i += 2;
            }

            DecodedValue::Utf16(
                String::from_utf16_lossy(&utf16)
            )
        }

        x if x == TDH_INTYPE_ANSISTRING.0 as u16 => {

            let end = buf
                .iter()
                .position(|&byte| byte == 0)
                .unwrap_or(buf.len());

            DecodedValue::Ansi(
                String::from_utf8_lossy(
                    &buf[..end]
                ).to_string()
            )
        }

        x if x == TDH_INTYPE_GUID.0 as u16 => {

            DecodedValue::Guid(
                u128::from_le_bytes(
                    buf.try_into().unwrap()
                )
            )
        }

        _ => {
            DecodedValue::Binary(
                buf.to_vec()
            )
        }
    }
}