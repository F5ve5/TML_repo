pub fn r_to_utf16_string(s: &str) -> Vec<u16> {
    s.encode_utf16()
    .chain(std::iter::once(0))
    .collect()
}

pub fn utf16_to_r_string(s_ptr: *const u16) -> String {
    let mut len = 0;
    while unsafe{*s_ptr.add(len) != 0}{
        len += 1;
    }

    let slice = unsafe{std::slice::from_raw_parts(s_ptr, len)};
    String::from_utf16_lossy(slice)
}

#[derive(Debug)]
pub enum EnumTW {
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
}

#[derive(Debug)]
pub enum EventHeaderField {
    Size,                       // u16
    HeaderType,                 // u16
    Flags,                      // u16
    EventProperty,              // u16
    ThreadId,                   // u32
    ProcessId,                  // u32
    TimeStamp,                  // i64
    ProviderId,                 // GUID
    EventId,                    // EventDescriptor.Id (u16)
    Version,                    // EventDescriptor.Version (u8)
    Channel,                    // EventDescriptor.Channel (u8)
    Level,                      // EventDescriptor.Level (u8)
    Opcode,                     // EventDescriptor.Opcode (u8)
    Task,                       // EventDescriptor.Task (u16)
    Keyword,                    // EventDescriptor.Keyword (u64)
    ActivityId,                 // GUID

    /// The union field at offset 16.
    /// Its interpretation depends on Flags:
    /// - If Flags & EVENT_HEADER_FLAG_32_BIT_HEADER != 0 → 32-bit timestamp
    /// - If Flags & EVENT_HEADER_FLAG_64_BIT_HEADER != 0 → 64-bit timestamp
    /// - If Flags & EVENT_HEADER_FLAG_CLASSIC_HEADER != 0 → classic header time
    /// - If Flags & EVENT_HEADER_FLAG_PROCESSOR_TIME != 0 → CPU time
    TimeUnion,
}

#[derive(Debug)]
pub enum PropertyType {
    U8,
    U16,
    U32,
    U64,
    I32,
    I64,
    Bool,
    String,
    Guid,
    Bytes,
}