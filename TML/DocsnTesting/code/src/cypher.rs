use windows::Win32::System::Diagnostics::Etw::*;

#[inline]
pub fn get_property_name_index(name: *const u16) -> PropertyNameIndex {
    let _a = unsafe{*name};
    let b = unsafe{*name.add(1)};
    let c = unsafe{*name.add(2)};
    //a points to the first u16 character, b 16 bits forward in memory etc.
    //accounts for as many characters as necessary to distinguish between the different possible property names
    //important to optimize because this code is going to be ran most often of all.
    //
    //The returned enum can be presented as a u8 which represents an index in pref::WANTED_EVENTS_PROPS
    match (b, c) {
        (110, 105) => PropertyNameIndex::UniqueProcessKey,  // Xni
        (114, 111) => PropertyNameIndex::ProcessId,         // Xro
        (97, 114) => PropertyNameIndex::ParentId,           // Xar
        (101, 115) => PropertyNameIndex::SessionId,         // Xes
        (120, 105) => PropertyNameIndex::ExitStatus,        // Xxi
        (105, 114) => PropertyNameIndex::DirectoryTableBase,// Xir
        (108, 97) => PropertyNameIndex::Flags,              // Xla
        (115, 101) => PropertyNameIndex::UserSID,           // Xse
        (109, 97) => PropertyNameIndex::ImageFileName,      // Xma
        (111, 109) => PropertyNameIndex::CommandLine,       // Xom
        (97, 99) => PropertyNameIndex::PackageFullName,     // Xac
        (112, 112) => PropertyNameIndex::ApplicationId,     // Xpp
        _ => PropertyNameIndex::Unknown,
    }
}
#[repr(u8)]
pub enum PropertyNameIndex {
    UniqueProcessKey = 0,
    ProcessId = 1,
    ParentId = 2,
    SessionId = 3,
    ExitStatus = 4,
    DirectoryTableBase = 5,
    Flags = 6,
    UserSID = 7,
    ImageFileName = 8,
    CommandLine = 9,
    PackageFullName = 10,
    ApplicationId = 11,
    Unknown = 12,
}



// Property type index equivalents:
// U8         = 0
// U16        = 1
// U32        = 2
// U64        = 3
// I8         = 4
// I16        = 5
// I32        = 6
// I64        = 7
// F32        = 8
// F64        = 9
// Bool       = 10
// Utf16      = 11
// Ansi       = 12
// Guid       = 13
// FileTime   = 14
// SystemTime = 15
// Hex        = 16
// Binary     = 17
//Similarily to the function above, this function takes the necessary data to understand the type of the property it's
//from and converts it into a u8 which represents an index in the enum cypher::PropertyValue, as also shown above.
//Written mostly by AI btw, since I don't want to learn type the type philosophy of TDH myself
pub fn get_property_type_index(
    union_bytes: &[u8],
    flags: u32,
) -> u8 {
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

    let intype =
        if flags & PROPERTY_STRUCT != 0 {

            // structType:
            //
            // [ StructStartIndex:u16 ]
            // [ NumOfStructMembers:u16 ]
            // [ padding:u32 ]

            // A struct does not have an InType of its own.
            // Its members have to be decoded individually.
            return 17;

        } else {

            // customSchemaType / nonStructType:
            //
            // [ InType:u16 ]
            // [ OutType:u16 ]
            // [ offset:u32 ]

            u16::from_le_bytes([
                union_bytes[0],
                union_bytes[1],
            ])
        };

    // ---------------------------------------------------------
    // 2. Decode according to InType
    // ---------------------------------------------------------

    match intype {

        x if x == TDH_INTYPE_UINT8.0 as u16 => 0,

        x if x == TDH_INTYPE_UINT16.0 as u16 => 1,

        x if x == TDH_INTYPE_UINT32.0 as u16 => 2,

        x if x == TDH_INTYPE_UINT64.0 as u16 => 3,

        x if x == TDH_INTYPE_INT8.0 as u16 => 4,

        x if x == TDH_INTYPE_INT16.0 as u16 => 5,

        x if x == TDH_INTYPE_INT32.0 as u16 => 6,

        x if x == TDH_INTYPE_INT64.0 as u16 => 7,

        x if x == TDH_INTYPE_FLOAT.0 as u16 => 8,

        x if x == TDH_INTYPE_DOUBLE.0 as u16 => 9,

        x if x == TDH_INTYPE_UNICODESTRING.0 as u16 => 11,

        x if x == TDH_INTYPE_ANSISTRING.0 as u16 => 12,

        x if x == TDH_INTYPE_GUID.0 as u16 => 13,

        _ => 17,
    }
}



