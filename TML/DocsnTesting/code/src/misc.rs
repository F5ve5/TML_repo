pub fn r_to_cstring(s: &str) -> Vec<u16> {
    s.encode_utf16()
    .chain(std::iter::once(0))
    .collect()
}