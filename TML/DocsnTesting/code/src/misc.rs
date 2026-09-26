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

#[inline]
pub fn round_filetime_to_nearest_second(ft: &u64) -> u64 {
    const TICKS_PER_SECOND: u64 = 10_000_000;

    let seconds = ft / TICKS_PER_SECOND;
    let remainder = ft % TICKS_PER_SECOND;

    //If remainder >= 0.5 seconds, round up
    if remainder >= TICKS_PER_SECOND / 2 {
        (seconds + 1) * TICKS_PER_SECOND
    } else {
        seconds * TICKS_PER_SECOND
    }
}