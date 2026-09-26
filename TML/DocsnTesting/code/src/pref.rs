// EVENT_HEADER
// 0 Size
// 1 HeaderType u16
// 2 Flags u16
// 3 EventProperty u16
// 4 ThreadId u32
// 5 ProcessId u32
// 6 TimeStamp i64
// 7 ProviderId GUID
// 8 EventDescriptor.Id u16
// 9 EventDescriptor.Version u8
// 10 EventDescriptor.Channel u8
// 11 EventDescriptor.Level u8
// 12 EventDescriptor.Opcode u8
// 13 EventDescriptor.Task u16
// 14 EventDescriptor.Keyword u64
// 15 ActivityId GUID
// 16 Time A union of 8 bytes which are interpreted differently according to the earlier Flags, therefore 2 has to be true for this one to be
pub const WANTED_EVENT_HEADER_INFO: [bool; 17] = [
    false,
    false,
    true,
    false,
    false,
    false,
    true,
    true,
    true,
    true,
    false,
    false,
    true,
    false,
    false,
    true,
    true,
];

// Property indexes and their typical type
// 0 UniqueProcessKey Bytes
// 1 ProcessId U32
// 2 ParentId U32
// 3 SessionId U32
// 4 ExitStatus I32
// 5 DirectoryTableBase Bytes
// 6 Flags U32
// 7 UserSID Byees
// 8 ImageFileName Bytes
// 9 CommandLine String
// 10 PackageFullName String
// 11 ApplicationId String
// 12 Unknown 0
pub const WANTED_PROPS: [bool; 13] = [
    false,
    true,
    true,
    true,
    false,
    false,
    false,
    false,
    true,
    true,
    false,
    false,
    false
];

pub const BINARY_ENCODING: bool = true;