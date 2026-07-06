use crate::ffi_CAliases::*;
use crate::ffi_Structs::*;

#[link(name = "advapi32")]
//This line is as per the # at the start an "attribute".
//An attribute is a piece of metadata which changes how the code that follows it is compiled.
//This attribute in particular is saying to link against library "advapi32" as it is compiled.
extern "system"
//This line is saying to make all function calls inside of its' scope according to a certain ABI, in this case the Windows Systems' ABI.
//The actual logic inside of the functions reside inside of the earlier linked library "advapi32", or rather the part of a library that I need.. not too sure yet.
//So when the function singatures inside of this script are called, the wonders of linking will have replaced those calls with calls to the corresponding functions in the library.
//The "unsafe" is there mainly because Rust can't tell if the functions that are to be defined in this scope are going to match those in the library.
//Or in other words, Rust can't tell right now if the linker will return an error later.
//
//Read all of the above with a grain of salt, I wish writing as if I were omnipotent wasn't engraved into my mind by school.
{
    pub fn OpenTraceW(
        logfile: *mut EVENT_TRACE_LOGFILEW,
    ) -> TRACEHANDLE;

    pub fn ProcessTrace(
        handle_array: *const TRACEHANDLE,
        handle_count: ULONG,
        start_time: *mut FILETIME,
        end_time: *mut FILETIME,
    ) -> ULONG;
}