#[derive(Debug)]
pub enum PropertyValue {
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
    Guid([u8; 16]),
    FileTime(u64),
    SystemTime(u64),
    Hex(Vec<u8>),
    Binary(Vec<u8>),
    Null(u8)
}
pub fn decode_property_value(type_index: u8, bytes: &[u8]) -> PropertyValue {
    match type_index {
0 => PropertyValue::U8(bytes[0]),

1 => PropertyValue::U16(
    u16::from_le_bytes([
        bytes[0],
        bytes[1],
    ])
),

2 => PropertyValue::U32(
    u32::from_le_bytes([
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
    ])
),

3 => PropertyValue::U64(
    u64::from_le_bytes([
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
    ])
),

4 => PropertyValue::I8(bytes[0] as i8),

5 => PropertyValue::I16(
    i16::from_le_bytes([
        bytes[0],
        bytes[1],
    ])
),

6 => PropertyValue::I32(
    i32::from_le_bytes([
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
    ])
),

7 => PropertyValue::I64(
    i64::from_le_bytes([
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
    ])
),

8 => PropertyValue::F32(
    f32::from_le_bytes([
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
    ])
),

9 => PropertyValue::F64(
    f64::from_le_bytes([
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
    ])
),

10 => PropertyValue::Bool(bytes[0] != 0),

11 => todo!("UTF-16"),

12 => todo!("ANSI"),

13 => todo!("GUID"),

14 => PropertyValue::FileTime(
    u64::from_le_bytes([
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
    ])
),

15 => todo!("SystemTime"),

16 => PropertyValue::Hex(bytes.to_vec()),

17 => PropertyValue::Binary(bytes.to_vec()),

_ => PropertyValue::Null(0)
    }
}


//The header values' index does not have to be evaluated as it is simply a variable in a struct rather than an array of mixed up properties
//Atleast I think they're mixed up, never seen it with my own eyes but GPT says so, so idk
#[derive(Debug)]
pub enum HeaderValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    Bool(bool),
    String(String),
    Guid(windows::core::GUID),
    Bytes(Vec<u8>),
    Null(u8)
}
#[inline]
pub fn decode_header_value(field: u8, bytes: &[u8]) -> HeaderValue {
    match field {
        0 => HeaderValue::U16(u16::from_le_bytes([
            bytes[0], bytes[1],
        ])),

        1 => HeaderValue::U16(u16::from_le_bytes([
            bytes[0], bytes[1],
        ])),

        2 => HeaderValue::U16(u16::from_le_bytes([
            bytes[0], bytes[1],
        ])),

        3 => HeaderValue::U16(u16::from_le_bytes([
            bytes[0], bytes[1],
        ])),

        4 => HeaderValue::U32(u32::from_le_bytes([
            bytes[0], bytes[1],
            bytes[2], bytes[3],
        ])),

        5 => HeaderValue::U32(u32::from_le_bytes([
            bytes[0], bytes[1],
            bytes[2], bytes[3],
        ])),

        6 => HeaderValue::I64(i64::from_le_bytes([
            bytes[0], bytes[1],
            bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
        ])),

        7 => HeaderValue::Guid(windows::core::GUID {
            data1: u32::from_le_bytes([
                bytes[0], bytes[1],
                bytes[2], bytes[3],
            ]),
            data2: u16::from_le_bytes([
                bytes[4], bytes[5],
            ]),
            data3: u16::from_le_bytes([
                bytes[6], bytes[7],
            ]),
            data4: ([
                bytes[8], bytes[9],
                bytes[10], bytes[11],
                bytes[12], bytes[13],
                bytes[14], bytes[15],
            ])
        }),

        8 => HeaderValue::U16(u16::from_le_bytes([
            bytes[0], bytes[1],
        ])),

        9 => HeaderValue::U8(bytes[0]),
        10 => HeaderValue::U8(bytes[0]),
        11 => HeaderValue::U8(bytes[0]),
        12 => HeaderValue::U8(bytes[0]),

        13 => HeaderValue::U16(u16::from_le_bytes([
            bytes[0], bytes[1],
        ])),

        14 => HeaderValue::U64(u64::from_le_bytes([
            bytes[0], bytes[1],
            bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
        ])),

        15 => HeaderValue::Guid(windows::core::GUID {
            data1: u32::from_le_bytes([
                bytes[0], bytes[1],
                bytes[2], bytes[3],
            ]),
            data2: u16::from_le_bytes([
                bytes[4], bytes[5],
            ]),
            data3: u16::from_le_bytes([
                bytes[6], bytes[7],
            ]),
            data4: ([
                bytes[8], bytes[9],
                bytes[10], bytes[11],
                bytes[12], bytes[13],
                bytes[14], bytes[15],
            ])
        }),

        16 => HeaderValue::U64(u64::from_le_bytes([
            bytes[0], bytes[1],
            bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
        ])),

        _ => HeaderValue::Null(0),
    }
}