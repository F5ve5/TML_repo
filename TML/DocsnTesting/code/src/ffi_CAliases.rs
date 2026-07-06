//I'll just leave this part out of the log here:
//
//Another script that I'll make is one that I'll call "FFI_Types.rs" and in there I'll write something called type aliases. It's quite simple and quite useful but
//I've never seen or heard of it before. How it works is that it allows you to assign existing variable types new aliases, in other words you can say that DWORD is
//just another way of saying u32. What makes this super useful is the fact that I can now just write the exact Windows Function arguments instead of looking fot
//their Rust equivalents.
pub type UCHAR = u8;
pub type USHORT = u16;
pub type ULONG = u32;
pub type ULONGLONG = u64;

pub type CHAR = i8;
pub type SHORT = i16;
pub type LONG = i32;
pub type LONGLONG = i64;

pub type DWORD = u32;
pub type WORD = u16;
pub type BYTE = u8;

pub type BOOL = i32;

pub type FLOAT = f32;
pub type DOUBLE = f64;

pub type WCHAR = u16;

pub type TRACEHANDLE = u64;