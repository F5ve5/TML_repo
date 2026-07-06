use crate::ffi_CAliases::*;
// A
// |
//This line makes it so that another Rust script written in the same directory can be refered to through this one,
//as long as "mod fileX;" is in main, for example "mod ffi_CAliases;". This is the only place I write this explanation so
//how very lucky of you to find it.

//This line makes it so that the structs are laid out in memory as if they were written in C
// |
// V
#[repr(C)]
pub struct EVENT_RECORD {
    pub EventHeader: EVENT_HEADER,
    pub BufferContext: ETW_BUFFER_CONTEXT,
    pub ExtendedDataCount: u16,
    pub UserDataLength: u16,
    pub ExtendedData: *mut EVENT_HEADER_EXTENDED_DATA_ITEM,
    pub UserData: *mut std::ffi::c_void,
    pub UserContext: *mut std::ffi::c_void,
}

#[repr(C)]
pub struct EVENT_TRACE_LOGFILEW {
    pub LogFileName: *mut WCHAR,
    pub LoggerName: *mut WCHAR,

    pub CurrentTime: LONGLONG,
    pub BuffersRead: ULONG,

    pub ProcessTraceMode: ULONG,

    pub CurrentEvent: EVENT_TRACE,

    pub LogfileHeader: TRACE_LOGFILE_HEADER,

    pub BufferCallback: Option<
        unsafe extern "system" fn(*mut EVENT_TRACE_LOGFILEW) -> ULONG
    >,

    pub BufferSize: ULONG,
    pub Filled: ULONG,

    pub EventsLost: ULONG,

    pub EventRecordCallback: Option<
        unsafe extern "system" fn(*mut EVENT_RECORD)
    >,

    pub IsKernelTrace: ULONG,

    pub Context: *mut core::ffi::c_void,
}

/////

#[repr(C)]
pub struct EVENT_HEADER {
    pub Size: USHORT,
    pub HeaderType: USHORT,
    pub Flags: USHORT,
    pub EventProperty: USHORT,

    pub ThreadId: ULONG,
    pub ProcessId: ULONG,

    pub TimeStamp: LONGLONG,

    pub ProviderId: GUID,

    pub EventDescriptor: EVENT_DESCRIPTOR,

    pub ProcessorTime: ULONGLONG,

    pub ActivityId: GUID,
}

#[repr(C)]
pub struct ETW_BUFFER_CONTEXT {
    pub ProcessorNumber: BYTE,
    pub Alignment: BYTE,
    pub LoggerId: USHORT,
}

#[repr(C)]
pub struct EVENT_HEADER_EXTENDED_DATA_ITEM {
    pub Reserved1: USHORT,
    pub ExtType: USHORT,
    pub Linkage: USHORT,
    pub DataSize: USHORT,
    pub DataPtr: ULONGLONG,
}

/////

#[repr(C)]
pub struct GUID {
    pub Data1: ULONG,
    pub Data2: WORD,
    pub Data3: WORD,
    pub Data4: [BYTE; 8],
}

#[repr(C)]
pub struct EVENT_DESCRIPTOR {
    pub Id: USHORT,
    pub Version: BYTE,
    pub Channel: BYTE,
    pub Level: BYTE,
    pub Opcode: BYTE,
    pub Task: USHORT,
    pub Keyword: ULONGLONG,
}

//The above structs allow support for functions