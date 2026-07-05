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

